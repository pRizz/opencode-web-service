---
phase: 09-dockerfile-version-pinning
uat_started: 2026-01-22
uat_completed: 2026-01-22
status: passed
tests_passed: 8
tests_failed: 0
issues_found: 1
issues_fixed: 1
---

# Phase 9: Dockerfile Version Pinning - UAT

## Test List

| # | Test | Status | Notes |
|---|------|--------|-------|
| 1 | Dockerfile has Version Pinning Policy header | passed | Header at line 20 with audit date 2026-01-22 |
| 2 | APT packages use version wildcards (pkg=X.Y.*) | passed | Found 10+ packages with =X.Y.* pattern |
| 3 | Security exceptions marked with # UNPINNED: comments | passed | 4 UNPINNED markers found |
| 4 | just check-updates runs without error | passed | Exit 0, shows 11 tools |
| 5 | Script shows tool/current/latest/status columns | passed | Table output with all columns |
| 6 | Script --help shows usage information | passed | Shows options, tools, and examples |
| 7 | No floating versions (@latest, /latest/) in Dockerfile | passed | Only comment reference found |
| 8 | Docker image builds successfully with pinned versions | passed | 7.46GB image built, 45 steps |

## Issues Found

### Issue 1: cargo-nextest version 0.9.123 doesn't exist (FIXED)

**Discovered:** During Docker build (Test 8)
**Symptom:** Build failed at step 31/45 - cargo install error
**Root cause:** Dockerfile pinned to `cargo-nextest@0.9.123` but this was a beta version. Latest stable is `0.9.122`.
**Fix:** Changed to `cargo-nextest@0.9.122` in Dockerfile
**Commit:** `807cb87`
**Verification:** Docker build now succeeds, update script shows all tools up-to-date

## Test Details

### Test 1: Dockerfile has Version Pinning Policy header
**Source:** 09-01-SUMMARY - "Added Version Pinning Policy documentation header"
**How to verify:** Check Dockerfile for Version Pinning Policy section with audit date
**Result:** PASSED - Header found at line 20 with complete policy documentation

### Test 2: APT packages use version wildcards
**Source:** 09-01-SUMMARY - "All APT packages use version wildcards (e.g., git=1:2.43.*)"
**How to verify:** Grep Dockerfile for =*.*\* patterns
**Result:** PASSED - Found patterns like git=1:2.43.*, curl=8.5.*, vim=2:9.1.*

### Test 3: Security exceptions marked with UNPINNED comments
**Source:** 09-01-SUMMARY - "Security-critical packages marked with # UNPINNED: comments"
**How to verify:** Check for UNPINNED markers on ca-certificates, gnupg, openssh-client
**Result:** PASSED - Found 4 UNPINNED markers (ca-certificates x2, gnupg, openssh-client)

### Test 4: just check-updates runs without error
**Source:** 09-02-SUMMARY - "just check-updates command provides local access"
**How to verify:** Run just check-updates and confirm exit code 0
**Result:** PASSED - Command runs successfully, shows version check results

### Test 5: Script shows tool/current/latest/status columns
**Source:** 09-02-SUMMARY - "Update checker script queries GitHub API for 6 tools and crates.io for 5 crates"
**How to verify:** Run script and confirm table output with columns
**Result:** PASSED - Table shows Tool/Current/Latest/Status for 11 tools (6 GitHub + 5 crates)

### Test 6: Script --help shows usage information
**Source:** Additional verification
**How to verify:** Run `./scripts/check-dockerfile-updates.sh --help`
**Result:** PASSED - Shows options (--apply, --help), environment variables (GITHUB_TOKEN), and tools checked

### Test 7: No floating versions in Dockerfile
**Source:** Additional verification
**How to verify:** Grep for `@latest` and `/latest/download` patterns in install commands
**Result:** PASSED - No floating versions found in actual install commands (only comment reference)

### Test 8: Docker image builds successfully
**Source:** Additional verification
**How to verify:** Run `docker build -f packages/core/src/docker/Dockerfile .`
**Result:** PASSED - Image built successfully (7.46GB, 45 steps)

Note: Initial attempt failed due to Issue 1 above. After fix, build succeeded.

## Summary

**8 passed, 0 failed** (1 issue found and fixed)

All Phase 9 deliverables verified through comprehensive testing including Docker build verification.

---
*UAT Session: 2026-01-22*
