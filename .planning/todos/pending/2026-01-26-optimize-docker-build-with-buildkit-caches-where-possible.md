---
created: 2026-01-26T19:19
title: Optimize Docker build with BuildKit caches
area: tooling
files:
  - packages/core/src/docker/Dockerfile
  - packages/core/src/docker/image.rs
---

## Problem

BuildKit cache mounts were added to speed up Docker rebuilds, but several
mounts caused permission errors when running as the non-root `opencode` user.
We removed or disabled some cache mounts to keep builds working, which reduces
the performance benefits.

## Solution

Revisit BuildKit cache usage and enable caches safely:
- Identify which steps can use cache mounts without ownership issues.
- Consider explicit cache ownership settings or alternate cache locations.
- Validate BuildKit builder/session settings with bollard.
