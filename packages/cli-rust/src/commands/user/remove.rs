//! User remove subcommand
//!
//! Removes a user from the container.

use anyhow::{Result, bail};
use clap::Args;
use console::style;
use dialoguer::Confirm;
use opencode_cloud_core::docker::{CONTAINER_NAME, DockerClient, delete_user, user_exists};
use opencode_cloud_core::{load_config, save_config};

/// Arguments for the user remove command
#[derive(Args)]
pub struct UserRemoveArgs {
    /// Username to remove
    pub username: String,

    /// Skip confirmation prompt
    #[arg(long, short)]
    pub force: bool,
}

/// Remove a user from the container
pub async fn cmd_user_remove(args: &UserRemoveArgs, quiet: bool, _verbose: u8) -> Result<()> {
    let client = DockerClient::new()?;
    let username = &args.username;

    // Check if user exists
    if !user_exists(&client, CONTAINER_NAME, username).await? {
        bail!("User '{}' does not exist in the container", username);
    }

    // Load config to check users count
    let mut config = load_config()?;

    // Check if this is the last tracked user
    let is_last_user = config.users.len() == 1 && config.users.contains(username);
    if is_last_user && !args.force {
        bail!(
            "Cannot remove last user. Add another user first or use --force.\n\n\
            To add a new user:\n  \
            occ user add <username>\n\n\
            To force removal:\n  \
            occ user remove {} --force",
            username
        );
    }

    // Confirm removal
    if !args.force {
        let confirm = Confirm::new()
            .with_prompt(format!("Remove user '{}'?", username))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirm {
            if !quiet {
                println!("Cancelled.");
            }
            return Ok(());
        }
    }

    // Delete the user
    delete_user(&client, CONTAINER_NAME, username).await?;

    // Update config - remove username from users array
    config.users.retain(|u| u != username);
    save_config(&config)?;

    // Display success
    if !quiet {
        println!(
            "{} User '{}' removed successfully",
            style("Success:").green().bold(),
            username
        );
    }

    Ok(())
}
