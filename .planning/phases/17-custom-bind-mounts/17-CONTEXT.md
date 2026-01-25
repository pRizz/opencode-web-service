---
phase: 17-custom-bind-mounts
created: 2026-01-25
status: complete
---

# Phase 17: Custom Bind Mounts - Implementation Context

## Overview

Allow users to mount local filesystem directories into the Docker container for working with local project files.

## Decisions

### Mount Path Format & Validation

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Path format | Absolute paths only | Clearer intent, no ambiguity about resolution |
| Symlink handling | Follow symlinks | Mount the target directory, not the symlink itself |
| Validation timing | Both config time AND start time | Immediate feedback when adding, catch stale paths on start |
| Container path restrictions | Warn on system paths | Warn if mounting over /etc, /usr, /bin but allow with confirmation |

**Implementation notes:**
- Reject relative paths with error: "Mount paths must be absolute. Use: /full/path/to/dir"
- Follow symlinks using `std::fs::canonicalize()`
- System paths to warn on: `/etc`, `/usr`, `/bin`, `/sbin`, `/lib`, `/var`

### Config Storage & Management

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Config format | Array of strings | `mounts: ["/host/path:/container/path", "/other:/mnt/other:ro"]` |
| Management commands | Dedicated commands | `occ mount add`, `occ mount remove`, `occ mount list` |
| Default container path | Suggest /workspace | If user omits container path, suggest `/workspace/{dirname}` |
| Removal method | By host path | `occ mount remove /host/path` matches source |

**Config schema addition:**
```json
{
  "mounts": [
    "/home/user/projects:/workspace/projects",
    "/home/user/data:/mnt/data:ro"
  ]
}
```

**Command structure:**
- `occ mount add /host/path:/container/path[:ro]` - Add a mount
- `occ mount remove /host/path` - Remove by host path
- `occ mount list` - Show configured mounts

### CLI Flags & One-Time Mounts

| Decision | Choice | Rationale |
|----------|--------|-----------|
| --mount interaction | Additive | CLI mounts ADD to configured mounts for this run only |
| Multiple flags | Yes | `occ start --mount /a:/mnt/a --mount /b:/mnt/b` |
| Restart support | Start only | Restart uses existing mounts from container creation |
| Skip config flag | --no-mounts | `occ start --no-mounts` to skip configured mounts |

**Flag combinations:**
- `occ start` - Uses configured mounts only
- `occ start --mount /x:/y` - Uses configured mounts + CLI mount
- `occ start --no-mounts` - No mounts at all
- `occ start --no-mounts --mount /x:/y` - CLI mount only

### Status Display & Error UX

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Status format | Separate section | Mounts section in status output |
| Error format | Styled with suggestion | Error includes fix command |
| Mount source display | Show source | Distinguish (config) vs (cli) mounts |
| List format | Table format | Clean tabular output |

**Status output example:**
```
Mounts:
  /home/user/projects → /workspace/projects (config)
  /tmp/test          → /mnt/test           (cli, ro)
```

**Error output example:**
```
Error: Mount path not found
  /home/user/old-project

Did the directory move? Run: occ mount remove /home/user/old-project
```

**Mount list output:**
```
HOST PATH              CONTAINER PATH         MODE
/home/user/projects    /workspace/projects    rw
/home/user/data        /mnt/data              ro
```

## Non-Decisions (Claude handles)

- Docker bind mount API details
- Config file parsing implementation
- Error handling patterns (use existing output/errors.rs)
- Test structure

## Deferred Ideas

None captured during discussion.
