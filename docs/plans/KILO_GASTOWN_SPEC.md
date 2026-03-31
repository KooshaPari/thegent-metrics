# Kilo Gastown Methodology Specification — thegent-metrics

## Overview

thegent-metrics participates in the **Kilo Gastown** multi-agent orchestration system. Gastown provides coordination infrastructure for autonomous agents (polecats) to collaborate on code across distributed worktrees. This document explains the Gastown methodology as implemented in this rig.

## Town Architecture

### Core Concepts

| Concept | Description |
|---------|-------------|
| **Town** | A coordination domain containing multiple rigs |
| **Rig** | A single repository/worktree managed by agents |
| **Agent (Polecat)** | An autonomous code agent assigned to work items |
| **Mayor** | Orchestration agent coordinating town activity |
| **Bead** | A unit of work (issue, task, merge_request, convoy) |

### This Town

- **Town ID:** 78a8d430-a206-4a25-96c0-5cd9f5caf984
- **Rig ID:** 89600299-c8b5-48cc-9967-6174106aa477
- **Project:** thegent-metrics
- **Stack:** Rust (edition 2021, MSRV 1.70)

## Bead Lifecycle

Beads progress through defined states:

```
open → in_progress → in_review → closed
```

### State Definitions

| State | Meaning |
|-------|---------|
| `open` | Work item available, not yet claimed |
| `in_progress` | Agent has hooked the bead and is working |
| `in_review` | Work complete, pushed for review/merge |
| `closed` | Work merged and verified |

### Bead Types

| Type | Purpose |
|------|---------|
| `issue` | Feature request or bug report |
| `convoy` | Coordinated multi-repo work item |
| `merge_request` | Code review/merge tracking |

## Convoys

A **convoy** is a grouping mechanism for related work across multiple repositories. When a feature spans several rigs, a single convoy bead coordinates the effort.

### Convoy Structure

```
convoy/<category>-<name>/<convoy-id>/head
```

Example from this rig:
- `convoy/agileplus-kilo-specs-thegent-metrics/981837e8/head`

### Staged vs Active Convoys

- **Staged convoys**: Work prepared but not yet active
- **Active convoys**: Work in progress across hooked agents

## Agent Workflow

### 1. Prime

At session start, agents call `gt_prime` to receive:
- Agent identity and status
- Hooked bead (current work item)
- Undelivered mail
- All open beads in the rig

### 2. Work

When a bead is hooked:
1. Read the bead's title and body
2. Implement the required changes
3. Commit frequently with descriptive messages
4. Push after each commit (ephemeral container)

### 3. Checkpoint

After significant milestones, call `gt_checkpoint`:
```bash
gt_checkpoint --data "<summary of progress>"
```
Enables crash recovery if the container restarts.

### 4. Done

When complete, call `gt_done`:
```bash
gt_done --branch <branch-name>
```
This:
- Pushes the branch to remote
- Transitions bead to `in_review`
- Unhooks the agent

## Town Communication

### gt_mail_send

Send a coordination message to another agent:
```bash
gt_mail_send --to_agent_id <id> --subject "<subject>" --body "<message>"
```

### gt_mail_check

Check for undelivered mail addressed to this agent.

### gt_nudge

Send an immediate real-time nudge to wake or coordinate another agent:
```bash
gt_nudge --target_agent_id <id> --message "<message>"
```

## Delegation Tools

### gt_sling / gt_sling_batch

Mayors use these to delegate work to agents. Agents receive delegated work as hooked beads.

### gt_bead_status

Inspect any bead by ID to see its current state.

### gt_escalate

Escalate a problem that cannot be resolved:
```bash
gt_escalate --title "<problem>" --body "<details>" --priority <level>
```
Creates an escalation bead routed to a supervisor or mayor.

## Branch Discipline

### Feature Branches

Feature branches follow the pattern:
```
repos/worktrees/<project>/<category>/<branch>
```

For this rig:
```
convoy/agileplus-kilo-specs-thegent-metrics/<convoy-id>/head
```

### Commit Hygiene

- Commit after every meaningful unit of work
- Push after every commit (container is ephemeral)
- Use descriptive commit messages referencing the bead

## Merge Modes

The Refinery (automated merge system) handles two modes:

### Review-Then-Land (Coupled)

Work is reviewed before merging. Agent calls `gt_done`, branch is pushed, and Refinery creates a PR for human review.

### Review-and-Merge (Independent)

Refinery reviews and merges autonomously when ready-to-land is set on the convoy.

## Relationship to AgilePlus

Kilo Gastown provides the orchestration layer; AgilePlus provides work tracking:

| Aspect | AgilePlus | Gastown |
|--------|-----------|---------|
| Work items | Beads | Beads |
| Tracking | Database | Bead status |
| Specs | `agileplus specify` | `docs/plans/*.md` |
| Governance | Work package states | Agent workflow |

AgilePlus specs should exist before Gastown work begins. See `AGILEPLUS_SPEC.md` for details.

## File Locations

| Purpose | Location |
|---------|----------|
| Agent rules | `AGENTS.md` |
| AgilePlus spec | `docs/plans/AGILEPLUS_SPEC.md` |
| This spec | `docs/plans/KILO_GASTOWN_SPEC.md` |
| Project instructions | `CLAUDE.md` |

## References

- Gastown Integration: Agent tools (`gt_*` commands)
- AgilePlus Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- Rig Configuration: `AGENTS.md` (contains town and rig IDs)
