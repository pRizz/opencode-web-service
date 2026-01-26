---
phase: 21-use-opencode-fork-with-pam-auth
plan: 02
subsystem: documentation
tags: [documentation, pam-auth, legacy-fields, deprecation]
requires:
  - 21-01
provides:
  - README.md authentication section
  - Config schema deprecation comments
  - Legacy field migration guidance
affects:
  - User documentation
  - Developer documentation
key-files:
  modified:
    - README.md
    - packages/core/src/config/schema.rs
decisions:
  - name: "Add authentication section to README after Usage"
    rationale: "Logical placement - users see usage first, then learn about authentication"
  - name: "Keep legacy fields in schema with deprecation comments"
    rationale: "Backward compatibility - existing deployments won't break, but users are guided to migrate"
  - name: "Update has_required_auth() comment to clarify deprecation"
    rationale: "Method still checks legacy fields for compatibility, but comment clarifies they're deprecated"
metrics:
  duration: "~5 min"
  completed: "2026-01-26"
---

# Phase 21 Plan 02: Documentation Updates Summary

**One-liner:** Updated README.md with PAM authentication documentation and marked legacy auth_username/auth_password fields as deprecated in config schema comments.

## What Was Built

- **README.md authentication section:** Added comprehensive section after "Usage" covering:
  - PAM-based authentication explanation
  - User creation commands (`occ user add`)
  - User management commands (list, passwd, remove, enable, disable)
  - Legacy field deprecation notice
  - Migration guidance from legacy fields

- **Config schema deprecation comments:** Updated `auth_username` and `auth_password` field comments to:
  - Mark fields as DEPRECATED
  - Explain backward compatibility rationale
  - Direct users to `occ user add` for new deployments
  - Note that passwords are stored in /etc/shadow via PAM, not config files

- **has_required_auth() method comment:** Updated to clarify that:
  - PAM users (users array) are preferred
  - Legacy fields are deprecated but still checked for backward compatibility
  - New deployments should use `occ user add`

## Verification

- README.md contains clear PAM authentication documentation
- README.md explains legacy field deprecation
- Config schema comments mark legacy fields as deprecated
- Documentation is consistent across files
- Migration path is clear and actionable

## Notes

- packages/core/README.md was checked but doesn't contain authentication-specific documentation, so no changes were needed
- Legacy fields remain in schema for backward compatibility but are clearly marked as deprecated
- Documentation emphasizes using `occ user add` for new deployments
- Migration guidance is simple: create PAM user, legacy fields auto-clear on next config save
