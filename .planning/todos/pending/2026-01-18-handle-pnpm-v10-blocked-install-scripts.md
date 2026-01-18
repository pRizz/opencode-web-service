---
created: 2026-01-18T16:18
title: Handle pnpm v10 blocked install scripts
area: tooling
files:
  - packages/core/package.json
  - packages/cli-node/src/index.ts
  - README.md
---

## Problem

In pnpm v10, dependency lifecycle scripts (preinstall/install/postinstall) are **blocked by default** for security (supply-chain hardening). Our `@opencode-cloud/core` package uses a `postinstall` script to compile Rust via NAPI-RS.

This means pnpm users will experience:
- Package installs "successfully" but the `.node` binary is missing
- Runtime errors when trying to use the CLI
- Confusing failure mode with no clear guidance

pnpm's allowlist workflow requires users to:
1. Run `pnpm approve-builds` and approve the package
2. Run `pnpm rebuild @opencode-cloud/core`

But users won't know this unless we tell them.

## Solution

Two-pronged approach:

**1. Runtime guard with actionable error**

In the Node CLI entry point, check if the `.node` binary exists before proceeding:

```typescript
// Detect pnpm
const isPnpm = process.env.npm_config_user_agent?.includes('pnpm');

// Check if binary exists
if (!binaryExists) {
  if (isPnpm) {
    console.error(`
It looks like pnpm blocked install scripts for @opencode-cloud/core.

Run these commands to fix:
  pnpm approve-builds
  pnpm rebuild @opencode-cloud/core
`);
  } else {
    console.error('Native binary not found. Try reinstalling: npm install');
  }
  process.exit(1);
}
```

**2. Documentation**

Add a "pnpm v10+ users" section to README:

```markdown
### pnpm v10+ users

pnpm v10 blocks install scripts by default. After installing, run:

\`\`\`bash
pnpm approve-builds        # approve @opencode-cloud/core
pnpm rebuild @opencode-cloud/core
\`\`\`

Or add to your `.npmrc`:
\`\`\`
onlyBuiltDependencies=@opencode-cloud/core
\`\`\`
```

**Alternative consideration:** Could we avoid postinstall entirely? Options:
- Ship prebuilt binaries (rejected for transparency reasons)
- Use WASM instead of native (performance trade-off)
- Require users to have Rust and build manually (current approach, but with better error messages)

Current approach (compile-on-install) is intentional for transparency. Focus on making the failure mode clear and actionable.
