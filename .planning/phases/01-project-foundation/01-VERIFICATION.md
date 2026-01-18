---
phase: 01-project-foundation
verified: 2026-01-18T22:15:00Z
status: gaps_found
score: 4/5 must-haves verified
gaps:
  - truth: "Only one instance can run per host (singleton enforcement)"
    status: failed
    reason: "InstanceLock module exists in core but is not wired into CLI main.rs"
    artifacts:
      - path: "packages/core/src/singleton/mod.rs"
        issue: "Module implemented and tested, but not imported/used in CLI"
      - path: "packages/cli-rust/src/main.rs"
        issue: "Does not call InstanceLock::acquire() anywhere"
    missing:
      - "Import InstanceLock from opencode_cloud_core in main.rs"
      - "Call InstanceLock::acquire() before processing service commands (start/stop/restart)"
      - "Display rich error when lock acquisition fails showing existing PID"
---

# Phase 01: Project Foundation Verification Report

**Phase Goal:** Establish monorepo structure with working npm and cargo CLI entry points that can read/write configuration
**Verified:** 2026-01-18T22:15:00Z
**Status:** gaps_found
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | User can install via `npx opencode-cloud --version` and see version output | VERIFIED | `node packages/cli-node/dist/index.js --version` outputs "0.1.0" |
| 2 | User can install via `cargo install` (from local path) and run `opencode-cloud --version` | VERIFIED | `cargo build --release -p opencode-cloud` succeeds, binary outputs "opencode-cloud 0.1.0" |
| 3 | Configuration file is created at platform-appropriate XDG-compliant path | VERIFIED | Config created at `~/.config/opencode-cloud/config.json` |
| 4 | Configuration file format matches documented JSON schema | VERIFIED | Config fields match `schemas/config.schema.json`, uses `deny_unknown_fields` |
| 5 | Only one instance can run per host (singleton enforcement) | FAILED | `InstanceLock` module exists but is not wired into CLI |

**Score:** 4/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `Cargo.toml` | Rust workspace root | VERIFIED | Contains `[workspace]` with members, resolver = "2" |
| `packages/core/Cargo.toml` | Core library package | VERIFIED | Named "opencode-cloud-core", dual crate-type |
| `packages/cli-rust/Cargo.toml` | Rust CLI package | VERIFIED | Named "opencode-cloud", depends on core |
| `packages/core/src/lib.rs` | Core library exports | VERIFIED | 36 lines, exports config, singleton, version modules |
| `packages/cli-rust/src/main.rs` | CLI entry point with clap | VERIFIED | 177 lines, uses `#[derive(Parser)]` |
| `packages/core/src/config/mod.rs` | Config loading/saving | VERIFIED | 166 lines, exports `load_config`, `save_config` |
| `packages/core/src/config/paths.rs` | XDG path resolution | VERIFIED | 109 lines, exports `get_config_dir`, `get_config_path` |
| `packages/core/src/singleton/mod.rs` | PID lock singleton | VERIFIED | 250 lines, exports `InstanceLock`, `SingletonError` |
| `schemas/config.schema.json` | JSON Schema for config | VERIFIED | Contains `$schema`, defines version/port/bind/auto_restart |
| `schemas/config.example.jsonc` | Example config with comments | VERIFIED | Contains `//` comments |
| `justfile` | Task orchestration | VERIFIED | Contains `build:` target |
| `pnpm-workspace.yaml` | Node workspace config | VERIFIED | Contains `packages/*` |
| `packages/cli-node/dist/index.js` | Compiled Node CLI | VERIFIED | Contains `getVersionJs` call |
| `packages/core/core.darwin-arm64.node` | NAPI native binding | VERIFIED | Binary exists for darwin-arm64 |

### Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| `packages/cli-rust/src/main.rs` | `packages/core/src/lib.rs` | Cargo dependency | WIRED | `use opencode_cloud_core::{Config, config, get_version, load_config}` |
| `packages/cli-rust/src/main.rs` | `packages/core/src/config/mod.rs` | `load_config` call | WIRED | `let config = load_config()` called in main |
| `packages/cli-rust/src/main.rs` | `packages/core/src/singleton/mod.rs` | `InstanceLock::acquire` | NOT WIRED | No import of InstanceLock, no acquire call |
| `packages/cli-node/dist/index.js` | `packages/core/index.js` | NAPI binding | WIRED | `import { getVersionJs } from "@opencode-cloud/core"` |
| `packages/core/src/config/mod.rs` | `packages/core/src/config/paths.rs` | path resolution | WIRED | `pub use paths::{get_config_dir, ...}` |

### Requirements Coverage

| Requirement | Status | Blocking Issue |
|-------------|--------|----------------|
| INST-01 (Install via npx/cargo install) | SATISFIED | - |
| CONF-04 (Config at platform-appropriate location) | SATISFIED | - |
| CONF-07 (Config format matches JSON schema) | SATISFIED | - |
| CONS-01 (Single instance per host) | BLOCKED | Singleton not wired into CLI |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `packages/cli-rust/src/main.rs` | 38-43 | Comment placeholders "Future commands" | Info | Expected - commands deferred to later phases |
| `packages/cli-rust/src/main.rs` | 156-173 | `config set` not implemented | Info | Expected - noted as "not yet implemented" |

### Human Verification Required

None required - all verifiable items checked programmatically or via command execution.

### Gaps Summary

**One gap blocking full goal achievement:**

The singleton enforcement module (`packages/core/src/singleton/mod.rs`) is fully implemented with:
- PID file creation at `~/.local/share/opencode-cloud/opencode-cloud.pid`
- Stale lock detection (checks if PID is still running)
- Automatic cleanup on drop
- Comprehensive unit tests (5 passing tests)

However, this module is NOT wired into the CLI (`packages/cli-rust/src/main.rs`). The `main()` function:
1. Does NOT import `InstanceLock` from core
2. Does NOT call `InstanceLock::acquire()` before processing commands
3. Does NOT display the rich error message when another instance is running

**Fix required:** Add singleton lock acquisition to the CLI for service management commands (start/stop/restart). Per the PLAN, read-only commands (config show, status, version) should NOT acquire the lock.

---

*Verified: 2026-01-18T22:15:00Z*
*Verifier: Claude (gsd-verifier)*
