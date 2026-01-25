# Phase 18: CLI Sync Strategy - Context

**Gathered:** 2026-01-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Establish patterns and processes to keep the Rust CLI (`packages/cli-rust`) and Node CLI (`packages/cli-node`) feature-complete and behavior-consistent. This phase creates the sync strategy - actual CLI consolidation is Phase 22.

</domain>

<decisions>
## Implementation Decisions

### Source of Truth
- Rust CLI is the authoritative implementation
- Node CLI delegates ALL commands to Rust via generic passthrough
- Passthrough is fully automatic - Node passes all args to Rust binary, zero configuration
- No Node-specific commands - everything delegates, including --version and --help

### Distribution Model
- Node CLI bundles prebuilt Rust binary for each platform
- Compile-on-install approach deferred to later phase (partially implemented already)
- Goal: `npm install` gives working CLI without requiring Rust toolchain

### Parity Testing
- Tests verify passthrough works (Node correctly spawns Rust and returns output)
- Cover ALL commands (not just a subset)
- Discover commands dynamically by parsing `occ --help` output
- Tests live in `packages/cli-node`
- More detailed verification (exact output, exit codes) deferred to later phase

### New Command Process
- Adding a command to Rust CLI automatically makes it available in Node CLI
- Document in: CONTRIBUTING.md + inline comments + README
- No registration required - fully automatic passthrough

### Drift Detection (CI)
- CI verifies: binary exists AND passthrough tests pass
- Failure behavior: fail the build (blocks merge/release)
- Prefer adding to existing CI workflows if feasible; otherwise create dedicated `cli-parity.yml`

### Claude's Discretion
- Exact passthrough implementation details
- Binary bundling mechanism (postinstall script, optionalDependencies, etc.)
- Help output parsing strategy for command discovery

</decisions>

<specifics>
## Specific Ideas

- "Could we make it so that the node cli essentially just delegates every argument to the rust cli?"
- "Can the delegation be made generic so that updates to the rust cli do not require code changes on the node end?"
- User wants simplicity: bundle binary by default, compile-on-install is a later concern

</specifics>

<deferred>
## Deferred Ideas

- Compile-on-install as fallback option (already partially implemented)
- More comprehensive parity testing (exact output comparison, exit code verification)
- Actual CLI consolidation and deduplication (Phase 22)

</deferred>

---

*Phase: 18-cli-sync-strategy*
*Context gathered: 2026-01-25*
