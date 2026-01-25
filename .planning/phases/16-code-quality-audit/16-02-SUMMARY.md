---
phase: 16
plan: 02
subsystem: cli-output
tags: [refactoring, deduplication, urls, code-quality]

dependency-graph:
  requires: [16-01]
  provides: [shared-url-formatting, resolve-remote-addr, format-cockpit-url]
  affects: []

tech-stack:
  added: []
  patterns: [helper-module, centralized-utilities]

key-files:
  created:
    - packages/cli-rust/src/output/urls.rs
  modified:
    - packages/cli-rust/src/output/mod.rs
    - packages/cli-rust/src/commands/start.rs
    - packages/cli-rust/src/commands/status.rs

decisions:
  - key: url-helper-location
    choice: output/urls.rs module
    reason: Consistent with output module pattern established in 16-01

metrics:
  duration: 6 min
  completed: 2026-01-24
  tests-before: 236
  tests-after: 236
---

# Phase 16 Plan 02: URL Formatting Deduplication Summary

Centralized URL formatting helpers to eliminate code duplication for Cockpit URL display and remote host address resolution.

## One-Liner

Created `urls.rs` module with `resolve_remote_addr()`, `format_cockpit_url()`, `normalize_bind_addr()` eliminating 4 duplications in start.rs and 2 in status.rs.

## What Was Done

### Task 1: Create shared urls.rs module
- Created `packages/cli-rust/src/output/urls.rs` with:
  - `resolve_remote_addr(host_name)` - looks up host config and returns hostname
  - `normalize_bind_addr(bind_addr)` - converts 0.0.0.0/:: to 127.0.0.1
  - `format_cockpit_url(remote_addr, bind_addr, port)` - formats Cockpit URL
  - `format_service_url(remote_addr, bind_addr, port)` - formats service URL
- Added comprehensive unit tests (12 tests)
- Updated `mod.rs` to export new functions

### Task 2: Update start.rs to use shared URL helpers
- Replaced inline `maybe_remote_addr` resolution (2 instances) with `resolve_remote_addr()`
- Replaced inline Cockpit URL formatting (2 instances) with `format_cockpit_url()`
- Replaced inline wildcard normalization in `open_browser_if_requested()` with `normalize_bind_addr()`
- Removed `load_hosts` import (now internal to URL module)
- Net change: -103 lines, +13 lines

### Task 3: Update status.rs to use shared URL helpers
- Replaced inline `maybe_remote_addr` resolution with `resolve_remote_addr()`
- Replaced inline Cockpit URL formatting with `format_cockpit_url()`
- Removed `load_hosts` import
- Net change: -55 lines, +15 lines

### Task 4: Verification
- All quality checks pass: fmt, lint, test, build
- 236 tests passing (89 CLI + 147 core)
- Clippy passes with 0 warnings
- Duplication confirmed eliminated in start.rs and status.rs

## Commits

| Hash | Message |
|------|---------|
| c50cab9 | feat(16-02): add centralized URL formatting module |
| e841491 | refactor(16-02): update start.rs to use shared URL helpers |
| f0674da | refactor(16-02): update status.rs to use shared URL helpers |

## Files Changed

### Created
- `packages/cli-rust/src/output/urls.rs` - URL formatting helpers with tests

### Modified
- `packages/cli-rust/src/output/mod.rs` - export new URL functions
- `packages/cli-rust/src/commands/start.rs` - use shared helpers (-103 lines)
- `packages/cli-rust/src/commands/status.rs` - use shared helpers (-55 lines)

## Deviations from Plan

None - plan executed exactly as written.

## Key Metrics

| Metric | Value |
|--------|-------|
| Lines removed | 158 |
| Lines added | 193 (includes 165 for new module with tests) |
| Net deduplication | 130 lines |
| Tests added | 12 |
| Tests total | 236 |

## Verification Results

```
just fmt    - passed
just lint   - passed (0 warnings)
just test   - passed (236 tests)
just build  - passed
```

## Notes

- The `format_service_url()` function was created for consistency but is not yet used
- Additional duplication exists in `cockpit.rs` and `setup.rs` but was out of plan scope
- The URL helper module follows the same pattern as the error helper module from 16-01
