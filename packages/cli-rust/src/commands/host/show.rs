//! occ host show - Show details for a host

use anyhow::Result;
use clap::Args;
use console::style;
use opencode_cloud_core::load_hosts;

/// Arguments for host show command
#[derive(Args)]
pub struct HostShowArgs {
    /// Name of the host to show
    pub name: String,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

pub async fn cmd_host_show(args: &HostShowArgs, quiet: bool, _verbose: u8) -> Result<()> {
    let hosts = load_hosts()?;

    let config = hosts
        .get_host(&args.name)
        .ok_or_else(|| anyhow::anyhow!("Host '{}' not found.", args.name))?;

    if args.json || quiet {
        // JSON output
        let json = serde_json::to_string_pretty(config)?;
        println!("{}", json);
        return Ok(());
    }

    // Pretty output
    let is_default = hosts.default_host.as_deref() == Some(&args.name);

    println!("{}", style(&args.name).cyan().bold());
    if is_default {
        println!("  {} (default)", style("*").green());
    }
    println!();

    println!("  {:<15} {}", style("Hostname:").dim(), config.hostname);
    println!("  {:<15} {}", style("User:").dim(), config.user);
    println!(
        "  {:<15} {}",
        style("Port:").dim(),
        config
            .port
            .map(|p| p.to_string())
            .unwrap_or_else(|| "22 (default)".to_string())
    );

    if let Some(key) = &config.identity_file {
        println!("  {:<15} {}", style("Identity:").dim(), key);
    }

    if let Some(jump) = &config.jump_host {
        println!("  {:<15} {}", style("Jump host:").dim(), jump);
    }

    if !config.groups.is_empty() {
        println!(
            "  {:<15} {}",
            style("Groups:").dim(),
            config.groups.join(", ")
        );
    }

    if let Some(desc) = &config.description {
        println!("  {:<15} {}", style("Description:").dim(), desc);
    }

    println!();
    println!(
        "  {} {}",
        style("Test connection:").dim(),
        style(format!("occ host test {}", args.name)).yellow()
    );

    Ok(())
}
