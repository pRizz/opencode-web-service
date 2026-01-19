//! Docker operations module
//!
//! This module provides Docker container management functionality including:
//! - Docker client wrapper with connection handling
//! - Docker-specific error types
//! - Embedded Dockerfile for building the opencode image

mod client;
mod dockerfile;
mod error;

pub use client::DockerClient;
pub use dockerfile::{DOCKERFILE, IMAGE_NAME_DOCKERHUB, IMAGE_NAME_GHCR, IMAGE_TAG_DEFAULT};
pub use error::DockerError;
