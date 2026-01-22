---
phase: 07-update-and-maintenance
plan: 01
subsystem: infra
tags: [docker, image-management, rollback, update]

# Dependency graph
requires:
  - phase: 02-docker-integration
    provides: Docker client, image operations (build, pull)
  - phase: 03-service-management
    provides: Container lifecycle (start, stop, setup_and_start, stop_service)
  - phase: 06-security-and-authentication
    provides: User management and config persistence
provides:
  - Image update with automatic backup for rollback
  - Rollback to previous image version
  - occ update command with --rollback flag
  - User recreation after container replacement
affects: [manual-administration, remote-administration]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Image tagging for version history (previous tag)"
    - "Step-by-step progress feedback during update flow"
    - "Confirmation prompts with --yes override for automation"

key-files:
  created:
    - packages/core/src/docker/update.rs
    - packages/cli-rust/src/commands/update.rs
  modified:
    - packages/core/src/docker/mod.rs
    - packages/cli-rust/src/commands/mod.rs
    - packages/cli-rust/src/lib.rs

key-decisions:
  - "Tag current image as 'previous' before update for rollback capability"
  - "Passwords NOT preserved during update (not stored in config) - users must reset"
  - "Use bollard TagImageOptions for image tagging operations"
  - "Require confirmation unless --yes flag for safety"

patterns-established:
  - "Update flow: stop service → backup image → pull latest → recreate container → recreate users"
  - "Rollback flow: stop service → restore previous → recreate container → recreate users"

# Metrics
duration: 5min
completed: 2026-01-22
---

# Phase 07 Plan 01: Update and Maintenance Summary

**Image update with rollback capability via `occ update` command, preserving data volumes while recreating containers and users**

## Performance

- **Duration:** 5 min
- **Started:** 2026-01-22T06:26:23Z
- **Completed:** 2026-01-22T06:31:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Users can update to latest image with `occ update` command
- Previous version automatically backed up for rollback with `occ update --rollback`
- Data volumes preserved across updates (no data loss)
- Users recreated from config after update (passwords must be reset)
- Step-by-step progress feedback during update operations

## Task Commits

Each task was committed atomically:

1. **Task 1: Create docker update module with rollback support** - `699f521` (feat)
2. **Task 2: Create occ update CLI command** - `faa5be8` (feat)

## Files Created/Modified
- `packages/core/src/docker/update.rs` - Image update and rollback operations with tag management
- `packages/cli-rust/src/commands/update.rs` - CLI command for update/rollback with progress feedback
- `packages/core/src/docker/mod.rs` - Export update operations
- `packages/cli-rust/src/commands/mod.rs` - Export update command
- `packages/cli-rust/src/lib.rs` - Add Update command variant

## Decisions Made

**1. Tag current image as "previous" before update**
- Enables rollback to last known-good version
- Uses bollard's `TagImageOptions` API
- Silently skips if current image doesn't exist (fresh install)

**2. Passwords NOT preserved during update**
- Config only stores usernames, not passwords (security by design)
- Users must reset passwords after update/rollback
- Clear warning displayed to users about password recreation requirement

**3. Step-by-step progress with confirmation**
- 5 steps for update: stop → backup → pull → recreate → recreate users
- 4 steps for rollback: stop → restore → recreate → recreate users
- Confirmation prompt required unless --yes flag (safety)

**4. Use bollard TagImageOptions**
- Modern API instead of deprecated TagImageOptions struct
- Cleaner syntax: `TagImageOptions { repo, tag }`

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - implementation proceeded smoothly with existing Docker and user management infrastructure.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Update capability complete, ready for automated update checks (Phase 14)
- Rollback tested and working for issue recovery
- User recreation pattern established for container replacement scenarios
- No blockers for next phase

---
*Phase: 07-update-and-maintenance*
*Completed: 2026-01-22*
