---
phase: 12
title: Web Desktop UI Investigation
type: research
created: 2026-01-23
---

# Phase 12: Web Desktop UI Investigation

## Phase Goal
Investigate integrating a secure web-exposed full custom browser desktop UI such as Friend OS, WDE, or similar.

## Success Criteria
1. Candidate web desktop solutions evaluated (Friend OS, WDE, others)
2. Security implications documented (auth, isolation, network exposure)
3. Integration feasibility assessed with current Docker architecture
4. Recommendation made: proceed with implementation or defer

## Research Scope

### Candidates to Evaluate
- **Friend OS** - Full web desktop environment with apps ecosystem
- **WDE (Web Desktop Environment)** - Lightweight web-based desktop
- **OS.js** - JavaScript web desktop framework
- **Puter** - Cloud computer in your browser
- **Daedalus OS** - Browser-based operating system
- **Windows 93** - Retro-style web desktop (for inspiration)

### Evaluation Criteria

#### Technical Fit
- Docker integration complexity
- Resource requirements (memory, CPU)
- Port requirements and network exposure
- Conflict potential with existing services (opencode web UI, Cockpit)

#### Security
- Authentication mechanisms
- Session isolation
- File system sandboxing
- Network exposure implications

#### User Experience
- Learning curve
- Customizability
- Mobile/tablet support
- Performance characteristics

#### Maintenance
- Project activity/health
- Documentation quality
- Community support
- Licensing compatibility

## Questions Resolved

### Q1: Primary Use Case
**Question**: What's the primary use case for adding a web desktop UI?
**Answer**: TBD

### Q2: Resource Constraints
**Question**: Are there resource constraints we need to consider (the container already runs opencode + Cockpit)?
**Answer**: TBD

### Q3: Integration Depth
**Question**: Should the web desktop be tightly integrated (sharing file system, apps) or loosely coupled (separate container)?
**Answer**: TBD

### Q4: Authentication Strategy
**Question**: Should web desktop auth be unified with existing PAM users or separate?
**Answer**: TBD

## Research Deliverables
1. Evaluation matrix comparing candidates
2. Security analysis document
3. Architecture proposal (if proceeding)
4. Recommendation with rationale
