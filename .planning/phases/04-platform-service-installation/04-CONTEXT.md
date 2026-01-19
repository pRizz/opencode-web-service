# Phase 4: Platform Service Installation - Context

**Gathered:** 2026-01-19
**Status:** Ready for planning

<domain>
## Phase Boundary

Register the opencode-cloud service with systemd (Linux) and launchd (macOS) so it survives host reboots and auto-restarts on crash. Service files are placed in user-level directories by default (no root required). This phase adds `occ install` and `occ uninstall` commands.

</domain>

<decisions>
## Implementation Decisions

### Service registration commands
- Dedicated subcommands: `occ install` registers, `occ uninstall` removes
- `occ install` also starts the service immediately (install + start in one step)
- No confirmation prompt for install — user explicitly ran the command
- If already installed: prompt "Service already installed. Reinstall? [y/N]"
- `--force` flag skips the overwrite prompt (for scripting)
- `occ uninstall` stops the service if running, then removes registration
- `occ uninstall --volumes --force` required for data deletion (double confirmation for destructive action)

### Boot behavior
- Default: user-level (starts on login, no root required)
- System-level available via config: `occ config set boot_mode system`
- Clear warning shown when system-level selected (requires root)
- macOS: launchd user agent by default, system daemon if `boot_mode=system`
- Linux: systemd --user by default, system service if `boot_mode=system`
- Service name: `opencode-cloud` (matches project name)

### Restart policy
- Default: 3 retries on crash
- Fixed 5-second delay between restart attempts
- After retries exhausted: stop and stay stopped (requires manual intervention)
- User-configurable via config file: `occ config set restart_retries 5`

### Error handling & feedback
- Permission errors: clear message with fix instructions (e.g., "Run with sudo for system-level install")
- Docker not running: error and abort (don't proceed without Docker)
- Success output: detailed with paths (show where files were installed)
- `occ status` shows both running state AND installation status (e.g., "Status: running | Installed: yes (starts on login)")
- `occ uninstall` shows paths being removed
- `occ uninstall` when not installed: exit 0 with message "Service not installed." (idempotent)
- Spinner feedback during install/uninstall operations

### Claude's Discretion
- Dry-run flag (--dry-run) implementation
- Exact service file templates (systemd unit, launchd plist)
- Daemon reload mechanism details
- Exact spinner messages during operations

</decisions>

<specifics>
## Specific Ideas

- Behavior should feel familiar to users of `systemctl enable/disable` and `launchctl load/unload`
- User-level installation is the happy path — no root for typical usage
- System-level is there for power users who understand the implications

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-platform-service-installation*
*Context gathered: 2026-01-19*
