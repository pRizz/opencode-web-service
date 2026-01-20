# Phase 6: Security and Authentication - Research

**Researched:** 2026-01-20
**Domain:** Container user management, network binding, rate limiting, PAM authentication
**Confidence:** HIGH

## Summary

This phase implements PAM-based authentication by managing Linux system users in the container (opencode authenticates against PAM), network binding controls (localhost by default with explicit opt-in for network exposure), and load balancer compatibility. The key architectural insight is that opencode-cloud does NOT implement authentication itself - it configures system users in the container, and opencode's server authenticates against PAM.

The codebase already uses bollard 0.18 for Docker operations. Container exec via `create_exec`/`start_exec` is the pattern for running user management commands (`useradd`, `usermod`, `passwd`, `chpasswd`) inside the running container. The Dockerfile uses Ubuntu 24.04 which has full PAM support. opencode's server already has a health endpoint at `/global/health` that returns `{ healthy: true, version: string }`.

**Primary recommendation:** Use bollard's exec API for all user management commands. Store usernames (not passwords) in config for persistence across rebuilds. Use `chpasswd` with stdin for password setting (non-interactive). Implement rate limiting in-memory using `std::collections::HashMap` with sliding window algorithm.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| bollard | 0.18 | Docker exec for user management commands | Already in project, mature, async |
| std::net | stdlib | IP address parsing and validation (IPv4/IPv6) | No external deps, handles `::1`, `::`, `0.0.0.0` |
| std::collections::HashMap | stdlib | Rate limiting state storage | Simple, in-memory, no external deps |
| rand | 0.9 | Secure password generation | Already in project from Phase 5 |
| dialoguer | 0.11 | Interactive prompts for confirmations | Already in project from Phase 5 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| chrono | already in project | Timestamp tracking for rate limiting | Rate limit window calculations |
| comfy-table | 7.x | User list display formatting | `occ user list` command output |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| In-memory rate limiting | rater crate | rater is lock-free but adds dependency; in-memory HashMap sufficient for single container |
| chpasswd | passwd interactive | passwd requires TTY; chpasswd reads from stdin non-interactively |

**Installation:**
```bash
# Already present - no new dependencies needed:
# bollard, rand, dialoguer, comfy-table, chrono
```

## Architecture Patterns

### Recommended Project Structure
```
packages/core/src/
├── config/
│   └── schema.rs            # Extend with security fields
├── docker/
│   ├── exec.rs              # NEW: Container exec wrapper
│   └── users.rs             # NEW: User management via exec

packages/cli-rust/src/
├── commands/
│   ├── user/                # NEW: User management commands
│   │   ├── mod.rs           # Router for add/remove/list/passwd/enable/disable
│   │   ├── add.rs
│   │   ├── remove.rs
│   │   ├── list.rs
│   │   ├── passwd.rs
│   │   └── enable.rs        # enable/disable in one file
│   ├── config.rs            # Extend for security config keys
│   └── start.rs             # Add security checks on startup
└── wizard/
    └── auth.rs              # Extend for first user creation
```

### Pattern 1: Container Exec for User Commands
**What:** Execute user management commands in running container via bollard
**When to use:** All `occ user` subcommands
**Example:**
```rust
// Source: bollard examples/exec.rs + docs.rs/bollard/exec
use bollard::Docker;
use bollard::models::ExecConfig;
use bollard::exec::StartExecResults;
use futures_util::StreamExt;

pub async fn exec_command(
    docker: &Docker,
    container: &str,
    cmd: Vec<&str>,
) -> Result<String, DockerError> {
    let exec = docker
        .create_exec(
            container,
            ExecConfig {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            },
        )
        .await?
        .id;

    let mut output = String::new();
    if let StartExecResults::Attached { mut output: stream, .. } =
        docker.start_exec(&exec, None).await?
    {
        while let Some(Ok(msg)) = stream.next().await {
            output.push_str(&msg.to_string());
        }
    }
    Ok(output)
}
```

### Pattern 2: Non-Interactive Password Setting
**What:** Set password via chpasswd with stdin (no TTY needed)
**When to use:** `occ user add` and `occ user passwd`
**Example:**
```rust
// Source: man chpasswd, bollard exec with attach_stdin
use bollard::models::ExecConfig;
use bollard::exec::StartExecOptions;

pub async fn set_user_password(
    docker: &Docker,
    container: &str,
    username: &str,
    password: &str,
) -> Result<(), DockerError> {
    // Create exec with stdin attached
    let exec = docker
        .create_exec(
            container,
            ExecConfig {
                attach_stdin: Some(true),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["chpasswd".to_string()]),
                ..Default::default()
            },
        )
        .await?
        .id;

    // Start exec and write password to stdin
    if let StartExecResults::Attached { mut input, .. } =
        docker.start_exec(&exec, Some(StartExecOptions { detach: false, ..Default::default() })).await?
    {
        let payload = format!("{}:{}\n", username, password);
        input.write_all(payload.as_bytes()).await?;
        input.shutdown().await?;
    }
    Ok(())
}
```

### Pattern 3: Rate Limiting with Sliding Window
**What:** Track auth failures per IP with increasing delays
**When to use:** Auth failure tracking (delegated to opencode, but configurable via opencode-cloud)
**Example:**
```rust
// Source: Shuttle blog + std patterns
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    attempts: HashMap<IpAddr, Vec<Instant>>,
    max_attempts: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn check(&mut self, ip: IpAddr) -> Result<(), Duration> {
        let now = Instant::now();
        let cutoff = now - self.window;

        let attempts = self.attempts.entry(ip).or_insert_with(Vec::new);
        attempts.retain(|t| *t > cutoff);

        if attempts.len() >= self.max_attempts as usize {
            // Calculate progressive delay: 1s, 5s, 30s, 5min
            let delay = match attempts.len() {
                0..=3 => Duration::from_secs(1),
                4..=6 => Duration::from_secs(5),
                7..=9 => Duration::from_secs(30),
                _ => Duration::from_secs(300),
            };
            return Err(delay);
        }

        attempts.push(now);
        Ok(())
    }
}
```

### Pattern 4: IP Address Validation (IPv4 and IPv6)
**What:** Parse and validate bind addresses
**When to use:** Config set bind_address, startup validation
**Example:**
```rust
// Source: doc.rust-lang.org/std/net
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn validate_bind_address(addr: &str) -> Result<IpAddr, String> {
    match addr {
        "localhost" => Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        _ => addr.parse::<IpAddr>().map_err(|_| {
            format!("Invalid IP address: {}. Use 127.0.0.1, ::1, 0.0.0.0, or ::", addr)
        })
    }
}

pub fn is_network_exposed(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => *ip == Ipv4Addr::UNSPECIFIED, // 0.0.0.0
        IpAddr::V6(ip) => *ip == Ipv6Addr::UNSPECIFIED, // ::
    }
}

pub fn is_localhost(addr: &IpAddr) -> bool {
    addr.is_loopback() // handles 127.0.0.1 and ::1
}
```

### Anti-Patterns to Avoid
- **Storing passwords in config:** Never store plaintext or hashed passwords. Passwords exist only in container's /etc/shadow via PAM.
- **Interactive password prompts via exec:** Use chpasswd with stdin, not passwd which requires TTY.
- **Blocking on rate limit:** Return delay duration, let caller decide to sleep or reject.
- **Exposing user enumeration:** Auth failures should be generic "Authentication failed" regardless of whether user exists.

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| IP address parsing | Regex matching | `std::net::IpAddr::parse()` | Handles IPv4, IPv6, zone IDs |
| Password generation | Custom random | `rand::Rng::sample_iter(Alphanumeric)` | Already in project, cryptographically secure |
| Container exec | Shell spawning | bollard `create_exec`/`start_exec` | Proper Docker API, async, handles attach |
| User creation | Manual /etc/passwd editing | `useradd -m -s /bin/bash` | Handles home dir, shell, groups |
| Password hashing | Manual crypt() | `chpasswd` command | Uses PAM's configured algorithm |
| TTY detection | Platform-specific | `std::io::IsTerminal` | Stable since Rust 1.70 |

**Key insight:** opencode handles authentication via PAM. opencode-cloud only manages the system users that PAM authenticates against.

## Common Pitfalls

### Pitfall 1: Container Not Running for User Commands
**What goes wrong:** `occ user add` fails silently or confusingly
**Why it happens:** User exec commands require running container
**How to avoid:** Check container state before any user command, return clear error: "Container not running. Start with `occ start` first."
**Warning signs:** Docker exec errors about container not found

### Pitfall 2: Password Visible in Process List
**What goes wrong:** `ps aux` shows password in arguments
**Why it happens:** Passing password as command argument to chpasswd
**How to avoid:** Always pipe password to stdin: `echo "user:pass" | chpasswd` or better, use bollard stdin attach
**Warning signs:** Security audit flags the CLI

### Pitfall 3: Removing Last User
**What goes wrong:** No users left to authenticate, service unusable
**Why it happens:** User removes only configured user
**How to avoid:** Check user count before removal, block with: "Cannot remove last user. Add another user first or use --force."
**Warning signs:** Service becomes inaccessible after user removal

### Pitfall 4: Network Exposure Without Warning
**What goes wrong:** User binds to 0.0.0.0 not understanding security implications
**Why it happens:** Config change seems harmless
**How to avoid:** Warn on `occ config set bind_address 0.0.0.0` AND on every start when exposed
**Warning signs:** Support requests about exposed services

### Pitfall 5: Rate Limit State Lost on CLI Restart
**What goes wrong:** Rate limiting doesn't persist, attacker can just wait for CLI restart
**Why it happens:** In-memory HashMap lost when occ process exits
**How to avoid:** Accept this limitation - rate limiting is primarily handled by opencode's server, not occ CLI. Document that rate limiting config is passed to opencode.
**Warning signs:** Confusion about where rate limiting is enforced

### Pitfall 6: IPv6 Bracket Confusion
**What goes wrong:** Users enter `[::1]:3000` when we just want `::1`
**Why it happens:** IPv6 URLs use brackets, but addresses alone don't
**How to avoid:** Accept both `::1` and `[::1]`, strip brackets during parsing
**Warning signs:** Validation rejects valid IPv6 addresses

## Code Examples

Verified patterns from official sources:

### User Creation via Container Exec
```rust
// Source: bollard docs + useradd man page
pub async fn create_user(
    client: &DockerClient,
    username: &str,
    shell: &str,
) -> Result<(), DockerError> {
    let cmd = vec![
        "useradd",
        "-m",              // create home directory
        "-s", shell,       // set shell (e.g., /bin/bash)
        username,
    ];

    exec_command(client.inner(), CONTAINER_NAME, cmd).await?;
    Ok(())
}
```

### Check If User Exists
```rust
// Source: Linux id command
pub async fn user_exists(
    client: &DockerClient,
    username: &str,
) -> Result<bool, DockerError> {
    let cmd = vec!["id", "-u", username];
    let result = exec_command(client.inner(), CONTAINER_NAME, cmd).await;
    Ok(result.is_ok())
}
```

### Lock/Unlock User Account
```rust
// Source: passwd man page
pub async fn lock_user(client: &DockerClient, username: &str) -> Result<(), DockerError> {
    let cmd = vec!["passwd", "-l", username];
    exec_command(client.inner(), CONTAINER_NAME, cmd).await?;
    Ok(())
}

pub async fn unlock_user(client: &DockerClient, username: &str) -> Result<(), DockerError> {
    let cmd = vec!["passwd", "-u", username];
    exec_command(client.inner(), CONTAINER_NAME, cmd).await?;
    Ok(())
}
```

### Health Endpoint Check
```rust
// Source: opencode.ai/docs/server
// opencode serves health at: GET /global/health
// Returns: { healthy: true, version: "..." }
pub async fn check_health(port: u16) -> Result<bool, reqwest::Error> {
    let url = format!("http://127.0.0.1:{}/global/health", port);
    let resp: serde_json::Value = reqwest::get(&url).await?.json().await?;
    Ok(resp.get("healthy").and_then(|v| v.as_bool()).unwrap_or(false))
}
```

### Config Schema Extension
```rust
// Source: Existing schema.rs pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    // ... existing fields ...

    /// Bind address for opencode web UI (default: "127.0.0.1")
    /// Use "0.0.0.0" or "::" for network exposure (requires explicit opt-in)
    #[serde(default = "default_bind_address")]
    pub bind_address: String,

    /// Trust proxy headers (X-Forwarded-For, etc.) for load balancer deployments
    #[serde(default)]
    pub trust_proxy: bool,

    /// Allow unauthenticated access when network exposed
    /// Requires double confirmation on first start
    #[serde(default)]
    pub allow_unauthenticated_network: bool,

    /// Maximum auth attempts before rate limiting
    #[serde(default = "default_rate_limit_attempts")]
    pub rate_limit_attempts: u32,

    /// Rate limit window in seconds
    #[serde(default = "default_rate_limit_window")]
    pub rate_limit_window_seconds: u32,

    /// List of usernames configured in container (for persistence tracking)
    /// Passwords are NOT stored here - only in container's /etc/shadow
    #[serde(default)]
    pub users: Vec<String>,
}

fn default_bind_address() -> String { "127.0.0.1".to_string() }
fn default_rate_limit_attempts() -> u32 { 5 }
fn default_rate_limit_window() -> u32 { 60 }
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Basic auth via environment vars | PAM authentication | User decision | opencode-cloud manages system users, opencode authenticates via PAM |
| opencode OPENCODE_SERVER_PASSWORD | PAM login | User decision (forked opencode) | More robust, standard Linux auth |
| atty crate for TTY | std::io::IsTerminal | Rust 1.70 | No external dep for TTY check |

**Deprecated/outdated:**
- `OPENCODE_SERVER_PASSWORD` environment variable: Still works in upstream opencode, but this deployment uses PAM fork
- Basic auth at CLI level: Replaced by PAM-based auth in container

## Open Questions

Things that couldn't be fully resolved:

1. **opencode PAM Integration Details**
   - What we know: CONTEXT.md says "opencode fork integrates PAM"
   - What's unclear: Exact fork details, how PAM is configured in opencode
   - Recommendation: Verify the fork exists and test PAM auth works before implementing. If fork doesn't exist, fall back to OPENCODE_SERVER_PASSWORD env var.

2. **Rate Limiting Enforcement Location**
   - What we know: Config has rate_limit_* fields
   - What's unclear: Does opencode server enforce rate limiting, or should opencode-cloud?
   - Recommendation: Pass rate limit config to opencode via environment or command args. If opencode doesn't support it, document as future enhancement.

3. **Health Endpoint Path**
   - What we know: Official opencode uses `/global/health`
   - What's unclear: Does PAM fork change this?
   - Recommendation: Use `/global/health`, make path configurable if needed

4. **User Persistence on Container Rebuild**
   - What we know: Users stored in config, recreated on rebuild
   - What's unclear: How to handle password prompts during automated rebuild
   - Recommendation: Store hashed passwords (from /etc/shadow) in config, restore via chpasswd -e on rebuild

## Sources

### Primary (HIGH confidence)
- bollard GitHub examples/exec.rs: https://github.com/fussybeaver/bollard/blob/master/examples/exec.rs
  - Container exec with output capture
- bollard exec module: https://docs.rs/bollard/latest/bollard/exec/index.html
  - CreateExecOptions, StartExecResults structs
- Linux chpasswd man page: https://man7.org/linux/man-pages/man8/chpasswd.8.html
  - Non-interactive password setting via stdin
- std::net Rust docs: https://doc.rust-lang.org/std/net/
  - IpAddr, Ipv4Addr, Ipv6Addr parsing and validation
- OpenCode server docs: https://opencode.ai/docs/server/
  - Health endpoint at /global/health, auth via environment vars

### Secondary (MEDIUM confidence)
- Shuttle rate limiting blog: https://www.shuttle.dev/blog/2024/02/22/api-rate-limiting-rust
  - Sliding window algorithm with HashMap
- Docker exec documentation: https://www.baeldung.com/linux/docker-run-interactive-tty-options
  - attach_stdin, TTY requirements

### Tertiary (LOW confidence)
- PAM fork of opencode: Not verified, based on CONTEXT.md statement
  - If fork doesn't exist, implementation may need adjustment

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - bollard already in project, std::net is stdlib
- Architecture: HIGH - exec pattern well-documented in bollard examples
- Pitfalls: HIGH - Common Docker and auth issues well-known
- PAM integration: MEDIUM - Depends on unverified fork assumption

**Research date:** 2026-01-20
**Valid until:** 2026-02-20 (30 days - stable domain)
