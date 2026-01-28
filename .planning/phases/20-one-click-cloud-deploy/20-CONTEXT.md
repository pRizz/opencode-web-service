---
phase: 20-one-click-cloud-deploy
status: discussed
last_updated: 2026-01-28
---

# Phase 20 Context: One-Click Cloud Deploy

## Scope and Provider Priority
- Phase 20 targets AWS, GCP, Azure, and DigitalOcean in scope, but the first executable path should focus on AWS.
- AWS is the initial implementation; other providers can follow in later phases if needed.
- For each provider, the intended UX is button + template + docs (no “button-only” shortcuts).
- Default AWS instance type: `t3.medium`.

## Provisioning Approach
- Quick deploy uses a standard Ubuntu VM + cloud-init to install Docker and pull the opencode-cloud image.
- Prefer portable provisioning that can carry to other providers; no custom AMI required initially.
- AWS quick path: CloudFormation for user-friendliness; Terraform can be offered as a power-user path later.
- Quick deploy creates a new security group with appropriate defaults.
- Quick deploy should assume public subnets for the ALB and keep the instance in private subnets.
- Provide a “Quick vs Power” experience with a single deploy button that exposes advanced parameters.
- SSH access: default to SSM Session Manager for quick deploy; allow SSH key options in advanced parameters.

## Security Defaults
- Quick deploy opens ports: 22, 3000, 3001, 3002, 9090 (optimize later).
- HTTPS by default if feasible and automatable.
- AWS default HTTPS approach: ALB + ACM certificates.
- Domain name is required for HTTPS; if user cannot provide a domain, expect non-HTTPS fallback to be explicit.
- Domain handling should clearly explain DNS validation requirements and provide guidance when domain is unavailable or already managed.
- Cockpit should not be exposed directly; route it behind the same ALB.
- Auto-create a user during provisioning with a strong random password; surface credentials clearly to the user.

## User-Facing Flow
- Deploy buttons should appear in both root `README.md` and provider-specific docs.
- Quick deploy required inputs: domain name (for automated TLS certs); other inputs should remain optional unless required.
- Deployment outputs should include:
  - High-level summary (URLs, credentials, next steps).
  - Detailed technical section (instance ID, SG, subnet, etc.).
- Single deploy button with “show advanced parameters” for customization.

## Deferred Ideas / Notes
- Consider “Power User Deploy” options for custom networking, SSH keys, or bypassing TLS automation.
- Evaluate non-HTTPS fallback behavior if domain is not supplied.
