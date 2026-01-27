# Phase 31: Update Rust and Node CLI READMEs with Latest Project Features - Context

**Gathered:** 2026-01-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Refresh the Rust CLI (`packages/cli-rust/README.md`) and Node CLI (`packages/cli-node/README.md`) documentation so they reflect the current feature set, installation model, and key commands. The goal is to align CLI package docs with the capabilities already described in the root README and implemented in the Rust CLI.
</domain>

<decisions>
## Documentation Decisions

### Source of Truth
- Rust CLI remains the authoritative implementation.
- Node CLI is a passthrough wrapper and should describe parity with Rust CLI.

### Scope
- Update the two CLI package READMEs only (no product changes).
- Keep existing structure where it is still accurate; only adjust outdated sections.

### Accuracy Requirements
- Reflect current install model for npm (prebuilt binaries, no Rust toolchain).
- Ensure commands/flags listed match actual CLI definitions.
</decisions>

<specifics>
## Features to Reflect

- Prebuilt image option (pull vs build) and rebuild flags
- PAM-based user management (`occ user ...`) and auth overview
- Remote host management (`occ host ...`, `--host` support)
- Cockpit web admin integration
- Bind mounts (`occ mount ...`)
- Update and maintenance workflow (`occ update`, health/status)
</specifics>

<deferred>
## Out of Scope

- Root README changes
- Any CLI behavior changes
- Windows support documentation beyond existing disclaimers
</deferred>

---

*Phase: 31-update-rust-and-node-cli-readmes-with-latest-project-features*
