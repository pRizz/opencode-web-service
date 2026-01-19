# opencode-cloud

[![CI](https://github.com/pRizz/opencode-cloud/actions/workflows/ci.yml/badge.svg)](https://github.com/pRizz/opencode-cloud/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-ready toolkit for deploying [opencode](https://github.com/anomalyco/opencode) as a persistent cloud service.

## Features

- Cross-platform CLI (`opencode-cloud` / `occ`)
- Docker container management
- Service lifecycle commands (start, stop, status, logs)
- Platform service integration (systemd/launchd)
- XDG-compliant configuration
- Singleton enforcement (one instance per host)

## Requirements

### For npm installation

- **Node.js 20+**
- **Rust 1.82+** (for compiling native bindings)
  - Install via [rustup](https://rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### For cargo installation

- **Rust 1.82+**

## Installation

### Via npm (compiles from source)

```bash
npx opencode-cloud --version
```

Or install globally:

```bash
npm install -g opencode-cloud
occ --version
```

### Via cargo

```bash
cargo install opencode-cloud
opencode-cloud --version
```

### From source

```bash
git clone https://github.com/pRizz/opencode-cloud.git
cd opencode-cloud
just build
cargo run -p opencode-cloud -- --version
```

## Usage

```bash
# Show version
occ --version

# Start the service (builds image on first run)
occ start

# Start on a custom port
occ start --port 8080

# Start and open browser
occ start --open

# Check service status
occ status

# View logs
occ logs

# Follow logs in real-time
occ logs -f

# Stop the service
occ stop

# Restart the service
occ restart

# Install as a system service (starts on login/boot)
occ install

# Uninstall the system service
occ uninstall

# View configuration
occ config show
```

### Rebuilding the Docker Image

When developing locally or after updating opencode-cloud, you may need to rebuild the Docker image to pick up changes in the embedded Dockerfile:

```bash
# Force rebuild the image from scratch (no Docker cache)
occ start --rebuild
```

The `--rebuild` flag is essential for local development because:
- Docker caches layers from previous builds, so changes to the Dockerfile may not take effect
- It stops and removes any existing container before rebuilding
- It ensures you're running the latest version of the image with all changes applied

**When to use `--rebuild`:**
- After pulling updates to opencode-cloud
- When modifying the Dockerfile during development
- When the container fails to start due to image issues
- When you want a completely fresh environment

## Configuration

Configuration is stored at:
- Linux/macOS: `~/.config/opencode-cloud/config.json`

Data (PID files, etc.) is stored at:
- Linux/macOS: `~/.local/share/opencode-cloud/`

## Development

```bash
# Install dependencies
pnpm install

# Configure git hooks (once after cloning)
git config core.hooksPath .githooks

# Build everything
just build

# Compile and run occ (arguments automatically get passed to the binary)
just run --version

# Run tests
just test

# Format and lint
just fmt
just lint
```

> **Note:** The git hooks automatically sync `README.md` to npm package directories on commit.

## Architecture

This is a monorepo with:
- `packages/core` - Rust core library with NAPI-RS bindings
- `packages/cli-rust` - Rust CLI binary
- `packages/cli-node` - Node.js CLI wrapper (calls into core via NAPI)

The npm package compiles the Rust core on install (no prebuilt binaries).

## License

MIT
