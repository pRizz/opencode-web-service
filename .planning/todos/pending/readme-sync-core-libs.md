# Todo: Separate README for Core Libraries

**Created:** 2026-01-19
**Priority:** low
**Tags:** tooling, documentation, npm, crates.io

## Description

Have a separate README for the core libraries which then populate both the Rust crate README content and the npm package README content so that when published, they show those custom READMEs.

## Context

Currently the root README is synced to npm packages. For better developer experience on crates.io and npm, each core library should have its own README that:
- Explains the specific package's purpose
- Shows installation for that package manager
- Has relevant API examples

## Acceptance Criteria

- [ ] Core library has its own README.md
- [ ] Rust crate (packages/core) uses this README when published to crates.io
- [ ] npm package (packages/cli-node or packages/core) uses this README when published to npm
- [ ] Build/release process automatically syncs or includes the README

## Notes

Related to existing todo about making README sync more robust.
