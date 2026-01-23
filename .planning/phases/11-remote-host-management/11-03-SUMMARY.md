---
phase: 11-remote-host-management
plan: 03
subsystem: cli
tags: [remote-hosts, ssh-tunnel, docker, global-flag, cli-routing]

# Dependency graph
requires:
  - phase: 11-01
    provides: DockerClient with SSH tunnel support (connect_remote method)
  - phase: 11-02
    provides: Host command tree for managing remote hosts
  - phase: 03-container-operations
    provides: Existing container command structure
provides:
  - Global --host flag on all container commands
  - resolve_docker_client helper for host resolution (flag > default > local)
  - format_host_message helper for prefixed remote output
  - Remote Docker connections via SSH tunnels for all operations
affects: [12-web-desktop-ui-investigation, future-multi-host-operations]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Global flag pattern with clap for host routing
    - Helper functions for Docker client resolution
    - Output prefixing pattern for remote operations
    - DockerClient reference passing instead of creation

key-files:
  created: []
  modified:
    - packages/cli-rust/src/lib.rs
    - packages/core/src/docker/client.rs
    - packages/cli-rust/src/commands/start.rs
    - packages/cli-rust/src/commands/stop.rs
    - packages/cli-rust/src/commands/restart.rs
    - packages/cli-rust/src/commands/status.rs
    - packages/cli-rust/src/commands/logs.rs
    - packages/cli-rust/src/commands/update.rs
    - packages/cli-rust/src/commands/cockpit.rs
    - packages/cli-rust/src/commands/user/mod.rs
    - packages/cli-rust/src/commands/user/add.rs
    - packages/cli-rust/src/commands/user/remove.rs
    - packages/cli-rust/src/commands/user/list.rs
    - packages/cli-rust/src/commands/user/passwd.rs
    - packages/cli-rust/src/commands/user/enable.rs

key-decisions:
  - "Global --host flag on Cli struct for all commands to inherit"
  - "resolve_docker_client helper centralizes host resolution logic"
  - "format_host_message helper ensures consistent [hostname] prefix formatting"
  - "User subcommands accept DockerClient reference instead of creating own - prevents duplicate connections"
  - "Host resolution order: explicit --host > default_host from hosts.json > local Docker"

patterns-established:
  - "Container commands use resolve_docker_client(maybe_host) for client creation"
  - "Remote output prefixed with [hostname] for clarity in multi-host scenarios"
  - "DockerClient lifecycle managed at command level, passed to suboperations"

# Metrics
duration: 9min 47sec
completed: 2026-01-23
---

# Phase 11 Plan 03: Container Commands Remote Host Integration Summary

**Global --host flag on all container commands with SSH tunnel-based remote Docker connections and prefixed output for multi-host clarity**

## Performance

- **Duration:** 9 minutes 47 seconds
- **Started:** 2026-01-23T18:32:20Z
- **Completed:** 2026-01-23T18:42:07Z
- **Tasks:** 3
- **Files modified:** 15

## Accomplishments
- Extended DockerClient with connect_remote and connect_remote_with_timeout methods
- Global --host flag added to CLI with resolve_docker_client helper
- All 8 container commands (start, stop, restart, status, logs, update, cockpit, user) support remote operations
- Remote output prefixed with [hostname] for clear source identification
- User subcommands refactored to accept DockerClient reference for efficiency

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend DockerClient with remote connection support** - `940ca89` (feat)
2. **Task 2: Add global --host flag and client resolution helpers** - `2c46d30` (feat)
3. **Task 3: Update all container commands to support --host flag** - `c532913` (feat)

## Files Created/Modified
- `packages/core/src/docker/client.rs` - Added connect_remote, connect_remote_with_timeout, host_name, is_remote methods
- `packages/cli-rust/src/lib.rs` - Global --host flag, resolve_docker_client and format_host_message helpers
- `packages/cli-rust/src/commands/start.rs` - Remote host support with prefixed output
- `packages/cli-rust/src/commands/stop.rs` - Remote host support with prefixed output
- `packages/cli-rust/src/commands/restart.rs` - Remote host support with prefixed output
- `packages/cli-rust/src/commands/status.rs` - Remote host support with host header
- `packages/cli-rust/src/commands/logs.rs` - Remote host support with line-by-line prefixing
- `packages/cli-rust/src/commands/update.rs` - Remote host support for update/rollback flows
- `packages/cli-rust/src/commands/cockpit.rs` - Remote host support
- `packages/cli-rust/src/commands/user/mod.rs` - Remote host support with client passing to subcommands
- `packages/cli-rust/src/commands/user/*.rs` - Accept DockerClient reference instead of creating own
- `packages/cli-rust/src/commands/setup.rs` - Fixed to pass None for host parameter

## Decisions Made

**Global Flag Placement:**
- Added --host as global flag on Cli struct so all commands inherit it automatically
- Clap's global = true ensures flag appears in --help for every command

**Host Resolution Strategy:**
- Created resolve_docker_client helper for consistent resolution logic
- Resolution order: explicit --host flag > default_host from hosts.json > local Docker
- Special case: --host local forces local Docker even when default_host is set

**Output Formatting:**
- format_host_message helper ensures consistent [hostname] prefix
- Logs get line-by-line prefixing for clarity when tailing multiple hosts
- Status shows host header at top for context

**DockerClient Lifecycle:**
- SSH tunnel stored in DockerClient._tunnel field for automatic cleanup via Drop
- host_name stored in DockerClient for display purposes
- User subcommands refactored to accept &DockerClient instead of creating own - prevents duplicate connections and tunnels

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed unused connect_docker functions**
- **Found during:** Task 3 verification (just lint)
- **Issue:** connect_docker functions in start.rs, stop.rs, restart.rs no longer used after refactor
- **Fix:** Removed unused functions to clean up code
- **Files modified:** packages/cli-rust/src/commands/start.rs, stop.rs, restart.rs
- **Verification:** just lint passes with no dead code warnings
- **Committed in:** c532913 (Task 3 commit)

**2. [Rule 1 - Bug] Fixed needless_borrow in user commands**
- **Found during:** Task 3 verification (just lint)
- **Issue:** Clippy reported needless_borrow errors on &client references in user subcommands
- **Fix:** Changed &client to client since parameter is already a reference
- **Files modified:** packages/cli-rust/src/commands/user/add.rs, enable.rs, list.rs, passwd.rs, remove.rs
- **Verification:** just lint passes with no clippy warnings
- **Committed in:** c532913 (Task 3 commit)

**3. [Rule 3 - Blocking] Updated setup.rs to pass host parameter**
- **Found during:** Task 3 verification (cargo build)
- **Issue:** setup.rs calls cmd_start with old signature (missing maybe_host parameter)
- **Fix:** Added None as host parameter to cmd_start call in setup wizard
- **Files modified:** packages/cli-rust/src/commands/setup.rs
- **Verification:** cargo build succeeds
- **Committed in:** c532913 (Task 3 commit)

**4. [Rule 1 - Bug] Removed unused DockerClient imports**
- **Found during:** Task 3 verification (just lint)
- **Issue:** DockerClient imported but unused in restart.rs, stop.rs, cockpit.rs, logs.rs, status.rs, user/mod.rs
- **Fix:** Removed unused imports
- **Files modified:** packages/cli-rust/src/commands/restart.rs, stop.rs, cockpit.rs, logs.rs, status.rs, user/mod.rs
- **Verification:** just lint passes with no unused import warnings
- **Committed in:** c532913 (Task 3 commit)

---

**Total deviations:** 4 auto-fixed (4 bugs)
**Impact on plan:** All auto-fixes necessary for clean compilation and lint passing. No functional changes or scope creep.

## Issues Encountered
None - all tasks executed as planned with expected refactoring cleanup.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Remote host management feature complete
- All container commands support --host flag
- SSH tunnels automatically managed for remote connections
- Output clearly identifies remote vs local operations
- Ready for Phase 12 (Web Desktop UI Investigation)
- Ready for future multi-host operations (parallel execution, host groups)

---
*Phase: 11-remote-host-management*
*Completed: 2026-01-23*
