//! Configuration schema for opencode-cloud
//!
//! Defines the structure and defaults for the config.json file.

use serde::{Deserialize, Serialize};

/// Main configuration structure for opencode-cloud
///
/// Serialized to/from `~/.config/opencode-cloud/config.json`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Config file version for migrations
    pub version: u32,

    /// Port for the opencode web UI (default: 8080)
    #[serde(default = "default_port")]
    pub port: u16,

    /// Bind address (default: "localhost")
    /// Use "localhost" for local-only access (secure default)
    /// Use "0.0.0.0" for network access (requires explicit opt-in)
    #[serde(default = "default_bind")]
    pub bind: String,

    /// Auto-restart service on crash (default: true)
    #[serde(default = "default_auto_restart")]
    pub auto_restart: bool,
}

fn default_port() -> u16 {
    8080
}

fn default_bind() -> String {
    "localhost".to_string()
}

fn default_auto_restart() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            port: default_port(),
            bind: default_bind(),
            auto_restart: default_auto_restart(),
        }
    }
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.port, 8080);
        assert_eq!(config.bind, "localhost");
        assert!(config.auto_restart);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_deserialize_with_missing_optional_fields() {
        let json = r#"{"version": 1}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.version, 1);
        assert_eq!(config.port, 8080);
        assert_eq!(config.bind, "localhost");
        assert!(config.auto_restart);
    }

    #[test]
    fn test_reject_unknown_fields() {
        let json = r#"{"version": 1, "unknown_field": "value"}"#;
        let result: Result<Config, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
