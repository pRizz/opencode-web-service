# Phase 9: Dockerfile Version Pinning - Research

**Researched:** 2026-01-22
**Domain:** Docker, supply chain security, version management
**Confidence:** HIGH

## Summary

This phase involves pinning explicit versions for all tools installed in the Dockerfile to improve security and reproducibility. The current Dockerfile installs approximately 20 GitHub-sourced tools and numerous apt packages without version constraints, creating supply chain risks and non-reproducible builds.

The research identifies current versions for all tools, documents the appropriate pinning strategy for each installation method (apt, cargo, go install, curl|bash scripts, git clone), and establishes patterns for a version update script that queries the GitHub API.

**Primary recommendation:** Use inline version variables with structured comments for all tools, verify SHA256 checksums for security-critical binaries, and create a bash script using `curl` and `jq` to detect available updates via GitHub's releases API.

## Standard Stack

### Core Tools for Update Script

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| curl | system | HTTP requests to GitHub API | Universally available, no deps |
| jq | system | JSON parsing for API responses | Industry standard for JSON in bash |
| sha256sum | system | Checksum verification | POSIX standard, always available |
| sed | system | In-place file modifications | POSIX standard |

### GitHub API Endpoint

The `/releases/latest` endpoint automatically returns the most recent non-prerelease, non-draft release:

```bash
curl -s "https://api.github.com/repos/OWNER/REPO/releases/latest" | jq -r '.tag_name'
```

No authentication required for public repositories.

## Current Tool Versions (January 2026)

### GitHub-Sourced Tools Requiring Version Pins

| Tool | Current Version | Source | Installation Method |
|------|-----------------|--------|---------------------|
| yq | v4.50.1 | mikefarah/yq | Direct binary download |
| fzf | v0.67.0 | junegunn/fzf | git clone |
| act | v0.2.84 | nektos/act | Install script |
| lazygit | v0.58.1 | jesseduffield/lazygit | go install |
| grpcurl | v1.9.3 | fullstorydev/grpcurl | go install |
| shfmt | v3.x (latest) | mvdan.cc/sh | go install |

### Rust Crates (cargo install)

| Crate | Current Version | Notes |
|-------|-----------------|-------|
| ripgrep | 15.1.0 | MSRV 1.85.0 |
| eza | 0.23.4 | MSRV 1.81.0 |
| cargo-nextest | 0.9.122 | MSRV 1.89.0 |
| cargo-audit | 0.22.0 | Security scanner |
| cargo-deny | 0.19.0 | License/security checker |

### Curl|Bash Installers (Trust to Self-Manage)

| Tool | Installer URL | Version Strategy |
|------|---------------|------------------|
| Oh My Zsh | ohmyzsh.sh | Let auto-update (disable in container) |
| Starship | starship.rs/install.sh | Supports VERSION env var |
| mise | mise.run | Latest via curl (self-updates) |
| rustup | sh.rustup.rs | Manages Rust versions |
| uv | astral.sh/uv/install.sh | Supports version in URL path |
| opencode | opencode.ai/install | Supports VERSION env var |

### Language Runtimes (via mise)

| Runtime | Current Pin | Strategy |
|---------|-------------|----------|
| Node.js | lts | Pin to minor (20.x) |
| Python | 3.12 | Already pinned to minor |
| Go | latest | Pin to minor (1.23.x) |

## Architecture Patterns

### Pattern 1: Inline Version Variables with Comments

**What:** Define version at download point with documentation comment
**When to use:** All GitHub binary downloads
**Example:**
```dockerfile
# yq v4.50.1 (2025-12-14) - YAML processor
# https://github.com/mikefarah/yq/releases
RUN curl -sL "https://github.com/mikefarah/yq/releases/download/v4.50.1/yq_linux_$(dpkg --print-architecture)" \
    -o /home/opencode/.local/bin/yq \
    && chmod +x /home/opencode/.local/bin/yq
```

### Pattern 2: Cargo Install with Version Flag

**What:** Pin cargo crates using `@version` syntax
**When to use:** All cargo install commands
**Example:**
```dockerfile
# Rust CLI tools - pinned versions (2026-01-22)
# ripgrep 15.1.0: https://github.com/BurntSushi/ripgrep/releases
# eza 0.23.4: https://github.com/eza-community/eza/releases
RUN . /home/opencode/.cargo/env \
    && cargo install --locked ripgrep@15.1.0 eza@0.23.4
```

### Pattern 3: Go Install with Version Tag

**What:** Pin Go tools using `@vX.Y.Z` syntax
**When to use:** All go install commands
**Example:**
```dockerfile
# lazygit v0.58.1 (2026-01-12) - Terminal UI for git
# https://github.com/jesseduffield/lazygit/releases
RUN eval "$(/home/opencode/.local/bin/mise activate bash)" \
    && go install github.com/jesseduffield/lazygit@v0.58.1
```

### Pattern 4: Git Clone with Specific Tag

**What:** Clone at specific tag instead of HEAD
**When to use:** Tools installed via git clone
**Example:**
```dockerfile
# fzf v0.67.0 (2025-11-16) - Fuzzy finder
# https://github.com/junegunn/fzf/releases
RUN git clone --branch v0.67.0 --depth 1 https://github.com/junegunn/fzf.git /home/opencode/.fzf \
    && /home/opencode/.fzf/install --all --no-bash --no-fish
```

### Pattern 5: Install Script with Version Parameter

**What:** Pass version to curl|bash installers that support it
**When to use:** act, uv (others trust to self-manage)
**Example:**
```dockerfile
# act v0.2.84 (2026-01-01) - Run GitHub Actions locally
# https://github.com/nektos/act/releases
RUN curl -sL https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash -s -- -b /home/opencode/.local/bin v0.2.84
```

### Pattern 6: APT Packages with Wildcards

**What:** Pin apt packages to major.minor with wildcard patch
**When to use:** All apt packages except security-critical ones
**Example:**
```dockerfile
# UNPINNED: ca-certificates - Security-critical, needs auto-updates
# UNPINNED: openssl - Security-critical, needs auto-updates
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git=1:2.43.* \
    curl=8.5.* \
    jq=1.7.* \
    && rm -rf /var/lib/apt/lists/*
```

### Pattern 7: Checksum Verification

**What:** Verify SHA256 checksums for security-critical binaries
**When to use:** Compilers, runtimes, CLI tools that touch source code
**Example:**
```dockerfile
# yq v4.50.1 with checksum verification
ARG YQ_VERSION=v4.50.1
ARG YQ_SHA256_AMD64=abc123...
ARG YQ_SHA256_ARM64=def456...

RUN ARCH=$(dpkg --print-architecture) \
    && if [ "$ARCH" = "amd64" ]; then EXPECTED_SHA="${YQ_SHA256_AMD64}"; \
       elif [ "$ARCH" = "arm64" ]; then EXPECTED_SHA="${YQ_SHA256_ARM64}"; fi \
    && curl -sL "https://github.com/mikefarah/yq/releases/download/${YQ_VERSION}/yq_linux_${ARCH}" -o /tmp/yq \
    && echo "${EXPECTED_SHA}  /tmp/yq" | sha256sum -c - \
    && mv /tmp/yq /home/opencode/.local/bin/yq \
    && chmod +x /home/opencode/.local/bin/yq
```

### Project Structure for Update Script

```
scripts/
  check-dockerfile-updates.sh    # Main update checker script
.github/workflows/
  dockerfile-updates.yml         # Weekly CI automation
```

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Version detection | Custom scraping | GitHub API `/releases/latest` | Stable, documented API |
| JSON parsing in bash | grep/awk parsing | jq | Handles edge cases, proper escaping |
| Checksum verification | Custom hash comparison | `sha256sum -c` | POSIX standard, exit codes |
| Scheduled CI jobs | Custom cron service | GitHub Actions schedule | Native integration, free |

**Key insight:** The GitHub releases API provides a stable, well-documented interface for version discovery. Don't scrape HTML or rely on file naming conventions.

## Common Pitfalls

### Pitfall 1: APT Package Exact Version Pinning

**What goes wrong:** Exact versions (`git=1:2.43.0-1ubuntu1`) become unavailable when Ubuntu updates mirrors
**Why it happens:** Ubuntu removes old package versions from mirrors after security updates
**How to avoid:** Use wildcard patch versions (`git=1:2.43.*`) to allow security patches
**Warning signs:** Docker build suddenly fails with "E: Version 'X' for 'Y' was not found"

### Pitfall 2: Forgetting --locked Flag for Cargo

**What goes wrong:** Builds become non-reproducible, may fail on dependency resolution
**Why it happens:** Without `--locked`, cargo resolves dependencies fresh each build
**How to avoid:** Always use `cargo install --locked` for reproducibility
**Warning signs:** Same Dockerfile produces different binary versions

### Pitfall 3: GitHub API Rate Limiting

**What goes wrong:** Update script fails with 403 errors
**Why it happens:** Unauthenticated requests limited to 60/hour
**How to avoid:** Check rate limits, add optional token support, cache responses
**Warning signs:** Script works locally but fails in CI

### Pitfall 4: Platform-Specific Checksums

**What goes wrong:** Checksum verification fails on different architectures
**Why it happens:** amd64 and arm64 binaries have different checksums
**How to avoid:** Store checksums per-architecture, detect at runtime
**Warning signs:** Works on x86 CI runner, fails on ARM

### Pitfall 5: Pre-release Versions from @latest

**What goes wrong:** `go install tool@latest` installs pre-release/unstable version
**Why it happens:** Go considers any tagged version, including alpha/beta
**How to avoid:** Pin to specific stable tag (`@v1.2.3`)
**Warning signs:** Unexpected behavior changes, "0.x" versions in production

### Pitfall 6: Floating Security Packages

**What goes wrong:** Forgetting to mark security exceptions, pinning ca-certificates
**Why it happens:** Treating all packages uniformly
**How to avoid:** Explicitly document `# UNPINNED: package - reason`
**Warning signs:** Container has outdated root certificates

## Code Examples

### GitHub API Version Query (verified pattern)

```bash
#!/usr/bin/env bash
# Source: https://docs.github.com/en/rest/releases/releases

get_latest_version() {
    local owner="$1"
    local repo="$2"
    curl -s "https://api.github.com/repos/${owner}/${repo}/releases/latest" \
        | jq -r '.tag_name'
}

# Example usage
YQ_LATEST=$(get_latest_version "mikefarah" "yq")
echo "Latest yq version: $YQ_LATEST"
```

### Dockerfile Version Extraction

```bash
#!/usr/bin/env bash
# Extract current version from Dockerfile comment

get_dockerfile_version() {
    local tool="$1"
    local dockerfile="$2"
    # Pattern: # tool vX.Y.Z (YYYY-MM-DD)
    grep -oP "# ${tool} v\K[0-9]+\.[0-9]+\.[0-9]+" "$dockerfile"
}

# Example
CURRENT=$(get_dockerfile_version "yq" "Dockerfile")
echo "Current yq in Dockerfile: v$CURRENT"
```

### Dockerfile Version Update (sed pattern)

```bash
#!/usr/bin/env bash
# Update version inline in Dockerfile

update_dockerfile_version() {
    local tool="$1"
    local old_version="$2"
    local new_version="$3"
    local dockerfile="$4"

    # Update comment: # tool vX.Y.Z (YYYY-MM-DD)
    sed -i "s/# ${tool} v${old_version}/# ${tool} v${new_version}/" "$dockerfile"

    # Update download URL
    sed -i "s/${old_version}/${new_version}/g" "$dockerfile"
}
```

### Weekly CI Workflow Structure

```yaml
# .github/workflows/dockerfile-updates.yml
name: Check Dockerfile Updates

on:
  schedule:
    - cron: '0 9 * * 1'  # Every Monday at 9am UTC
  workflow_dispatch:  # Allow manual trigger

jobs:
  check-updates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Check for updates
        id: check
        run: |
          ./scripts/check-dockerfile-updates.sh > updates.md
          if [ -s updates.md ]; then
            echo "updates_available=true" >> $GITHUB_OUTPUT
          fi

      - name: Create PR
        if: steps.check.outputs.updates_available == 'true'
        uses: peter-evans/create-pull-request@v5
        with:
          title: "chore(docker): update pinned versions"
          body-path: updates.md
          branch: dockerfile-version-updates
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `go install tool@latest` | `go install tool@vX.Y.Z` | Always best | Reproducible builds |
| No apt version pins | Wildcard pins `pkg=X.Y.*` | 2024+ | Balance reproducibility/security |
| Trust curl\|bash installers | Pin where possible, trust for self-managing | Current | Pragmatic security |
| `git clone` HEAD | `git clone --branch vX.Y.Z` | Always best | Reproducible builds |

**Deprecated/outdated:**
- Using `@master` or `@main` for go install (unreliable, may break)
- Exact apt version pins (become stale, break builds)
- Docker image SHA pinning for base image (blocks security updates for LTS)

## Security-Critical Exceptions

Based on the CONTEXT.md decision to let security-critical packages float:

| Package | Category | Reason to Float |
|---------|----------|-----------------|
| ca-certificates | TLS trust | Root cert updates critical for HTTPS |
| openssl | Cryptography | Vulnerability patches essential |
| gnupg | Cryptography | Key management security |

These should be explicitly marked with `# UNPINNED: pkg - security reason`

## Checksum Verification Candidates

Based on CONTEXT.md: "Verify SHA256 checksums for compilers, runtimes, and CLI tools that touch source code"

| Tool | Category | Checksum Source |
|------|----------|-----------------|
| yq | Source manipulation | GitHub release checksums.txt |
| act | CI execution | GitHub release checksums.txt |
| Node.js (via mise) | Runtime | mise handles verification |
| Python (via mise) | Runtime | mise handles verification |
| Go (via mise) | Compiler | mise handles verification |
| Rust (via rustup) | Compiler | rustup handles verification |

For cargo-installed tools, the `--locked` flag plus Cargo.lock provides integrity.

## Open Questions

1. **Exact apt package versions for Ubuntu 24.04**
   - What we know: General pattern is `pkg=X.Y.*`
   - What's unclear: Exact current versions in noble repositories
   - Recommendation: Query at plan time with `apt-cache policy` or accept wildcards

2. **mise version pinning granularity**
   - What we know: mise handles downloads and potentially checksums
   - What's unclear: Whether mise should be pinned itself
   - Recommendation: Trust mise as a version manager (it self-updates)

3. **GSD plugin version pinning**
   - What we know: Cloned from rokicool/gsd-opencode
   - What's unclear: Whether specific tags exist, release cadence
   - Recommendation: Clone at HEAD initially, add version pin if project starts tagging

## Sources

### Primary (HIGH confidence)
- [GitHub REST API - Releases](https://docs.github.com/en/rest/releases/releases) - Official API documentation
- [Docker Best Practices](https://docs.docker.com/build/building/best-practices/) - Official Docker guidance
- [yq releases](https://github.com/mikefarah/yq/releases) - v4.50.1 (2025-12-14)
- [fzf releases](https://github.com/junegunn/fzf/releases) - v0.67.0 (2025-11-16)
- [act releases](https://github.com/nektos/act/releases) - v0.2.84 (2026-01-01)
- [lazygit releases](https://github.com/jesseduffield/lazygit/releases) - v0.58.1 (2026-01-12)
- [cargo-nextest](https://crates.io/crates/cargo-nextest) - 0.9.122
- [cargo-audit](https://crates.io/crates/cargo-audit) - 0.22.0
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny/releases) - 0.19.0
- [uv installation](https://docs.astral.sh/uv/getting-started/installation/) - Supports version in URL
- [ripgrep releases](https://github.com/BurntSushi/ripgrep/releases) - 15.1.0
- [eza releases](https://github.com/eza-community/eza/releases) - 0.23.4
- [grpcurl releases](https://github.com/fullstorydev/grpcurl/releases) - v1.9.3
- [starship releases](https://github.com/starship/starship/releases) - v1.24.2
- [mise releases](https://github.com/jdx/mise/releases) - 2026.1.6

### Secondary (MEDIUM confidence)
- [Datadog APT pinning rule](https://docs.datadoghq.com/code_analysis/static_analysis_rules/docker-best-practices/apt-pin-version/) - Best practice guidance
- [Gist: Get latest release from GitHub](https://gist.github.com/lukechilds/a83e1d7127b78fef38c2914c4ececc3c) - curl/jq pattern

### Tertiary (LOW confidence)
- General web search results for version numbers (verified against GitHub releases)

## Metadata

**Confidence breakdown:**
- Tool versions: HIGH - Direct from GitHub releases pages
- APT pinning patterns: HIGH - Docker official docs
- Update script patterns: HIGH - GitHub API docs
- Checksum approach: MEDIUM - Standard practice, implementation varies
- Security exceptions list: MEDIUM - Based on common practice, project-specific

**Research date:** 2026-01-22
**Valid until:** 2026-02-22 (versions change frequently, re-verify before implementation)
