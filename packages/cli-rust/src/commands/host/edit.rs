//! occ host edit - Edit host configuration

use anyhow::Result;
use clap::Args;
use console::style;
use opencode_cloud_core::{load_hosts, save_hosts};

/// Arguments for host edit command
#[derive(Args)]
pub struct HostEditArgs {
    /// Name of the host to edit
    pub name: String,

    /// New hostname
    #[arg(long)]
    pub hostname: Option<String>,

    /// New SSH username
    #[arg(short, long)]
    pub user: Option<String>,

    /// New SSH port
    #[arg(short, long)]
    pub port: Option<u16>,

    /// New identity file path
    #[arg(short, long)]
    pub identity_file: Option<String>,

    /// New jump host (use empty string to clear)
    #[arg(short = 'J', long)]
    pub jump_host: Option<String>,

    /// Add a group
    #[arg(long)]
    pub add_group: Vec<String>,

    /// Remove a group
    #[arg(long)]
    pub remove_group: Vec<String>,

    /// New description (use empty string to clear)
    #[arg(short, long)]
    pub description: Option<String>,
}

pub async fn cmd_host_edit(args: &HostEditArgs, quiet: bool, _verbose: u8) -> Result<()> {
    let mut hosts = load_hosts()?;

    let config = hosts
        .get_host_mut(&args.name)
        .ok_or_else(|| anyhow::anyhow!("Host '{}' not found.", args.name))?;

    let mut changed = false;

    // Apply changes
    if let Some(hostname) = &args.hostname {
        config.hostname = hostname.clone();
        changed = true;
    }

    if let Some(user) = &args.user {
        config.user = user.clone();
        changed = true;
    }

    if let Some(port) = args.port {
        config.port = Some(port);
        changed = true;
    }

    if let Some(key) = &args.identity_file {
        config.identity_file = if key.is_empty() {
            None
        } else {
            Some(key.clone())
        };
        changed = true;
    }

    if let Some(jump) = &args.jump_host {
        config.jump_host = if jump.is_empty() {
            None
        } else {
            Some(jump.clone())
        };
        changed = true;
    }

    for group in &args.add_group {
        if !config.groups.contains(group) {
            config.groups.push(group.clone());
            changed = true;
        }
    }

    for group in &args.remove_group {
        if let Some(pos) = config.groups.iter().position(|g| g == group) {
            config.groups.remove(pos);
            changed = true;
        }
    }

    if let Some(desc) = &args.description {
        config.description = if desc.is_empty() {
            None
        } else {
            Some(desc.clone())
        };
        changed = true;
    }

    if !changed {
        if !quiet {
            println!("No changes specified. Use --help to see available options.");
        }
        return Ok(());
    }

    // Save
    save_hosts(&hosts)?;

    if !quiet {
        println!(
            "{} Host '{}' updated.",
            style("Updated:").green(),
            style(&args.name).cyan()
        );
        println!(
            "  {} {}",
            style("View changes:").dim(),
            style(format!("occ host show {}", args.name)).yellow()
        );
    }

    Ok(())
}
