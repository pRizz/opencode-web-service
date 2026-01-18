//! opencode-cloud CLI - Manage your opencode cloud service
//!
//! This is the main entry point for the Rust CLI binary.

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use opencode_cloud_core::{Config, config, get_version, load_config};

/// Manage your opencode cloud service
#[derive(Parser)]
#[command(name = "opencode-cloud")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Manage your opencode cloud service", long_about = None)]
#[command(after_help = get_banner())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Increase verbosity level
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommands),
    // Future commands (Phase 3+):
    // Start - Start the opencode service
    // Stop - Stop the opencode service
    // Status - Show service status
    // Restart - Restart the opencode service
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key to set
        key: String,
        /// Value to set
        value: String,
    },
}

/// Get the ASCII banner for help display
fn get_banner() -> &'static str {
    r#"
  ___  _ __   ___ _ __   ___ ___   __| | ___
 / _ \| '_ \ / _ \ '_ \ / __/ _ \ / _` |/ _ \
| (_) | |_) |  __/ | | | (_| (_) | (_| |  __/
 \___/| .__/ \___|_| |_|\___\___/ \__,_|\___|
      |_|                            cloud
"#
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Configure color output
    if cli.no_color {
        console::set_colors_enabled(false);
    }

    // Load config (creates default if missing)
    let config_path = config::paths::get_config_path()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config path"))?;

    let config = match load_config() {
        Ok(config) => {
            // If config was just created, inform the user
            if cli.verbose > 0 {
                eprintln!(
                    "{} Config loaded from: {}",
                    style("[info]").cyan(),
                    config_path.display()
                );
            }
            config
        }
        Err(e) => {
            // Display rich error for invalid config
            eprintln!("{} Configuration error", style("Error:").red().bold());
            eprintln!();
            eprintln!("  {}", e);
            eprintln!();
            eprintln!("  Config file: {}", style(config_path.display()).yellow());
            eprintln!();
            eprintln!(
                "  {} Check the config file for syntax errors or unknown fields.",
                style("Tip:").cyan()
            );
            eprintln!(
                "  {} See schemas/config.example.jsonc for valid configuration.",
                style("Tip:").cyan()
            );
            std::process::exit(1);
        }
    };

    // Show verbose info if requested
    if cli.verbose > 0 {
        let data_dir = config::paths::get_data_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        eprintln!(
            "{} Config: {}",
            style("[info]").cyan(),
            config_path.display()
        );
        eprintln!("{} Data: {}", style("[info]").cyan(), data_dir);
    }

    match cli.command {
        Some(Commands::Config(cmd)) => handle_config(cmd, &config),
        None => {
            // No command - show a welcome message and hint to use --help
            if !cli.quiet {
                println!(
                    "{} {}",
                    style("opencode-cloud").cyan().bold(),
                    style(get_version()).dim()
                );
                println!();
                println!("Run {} for available commands.", style("--help").green());
            }
            Ok(())
        }
    }
}

fn handle_config(cmd: ConfigCommands, config: &Config) -> Result<()> {
    match cmd {
        ConfigCommands::Show => {
            // Display current config as formatted JSON
            let json =
                serde_json::to_string_pretty(config).expect("Config should always be serializable");
            println!("{}", json);
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            // Placeholder - not yet implemented
            eprintln!(
                "{} config set is not yet implemented",
                style("Note:").yellow()
            );
            eprintln!();
            eprintln!(
                "  Attempted to set: {} = {}",
                style(&key).cyan(),
                style(&value).green()
            );
            eprintln!();
            eprintln!("  For now, edit the config file directly at:");
            if let Some(path) = config::paths::get_config_path() {
                eprintln!("  {}", style(path.display()).yellow());
            }
            Ok(())
        }
    }
}
