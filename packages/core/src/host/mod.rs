//! Host management module
//!
//! Provides functionality for managing remote Docker hosts:
//! - Host configuration schema and storage
//! - SSH tunnel management for remote Docker access
//! - Connection testing and validation

mod error;
mod schema;
mod storage;
mod tunnel;

// Public exports
pub use error::HostError;
pub use schema::{HostConfig, HostsFile};
pub use storage::{load_hosts, save_hosts};
pub use tunnel::{SshTunnel, test_connection};
