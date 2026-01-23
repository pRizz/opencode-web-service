# Phase 11: Remote Host Management - Context

**Gathered:** 2026-01-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Allow occ to remotely install and interact with Docker containers running on different hosts. Users can add remote hosts, run existing occ commands against them, and manage a fleet of opencode instances from a single CLI.

</domain>

<decisions>
## Implementation Decisions

### Connection Method
- Use SSH tunnel for all remote Docker communication (not Docker TLS API)
- Works with existing SSH infrastructure, no Docker daemon exposure needed
- Auto-install Docker on remote if missing, but confirm with user first
- SSH key authentication only (no password fallback)
- Support full SSH config options: custom port, jump hosts (bastion), identity file

### Host Management
- CLI command (`occ host add`) AND config file editing both supported
- Hosts support groups/tags for organization: `occ host add prod-1 --group production`
- Full CRUD operations: add, remove, list, show, edit, test (connection test)
- Default host concept with local fallback:
  - If default host set, commands use it
  - If no default, commands use local Docker
  - Explicit `--host` always overrides

### Command Routing
- Per-command `--host` flag: `occ start --host prod-1`
- Single host per command (no multi-host in one command, use shell loops)
- ALL existing container commands support `--host`:
  - start, stop, restart, status, logs, update, cockpit
- Remote output prefixed with host name: `[prod-1] Container running...`

### Credential Handling
- Per-host identity_file in config, fallback to system SSH agent/config
- Validate SSH connection when adding host by default
- `--no-verify` flag to skip connection test on add
- Support SSH passphrase prompts for encrypted keys
- Retry 3 times with exponential backoff on connection failure

### Claude's Discretion
- SSH library choice (native vs external ssh command)
- Exact retry timing for backoff
- Host config file schema details
- Error message formatting

</decisions>

<specifics>
## Specific Ideas

- Should feel like running local commands, just on a different machine
- Host prefix on output helps identify source when running multiple commands
- Groups enable future expansion (e.g., "restart all production hosts")

</specifics>

<deferred>
## Deferred Ideas

- Multi-host commands in single invocation (e.g., `--group production`) — future phase
- Host health monitoring/dashboard — future phase
- Syncing config across hosts — future phase

</deferred>

---

*Phase: 11-remote-host-management*
*Context gathered: 2026-01-23*
