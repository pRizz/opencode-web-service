//! Docker client wrapper with connection handling
//!
//! This module provides a wrapped Docker client that handles connection
//! errors gracefully and provides clear error messages.

use bollard::Docker;

use super::error::DockerError;

/// Docker client wrapper with connection handling
pub struct DockerClient {
    inner: Docker,
}

impl DockerClient {
    /// Create new client connecting to local Docker daemon
    ///
    /// Uses platform-appropriate socket (Unix socket on Linux/macOS).
    /// Returns a clear error if Docker is not running or accessible.
    pub fn new() -> Result<Self, DockerError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| DockerError::Connection(e.to_string()))?;

        Ok(Self { inner: docker })
    }

    /// Create client with custom timeout (in seconds)
    ///
    /// Use for long-running operations like image builds.
    /// Default timeout is 120 seconds; build timeout should be 600+ seconds.
    pub fn with_timeout(timeout_secs: u64) -> Result<Self, DockerError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| DockerError::Connection(e.to_string()))?
            .with_timeout(std::time::Duration::from_secs(timeout_secs));

        Ok(Self { inner: docker })
    }

    /// Verify connection to Docker daemon
    ///
    /// Returns Ok(()) if connected, descriptive error otherwise.
    pub async fn verify_connection(&self) -> Result<(), DockerError> {
        self.inner.ping().await.map_err(DockerError::from)?;
        Ok(())
    }

    /// Get Docker version info (useful for debugging)
    pub async fn version(&self) -> Result<String, DockerError> {
        let version = self.inner.version().await.map_err(DockerError::from)?;

        let version_str = format!(
            "Docker {} (API {})",
            version.version.unwrap_or_else(|| "unknown".to_string()),
            version.api_version.unwrap_or_else(|| "unknown".to_string())
        );

        Ok(version_str)
    }

    /// Access inner Bollard client for advanced operations
    pub fn inner(&self) -> &Docker {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_client_creation_does_not_panic() {
        // This test just verifies the code compiles and doesn't panic
        // Actual connection test requires Docker to be running
        let result = DockerClient::new();
        // We don't assert success because Docker may not be running in CI
        drop(result);
    }

    #[test]
    fn docker_client_with_timeout_does_not_panic() {
        let result = DockerClient::with_timeout(600);
        drop(result);
    }
}
