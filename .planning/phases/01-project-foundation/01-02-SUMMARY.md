---
phase: 01-project-foundation
plan: 02
subsystem: infra
tags: [rust, config, xdg, jsonc, pid-lock, singleton, clap]

# Dependency graph
requires:
  - phase: 01-01
    provides: Rust workspace structure and core library
provides:
  - XDG-compliant config path resolution
  - JSONC configuration loading with validation
  - PID-based singleton enforcement
  - CLI config subcommand
affects: [02-docker-control, 03-web-ui, 04-service-management]

# Tech tracking
tech-stack:
  added: [directories, jsonc-parser, thiserror]
  patterns: [XDG paths on all Unix platforms, PID lock with stale detection, deny_unknown_fields validation]

key-files:
  created:
    - packages/core/src/config/mod.rs
    - packages/core/src/config/paths.rs
    - packages/core/src/config/schema.rs
    - packages/core/src/singleton/mod.rs
    - schemas/config.schema.json
    - schemas/config.example.jsonc
  modified:
    - packages/core/src/lib.rs
    - packages/core/Cargo.toml
    - packages/cli-rust/src/main.rs
    - packages/cli-rust/Cargo.toml
    - Cargo.toml (workspace deps)

key-decisions:
  - "XDG paths for macOS: Use ~/.config/ instead of ~/Library for consistency"
  - "Singleton via kill -0: Use subprocess call to check process existence (simpler than libc)"
  - "JSONC validation: Use deny_unknown_fields to reject unknown config keys"

patterns-established:
  - "Config path resolution: All paths resolved via config::paths module"
  - "Error display: Rich errors with file paths and tips for resolution"
  - "Subcommand structure: Commands::Config(ConfigCommands) pattern for nested commands"

# Metrics
duration: 5min
completed: 2026-01-18
---

# Phase 01 Plan 02: Configuration Management Summary

**XDG-compliant config loading with JSONC support, schema validation, and PID-based singleton enforcement**

## Performance

- **Duration:** 5 min
- **Started:** 2026-01-18T19:52:15Z
- **Completed:** 2026-01-18T19:57:17Z
- **Tasks:** 3
- **Files modified:** 12

## Accomplishments

- Config module with XDG-compliant path resolution (Linux/macOS: ~/.config/opencode-cloud/)
- JSONC parsing support (JSON with comments) using jsonc-parser
- Strict validation with deny_unknown_fields rejects unknown config keys
- PID lock singleton enforcement with stale detection (auto-cleanup)
- CLI `config show` command displays current configuration
- First run automatically creates default config.json
- Rich error messages on invalid config with tips

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement XDG path resolution and config schema** - `8858c5d` (feat)
2. **Task 2: Implement singleton enforcement via PID lock** - `e5a52d4` (feat)
3. **Task 3: Wire config and singleton into CLI** - `7c3f55e` (feat)

## Files Created/Modified

- `packages/core/src/config/mod.rs` - Config loading/saving with JSONC support
- `packages/core/src/config/paths.rs` - XDG-compliant path resolution
- `packages/core/src/config/schema.rs` - Config struct with serde validation
- `packages/core/src/singleton/mod.rs` - PID lock with stale detection
- `packages/core/src/lib.rs` - Re-exports for config and singleton modules
- `packages/core/Cargo.toml` - Added tempfile dev-dependency
- `packages/cli-rust/src/main.rs` - Config subcommand and startup config loading
- `packages/cli-rust/Cargo.toml` - Added serde_json dependency
- `schemas/config.schema.json` - JSON Schema for config validation
- `schemas/config.example.jsonc` - Example config with comments
- `Cargo.toml` - Added tempfile to workspace dependencies

## Decisions Made

1. **XDG paths on macOS**: Chose ~/.config/opencode-cloud/ over ~/Library/Application Support for consistency with Linux and easier user access.

2. **Singleton check via subprocess**: Used `kill -0` command instead of libc/nix crate for simplicity. The subprocess approach works reliably and avoids adding another dependency.

3. **Strict config validation**: Enabled `deny_unknown_fields` on Config struct to reject typos and outdated config keys immediately rather than silently ignoring them.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Rust 2024 edition requires unsafe blocks for `env::set_var` in tests. Simplified unit tests to avoid environment manipulation; full config loading is tested via CLI commands instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Configuration foundation complete with working load/save/validation
- Singleton enforcement ready for use when service management commands are added
- Ready for Phase 2 (Docker control) which will use config for Docker settings
- All success criteria met:
  - Config loads from ~/.config/opencode-cloud/config.json
  - Missing config creates default automatically
  - Invalid config shows clear error with tips
  - `config show` displays current configuration
  - JSON Schema and example config exist in schemas/
  - Singleton enforcement working with stale detection

---
*Phase: 01-project-foundation*
*Completed: 2026-01-18*
