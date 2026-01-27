//! Docker image build and pull operations
//!
//! This module provides functionality to build Docker images from the embedded
//! Dockerfile and pull images from registries with progress feedback.

use super::progress::ProgressReporter;
use super::{
    DOCKERFILE, DockerClient, DockerError, IMAGE_NAME_DOCKERHUB, IMAGE_NAME_GHCR, IMAGE_TAG_DEFAULT,
};
use bollard::image::{BuildImageOptions, BuilderVersion, CreateImageOptions};
use bollard::models::BuildInfoAux;
use bytes::Bytes;
use flate2::Compression;
use flate2::write::GzEncoder;
use futures_util::StreamExt;
use std::collections::VecDeque;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tar::Builder as TarBuilder;
use tracing::{debug, warn};

/// Default number of recent build log lines to capture for error context
const DEFAULT_BUILD_LOG_BUFFER_SIZE: usize = 20;

/// Default number of error lines to capture separately
const DEFAULT_ERROR_LOG_BUFFER_SIZE: usize = 10;

/// Read a log buffer size from env with bounds
fn read_log_buffer_size(var_name: &str, default: usize) -> usize {
    let Ok(value) = env::var(var_name) else {
        return default;
    };
    let Ok(parsed) = value.trim().parse::<usize>() else {
        return default;
    };
    parsed.clamp(5, 500)
}

/// Check if a line looks like an error message
fn is_error_line(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("error")
        || lower.contains("failed")
        || lower.contains("cannot")
        || lower.contains("unable to")
        || lower.contains("not found")
        || lower.contains("permission denied")
}

/// Check if an image exists locally
pub async fn image_exists(
    client: &DockerClient,
    image: &str,
    tag: &str,
) -> Result<bool, DockerError> {
    let full_name = format!("{image}:{tag}");
    debug!("Checking if image exists: {}", full_name);

    match client.inner().inspect_image(&full_name).await {
        Ok(_) => Ok(true),
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => Ok(false),
        Err(e) => Err(DockerError::from(e)),
    }
}

/// Build the opencode image from embedded Dockerfile
///
/// Shows real-time build progress with streaming output.
/// Returns the full image:tag string on success.
///
/// # Arguments
/// * `client` - Docker client
/// * `tag` - Image tag (defaults to IMAGE_TAG_DEFAULT)
/// * `progress` - Progress reporter for build feedback
/// * `no_cache` - If true, build without using Docker layer cache
pub async fn build_image(
    client: &DockerClient,
    tag: Option<&str>,
    progress: &mut ProgressReporter,
    no_cache: bool,
) -> Result<String, DockerError> {
    let tag = tag.unwrap_or(IMAGE_TAG_DEFAULT);
    let full_name = format!("{IMAGE_NAME_GHCR}:{tag}");
    debug!("Building image: {} (no_cache: {})", full_name, no_cache);

    // Create tar archive containing Dockerfile
    let context = create_build_context()
        .map_err(|e| DockerError::Build(format!("Failed to create build context: {e}")))?;

    // Set up build options
    // Explicitly use BuildKit builder to support cache mounts (--mount=type=cache)
    // BuildKit requires a unique session ID for each build
    let session_id = format!(
        "opencode-cloud-build-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let options = BuildImageOptions {
        t: full_name.clone(),
        dockerfile: "Dockerfile".to_string(),
        version: BuilderVersion::BuilderBuildKit,
        session: Some(session_id),
        rm: true,
        nocache: no_cache,
        ..Default::default()
    };

    // Create build body from context
    let body = Bytes::from(context);

    // Start build with streaming output
    let mut stream = client.inner().build_image(options, None, Some(body));

    // Add main build spinner (context prefix like "Building image" is set by caller)
    progress.add_spinner("build", "Initializing...");

    let mut maybe_image_id = None;
    let build_log_buffer_size = read_log_buffer_size(
        "OPENCODE_DOCKER_BUILD_LOG_TAIL",
        DEFAULT_BUILD_LOG_BUFFER_SIZE,
    );
    let error_log_buffer_size = read_log_buffer_size(
        "OPENCODE_DOCKER_BUILD_ERROR_TAIL",
        DEFAULT_ERROR_LOG_BUFFER_SIZE,
    );
    let mut recent_logs: VecDeque<String> = VecDeque::with_capacity(build_log_buffer_size);
    let mut error_logs: VecDeque<String> = VecDeque::with_capacity(error_log_buffer_size);

    while let Some(result) = stream.next().await {
        match result {
            Ok(info) => {
                // Handle stream output (build log messages)
                if let Some(stream_msg) = info.stream {
                    let msg = stream_msg.trim();
                    if !msg.is_empty() {
                        progress.update_spinner("build", msg);

                        // Capture recent log lines for error context
                        if recent_logs.len() >= build_log_buffer_size {
                            recent_logs.pop_front();
                        }
                        recent_logs.push_back(msg.to_string());

                        // Also capture error-like lines separately (they might scroll off)
                        if is_error_line(msg) {
                            if error_logs.len() >= error_log_buffer_size {
                                error_logs.pop_front();
                            }
                            error_logs.push_back(msg.to_string());
                        }

                        // Capture step information for better progress
                        if msg.starts_with("Step ") {
                            debug!("Build step: {}", msg);
                        }
                    }
                }

                // Handle error messages
                if let Some(error_msg) = info.error {
                    progress.abandon_all(&error_msg);
                    let context =
                        format_build_error_with_context(&error_msg, &recent_logs, &error_logs);
                    return Err(DockerError::Build(context));
                }

                // Capture the image ID from aux field
                if let Some(aux) = info.aux {
                    match aux {
                        BuildInfoAux::Default(image_id) => {
                            if let Some(id) = image_id.id {
                                maybe_image_id = Some(id);
                            }
                        }
                        BuildInfoAux::BuildKit(_) => {
                            // BuildKit responses are handled via stream messages
                        }
                    }
                }
            }
            Err(e) => {
                progress.abandon_all("Build failed");
                let error_str = e.to_string();

                // Check if this might be a BuildKit-related error
                let buildkit_hint = if error_str.contains("mount")
                    || error_str.contains("--mount")
                    || recent_logs
                        .iter()
                        .any(|log| log.contains("--mount") && log.contains("cache"))
                {
                    "\n\nNote: This Dockerfile uses BuildKit cache mounts (--mount=type=cache).\n\
                     The build is configured to use BuildKit, but the Docker daemon may not support it.\n\
                     Ensure BuildKit is enabled in Docker Desktop settings and the daemon is restarted."
                } else {
                    ""
                };

                let context = format!(
                    "{}{}",
                    format_build_error_with_context(&error_str, &recent_logs, &error_logs),
                    buildkit_hint
                );
                return Err(DockerError::Build(context));
            }
        }
    }

    let image_id = maybe_image_id.unwrap_or_else(|| "unknown".to_string());
    let finish_msg = format!("Build complete: {image_id}");
    progress.finish("build", &finish_msg);

    Ok(full_name)
}

/// Pull the opencode image from registry with automatic fallback
///
/// Tries GHCR first, falls back to Docker Hub on failure.
/// Returns the full image:tag string on success.
pub async fn pull_image(
    client: &DockerClient,
    tag: Option<&str>,
    progress: &mut ProgressReporter,
) -> Result<String, DockerError> {
    let tag = tag.unwrap_or(IMAGE_TAG_DEFAULT);

    // Try GHCR first
    debug!("Attempting to pull from GHCR: {}:{}", IMAGE_NAME_GHCR, tag);
    let ghcr_err = match pull_from_registry(client, IMAGE_NAME_GHCR, tag, progress).await {
        Ok(()) => {
            let full_name = format!("{IMAGE_NAME_GHCR}:{tag}");
            return Ok(full_name);
        }
        Err(e) => e,
    };

    warn!(
        "GHCR pull failed: {}. Trying Docker Hub fallback...",
        ghcr_err
    );

    // Try Docker Hub as fallback
    debug!(
        "Attempting to pull from Docker Hub: {}:{}",
        IMAGE_NAME_DOCKERHUB, tag
    );
    match pull_from_registry(client, IMAGE_NAME_DOCKERHUB, tag, progress).await {
        Ok(()) => {
            let full_name = format!("{IMAGE_NAME_DOCKERHUB}:{tag}");
            Ok(full_name)
        }
        Err(dockerhub_err) => Err(DockerError::Pull(format!(
            "Failed to pull from both registries. GHCR: {ghcr_err}. Docker Hub: {dockerhub_err}"
        ))),
    }
}

/// Maximum number of retry attempts for pull operations
const MAX_PULL_RETRIES: usize = 3;

/// Pull from a specific registry with retry logic
async fn pull_from_registry(
    client: &DockerClient,
    image: &str,
    tag: &str,
    progress: &mut ProgressReporter,
) -> Result<(), DockerError> {
    let full_name = format!("{image}:{tag}");

    // Manual retry loop since async closures can't capture mutable references
    let mut last_error = None;
    for attempt in 1..=MAX_PULL_RETRIES {
        debug!(
            "Pull attempt {}/{} for {}",
            attempt, MAX_PULL_RETRIES, full_name
        );

        match do_pull(client, image, tag, progress).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                warn!("Pull attempt {} failed: {}", attempt, e);
                last_error = Some(e);

                if attempt < MAX_PULL_RETRIES {
                    // Exponential backoff: 1s, 2s, 4s
                    let delay_ms = 1000 * (1 << (attempt - 1));
                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        DockerError::Pull(format!(
            "Pull failed for {full_name} after {MAX_PULL_RETRIES} attempts"
        ))
    }))
}

/// Perform the actual pull operation
async fn do_pull(
    client: &DockerClient,
    image: &str,
    tag: &str,
    progress: &mut ProgressReporter,
) -> Result<(), DockerError> {
    let full_name = format!("{image}:{tag}");

    let options = CreateImageOptions {
        from_image: image,
        tag,
        ..Default::default()
    };

    let mut stream = client.inner().create_image(Some(options), None, None);

    // Add main spinner for overall progress
    progress.add_spinner("pull", &format!("Pulling {full_name}..."));

    while let Some(result) = stream.next().await {
        match result {
            Ok(info) => {
                // Handle errors from the stream
                if let Some(error_msg) = info.error {
                    progress.abandon_all(&error_msg);
                    return Err(DockerError::Pull(error_msg));
                }

                // Handle layer progress
                if let Some(layer_id) = &info.id {
                    let status = info.status.as_deref().unwrap_or("");

                    match status {
                        "Already exists" => {
                            progress.finish(layer_id, "Already exists");
                        }
                        "Pull complete" => {
                            progress.finish(layer_id, "Pull complete");
                        }
                        "Downloading" | "Extracting" => {
                            if let Some(progress_detail) = &info.progress_detail {
                                let current = progress_detail.current.unwrap_or(0) as u64;
                                let total = progress_detail.total.unwrap_or(0) as u64;

                                if total > 0 {
                                    progress.update_layer(layer_id, current, total, status);
                                }
                            }
                        }
                        _ => {
                            // Other statuses (Waiting, Verifying, etc.)
                            progress.update_spinner(layer_id, status);
                        }
                    }
                } else if let Some(status) = &info.status {
                    // Overall status messages (no layer id)
                    progress.update_spinner("pull", status);
                }
            }
            Err(e) => {
                progress.abandon_all("Pull failed");
                return Err(DockerError::Pull(format!("Pull failed: {e}")));
            }
        }
    }

    progress.finish("pull", &format!("Pull complete: {full_name}"));
    Ok(())
}

/// Format a build error with recent log context for actionable debugging
fn format_build_error_with_context(
    error: &str,
    recent_logs: &VecDeque<String>,
    error_logs: &VecDeque<String>,
) -> String {
    let mut message = String::new();

    // Add main error message
    message.push_str(error);

    // Add captured error lines if they differ from recent logs
    // (these are error-like lines that may have scrolled off)
    if !error_logs.is_empty() {
        // Check if error_logs contains lines not in recent_logs
        let recent_set: std::collections::HashSet<_> = recent_logs.iter().collect();
        let unique_errors: Vec<_> = error_logs
            .iter()
            .filter(|line| !recent_set.contains(line))
            .collect();

        if !unique_errors.is_empty() {
            message.push_str("\n\nPotential errors detected during build:");
            for line in unique_errors {
                message.push_str("\n  ");
                message.push_str(line);
            }
        }
    }

    // Add recent log context if available
    if !recent_logs.is_empty() {
        message.push_str("\n\nRecent build output:");
        for line in recent_logs {
            message.push_str("\n  ");
            message.push_str(line);
        }
    } else {
        message.push_str("\n\nNo build output was received from the Docker daemon.");
        message.push_str("\nThis usually means the build failed before any logs were streamed.");
    }

    // Add actionable suggestions based on common error patterns
    let error_lower = error.to_lowercase();
    if error_lower.contains("network")
        || error_lower.contains("connection")
        || error_lower.contains("timeout")
    {
        message.push_str("\n\nSuggestion: Check your network connection and Docker's ability to reach the internet.");
    } else if error_lower.contains("disk")
        || error_lower.contains("space")
        || error_lower.contains("no space")
    {
        message.push_str("\n\nSuggestion: Free up disk space with 'docker system prune' or check available storage.");
    } else if error_lower.contains("permission") || error_lower.contains("denied") {
        message.push_str("\n\nSuggestion: Check Docker permissions. You may need to add your user to the 'docker' group.");
    }

    message
}

/// Create a gzipped tar archive containing the Dockerfile
fn create_build_context() -> Result<Vec<u8>, std::io::Error> {
    let mut archive_buffer = Vec::new();

    {
        let encoder = GzEncoder::new(&mut archive_buffer, Compression::default());
        let mut tar = TarBuilder::new(encoder);

        // Add Dockerfile to archive
        let dockerfile_bytes = DOCKERFILE.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_path("Dockerfile")?;
        header.set_size(dockerfile_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();

        tar.append(&header, dockerfile_bytes)?;
        tar.finish()?;

        // Finish gzip encoding
        let encoder = tar.into_inner()?;
        encoder.finish()?;
    }

    Ok(archive_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_build_context_succeeds() {
        let context = create_build_context().expect("should create context");
        assert!(!context.is_empty(), "context should not be empty");

        // Verify it's gzip-compressed (gzip magic bytes)
        assert_eq!(context[0], 0x1f, "should be gzip compressed");
        assert_eq!(context[1], 0x8b, "should be gzip compressed");
    }

    #[test]
    fn default_tag_is_latest() {
        assert_eq!(IMAGE_TAG_DEFAULT, "latest");
    }

    #[test]
    fn format_build_error_includes_recent_logs() {
        let mut logs = VecDeque::new();
        logs.push_back("Step 1/5 : FROM ubuntu:22.04".to_string());
        logs.push_back("Step 2/5 : RUN apt-get update".to_string());
        logs.push_back("E: Unable to fetch some archives".to_string());
        let error_logs = VecDeque::new();

        let result =
            format_build_error_with_context("Build failed: exit code 1", &logs, &error_logs);

        assert!(result.contains("Build failed: exit code 1"));
        assert!(result.contains("Recent build output:"));
        assert!(result.contains("Step 1/5"));
        assert!(result.contains("Unable to fetch"));
    }

    #[test]
    fn format_build_error_handles_empty_logs() {
        let logs = VecDeque::new();
        let error_logs = VecDeque::new();
        let result = format_build_error_with_context("Stream error", &logs, &error_logs);

        assert!(result.contains("Stream error"));
        assert!(!result.contains("Recent build output:"));
    }

    #[test]
    fn format_build_error_adds_network_suggestion() {
        let logs = VecDeque::new();
        let error_logs = VecDeque::new();
        let result = format_build_error_with_context("connection timeout", &logs, &error_logs);

        assert!(result.contains("Check your network connection"));
    }

    #[test]
    fn format_build_error_adds_disk_suggestion() {
        let logs = VecDeque::new();
        let error_logs = VecDeque::new();
        let result = format_build_error_with_context("no space left on device", &logs, &error_logs);

        assert!(result.contains("Free up disk space"));
    }

    #[test]
    fn format_build_error_shows_error_lines_separately() {
        let mut recent_logs = VecDeque::new();
        recent_logs.push_back("Compiling foo v1.0".to_string());
        recent_logs.push_back("Successfully installed bar".to_string());

        let mut error_logs = VecDeque::new();
        error_logs.push_back("error: failed to compile dust".to_string());
        error_logs.push_back("error: failed to compile glow".to_string());

        let result = format_build_error_with_context("Build failed", &recent_logs, &error_logs);

        assert!(result.contains("Potential errors detected during build:"));
        assert!(result.contains("failed to compile dust"));
        assert!(result.contains("failed to compile glow"));
    }

    #[test]
    fn is_error_line_detects_errors() {
        assert!(is_error_line("error: something failed"));
        assert!(is_error_line("Error: build failed"));
        assert!(is_error_line("Failed to install package"));
        assert!(is_error_line("cannot find module"));
        assert!(is_error_line("Unable to locate package"));
        assert!(!is_error_line("Compiling foo v1.0"));
        assert!(!is_error_line("Successfully installed"));
    }
}
