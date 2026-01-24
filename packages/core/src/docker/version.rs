//! Docker image version detection
//!
//! Reads version information from Docker image labels.

use super::{DockerClient, DockerError};

/// Version label key in Docker image
pub const VERSION_LABEL: &str = "org.opencode-cloud.version";

/// Get version from image label
///
/// Returns None if image doesn't exist or has no version label.
/// Version label is set during automated builds; local builds have "dev".
pub async fn get_image_version(
    client: &DockerClient,
    image_name: &str,
) -> Result<Option<String>, DockerError> {
    let inspect = match client.inner().inspect_image(image_name).await {
        Ok(info) => info,
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => {
            return Ok(None);
        }
        Err(e) => {
            return Err(DockerError::Connection(format!(
                "Failed to inspect image: {e}"
            )));
        }
    };

    // Extract version from labels
    let version = inspect
        .config
        .and_then(|c| c.labels)
        .and_then(|labels| labels.get(VERSION_LABEL).cloned());

    Ok(version)
}

/// CLI version from Cargo.toml
pub fn get_cli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Compare versions and determine if they match
///
/// Returns true if versions are compatible (same or dev build).
/// Returns false if versions differ and user should be prompted.
pub fn versions_compatible(cli_version: &str, image_version: Option<&str>) -> bool {
    match image_version {
        None => true,        // No version label = local build, assume compatible
        Some("dev") => true, // Dev build, assume compatible
        Some(img_ver) => cli_version == img_ver,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_versions_compatible_none() {
        assert!(versions_compatible("1.0.8", None));
    }

    #[test]
    fn test_versions_compatible_dev() {
        assert!(versions_compatible("1.0.8", Some("dev")));
    }

    #[test]
    fn test_versions_compatible_same() {
        assert!(versions_compatible("1.0.8", Some("1.0.8")));
    }

    #[test]
    fn test_versions_compatible_different() {
        assert!(!versions_compatible("1.0.8", Some("1.0.7")));
    }

    #[test]
    fn test_get_cli_version_format() {
        let version = get_cli_version();
        // Should be semver format
        assert!(version.contains('.'));
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_version_label_constant() {
        assert_eq!(VERSION_LABEL, "org.opencode-cloud.version");
    }
}
