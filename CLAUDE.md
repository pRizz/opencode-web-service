# Claude Code Instructions

## Pre-Commit Requirements

Before creating any git commit, you MUST run these checks in order:

1. **Format code:** `just fmt`
2. **Lint:** `just lint`
3. **Run tests:** `just test`
4. **Verify release build:** `just build`

Only proceed with the commit if all four pass. If any fail, fix the issues first.

## Project Structure

This is a polyglot monorepo with Rust and TypeScript:

- `packages/core/` - Rust core library with NAPI-RS bindings
- `packages/cli-rust/` - Rust CLI binary
- `packages/cli-node/` - Node.js CLI wrapper

## Key Commands

```bash
just build       # Build all packages
just test        # Run all tests
just fmt         # Format all code
just lint        # Lint all code
just clean       # Clean build artifacts
```

## Architecture Notes

- npm package uses compile-on-install (no prebuilt binaries)
- Users need Rust 1.82+ installed for npm install
- Config stored at `~/.config/opencode-cloud/config.json`
- Data stored at `~/.local/share/opencode-cloud/`
