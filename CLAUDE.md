# CLAUDE.md

## Child Agent Usage

Use child agents for discovery/verification waves when feasible:

- Prefer scoped child-agent lanes for parallel file discovery and verification.
- Keep edits constrained to the smallest needed file set.
- Sync updates to this document and adjacent workflow artifacts when behavior changes.

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

thegent-metrics provides metrics and telemetry instrumentation for the thegent CLI framework. It instruments CLI command execution, latency, error rates, and resource usage using OpenTelemetry, with Prometheus exposition and optional OTLP export.

Key capabilities:
- Command execution counter: track invocations per command with outcome labels (success/error)
- Command latency histogram: record execution duration per command in milliseconds
- Error rate metric: derived from counter labels; exported as gauge
- Resource metrics: memory usage, goroutine count, CPU time per command invocation
- OpenTelemetry integration with Prometheus exporter and OTLP gRPC export
- Auto-instrumentation middleware for thegent CLI framework

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

## Code Conventions

### Rust Style

- Follow Rust idioms and `rustfmt` conventions
- Use `clippy` for linting: `cargo clippy --all-targets -- -D warnings`
- Format check: `cargo fmt --check`
- No comments unless explicitly requested

### Error Handling

- Use `Result` types for fallible operations
- Propagate errors with `?` operator
- Log errors to stderr; do not crash the CLI

### Concurrency

- Use `dashmap` for concurrent map operations
- Use `parking_lot` for mutexes (faster than std)
- Ensure thread-safety for all shared state

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

## Architecture

### Core Abstractions

1. **Metrics Collection** (`src/`): Instruments CLI command execution, latency, error rates, and resource usage
2. **OpenTelemetry Integration**: OTel SDK setup with MeterProvider, Prometheus exporter, OTLP gRPC export
3. **CLI Middleware**: Auto-instrumentation middleware for thegent CLI framework

### Key Patterns

- Zero metrics overhead when telemetry is disabled
- Prometheus endpoint starts on first command invocation
- All OTel SDK errors are logged to stderr; they do not crash the CLI

## Agent Behavior Rules

**This project is managed through AgilePlus.**

### AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
- CLI: cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>

### Kilo Gastown Mechanics

This repo is a rig in town 78a8d430.

- **Kilo Rig ID:** 89600299-c8b5-48cc-9967-6174106aa477
- **Town:** 78a8d430-a206-4a25-96c0-5cd9f5caf984

Work delegation uses `gt_sling` / `gt_sling_batch`. Agents should:
- Check for hooked beads via `gt_prime`
- Use `gt_done` to signal completion and push branch for review
- Call `gt_checkpoint` after significant milestones for crash-recovery
- Escalate blocked work via `gt_escalate`

### Town Communication

- `gt_mail_send`: Send coordination messages to other agents
- `gt_mail_check`: Check for undelivered mail
- `gt_nudge`: Send real-time nudges to wake or coordinate agents

### Work Requirements

1. Check for AgilePlus spec before implementing
2. Update work package status as work progresses
3. No code without corresponding AgilePlus spec

### UTF-8 Encoding

All markdown files must use UTF-8. Avoid smart quotes, em-dashes, and special characters.

```bash
# Validate encoding (in AgilePlus repo)
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

## Branch Discipline

- Feature branches in repos/worktrees/<project>/<category>/<branch>
- Canonical repository tracks main only
- Return to main for merge/integration checkpoints

## AgilePlus Reference

- Specs: AgilePlus/kitty-specs/<feature-id>/
- Docs: AgilePlus/docs/
- Workflows: AgilePlus/docs/workflow/
- Worklog: AgilePlus/.work-audit/worklog.md
