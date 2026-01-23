---
phase: 11-remote-host-management
plan: 01
subsystem: infra
tags: [ssh, remote-hosts, docker, tunnel, hosts-json]

# Dependency graph
requires:
  - phase: 02-docker-integration
    provides: DockerClient pattern and error handling
  - phase: 03-configuration
    provides: XDG path resolution and config storage
provides:
  - Host configuration schema (HostConfig, HostsFile)
  - SSH tunnel management for remote Docker access
  - Host storage at ~/.config/opencode-cloud/hosts.json
  - Connection testing via SSH
affects: [12-cli-host-commands, 13-remote-operations]

# Tech tracking
tech-stack:
  added: [whoami]
  patterns:
    - Builder pattern for HostConfig
    - Drop trait for SSH tunnel cleanup
    - SSH BatchMode for non-interactive auth

key-files:
  created:
    - packages/core/src/host/error.rs
    - packages/core/src/host/schema.rs
    - packages/core/src/host/storage.rs
    - packages/core/src/host/tunnel.rs
    - packages/core/src/host/mod.rs
  modified:
    - packages/core/src/config/paths.rs
    - packages/core/src/lib.rs
    - packages/core/Cargo.toml

key-decisions:
  - "SSH tunnel via system ssh command (not library) - uses existing SSH config/agent"
  - "BatchMode=yes for non-interactive auth - fail fast if key not in agent"
  - "StrictHostKeyChecking=accept-new - accept new hosts automatically on first connection"
  - "Port 0 binding to find available local port for tunnel"
  - "Drop trait ensures SSH process cleanup on tunnel destruction"

patterns-established:
  - "Host module structure: error, schema, storage, tunnel"
  - "Builder pattern for configuration structs"
  - "Exponential backoff for tunnel readiness (100ms, 200ms, 400ms)"
  - "Backup creation (.bak) before overwriting hosts.json"

# Metrics
duration: 4min 31sec
completed: 2026-01-23
---

# Phase 11 Plan 01: Core Host Management Module Summary

**SSH tunnel-based remote host management with HostConfig schema, hosts.json storage, and system ssh integration**

## Performance

- **Duration:** 4 minutes 31 seconds
- **Started:** 2026-01-23T18:18:13Z
- **Completed:** 2026-01-23T18:22:44Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments
- Complete host module with error types, schema, storage, and SSH tunnel management
- HostConfig/HostsFile structs with builder pattern and JSON serialization
- SSH tunnel spawning with BatchMode, jump host support, and proper cleanup via Drop
- Connection testing function to verify both SSH and remote Docker availability
- Integrated get_hosts_path() into config/paths.rs for XDG compliance

## Task Commits

Each task was committed atomically:

1. **Task 1: Create host error types and schema** - `a31f6c1` (feat)
2. **Task 2: Create host storage and paths integration** - `ba2f83e` (feat)
3. **Task 3: Create SSH tunnel management and wire up module** - `84cb8d2` (feat)

## Files Created/Modified
- `packages/core/src/host/error.rs` - HostError enum with SSH, auth, and tunnel errors
- `packages/core/src/host/schema.rs` - HostConfig and HostsFile with builder pattern
- `packages/core/src/host/storage.rs` - load_hosts/save_hosts with backup support
- `packages/core/src/host/tunnel.rs` - SshTunnel struct with Drop cleanup, test_connection
- `packages/core/src/host/mod.rs` - Public exports for host module
- `packages/core/src/config/paths.rs` - Added get_hosts_path() function
- `packages/core/src/lib.rs` - Wired up host module with re-exports
- `packages/core/Cargo.toml` - Added whoami dependency

## Decisions Made

**SSH Implementation Approach:**
- Use system `ssh` command instead of SSH library - leverages user's existing SSH config, agent, and keys
- BatchMode=yes ensures non-interactive operation and fail-fast auth errors
- StrictHostKeyChecking=accept-new accepts new host keys on first connection without prompting

**Port Allocation:**
- Bind to port 0 to get OS-assigned available port for tunnel
- Avoids port conflicts and doesn't require privileged ports

**Cleanup Strategy:**
- Implement Drop trait on SshTunnel to ensure SSH process cleanup
- Call kill() then wait() to reap zombie processes
- Prevents resource leaks on tunnel destruction

**Configuration Schema:**
- Builder pattern for HostConfig enables fluent API
- Default user from whoami crate for convenience
- Support for jump hosts, custom ports, identity files, and groups

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy warning for unnecessary cast**
- **Found during:** Task 3 verification (just lint)
- **Issue:** `attempt as u32` cast was unnecessary since `attempt` is already u32
- **Fix:** Removed `as u32` cast in wait_ready() exponential backoff calculation
- **Files modified:** packages/core/src/host/tunnel.rs
- **Verification:** just lint passes with no warnings
- **Committed in:** 84cb8d2 (Task 3 commit, amended)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Clippy warning fix required for clean build. No functional changes.

## Issues Encountered
None - all tasks executed as planned with clean builds and passing tests.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Core host management module complete and tested
- Ready for CLI commands (host add/remove/list/show/test)
- Ready for remote operation routing (--host flag on container commands)
- SSH tunnel creation verified with proper cleanup
- Hosts.json storage with backup mechanism ready for production use

---
*Phase: 11-remote-host-management*
*Completed: 2026-01-23*
