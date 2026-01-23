//! Host management module
//!
//! Provides functionality for managing remote Docker hosts:
//! - Host configuration schema and storage
//! - SSH tunnel management for remote Docker access
//! - Connection testing and validation
//! - SSH config file parsing and writing

mod error;
mod schema;
mod ssh_config;
mod storage;
mod tunnel;

// Public exports
pub use error::HostError;
pub use schema::{HostConfig, HostsFile};
pub use ssh_config::{
    SshConfigMatch, get_ssh_config_path, host_exists_in_ssh_config, query_ssh_config,
    write_ssh_config_entry,
};
pub use storage::{load_hosts, save_hosts};
pub use tunnel::{SshTunnel, test_connection};
