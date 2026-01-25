//! Mount list subcommand

use anyhow::Result;
use clap::Args;
use comfy_table::{Cell, Table, presets::UTF8_FULL_CONDENSED};
use console::style;
use opencode_cloud_core::config::load_config;
use opencode_cloud_core::docker::ParsedMount;

#[derive(Args)]
pub struct MountListArgs {
    /// Output only host paths (for scripting)
    #[arg(long)]
    pub names_only: bool,
}

pub async fn cmd_mount_list(args: &MountListArgs, quiet: bool, _verbose: u8) -> Result<()> {
    let config = load_config()?;

    if config.mounts.is_empty() {
        if !quiet && !args.names_only {
            println!("No mounts configured.");
            println!();
            println!(
                "Add a mount with: {}",
                style("occ mount add /host/path:/container/path").cyan()
            );
        }
        return Ok(());
    }

    // Names only mode for scripting
    if args.names_only {
        for mount_str in &config.mounts {
            if let Ok(parsed) = ParsedMount::parse(mount_str) {
                println!("{}", parsed.host_path.display());
            }
        }
        return Ok(());
    }

    // Table output
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec![
        Cell::new("HOST PATH"),
        Cell::new("CONTAINER PATH"),
        Cell::new("MODE"),
    ]);

    for mount_str in &config.mounts {
        match ParsedMount::parse(mount_str) {
            Ok(parsed) => {
                let mode = if parsed.read_only { "ro" } else { "rw" };
                table.add_row(vec![
                    Cell::new(parsed.host_path.display().to_string()),
                    Cell::new(&parsed.container_path),
                    Cell::new(mode),
                ]);
            }
            Err(_) => {
                // Show raw string for unparseable mounts
                table.add_row(vec![
                    Cell::new(mount_str),
                    Cell::new("(invalid)"),
                    Cell::new("-"),
                ]);
            }
        }
    }

    println!("{table}");

    Ok(())
}
