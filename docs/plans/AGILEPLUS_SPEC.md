# AgilePlus Methodology Specification — thegent-metrics

## Overview

thegent-metrics is managed using **AgilePlus**, a work-tracking methodology that ensures all code changes are tied to documented specs before implementation begins. AgilePlus provides governance, traceability, and structured workflow for this project.

## Core Principles

### 1. All Work Must Be Tracked

Every feature, fix, or change must have a corresponding AgilePlus work item (bead) before code is written.

- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`

### 2. Spec Before Code

No implementation without a spec. Before writing code:

1. Create a spec in AgilePlus: `agileplus specify --title "<feature>" --description "<desc>"`
2. The spec defines what will be built and why
3. Implementation follows the spec

### 3. Work Package Status Updates

As work progresses, update the work package state:

```bash
agileplus status <feature-id> --wp <wp-id> --state <state>
```

States: `open` → `in_progress` → `in_review` → `shipped`

## AgilePlus Structure in this Repo

### Branch Discipline

Feature branches follow the pattern:
```
repos/worktrees/<project>/<category>/<branch>
```

For this repo:
- Feature branches: `convoy/agileplus-kilo-specs-thegent-metrics/<convoy-id>/head`
- Canonical repository tracks `main` only
- Return to `main` for merge/integration checkpoints

### File Locations

| Purpose | Location |
|---------|----------|
| Agent rules | `AGENTS.md` |
| Project instructions | `CLAUDE.md` |
| Feature specs | `kitty-specs/<feature-id>/` |
| Work tracking | AgilePlus database |
| Worklog | `worklog.md` |
| ADRs | `ADR.md` |
| PRD | `PRD.md` |
| Functional requirements | `FUNCTIONAL_REQUIREMENTS.md` |

## Applied Workflow

### 1. Planning Phase

When a new feature is needed:

1. Create spec in AgilePlus via `agileplus specify`
2. Define acceptance criteria and verification steps
3. Break down into work packages (epics, stories, tasks)

### 2. Implementation Phase

1. Check for existing AgilePlus spec before implementing
2. Create feature branch from worktree pattern
3. Implement code following project conventions
4. Update work package status as work progresses
5. No code without corresponding AgilePlus spec

### 3. Review Phase

1. Push branch when work is complete
2. Submit for review (handled by Refinery)
3. Address any rework requests
4. Bead transitions to `in_review`

### 4. Ship Phase

1. Merge to `main` via Refinery
2. Close bead in AgilePlus
3. Mark spec as `shipped`

## AgilePlus Artifacts

### Specs (kitty-specs/)

Feature specifications in `kitty-specs/<feature-id>/`:

- `spec.md`: Feature description, state, verification criteria
- `plan.md`: Timeline and phases

### ADRs (Architecture Decision Records)

Document significant architectural decisions in `ADR.md`:
- Title, status, context, decision, rationale
- Status: Proposed → Accepted → Deprecated/Superseded

### PRD (Product Requirements Document)

High-level product requirements in `PRD.md`:
- Epics with feature IDs
- Acceptance criteria

### Functional Requirements

Traceable requirements in `FUNCTIONAL_REQUIREMENTS.md`:
- Requirements linked to epics (e.g., `FR-INSTR-001` traces to `E1.1`)
- Clear "SHALL" statements

## xDD Methodologies Applied

The codebase applies experimental (xDD) software development methodologies:

| Methodology | Application |
|------------|-------------|
| **TDD** | Tests written first (see `cargo test`) |
| **DDD** | Bounded contexts for metrics types |
| **SOLID** | Single responsibility per module |
| **CQRS** | Separate command and query interfaces |
| **EDA** | Domain events for metric changes |
| **TraceDD** | Trace identifiers on all operations |

## Encoding Standards

All markdown files must use UTF-8 encoding. Avoid smart quotes, em-dashes, and special characters.

Validate encoding:
```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

## CI/CD Pipeline

The project uses GitHub Actions with quality gates:

1. **Lint**: `cargo fmt`, `cargo clippy`
2. **Test**: `cargo test --all-features`
3. **Benchmark**: `cargo bench`
4. **Build**: `cargo build --release --all-features`
5. **Security**: `cargo audit`

## Relationship to Other Specifications

This spec works in conjunction with:

- **KILO_GASTOWN_SPEC.md**: Explains Kilo Gastown multi-agent orchestration used in this ecosystem
- **AGENTS.md**: Agent behavior rules for this project
- **CLAUDE.md**: Project-specific instructions for AI assistants

## References

- AgilePlus Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- AgilePlus Docs: `AgilePlus/docs/`
- AgilePlus Workflows: `AgilePlus/docs/workflow/`
