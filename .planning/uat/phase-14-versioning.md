# UAT Session: Phase 14 - Versioning and Release Automation

**Started:** 2026-01-23
**Status:** completed

## Testable Deliverables

### Plan 14-01: Docker Version Label
1. [x] Dockerfile has version build arg and label - PASS (ARG OPENCODE_CLOUD_VERSION at lines 58, 556; LABEL at line 69)
2. [x] Version file written to /etc/opencode-cloud-version - PASS (line 557)
3. [x] docker-publish.yml workflow exists - PASS

### Plan 14-02: Version Detection in CLI
4. [x] `occ status` shows CLI version - PASS (shows "CLI: v1.0.8")
5. [x] `occ status` shows image version when available - PASS (code at status.rs:197-209; filters "dev" builds)
6. [x] `occ start --ignore-version` flag exists - PASS (visible in --help)
7. [x] Version mismatch prompts user (when applicable) - PASS (code at start.rs:103-150)

### Plan 14-03: Version Bump Workflow
8. [x] version-bump.yml workflow exists with workflow_dispatch - PASS
9. [x] publish.yml accepts both v* and release/v* tags - PASS

## Test Results

### Code-Level Verification
- Dockerfile: ARG OPENCODE_CLOUD_VERSION present twice (for multi-stage)
- Dockerfile: LABEL org.opencode-cloud.version="${OPENCODE_CLOUD_VERSION}" present
- Dockerfile: RUN echo to /etc/opencode-cloud-version present
- version.rs module: get_cli_version(), get_image_version(), versions_compatible() all present
- start.rs: --ignore-version flag and version mismatch check present
- status.rs: CLI version display and image version display present
- docker-publish.yml: multi-arch build workflow present
- version-bump.yml: workflow_dispatch with patch/minor/major options
- publish.yml: triggers on both release/v* and v* tags

### Runtime Verification
- `occ --version` outputs "opencode-cloud 1.0.8"
- `occ start --help` shows --ignore-version flag with description
- `occ status` shows "CLI: v1.0.8" (image version hidden for dev builds)

## Summary

All 9 testable deliverables verified. Phase 14 implementation is complete and correct.
