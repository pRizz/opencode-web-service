//! Setup command implementation
//!
//! Runs the interactive setup wizard.

use anyhow::{Result, bail};
use clap::Args;
use console::style;
use dialoguer::Confirm;
use opencode_cloud_core::{load_config, save_config};

use crate::commands::cmd_start;
use crate::wizard::run_wizard;

/// Arguments for the setup command
#[derive(Args)]
pub struct SetupArgs {
    /// Skip wizard if auth credentials are already configured
    #[arg(long, short)]
    pub yes: bool,
}

/// Run the setup command
pub async fn cmd_setup(args: &SetupArgs, quiet: bool) -> Result<()> {
    // Load existing config (or create default)
    let existing_config = load_config().ok();

    // Handle --yes flag for non-interactive mode
    if args.yes {
        if let Some(ref config) = existing_config {
            if config.has_required_auth() {
                if !quiet {
                    println!("{}", style("Configuration already set").green());
                }
                return Ok(());
            }
        }

        bail!(
            "Non-interactive mode requires auth credentials to be pre-set.\n\n\
            Use:\n  \
            occ config set username <user>\n  \
            occ config set password"
        );
    }

    // Run the wizard
    let new_config = run_wizard(existing_config.as_ref()).await?;

    // Save the config
    save_config(&new_config)?;

    if !quiet {
        println!();
        println!(
            "{} Configuration saved successfully!",
            style("Success:").green().bold()
        );
        println!();

        // Offer to start the service
        let start_now = Confirm::new()
            .with_prompt("Start opencode-cloud now?")
            .default(true)
            .interact()
            .unwrap_or(false);

        if start_now {
            println!();
            // Create minimal start args
            let start_args = crate::commands::StartArgs {
                port: Some(new_config.opencode_web_port),
                open: false,
                no_daemon: false,
                cached_rebuild: false,
                full_rebuild: false,
            };
            cmd_start(&start_args, quiet, 0).await?;
        }
    }

    Ok(())
}
