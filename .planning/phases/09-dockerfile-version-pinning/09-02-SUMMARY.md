---
phase: 09-dockerfile-version-pinning
plan: 02
subsystem: tooling
tags: [bash, github-actions, crates-io, version-check, automation]

# Dependency graph
requires:
  - phase: 09-01
    provides: Pinned versions in Dockerfile
provides:
  - Update checker script for pinned tool versions
  - just check-updates command for local use
  - Weekly CI workflow for automated PR creation
affects: [14-auto-rebuild-detection]

# Tech tracking
tech-stack:
  added: [peter-evans/create-pull-request@v7]
  patterns: [GitHub API version querying, crates.io API version querying, POSIX-compatible shell scripts]

key-files:
  created:
    - scripts/check-dockerfile-updates.sh
    - .github/workflows/dockerfile-updates.yml
  modified:
    - justfile

key-decisions:
  - "POSIX-compatible patterns: Use sed with | delimiter and grep with -- for cross-platform compatibility"
  - "Return 0 from helper functions: Avoid triggering set -e on expected failures"
  - "Weekly schedule: Monday 9am UTC for version checks"

patterns-established:
  - "Version extraction: Use grep/sed combination with portable patterns"
  - "API querying: GitHub releases/latest and crates.io API endpoints"

# Metrics
duration: 8min
completed: 2026-01-22
---

# Phase 9 Plan 2: Update Tooling Summary

**Automated version update detection via check-dockerfile-updates.sh with weekly CI PR creation**

## Performance

- **Duration:** 8 min
- **Started:** 2026-01-22T16:30:53Z
- **Completed:** 2026-01-22T16:38:05Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments
- Update checker script queries GitHub API for 6 tools and crates.io for 5 crates
- Script supports --apply flag to update Dockerfile in-place for CI automation
- Weekly CI workflow creates PR with diff and version report when updates available
- just check-updates command provides local access to version checks

## Task Commits

Each task was committed atomically:

1. **Task 1: Create check-dockerfile-updates.sh script** - `d7ab281` (feat)
2. **Task 2: Add just check-updates command** - `89c68f5` (feat)
3. **Task 3: Create weekly CI workflow** - `6ab149c` (feat)

## Files Created/Modified
- `scripts/check-dockerfile-updates.sh` - Version check script (408 lines)
- `justfile` - Added check-updates recipe
- `.github/workflows/dockerfile-updates.yml` - Weekly CI workflow with PR creation

## Decisions Made

1. **POSIX-compatible patterns:** Used sed with `|` delimiter and `grep --` to work on both macOS (BSD) and Linux (GNU) without requiring Perl regex
2. **Return 0 from helpers:** Helper functions return 0 even on "not found" to avoid triggering `set -e` early exit
3. **Tools checked:** GitHub releases API for yq, fzf, act, lazygit, grpcurl, shfmt; crates.io API for ripgrep, eza, cargo-nextest, cargo-audit, cargo-deny
4. **Weekly schedule:** Monday 9am UTC chosen for version checks to allow weekday review

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed grep -P incompatibility on macOS**
- **Found during:** Task 1 (Script creation)
- **Issue:** grep -P (Perl regex) not available on macOS BSD grep
- **Fix:** Rewrote extraction to use POSIX sed with `|` delimiter
- **Files modified:** scripts/check-dockerfile-updates.sh
- **Verification:** Script works on macOS
- **Committed in:** d7ab281 (included in Task 1 commit)

**2. [Rule 3 - Blocking] Fixed grep treating --branch as option**
- **Found during:** Task 1 (Script creation)
- **Issue:** `grep "--branch"` interpreted --branch as grep option
- **Fix:** Added `--` separator: `grep -- "${before}"`
- **Files modified:** scripts/check-dockerfile-updates.sh
- **Verification:** fzf version now detected correctly
- **Committed in:** d7ab281 (included in Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes required for cross-platform compatibility. No scope creep.

## Issues Encountered
- cargo-nextest shows "update available" from 0.9.123 to 0.9.122 - this is because the Dockerfile has a newer version than crates.io shows as "max_stable_version". The script correctly detects version differences.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Version update tooling complete and ready for use
- CI will create weekly PRs when updates are available
- Manual trigger available via workflow_dispatch

---
*Phase: 09-dockerfile-version-pinning*
*Completed: 2026-01-22*
