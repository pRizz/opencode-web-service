# Phase 5: Interactive Setup Wizard - Context

**Gathered:** 2026-01-20
**Status:** Ready for planning

<domain>
## Phase Boundary

First-time configuration experience that guides users through username/password, port, hostname, and optional settings via an interactive CLI wizard. Also adds `occ setup`, `occ config`, `occ config get`, `occ config set`, `occ config reset`, and `occ config env` commands.

</domain>

<decisions>
## Implementation Decisions

### Wizard Trigger and Flow
- Wizard launches automatically when required config is missing (auth credentials)
- Users can rerun wizard via `occ setup` to generate fresh config
- Make this clear to the user during the wizard itself
- `occ setup` runs the exact same wizard as auto-triggered
- Linear flow only (forward-only) - if user needs to change something, finish and use `config set`
- Show step counter: "Step 2 of 5" style progress indicator
- Step order: Auth → Port → Hostname → Env vars (env vars skipped in wizard)
- Offer "quick setup" mode first: "Use defaults for everything except credentials? [y/N]"

### Wizard Completion
- After wizard completes, prompt: "Start opencode-cloud now? [Y/n]"
- Show full summary of all configured values (with password masked) before the "Start now?" prompt
- Offer to test configuration after setup: "Test configuration? [Y/n]"

### Cancel and Resume
- On Ctrl+C mid-wizard: confirm "Discard changes?" before exiting
- No partial saves - config stays as it was before wizard started

### Pre-checks
- Validate Docker is running/available before starting wizard - fail early with guidance
- Check port availability when user enters port - if in use, suggest next available port

### Re-running Setup
- `occ setup` shows current config summary first, then asks "Reconfigure? [y/N]"

### Credentials
- Allow user to choose between:
  - Randomly generated credentials (using secure platform crypto primitives - make this clear)
  - Their own username/password
- Password confirmation required: prompt twice, must match
- Username validation: non-empty, alphanumeric + underscore, 3-32 chars
- No password requirements - any non-empty password accepted (user's responsibility)

### Import Existing Config
- Detect existing opencode installation and offer to import settings
- Import all compatible settings that map to opencode-cloud config
- Warn per item for incompatible/unknown settings as encountered

### Non-Interactive Mode
- Fail with guidance when no TTY: "No TTY detected. Use occ config set or provide config file."
- Add `--yes` or `--non-interactive` flag for scripted deployments (requires auth to be pre-set)

### Visual Style
- Colored prompts for emphasis but no heavy styling (like cargo/npm)
- Not minimal plain text, not full box characters - middle ground

### Prompt Behavior
- Default values displayed on separate line above input: "Port (default: 3000):"
- Password input: hidden (nothing shown as user types)
- Validation happens on submit (Enter pressed), not while typing
- Infinite retries on invalid input - user can Ctrl+C to exit
- Empty input skips optional fields - no explicit skip option needed

### Port Configuration
- Valid range: 1-65535 (full range)
- Warn if port < 1024 requires elevated privileges
- Default port: 3000

### Hostname Configuration
- Default: localhost
- Explain difference clearly: localhost (local machine only) vs 0.0.0.0 (network accessible)
- Basic validation - warn if hostname/IP seems invalid

### Config Command Output
- `occ config` defaults to table format with aligned columns, passwords masked
- `--json` flag for JSON output
- Always show config file location at top/bottom

### Config Command Syntax
- Both flat keys and dotted paths supported: `occ config set port 3000` OR `occ config set service.port 3000`
- Make this clear to the user in help/documentation
- `occ config set password` prompts interactively only - never accept password as command argument
- `occ config get <key>` returns just the value (useful for scripts)
- `occ config reset` prompts for confirmation, `--force` skips prompt

### Config Editability
- All config values editable via `config set` - none read-only
- Config changes saved immediately while service running
- Show warning: "Restart required for changes to take effect"

### Environment Variables
- Stored in config as array with clear naming (e.g., `container_env`) so user knows it's passed to container
- Format: `["KEY=value", "KEY2=value2"]`
- Dedicated subcommands: `occ config env set KEY=value`, `occ config env list`, `occ config env remove KEY`
- Not prompted during wizard - users add later via `occ config env`
- No validation - accept any KEY=value pair

### Claude's Discretion
- Exact step text and wording
- Error message phrasing
- Which settings are considered "compatible" for import
- Test configuration implementation details

</decisions>

<specifics>
## Specific Ideas

- Quick setup mode should feel efficient - one yes/no to skip most prompts
- Import detection should feel helpful, not intrusive - only offer if something is found
- Visual style should match cargo/npm - familiar to developers

</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope

</deferred>

---

*Phase: 05-interactive-setup-wizard*
*Context gathered: 2026-01-20*
