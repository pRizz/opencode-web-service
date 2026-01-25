//! Config show subcommand
//!
//! Displays current configuration in table or JSON format.
//! Uses serde serialization to automatically include all Config fields.

use anyhow::Result;
use comfy_table::{Cell, Color, Table};
use opencode_cloud_core::{Config, config};
use serde_json::Value;

/// Fields that should have their values masked in output
const SENSITIVE_FIELDS: &[&str] = &["auth_password"];

/// Fields that should be highlighted when they indicate security concerns
const SECURITY_FIELDS: &[(&str, &str)] = &[
    ("bind_address", "0.0.0.0"),               // Network exposed
    ("bind_address", "::"),                    // Network exposed (IPv6)
    ("allow_unauthenticated_network", "true"), // No auth required
];

/// Show current configuration
///
/// Displays all configuration values in a formatted table.
/// Uses serde serialization to automatically include all fields.
/// Passwords are masked for security.
pub fn cmd_config_show(config: &Config, json: bool, _quiet: bool) -> Result<()> {
    if json {
        // Output as JSON with password masked
        let mut value = serde_json::to_value(config)?;
        mask_sensitive_fields(&mut value);
        let output = serde_json::to_string_pretty(&value)?;
        println!("{output}");
    } else {
        // Serialize config to JSON Value to get all fields automatically
        let value = serde_json::to_value(config)?;
        let obj = value
            .as_object()
            .expect("Config should serialize to object");

        let mut table = Table::new();
        table.set_header(vec!["Key", "Value"]);

        // Iterate over all fields in serialization order
        for (key, val) in obj {
            let display_value = format_value(key, val);
            let cell = apply_cell_styling(key, val, display_value);
            table.add_row(vec![Cell::new(key), cell]);
        }

        println!("{table}");

        // Show config file location
        if let Some(path) = config::paths::get_config_path() {
            println!();
            println!("Config file: {}", path.display());
        }
    }

    Ok(())
}

/// Format a JSON value for display
fn format_value(key: &str, value: &Value) -> String {
    // Mask sensitive fields
    if SENSITIVE_FIELDS.contains(&key) {
        return match value {
            Value::String(s) if !s.is_empty() => "********".to_string(),
            Value::Null => "(not set)".to_string(),
            _ => "(not set)".to_string(),
        };
    }

    match value {
        Value::Null => "(not set)".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            if s.is_empty() {
                // Special case: Optional<String> that serialized as empty string
                if key == "auth_username" || key == "auth_password" {
                    "(not set)".to_string()
                } else {
                    s.clone()
                }
            } else {
                s.clone()
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                "(none)".to_string()
            } else {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
        Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// Apply color styling to cells based on security implications
fn apply_cell_styling(key: &str, value: &Value, display_value: String) -> Cell {
    // Check if this field+value combination has security implications
    let value_str = match value {
        Value::String(s) => s.as_str(),
        Value::Bool(b) => {
            if *b {
                "true"
            } else {
                "false"
            }
        }
        _ => "",
    };

    for (field, dangerous_value) in SECURITY_FIELDS {
        if key == *field && value_str == *dangerous_value {
            return Cell::new(display_value).fg(Color::Yellow);
        }
    }

    // Green for secure bind_address values
    if key == "bind_address" {
        let is_localhost =
            value_str == "127.0.0.1" || value_str == "::1" || value_str == "localhost";
        if is_localhost {
            return Cell::new(display_value).fg(Color::Green);
        }
    }

    Cell::new(display_value)
}

/// Mask sensitive fields in a JSON Value (for JSON output)
fn mask_sensitive_fields(value: &mut Value) {
    if let Value::Object(obj) = value {
        for (key, val) in obj.iter_mut() {
            if SENSITIVE_FIELDS.contains(&key.as_str()) {
                if let Value::String(s) = val {
                    if !s.is_empty() {
                        *val = Value::String("********".to_string());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_masks_password() {
        let val = Value::String("secret".to_string());
        assert_eq!(format_value("auth_password", &val), "********");
    }

    #[test]
    fn test_format_value_shows_not_set_for_empty_password() {
        let val = Value::String(String::new());
        assert_eq!(format_value("auth_password", &val), "(not set)");
    }

    #[test]
    fn test_format_value_preserves_normal_strings() {
        let val = Value::String("localhost".to_string());
        assert_eq!(format_value("bind", &val), "localhost");
    }

    #[test]
    fn test_format_value_formats_arrays() {
        let val = Value::Array(vec![
            Value::String("FOO=bar".to_string()),
            Value::String("BAZ=qux".to_string()),
        ]);
        assert_eq!(format_value("container_env", &val), "FOO=bar, BAZ=qux");
    }

    #[test]
    fn test_format_value_shows_none_for_empty_array() {
        let val = Value::Array(vec![]);
        assert_eq!(format_value("container_env", &val), "(none)");
    }

    #[test]
    fn test_all_config_fields_serialize() {
        // This test ensures that Config can be serialized to JSON
        // If a field is added that can't be serialized, this will fail
        let config = Config::default();
        let value = serde_json::to_value(&config).expect("Config should serialize");
        let obj = value.as_object().expect("Should be an object");

        // Verify some expected fields exist
        assert!(obj.contains_key("version"));
        assert!(obj.contains_key("opencode_web_port"));
        assert!(obj.contains_key("image_source"));
        assert!(obj.contains_key("update_check"));
    }

    #[test]
    fn test_mask_sensitive_fields() {
        let mut value = serde_json::json!({
            "auth_username": "admin",
            "auth_password": "secret123",
            "bind": "localhost"
        });
        mask_sensitive_fields(&mut value);

        let obj = value.as_object().unwrap();
        assert_eq!(obj["auth_username"], "admin"); // Not masked
        assert_eq!(obj["auth_password"], "********"); // Masked
        assert_eq!(obj["bind"], "localhost"); // Not masked
    }
}
