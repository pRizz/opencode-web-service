---
phase: 17
plan: 01
subsystem: docker
tags: [mount, bind-mount, validation, config]
requires: []
provides: [mount-parsing, mount-validation, config-mounts-field]
affects: [17-02, 17-03]
tech-stack:
  added: []
  patterns: [ParsedMount struct, MountError enum with thiserror]
key-files:
  created:
    - packages/core/src/docker/mount.rs
  modified:
    - packages/core/src/docker/mod.rs
    - packages/core/src/config/schema.rs
decisions:
  - "Mount format: /host:/container[:ro|rw] with rw as default"
  - "System paths warning: /etc, /usr, /bin, /sbin, /lib, /var"
  - "Validation uses canonicalize() to resolve symlinks and verify existence"
metrics:
  duration: "3 min"
  completed: "2026-01-25"
---

# Phase 17 Plan 01: Mount Parsing and Config Schema Summary

**One-liner:** Bind mount parsing with format `/host:/container[:ro|rw]` and config mounts field added.

## What Changed

### New Files

**packages/core/src/docker/mount.rs**
- `MountError` enum with variants: `RelativePath`, `InvalidFormat`, `PathNotFound`, `NotADirectory`, `PermissionDenied`
- `ParsedMount` struct with `host_path: PathBuf`, `container_path: String`, `read_only: bool`
- `ParsedMount::parse()` - parses mount strings in Docker format
- `ParsedMount::to_bollard_mount()` - converts to Bollard Mount type for Docker API
- `validate_mount_path()` - validates host path existence, type, and permissions
- `check_container_path_warning()` - warns about mounting to system paths
- 19 unit tests covering all parsing and validation cases

### Modified Files

**packages/core/src/docker/mod.rs**
- Added `pub mod mount;`
- Export `MountError`, `ParsedMount`, `check_container_path_warning`, `validate_mount_path`

**packages/core/src/config/schema.rs**
- Added `mounts: Vec<String>` field with `#[serde(default)]`
- Updated `Default::default()` to include `mounts: Vec::new()`
- Added 3 tests for mounts field

## Decisions Made

| Decision | Rationale |
|----------|-----------|
| Format: `/host:/container[:ro\|rw]` | Docker standard format, familiar to users |
| Default mode: read-write | Docker convention, least surprise |
| System paths: warn only | Users may legitimately need to mount system paths |
| Validation: canonicalize + metadata | Resolves symlinks, verifies directory type |

## Verification Results

- `just build` - Passed
- `just test` - Passed (169 core tests, 89 CLI tests)
- `just lint` - Passed (no clippy warnings)
- `just fmt` - Passed

## Commits

| Hash | Description |
|------|-------------|
| acdae4d | feat(17-01): add mount parsing and validation module |
| 0306877 | feat(17-01): add mounts field to config schema |

## Deviations from Plan

None - plan executed exactly as written.

## Next Plan Readiness

Plan 17-02 can proceed:
- ParsedMount ready for container integration
- Config mounts field ready for CLI commands
- All exports available in docker module
