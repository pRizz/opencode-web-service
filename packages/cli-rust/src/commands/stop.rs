//! Stop command implementation
//!
//! Stops the opencode service with a graceful timeout.
//! Docker sends SIGTERM first, then SIGKILL if timeout expires.

use crate::output::{CommandSpinner, format_docker_error, show_docker_error};
use anyhow::{Result, anyhow};
use clap::Args;
use console::style;
use opencode_cloud_core::docker::{
    CONTAINER_NAME, DEFAULT_STOP_TIMEOUT_SECS, container_is_running, stop_service,
};
use std::time::Instant;

/// Arguments for the stop command
#[derive(Args, Default)]
pub struct StopArgs {
    /// Graceful shutdown timeout in seconds (default: 30)
    #[arg(long, short, default_value_t = DEFAULT_STOP_TIMEOUT_SECS)]
    pub timeout: i64,
}

/// Stop the opencode service
///
/// This command:
/// 1. Connects to Docker
/// 2. Checks if service is running (idempotent - exits 0 if already stopped)
/// 3. Stops the container with graceful timeout (default 30s)
pub async fn cmd_stop(args: &StopArgs, maybe_host: Option<&str>, quiet: bool) -> Result<()> {
    // Resolve Docker client (local or remote)
    let (client, host_name) = crate::resolve_docker_client(maybe_host).await?;

    // Verify connection
    client.verify_connection().await.map_err(|e| {
        let msg = format_docker_error(&e);
        anyhow!("{msg}")
    })?;

    // Check if already stopped (idempotent behavior)
    if !container_is_running(&client, CONTAINER_NAME).await? {
        if !quiet {
            let msg =
                crate::format_host_message(host_name.as_deref(), "Service is already stopped");
            println!("{}", style(msg).dim());
        }
        return Ok(());
    }

    // Create spinner
    let msg = crate::format_host_message(host_name.as_deref(), "Stopping service...");
    let spinner = CommandSpinner::new_maybe(&msg, quiet);
    spinner.update(&crate::format_host_message(
        host_name.as_deref(),
        &format!(
            "Stopping service ({}s graceful timeout, then force kill)...",
            args.timeout
        ),
    ));

    // Stop with graceful timeout, track how long it takes
    let start = Instant::now();
    match stop_service(&client, false, Some(args.timeout)).await {
        Ok(()) => {
            let elapsed = start.elapsed();
            let elapsed_secs = elapsed.as_secs();

            // If it took >= timeout, Docker likely had to force kill
            if elapsed_secs >= args.timeout as u64 {
                spinner.success(&crate::format_host_message(
                    host_name.as_deref(),
                    &format!(
                        "Service stopped (force killed after {}s timeout)",
                        args.timeout
                    ),
                ));
                if !quiet {
                    eprintln!(
                        "{}",
                        style("Note: Container did not stop gracefully within timeout.").dim()
                    );
                }
            } else {
                spinner.success(&crate::format_host_message(
                    host_name.as_deref(),
                    &format!("Service stopped ({elapsed_secs}s)"),
                ));
            }
        }
        Err(e) => {
            spinner.fail(&crate::format_host_message(
                host_name.as_deref(),
                "Failed to stop",
            ));
            show_docker_error(&e);
            return Err(e.into());
        }
    }

    Ok(())
}
