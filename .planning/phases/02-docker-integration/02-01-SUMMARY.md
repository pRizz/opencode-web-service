---
phase: 02-docker-integration
plan: 01
subsystem: docker
tags: [bollard, docker, dockerfile, rust, async]

# Dependency graph
requires:
  - phase: 01-foundation
    provides: Core library structure, Cargo workspace setup
provides:
  - DockerClient wrapper with connection handling
  - DockerError enum with clear error messages
  - Embedded Dockerfile for opencode container image
  - Docker module structure (client, error, dockerfile)
affects: [02-02 image-build, 02-03 container-lifecycle, 02-04 volume-persistence]

# Tech tracking
tech-stack:
  added:
    - bollard (0.18, async Docker client)
    - futures-util (0.3, stream utilities)
    - tar (0.4, archive creation)
    - flate2 (1.0, gzip compression)
    - tokio-retry (0.3, resilient operations)
    - indicatif (0.17, progress bars)
    - http-body-util (0.1, HTTP body utilities)
  patterns:
    - Docker client wrapper pattern for connection handling
    - Graceful error translation (Bollard errors to user-friendly messages)
    - Embedded static files via include_str!

key-files:
  created:
    - packages/core/src/docker/mod.rs
    - packages/core/src/docker/client.rs
    - packages/core/src/docker/error.rs
    - packages/core/src/docker/dockerfile.rs
    - packages/core/src/docker/Dockerfile
  modified:
    - Cargo.toml
    - packages/core/Cargo.toml
    - packages/core/src/lib.rs

key-decisions:
  - "Use Bollard 0.18 with chrono and buildkit features for Docker operations"
  - "Connection errors categorized: NotRunning, PermissionDenied, Connection, Timeout"
  - "Dockerfile embedded via include_str! for single-binary distribution"
  - "Ubuntu 24.04 LTS as base image for stability"
  - "mise for multi-language runtime management (Node, Python, Go)"
  - "Rust installed via rustup (mise Rust support is experimental)"

patterns-established:
  - "Docker module follows existing module pattern (mod.rs + submodules)"
  - "Error types use thiserror for std::error::Error implementation"
  - "Client wrapper exposes inner() for advanced operations"

# Metrics
duration: 5 min
completed: 2026-01-19
---

# Phase 2 Plan 1: Docker Client Foundation Summary

**Bollard-based Docker client wrapper with embedded Dockerfile for comprehensive development container**

## Performance

- **Duration:** 5 min
- **Started:** 2026-01-19T16:46:42Z
- **Completed:** 2026-01-19T16:51:59Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Created Docker module structure with client, error, and dockerfile submodules
- Implemented DockerClient wrapper with clear connection error handling
- Added comprehensive Dockerfile (500+ lines) with all tools from CONTEXT.md
- Added 7 workspace dependencies for Docker operations

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Bollard dependencies and create Docker module structure** - `c2b6179` (feat)
2. **Task 2: Implement Docker client wrapper** - Included in Task 1 commit (client.rs was created with full implementation)
3. **Task 3: Create embedded Dockerfile** - `880eb86` (feat)

**Note:** Task 2 was effectively completed within Task 1's atomic commit since the client implementation was straightforward and created as part of the module structure.

## Files Created/Modified

- `packages/core/src/docker/mod.rs` - Docker module exports
- `packages/core/src/docker/client.rs` - DockerClient wrapper with new(), with_timeout(), verify_connection(), version(), inner()
- `packages/core/src/docker/error.rs` - DockerError enum with NotRunning, PermissionDenied, Connection, Build, Pull, Container, Volume, Timeout variants
- `packages/core/src/docker/dockerfile.rs` - DOCKERFILE constant, IMAGE_NAME_GHCR, IMAGE_NAME_DOCKERHUB, IMAGE_TAG_DEFAULT
- `packages/core/src/docker/Dockerfile` - Comprehensive 505-line Dockerfile with all specified tools
- `Cargo.toml` - Added workspace dependencies for bollard, futures-util, tar, flate2, tokio-retry, indicatif, http-body-util
- `packages/core/Cargo.toml` - Added Docker dependencies using workspace pattern
- `packages/core/src/lib.rs` - Added pub mod docker and re-exports for DockerClient, DockerError

## Decisions Made

1. **Bollard 0.18 with buildkit feature** - Enables BuildKit support for future build optimizations
2. **Connection error categorization** - Translate Bollard errors to user-friendly messages (NotRunning vs PermissionDenied)
3. **Dockerfile embedding** - Use include_str! for compile-time embedding, enables single-binary distribution
4. **Ubuntu 24.04 base** - LTS for stability, wide package availability
5. **mise for runtimes** - Single tool for Node/Python/Go version management
6. **Rustup for Rust** - mise Rust support is experimental; rustup is more reliable

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - all tasks completed successfully.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Docker client foundation ready for image build/pull operations
- Dockerfile defined and embedded for container builds
- Error types ready for use in CLI commands
- Next plan (02-02) can implement image build with progress streaming

---
*Phase: 02-docker-integration*
*Completed: 2026-01-19*
