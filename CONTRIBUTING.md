# Contributing to opencode-cloud

Thank you for your interest in contributing to opencode-cloud! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- **Rust 1.85+** (for Rust 2024 edition)
- **Node.js 20+**
- **pnpm 9+**
- **just** (task runner)

### Installation

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install just
cargo install just
# or: brew install just

# Install pnpm
npm install -g pnpm

# Clone the repository
git clone https://github.com/pRizz/opencode-cloud.git
cd opencode-cloud

# Install dependencies
pnpm install

# Build everything
just build
```

### Running Tests

```bash
# Run all tests
just test

# Run only Rust tests
just test-rust

# Run only Node tests
just test-node
```

### Linting and Formatting

```bash
# Check linting
just lint

# Auto-format code
just fmt
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect meaning (formatting, etc.)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding or correcting tests
- `chore`: Changes to build process or auxiliary tools

### Examples

```
feat(cli): add --json flag for machine-readable output
fix(config): handle missing config directory on first run
docs(readme): add installation instructions for Windows
```

## Pull Request Process

1. **Fork** the repository and create your branch from `main`
2. **Make your changes** following our coding standards
3. **Write tests** for any new functionality
4. **Ensure all tests pass**: `just test`
5. **Ensure linting passes**: `just lint`
6. **Update documentation** if needed
7. **Submit a pull request** with a clear description

## Project Structure

```
opencode-cloud/
├── packages/
│   ├── core/           # Rust core library + NAPI bindings
│   ├── cli-rust/       # Rust CLI binary
│   └── cli-node/       # Node.js CLI wrapper
├── Cargo.toml          # Rust workspace root
├── package.json        # Node.js workspace root
├── pnpm-workspace.yaml # pnpm workspace config
└── justfile            # Task orchestration
```

## Code Style

### Rust

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer `?` for error propagation over `unwrap()`
- Document public APIs

### TypeScript

- Use strict mode
- Follow ESM conventions
- Keep the Node CLI thin - logic belongs in Rust core

## Questions?

Feel free to open an issue for any questions about contributing.
