# Phase 10 Plan 02: Cockpit Config and Container Systemd Support Summary

Config schema with cockpit_port/cockpit_enabled, container creation with CAP_SYS_ADMIN, tmpfs, and cgroup mounts, CLI wiring complete.

## Metadata

- **Phase:** 10 (Remote Administration via Cockpit)
- **Plan:** 02
- **Subsystem:** config, docker, cli
- **Tags:** cockpit, systemd, container, config

## Dependency Graph

- **Requires:** 10-01 (Dockerfile Cockpit Integration)
- **Provides:** Container configured for systemd and Cockpit port exposure
- **Affects:** 10-03 (Documentation and Testing)

## Changes

### packages/core/src/config/schema.rs

- Added `cockpit_port` field (default 9090)
- Added `cockpit_enabled` field (default true)
- Added default functions `default_cockpit_port()` and `default_cockpit_enabled()`
- Updated `Default` impl to include new fields
- Added tests for new fields and backward compatibility

### packages/core/src/docker/container.rs

- Added `cockpit_port` and `cockpit_enabled` parameters to `create_container`
- Added CAP_SYS_ADMIN capability for systemd cgroup access
- Added tmpfs mounts for /run and /tmp (required for systemd)
- Added cgroup bind mount /sys/fs/cgroup:ro
- Added Cockpit port mapping (host:config_port -> container:9090) when enabled
- Updated exposed_ports to include 9090/tcp when Cockpit enabled
- Added `#[allow(clippy::too_many_arguments)]` to handle 8-argument function

### packages/core/src/docker/mod.rs

- Added `cockpit_port` and `cockpit_enabled` parameters to `setup_and_start`
- Updated doc comments to document new parameters
- Pass new parameters through to `create_container`

### packages/cli-rust/src/commands/start.rs

- Updated `start_container` helper to accept cockpit parameters
- Updated call in `cmd_start` to pass `config.cockpit_port` and `config.cockpit_enabled`

### packages/cli-rust/src/commands/restart.rs

- Updated `setup_and_start` call to pass cockpit parameters from config

### packages/cli-rust/src/commands/update.rs

- Updated both `setup_and_start` calls (update and rollback flows) to pass cockpit parameters

## Tech Stack

- **Added:** None (uses existing bollard HostConfig fields)
- **Patterns:** Capability-based container security, tmpfs for ephemeral storage

## Key Files

- **Created:** None
- **Modified:**
  - packages/core/src/config/schema.rs
  - packages/core/src/docker/container.rs
  - packages/core/src/docker/mod.rs
  - packages/cli-rust/src/commands/start.rs
  - packages/cli-rust/src/commands/restart.rs
  - packages/cli-rust/src/commands/update.rs

## Decisions Made

| Decision | Rationale | Alternatives Considered |
|----------|-----------|------------------------|
| Default cockpit_port 9090 | Standard Cockpit port | Custom default |
| Default cockpit_enabled true | Cockpit integrated in image by 10-01 | Default false |
| CAP_SYS_ADMIN capability | Required for systemd cgroup operations | None for systemd |
| tmpfs for /run and /tmp | Required for systemd runtime | None for systemd |
| cgroup mount read-only | Security: prevent container cgroup manipulation | Read-write |
| Container port always 9090 | Cockpit listens on 9090 internally | Configurable |

## Verification Results

- [x] Config schema has cockpit_port with default 9090
- [x] Config schema has cockpit_enabled with default true
- [x] Old configs without cockpit fields deserialize correctly with defaults
- [x] Container creation includes CAP_SYS_ADMIN capability
- [x] Container creation includes tmpfs for /run and /tmp
- [x] Container creation includes cgroup bind mount
- [x] Cockpit port mapping: host:config_port -> container:9090 (when enabled)
- [x] setup_and_start accepts cockpit_port and cockpit_enabled parameters
- [x] start.rs passes config.cockpit_port and config.cockpit_enabled to setup_and_start
- [x] All tests pass: `just test`
- [x] Build succeeds: `just build`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added clippy too_many_arguments allow**

- **Found during:** Task 2
- **Issue:** create_container function now has 8 arguments, exceeds clippy default of 7
- **Fix:** Added `#[allow(clippy::too_many_arguments)]` attribute
- **Files modified:** packages/core/src/docker/container.rs
- **Commit:** e039ddc

## Metrics

- **Duration:** 4 min
- **Completed:** 2026-01-22

## Next Phase Readiness

Phase 10 plan 03 (Documentation and Testing) can proceed:

- Config fields are available via `config.cockpit_port` and `config.cockpit_enabled`
- Container creation is systemd-compatible with proper capabilities and mounts
- Cockpit port is exposed when `cockpit_enabled: true`
- All CLI commands (start, restart, update) pass cockpit settings through

## Commits

- `7a0fc62`: feat(10-02): add cockpit_port and cockpit_enabled config fields
- `e039ddc`: feat(10-02): update container creation for systemd and Cockpit support
