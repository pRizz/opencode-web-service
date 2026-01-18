---
phase: 01-project-foundation
plan: 01
subsystem: infra
tags: [rust, napi-rs, pnpm, monorepo, cli, clap]

# Dependency graph
requires: []
provides:
  - Rust workspace with core library and CLI
  - NAPI-RS bindings for Node.js
  - Node CLI wrapper
  - justfile task orchestration
  - GitHub templates and CONTRIBUTING.md
affects: [02-config-management, 03-docker-control]

# Tech tracking
tech-stack:
  added: [clap, napi-rs, pnpm, just, tokio, serde, console]
  patterns: [monorepo with packages/, workspace dependencies, feature flags for optional deps]

key-files:
  created:
    - Cargo.toml (workspace root)
    - packages/core/Cargo.toml
    - packages/core/src/lib.rs
    - packages/core/src/version.rs
    - packages/cli-rust/Cargo.toml
    - packages/cli-rust/src/main.rs
    - packages/cli-node/package.json
    - packages/cli-node/src/index.ts
    - justfile
    - pnpm-workspace.yaml
    - CONTRIBUTING.md
    - .github/ISSUE_TEMPLATE/bug_report.md
    - .github/ISSUE_TEMPLATE/feature_request.md
    - .github/PULL_REQUEST_TEMPLATE.md
  modified: []

key-decisions:
  - "Feature flag for NAPI: Use cargo feature 'napi' to conditionally compile NAPI bindings"
  - "Manual JS bindings: Generate index.js/index.d.ts manually due to napi-cli type generation issues"
  - "Dual crate-type: Core library uses both cdylib (for NAPI) and rlib (for Rust CLI)"

patterns-established:
  - "Workspace dependencies: Define versions in root Cargo.toml, use .workspace = true in packages"
  - "Feature-gated optional deps: Use dep:name syntax for optional dependencies"
  - "justfile orchestration: Use just targets to coordinate Rust and Node builds"

# Metrics
duration: 9min
completed: 2026-01-18
---

# Phase 01 Plan 01: Monorepo Structure Summary

**Rust monorepo with NAPI-RS Node bindings, dual CLIs (Rust + Node), and justfile build orchestration**

## Performance

- **Duration:** 9 min
- **Started:** 2026-01-18T19:41:10Z
- **Completed:** 2026-01-18T19:50:33Z
- **Tasks:** 4
- **Files modified:** 28+

## Accomplishments

- Rust workspace with core library and CLI binary compiling successfully
- NAPI-RS native module built for darwin-arm64
- Node CLI wrapper calling into Rust core via NAPI bindings
- Both `cargo run -p opencode-cloud -- --version` and `node packages/cli-node/dist/index.js --version` output "0.1.0"
- justfile with build, test, lint targets working end-to-end
- GitHub issue/PR templates and CONTRIBUTING.md for contributors

## Task Commits

Each task was committed atomically:

1. **Task 1: Create monorepo structure** - `f05e57d` (chore)
2. **Task 2: Create Rust core library** - `ec4d94d` (feat)
3. **Task 3: Create Rust CLI and Node CLI** - `bcef8db` (feat)
4. **Task 4: Build NAPI and verify Node CLI** - `bd9a033` (feat)

## Files Created/Modified

- `Cargo.toml` - Rust workspace root with member packages and workspace dependencies
- `packages/core/Cargo.toml` - Core library with NAPI feature flag
- `packages/core/src/lib.rs` - Library exports with conditional NAPI bindings
- `packages/core/src/version.rs` - Version functions for Rust and Node
- `packages/core/build.rs` - NAPI-RS build configuration
- `packages/core/package.json` - npm package for NAPI bindings
- `packages/core/index.js` - Manual JS loader for native bindings
- `packages/core/index.d.ts` - TypeScript type definitions
- `packages/cli-rust/Cargo.toml` - Rust CLI with opencode-cloud and occ binaries
- `packages/cli-rust/src/main.rs` - clap-based CLI with global flags and ASCII banner
- `packages/cli-node/package.json` - Node CLI wrapper package
- `packages/cli-node/src/index.ts` - Thin CLI calling into NAPI core
- `packages/cli-node/tsconfig.json` - TypeScript configuration
- `justfile` - Build orchestration for Rust and Node
- `pnpm-workspace.yaml` - pnpm workspace configuration
- `package.json` - Root package with just script wrappers
- `.gitignore` - Combined Rust + Node ignores
- `CONTRIBUTING.md` - Contributor guidelines
- `.github/ISSUE_TEMPLATE/bug_report.md` - Bug report template
- `.github/ISSUE_TEMPLATE/feature_request.md` - Feature request template
- `.github/PULL_REQUEST_TEMPLATE.md` - PR template

## Decisions Made

1. **NAPI feature flag**: Used cargo feature `napi` to conditionally compile NAPI bindings, allowing the core library to be used as both a Rust library and a Node native module without linker conflicts.

2. **Manual JS bindings**: The napi-cli was generating empty .d.ts files due to conditional compilation. Created manual `src/bindings.js` and `src/bindings.d.ts` files that are copied after the NAPI build.

3. **Dual crate-type**: Core library uses `crate-type = ["cdylib", "rlib"]` to support both NAPI (cdylib) and Rust CLI (rlib) consumers.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] NAPI linker errors with Rust CLI**
- **Found during:** Task 3 (Rust CLI build)
- **Issue:** NAPI symbols were missing when linking the Rust CLI binary because it was trying to link against cdylib symbols
- **Fix:** Made NAPI dependencies optional behind a cargo feature flag, conditionally compiled NAPI bindings
- **Files modified:** packages/core/Cargo.toml, packages/core/src/lib.rs
- **Verification:** `cargo build --workspace` succeeds, CLI runs
- **Committed in:** bcef8db

**2. [Rule 3 - Blocking] NAPI-CLI generating empty type definitions**
- **Found during:** Task 4 (NAPI build)
- **Issue:** napi-cli was generating empty index.d.ts files, breaking TypeScript compilation
- **Fix:** Created manual JS and TypeScript binding files in src/, copy them after NAPI build
- **Files modified:** packages/core/package.json, packages/core/src/bindings.js, packages/core/src/bindings.d.ts
- **Verification:** `just build` succeeds, Node CLI runs and outputs version
- **Committed in:** bd9a033

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes were necessary for the build to work. No scope creep.

## Issues Encountered

- clap `version` attribute requires static string, not runtime function call - used `env!("CARGO_PKG_VERSION")` instead
- Cargo warning about multiple bin targets sharing same source file - expected behavior for occ alias

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Monorepo foundation complete with working CLIs
- Ready for Plan 02: Config management implementation
- Patterns established: workspace dependencies, feature flags, justfile orchestration

---
*Phase: 01-project-foundation*
*Completed: 2026-01-18*
