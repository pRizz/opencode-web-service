//! Embedded Dockerfile content
//!
//! This module contains the Dockerfile for building the opencode-cloud container image,
//! embedded at compile time for distribution with the CLI.

/// The Dockerfile for building the opencode-cloud container image
pub const DOCKERFILE: &str = include_str!("Dockerfile");

/// Docker image name for GHCR (primary registry)
pub const IMAGE_NAME_GHCR: &str = "ghcr.io/prizz/opencode-cloud";

/// Docker image name for Docker Hub (fallback registry)
pub const IMAGE_NAME_DOCKERHUB: &str = "prizz/opencode-cloud";

/// Default image tag
pub const IMAGE_TAG_DEFAULT: &str = "latest";
