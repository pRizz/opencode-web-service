//! XDG-compliant path resolution for opencode-cloud
//!
//! Provides consistent path resolution across platforms:
//! - Linux/macOS: ~/.config/opencode-cloud/ and ~/.local/share/opencode-cloud/
//! - Windows: %APPDATA%\opencode-cloud\ and %LOCALAPPDATA%\opencode-cloud\

use std::path::PathBuf;

/// Get the configuration directory path
///
/// Returns the directory where config.json should be stored:
/// - Linux: `~/.config/opencode-cloud/`
/// - macOS: `~/.config/opencode-cloud/` (XDG-style, not ~/Library)
/// - Windows: `%APPDATA%\opencode-cloud\`
pub fn get_config_dir() -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        directories::BaseDirs::new()
            .map(|dirs| dirs.home_dir().join(".config").join("opencode-cloud"))
    }
    #[cfg(target_os = "windows")]
    {
        directories::BaseDirs::new()
            .and_then(|dirs| dirs.config_dir().map(|d| d.to_path_buf()))
            .map(|d| d.join("opencode-cloud"))
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// Get the data directory path
///
/// Returns the directory where runtime data (PID file, logs, etc.) should be stored:
/// - Linux: `~/.local/share/opencode-cloud/`
/// - macOS: `~/.local/share/opencode-cloud/` (XDG-style, not ~/Library)
/// - Windows: `%LOCALAPPDATA%\opencode-cloud\`
pub fn get_data_dir() -> Option<PathBuf> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        directories::BaseDirs::new().map(|dirs| {
            dirs.home_dir()
                .join(".local")
                .join("share")
                .join("opencode-cloud")
        })
    }
    #[cfg(target_os = "windows")]
    {
        directories::BaseDirs::new()
            .and_then(|dirs| dirs.data_local_dir().map(|d| d.to_path_buf()))
            .map(|d| d.join("opencode-cloud"))
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// Get the full path to the config file
///
/// Returns: `{config_dir}/config.json`
pub fn get_config_path() -> Option<PathBuf> {
    get_config_dir().map(|d| d.join("config.json"))
}

/// Get the full path to the PID lock file
///
/// Returns: `{data_dir}/opencode-cloud.pid`
pub fn get_pid_path() -> Option<PathBuf> {
    get_data_dir().map(|d| d.join("opencode-cloud.pid"))
}

/// Get the full path to the hosts configuration file
///
/// Returns: `{config_dir}/hosts.json`
pub fn get_hosts_path() -> Option<PathBuf> {
    get_config_dir().map(|d| d.join("hosts.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_exists() {
        let dir = get_config_dir();
        assert!(dir.is_some());
        let path = dir.unwrap();
        assert!(path.ends_with("opencode-cloud"));
    }

    #[test]
    fn test_data_dir_exists() {
        let dir = get_data_dir();
        assert!(dir.is_some());
        let path = dir.unwrap();
        assert!(path.ends_with("opencode-cloud"));
    }

    #[test]
    fn test_config_path_ends_with_config_json() {
        let path = get_config_path();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with("config.json"));
    }

    #[test]
    fn test_pid_path_ends_with_pid() {
        let path = get_pid_path();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with("opencode-cloud.pid"));
    }

    #[test]
    fn test_hosts_path_ends_with_hosts_json() {
        let path = get_hosts_path();
        assert!(path.is_some());
        assert!(path.unwrap().ends_with("hosts.json"));
    }
}
