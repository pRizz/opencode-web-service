---
phase: 16-code-quality-audit
plan: 01
subsystem: cli
tags: [rust, error-handling, refactoring, docker]

# Dependency graph
requires:
  - phase: 03-docker-lifecycle
    provides: DockerError type and Docker commands
provides:
  - Centralized Docker error formatting module (output/errors.rs)
  - format_docker_error(), format_docker_error_anyhow(), show_docker_error() functions
affects: [17-custom-bind-mounts, 23-container-shell-access]

# Tech tracking
tech-stack:
  added: []
  patterns: [centralized-error-formatting, shared-output-module]

key-files:
  created:
    - packages/cli-rust/src/output/errors.rs
  modified:
    - packages/cli-rust/src/output/mod.rs
    - packages/cli-rust/src/commands/start.rs
    - packages/cli-rust/src/commands/status.rs
    - packages/cli-rust/src/commands/stop.rs
    - packages/cli-rust/src/commands/restart.rs
    - packages/cli-rust/src/commands/logs.rs

key-decisions:
  - "Centralized errors in output/errors.rs - single source of truth"
  - "Two function variants: String (for format!) and anyhow::Error (for map_err)"

patterns-established:
  - "Shared output helpers in output/ module"
  - "Docker error formatting includes troubleshooting docs link"

# Metrics
duration: 8min
completed: 2026-01-24
---

# Phase 16 Plan 01: Docker Error Formatting Deduplication Summary

**Centralized Docker error formatting in output/errors.rs, eliminating 5 duplicate implementations across CLI commands**

## Performance

- **Duration:** 8 min
- **Started:** 2026-01-24T10:00:00Z
- **Completed:** 2026-01-24T10:08:00Z
- **Tasks:** 3
- **Files modified:** 7

## Accomplishments
- Created centralized errors.rs module with format_docker_error(), format_docker_error_anyhow(), show_docker_error()
- Removed 4 duplicate format_docker_error() implementations from stop.rs, restart.rs, logs.rs (start.rs/status.rs already done by 16-02)
- All 5 command files now use shared error formatting from output/errors.rs
- Comprehensive test coverage with 5 new tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Create shared errors.rs module** - `23cf8e9` (feat)
2. **Task 2: Update command files to use shared errors module** - `e4de1a8` (refactor)
3. **Task 3: Verify refactoring** - (verification only, no commit)

## Files Created/Modified
- `packages/cli-rust/src/output/errors.rs` - New centralized Docker error formatting module with tests
- `packages/cli-rust/src/output/mod.rs` - Export new errors module functions
- `packages/cli-rust/src/commands/start.rs` - Fixed clippy warnings for format args
- `packages/cli-rust/src/commands/stop.rs` - Use shared format_docker_error/show_docker_error
- `packages/cli-rust/src/commands/restart.rs` - Use shared format_docker_error/show_docker_error
- `packages/cli-rust/src/commands/logs.rs` - Use shared format_docker_error_anyhow
- `packages/cli-rust/src/output/urls.rs` - Added allow(dead_code) for future-use functions

## Decisions Made
- **Centralized in output/errors.rs:** Follows existing pattern of output module for CLI formatting helpers
- **Two function variants:** format_docker_error() -> String for anyhow!("{msg}") usage, format_docker_error_anyhow() -> anyhow::Error for direct map_err usage
- **Most complete error formatting preserved:** start.rs version had docs link and port conflict handling, this became the canonical implementation

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed unused import warnings**
- **Found during:** Task 2 (updating command files)
- **Issue:** After removing local format_docker_error, DockerError import became unused in logs.rs and status.rs
- **Fix:** Removed unused DockerError from imports
- **Files modified:** logs.rs, status.rs
- **Verification:** just lint passes with 0 warnings
- **Committed in:** e4de1a8

**2. [Rule 1 - Bug] Fixed clippy uninlined_format_args warnings**
- **Found during:** Task 2
- **Issue:** start.rs had `println!("...", cockpit_url)` instead of `println!("...{cockpit_url}...")`
- **Fix:** Changed to inlined format args
- **Files modified:** start.rs (2 occurrences)
- **Verification:** just lint passes
- **Committed in:** e4de1a8

**3. [Rule 3 - Blocking] Fixed dead_code warnings for urls module**
- **Found during:** Task 2
- **Issue:** Previous plan (16-02) added urls module but some functions not yet integrated, causing dead_code warnings
- **Fix:** Added #![allow(dead_code)] to urls.rs (functions intended for future use)
- **Files modified:** urls.rs
- **Verification:** just lint passes
- **Committed in:** e4de1a8

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All auto-fixes necessary for lint compliance. No scope creep.

## Issues Encountered
- Plan 16-02 was executed in parallel, adding URL helpers and partially refactoring some commands. This required coordinating changes to avoid conflicts but ultimately both plans complemented each other.

## Next Phase Readiness
- Code quality improved: Docker error formatting now defined once, used everywhere
- Pattern established: shared CLI output helpers in output/ module
- Ready for Plan 16-02 (if not already complete) or Phase 17

---
*Phase: 16-code-quality-audit*
*Completed: 2026-01-24*
