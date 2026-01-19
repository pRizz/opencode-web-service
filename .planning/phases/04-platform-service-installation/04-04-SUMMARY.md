---
phase: 04-platform-service-installation
plan: 04
subsystem: cli
tags: [clap, dialoguer, service-manager, install, uninstall]

# Dependency graph
requires:
  - phase: 04-02
    provides: systemd service manager implementation
  - phase: 04-03
    provides: launchd service manager implementation
provides:
  - Install command for service registration
  - Uninstall command for service removal
  - Status command installation display
affects: [05-remote-access, 06-basic-auth]

# Tech tracking
tech-stack:
  added: [dialoguer 0.11]
  patterns: [confirmation-prompts, cli-service-management]

key-files:
  created:
    - packages/cli-rust/src/commands/install.rs
    - packages/cli-rust/src/commands/uninstall.rs
  modified:
    - packages/cli-rust/src/commands/mod.rs
    - packages/cli-rust/src/commands/status.rs
    - packages/cli-rust/src/lib.rs
    - packages/cli-rust/Cargo.toml
    - Cargo.toml

key-decisions:
  - "dialoguer for interactive confirmation prompts"
  - "Idempotent uninstall: exit 0 if service not installed"
  - "--volumes requires --force for safety"

patterns-established:
  - "Install command uses dialoguer::Confirm for reinstall prompts"
  - "Uninstall validates destructive flags require --force"

# Metrics
duration: 3min
completed: 2026-01-19
---

# Phase 4 Plan 4: Install/Uninstall CLI Commands Summary

**User-facing install/uninstall commands with dialoguer confirmation prompts and status integration**

## Performance

- **Duration:** 3 min
- **Started:** 2026-01-19T19:53:50Z
- **Completed:** 2026-01-19T19:56:55Z
- **Tasks:** 3
- **Files modified:** 7

## Accomplishments

- Added `occ install` command to register service with platform service manager
- Added `occ uninstall` command to remove service registration with optional volume cleanup
- Updated `occ status` to show installation status (Installed: yes/no with boot mode)
- Implemented confirmation prompts via dialoguer for reinstallation scenarios
- Added safety validation requiring --force with --volumes flag

## Task Commits

Each task was committed atomically:

1. **Task 1: Add dialoguer and create install command** - `0d14efc` (feat)
2. **Task 2: Create uninstall command** - `f4524a5` (feat)
3. **Task 3: Wire commands to CLI and update status** - `74d4bd9` (feat)

## Files Created/Modified

- `packages/cli-rust/src/commands/install.rs` - Install command with --force, --dry-run flags
- `packages/cli-rust/src/commands/uninstall.rs` - Uninstall command with --volumes, --force flags
- `packages/cli-rust/src/commands/mod.rs` - Export install/uninstall modules
- `packages/cli-rust/src/commands/status.rs` - Added installation status display
- `packages/cli-rust/src/lib.rs` - Added Install/Uninstall to Commands enum
- `packages/cli-rust/Cargo.toml` - Added dialoguer dependency
- `Cargo.toml` - Added dialoguer to workspace dependencies

## Decisions Made

1. **dialoguer for prompts** - Used dialoguer 0.11 for interactive confirmation prompts as it provides a clean API for terminal interaction
2. **Idempotent uninstall** - Exit 0 when service not installed (idempotent behavior like systemctl)
3. **Safety validation** - --volumes requires --force to prevent accidental data deletion

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 4 complete: All service installation commands implemented
- Ready for Phase 5 (Remote Access Setup)
- Users can now run `occ install` to register the service with systemd/launchd

---
*Phase: 04-platform-service-installation*
*Completed: 2026-01-19*
