//! Summary display
//!
//! Shows configuration summary before saving.

use crate::wizard::WizardState;
use comfy_table::{Cell, Table};
use console::style;
use opencode_cloud_core::config::paths::get_config_path;

/// Display configuration summary
///
/// Shows all collected values with password masked.
pub fn display_summary(state: &WizardState) {
    println!("{}", style("Configuration Summary").bold());
    println!("{}", style("-".repeat(22)).dim());

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING);

    table.add_row(vec![
        Cell::new("Username:"),
        Cell::new(state.auth_username.as_deref().unwrap_or("-")),
    ]);
    table.add_row(vec![Cell::new("Password:"), Cell::new("********")]);
    table.add_row(vec![Cell::new("Port:"), Cell::new(state.port)]);
    table.add_row(vec![Cell::new("Binding:"), Cell::new(&state.bind)]);

    println!("{table}");

    // Show config file location
    if let Some(path) = get_config_path() {
        println!();
        println!("Config will be saved to: {}", style(path.display()).dim());
    }
}
