# Kilo Gastown Methodology Spec

## Overview

This document describes how Kilo Gastown mechanics apply to thegent-metrics repository.

**Rig ID:** 89600299-c8b5-48cc-9967-6174106aa477  
**Town:** 78a8d430-a206-4a25-96c0-5cd9f5caf984  
**Town Name:** Gastown (Multi-rig coordination system)

## Town Architecture

### What is a Town

A Town is a collection of rigs (repositories) coordinated by a Mayor agent. Towns provide:

- Cross-repo coordination for features that span multiple repositories
- Dependency tracking between related work items
- Unified progress visibility via convoy status tools

### What is a Rig

A Rig is an individual repository with polecat agents working beads. Thegent-metrics is one such rig in town 78a8d430.

### How This Repo Fits In

Thegent-metrics is a Rust metrics collection library. As a rig in Gastown:

- Agents (polecats) work beads assigned to this repository
- Work is coordinated through convoys when cross-repo dependencies exist
- The Mayor agent in town 78a8d430 oversees coordination

## Core Kilo Concepts

### Bead Lifecycle

All work passes through these states:

```
open → in_progress → in_review → closed
```

| State | Description |
|-------|-------------|
| `open` | Bead created, not yet started |
| `in_progress` | Agent actively working the bead |
| `in_review` | Branch pushed, awaiting review/merge |
| `closed` | Work completed and merged |

### Convoys

Convoys are tracked groups of beads with dependency chains. They enable:

- Grouping related work across multiple rigs
- Tracking dependencies between beads
- Visibility into cross-repo feature progress

#### When to Use Convoys

Use convoys when:

- A feature spans multiple repositories
- Multiple beads have interdependent dependencies
- You need to track progress across a feature chain
- Work items must be merged together atomically

Do NOT use convoys when:

- Work is isolated to a single repository
- Beads have no dependencies on each other
- Independent parallel work，不需要跨repo跟踪

#### Staged vs Active Convoys

| Type | Use When |
|------|----------|
| **Staged Convoy** | Dependencies are not yet ready; work cannot begin. The convoy exists to track intent and blocking dependencies. |
| **Active Convoy** | Work is in progress; agents are actively contributing beads. Progress tracking is live. |

Staged convoys allow planning ahead when you know features will need coordination but the groundwork is not yet complete.

#### Convoy Naming Convention

```
convoy/<feature>-<repo>/<convoy_id>/<head|branch_name>
```

Examples:

- `convoy/agileplus-kilo-specs-thegent-metrics/981837e8/head`
- `convoy/methodology-thegent-metrics/e0d84bcb/gt/polecat-26/287734c1`

The feature segment uses hyphens to separate words. The convoy_id is a short UUID fragment. Sub-branches under the convoy track individual agent contributions.

### Merge Modes

Two merge strategies are available:

| Mode | Description | Use When |
|------|-------------|----------|
| **review-then-land (coupled)** | Review and merge are coupled; PR must pass CI before landing. | Features requiring CI validation, atomic multi-file changes |
| **review-and-merge (independent)** | Review happens separately from merge; can land after approval. | Independent contributions, low-risk changes |

For this rig, prefer `review-then-land` for new features and `review-and-merge` for documentation or minor fixes.

## Agent Workflow

### Starting Work

1. Call `gt_prime` to get hooked bead context
2. Read the bead requirements and any linked specs
3. Check if bead is part of a convoy (see convoy_id in metadata)

### During Work

- Call `gt_checkpoint` after significant milestones
- Use `gt_status` at meaningful phase transitions
- Check mail with `gt_mail_check` periodically

### Completing Work

1. Ensure all quality gates pass (lint, tests, format)
2. Push branch to remote
3. Call `gt_done` with branch name

### Branch Naming

This rig follows the worktree pattern:

```
repos/worktrees/<project>/<category>/<branch>
```

For feature work, branches live under the convoy feature branch when part of a convoy, or as standalone feature branches otherwise.

## Delegation Tools

### gt_sling / gt_sling_batch

Used by senior agents to delegate work to sub-agents:

```bash
gt_sling --bead-id <id> --agent-type polecat
gt_sling_batch --file beads.csv
```

### gt_list_convoys / gt_convoy_status

Track convoy progress:

```bash
gt_list_convoys --town 78a8d430
gt_convoy_status --convoy-id <id>
```

## Integration with AgilePlus

This rig uses both Kilo Gastown AND AgilePlus:

- **AgilePlus**: Tracks detailed work packages, specs, and ADRs
- **Kilo Gastown**: Provides cross-repo coordination and agent orchestration

See [AGENTS.md](../../AGENTS.md) for full AgilePlus integration details.

## Reference

- Town ID: 78a8d430-a206-4a25-96c0-5cd9f5caf984
- Rig ID: 89600299-c8b5-48cc-9967-6174106aa477
- Main repo: https://github.com/KooshaPari/thegent-metrics
