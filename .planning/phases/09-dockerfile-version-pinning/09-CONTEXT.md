# Phase 9: Dockerfile Version Pinning - Context

**Gathered:** 2026-01-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Pin explicit versions for tools installed from GitHub in the Dockerfile to improve security and reproducibility. Includes creating an update script to discover and apply version updates, with CI automation for weekly checks.

</domain>

<decisions>
## Implementation Decisions

### Scope of Pinning
- **Pin everything**: All apt packages, GitHub-installed tools, and language runtimes get explicit versions
- **Security exceptions**: Let security-critical packages float (ca-certificates, openssl) to receive automatic updates
- **Base image**: Use version tag (ubuntu:24.04) not digest — allows LTS security patches
- **Language runtimes**: Pin to minor version (20.x, 3.12.x) — allows patch updates
- **Installer scripts**: Trust curl|bash installers (rustup, etc.) to manage their own versions
- **Checksum verification**: Verify SHA256 checksums for compilers, runtimes, and CLI tools that touch source code

### Version Format
- **GitHub releases**: Use release tags (v1.2.3), not commit SHAs
- **Apt packages**: Use major.minor wildcard (git=1:2.43.*) — allows patch updates
- **Inline versions**: Keep version numbers inline in download URLs for locality
- **Pre-release tools (0.x.y)**: Claude's discretion per tool based on breaking change risk

### Documentation Approach
- **Inline comments**: Document each tool with version, date, and purpose
  - Format: `# tool vX.Y.Z (YYYY-MM-DD) - purpose description`
- **Mark exceptions**: Explicitly mark unpinned packages with `# UNPINNED: package - reason`
- **Dockerfile only**: No separate VERSIONS.md file — Dockerfile is source of truth

### Update Workflow
- **Update script**: Bash script in scripts/ directory using curl and jq
- **Script capabilities**:
  - Reports available updates with changelog links
  - Optional --apply flag to update Dockerfile directly
- **Just command**: Add `just check-updates` command
- **CI automation**: Weekly GitHub Action creates single PR with all updates
- **PR content**: Include changelog summaries and links in PR body

### Claude's Discretion
- Identifying which specific packages are security-critical exceptions
- Evaluating pre-release (0.x.y) tools for exact vs wildcard pinning
- Checksum file format and verification approach

</decisions>

<specifics>
## Specific Ideas

- Comment format should include release date for freshness visibility: `# ripgrep v14.1.0 (2024-01-15) - fast grep replacement`
- CI PR should be comprehensive enough to review and merge in one go
- Weekly cadence balances freshness with review burden

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 09-dockerfile-version-pinning*
*Context gathered: 2026-01-22*
