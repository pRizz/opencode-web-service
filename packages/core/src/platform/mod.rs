//! Platform-specific service manager abstraction
//!
//! This module provides a unified interface for registering the opencode-cloud
//! service with platform-specific init systems (systemd on Linux, launchd on macOS).

use std::path::PathBuf;

use anyhow::{Result, anyhow};

/// Configuration for service installation
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Path to the executable to run
    pub executable_path: PathBuf,

    /// Number of restart attempts on crash
    pub restart_retries: u32,

    /// Seconds between restart attempts
    pub restart_delay: u32,

    /// Boot mode: "user" (starts on login) or "system" (starts on boot)
    pub boot_mode: String,
}

/// Result of a service installation operation
#[derive(Debug, Clone)]
pub struct InstallResult {
    /// Path to the service file that was created
    pub service_file_path: PathBuf,

    /// Name of the service (e.g., "opencode-cloud")
    pub service_name: String,

    /// Whether the service was started after installation
    pub started: bool,

    /// Whether root/sudo is required for this installation type
    pub requires_root: bool,
}

/// Trait for platform-specific service managers
///
/// Implementations handle the details of registering services with
/// systemd (Linux) or launchd (macOS).
pub trait ServiceManager: Send + Sync {
    /// Install the service with the given configuration
    ///
    /// Creates the service file and registers it with the init system.
    /// Also starts the service immediately after registration.
    fn install(&self, config: &ServiceConfig) -> Result<InstallResult>;

    /// Uninstall the service
    ///
    /// Stops the service if running and removes the registration.
    fn uninstall(&self) -> Result<()>;

    /// Check if the service is currently installed
    fn is_installed(&self) -> Result<bool>;

    /// Get the path to the service file
    fn service_file_path(&self) -> PathBuf;

    /// Get the service name
    fn service_name(&self) -> &str;
}

/// Get the appropriate service manager for the current platform
///
/// Returns an error if the platform is not supported or if the
/// service manager implementation is not yet available.
pub fn get_service_manager() -> Result<Box<dyn ServiceManager>> {
    #[cfg(target_os = "linux")]
    {
        // Return stub that will be implemented in 04-02
        Err(anyhow!("systemd manager not yet implemented"))
    }
    #[cfg(target_os = "macos")]
    {
        // Return stub that will be implemented in 04-03
        Err(anyhow!("launchd manager not yet implemented"))
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        Err(anyhow!("Unsupported platform for service registration"))
    }
}

/// Check if service registration is supported on the current platform
///
/// Returns true for Linux (systemd) and macOS (launchd).
pub fn is_service_registration_supported() -> bool {
    cfg!(any(target_os = "linux", target_os = "macos"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config_creation() {
        let config = ServiceConfig {
            executable_path: PathBuf::from("/usr/local/bin/occ"),
            restart_retries: 3,
            restart_delay: 5,
            boot_mode: "user".to_string(),
        };

        assert_eq!(config.executable_path, PathBuf::from("/usr/local/bin/occ"));
        assert_eq!(config.restart_retries, 3);
        assert_eq!(config.restart_delay, 5);
        assert_eq!(config.boot_mode, "user");
    }

    #[test]
    fn test_install_result_creation() {
        let result = InstallResult {
            service_file_path: PathBuf::from("/etc/systemd/user/opencode-cloud.service"),
            service_name: "opencode-cloud".to_string(),
            started: true,
            requires_root: false,
        };

        assert_eq!(
            result.service_file_path,
            PathBuf::from("/etc/systemd/user/opencode-cloud.service")
        );
        assert_eq!(result.service_name, "opencode-cloud");
        assert!(result.started);
        assert!(!result.requires_root);
    }

    #[test]
    fn test_is_service_registration_supported() {
        // On macOS/Linux this should return true, on other platforms false
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        assert!(is_service_registration_supported());

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        assert!(!is_service_registration_supported());
    }

    #[test]
    fn test_get_service_manager_returns_error() {
        // Currently returns error since implementations aren't available
        let result = get_service_manager();
        assert!(result.is_err());
    }
}
