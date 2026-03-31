
# Agent Rules

**This project is managed through AgilePlus.**

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
- CLI: cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>

## Kilo Gastown Mechanics

This repo is a rig in town 78a8d430.

- **Kilo Rig ID:** 89600299-c8b5-48cc-9967-6174106aa477
- **Town:** 78a8d430-a206-4a25-96c0-5cd9f5caf984

### Kilo Integration

Work delegation uses `gt_sling` / `gt_sling_batch`. Agents should:
- Check for hooked beads via `gt_prime`
- Use `gt_done` to signal completion and push branch for review
- Call `gt_checkpoint` after significant milestones for crash-recovery
- Escalate blocked work via `gt_escalate`

### Town Communication

- `gt_mail_send`: Send coordination messages to other agents
- `gt_mail_check`: Check for undelivered mail
- `gt_nudge`: Send real-time nudges to wake or coordinate agents

## Branch Discipline

- Feature branches in repos/worktrees/<project>/<category>/<branch>
- Canonical repository tracks main only
- Return to main for merge/integration checkpoints

## Work Requirements

1. Check for AgilePlus spec before implementing
2. Update work package status as work progresses
3. No code without corresponding AgilePlus spec

## UTF-8 Encoding

All markdown files must use UTF-8. Avoid smart quotes, em-dashes, and special characters.

```bash
# Validate encoding (in AgilePlus repo)
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

## Stack

- **Language:** Rust (edition 2021, MSRV 1.70)
- **Build:** Cargo
- **Binary name:** thegent-metrics
- **Library name:** thegent_metrics
- **License:** MIT

### Key Dependencies

- `serde` / `serde_json` — Serialization
- `dashmap` / `parking_lot` — Concurrency
- `clap` — CLI argument parsing

### Dev Dependencies

- `criterion` — Benchmarking
- `proptest` — Property-based testing
- `cargo-mutants` — Mutation testing

## Build and Test

```bash
# Build
cargo build --release

# Test
cargo test

# Benchmark
cargo bench

# Mutation testing
cargo mutants

# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --check
```
