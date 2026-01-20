---
phase: 06-security-and-authentication
plan: 01
subsystem: auth
tags: [docker, exec, users, pam, bollard, config]

# Dependency graph
requires:
  - phase: 02-docker-operations
    provides: DockerClient, bollard integration
provides:
  - Extended Config struct with security fields (bind_address, trust_proxy, rate_limit_*, users)
  - validate_bind_address function for IPv4/IPv6 parsing
  - Container exec wrapper (exec_command, exec_command_with_stdin, exec_command_exit_code)
  - User management operations (create_user, set_user_password, user_exists, lock_user, unlock_user, delete_user, list_users)
  - UserInfo struct for container user information
affects: [06-02, 06-03, 06-04, 06-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Container exec via bollard create_exec/start_exec
    - Secure password setting via chpasswd stdin (never in command args)
    - IP address validation using std::net::IpAddr

key-files:
  created:
    - packages/core/src/docker/exec.rs
    - packages/core/src/docker/users.rs
  modified:
    - packages/core/src/config/schema.rs
    - packages/core/src/config/mod.rs
    - packages/core/src/docker/mod.rs

key-decisions:
  - "Password via stdin: chpasswd reads username:password from stdin for security"
  - "Default bind_address: 127.0.0.1 (localhost-only by default)"
  - "Rate limit defaults: 5 attempts per 60 seconds"
  - "validate_bind_address accepts localhost, IPv4, IPv6, and bracketed [::1]"
  - "list_users filters by /home/ directory to exclude system users"

patterns-established:
  - "exec_command pattern: create_exec + start_exec + drain output stream"
  - "Secure password handling: never pass passwords as command arguments"
  - "UserInfo struct for user management responses"

# Metrics
duration: 6min
completed: 2026-01-20
---

# Phase 6 Plan 1: Config Schema and Exec Wrapper Summary

**Config security fields + Docker exec wrapper + user management module for PAM-based authentication foundation**

## Performance

- **Duration:** 6 min
- **Started:** 2026-01-20T21:17:12Z
- **Completed:** 2026-01-20T21:23:33Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments
- Extended Config struct with security fields: bind_address, trust_proxy, allow_unauthenticated_network, rate_limit_attempts, rate_limit_window_seconds, users
- Created validate_bind_address function supporting IPv4, IPv6, localhost, and bracketed IPv6 addresses
- Added is_network_exposed and is_localhost helper methods to Config
- Created Docker exec wrapper with exec_command, exec_command_with_stdin, and exec_command_exit_code functions
- Created user management module with create_user, set_user_password, user_exists, lock_user, unlock_user, delete_user, list_users operations
- Added UserInfo struct with username, uid, home, shell, locked fields

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend config schema with security fields** - `1da17a2` (feat)
2. **Task 2: Create Docker exec wrapper module** - `9649a94` (feat)
3. **Task 3: Create user management module** - `6e08d38` (feat)

## Files Created/Modified
- `packages/core/src/config/schema.rs` - Extended with security fields, validation, and helper methods
- `packages/core/src/config/mod.rs` - Re-export validate_bind_address
- `packages/core/src/docker/exec.rs` - Container exec wrapper using bollard
- `packages/core/src/docker/users.rs` - User management operations via container exec
- `packages/core/src/docker/mod.rs` - Module declarations and re-exports

## Decisions Made
- **Password via stdin:** Using chpasswd with stdin input ensures passwords never appear in command arguments or process listings
- **Default bind_address:** Set to "127.0.0.1" for secure localhost-only access by default
- **Rate limit defaults:** 5 attempts per 60 second window (configurable)
- **IP validation:** validate_bind_address handles localhost string, IPv4, IPv6, and bracketed IPv6 like [::1]
- **User filtering:** list_users filters by /home/ directory to exclude system users

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Config schema ready for security-related settings
- Docker exec wrapper ready for running commands in container
- User management module ready for PAM user operations
- Ready for 06-02: User Management CLI Commands

---
*Phase: 06-security-and-authentication*
*Completed: 2026-01-20*
