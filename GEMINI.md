# thegent-metrics Agent Instructions

This file provides guidance to Google Gemini when working with code in this repository.

## Project Overview

thegent-metrics is a Prometheus-compatible metrics collection and observability library for agent orchestration. It provides Counter, Gauge, Histogram, and Summary metric types with resource tracking and performance monitoring capabilities.

This crate is the canonical observability core for the ecosystem. Simpler registries should reuse shared core abstractions rather than copying them.

## Stack

- **Language:** Rust (edition 2021, MSRV 1.70)
- **Binary name:** thegent-metrics
- **Library name:** thegent_metrics
- **Key deps:** serde/serde_json (serialization), dashmap/parking_lot (concurrency), clap (CLI)
- **Dev deps:** criterion (benchmarks), proptest (property testing), cargo-mutants (mutation testing)

## Key Paths

- `src/domain/entities/` — Metric primitives (Counter, Gauge, Histogram, Summary)
- `src/domain/value_objects/` — Metric metadata types
- `src/domain/events/` — Domain events
- `src/ports/driving/` — Inbound ports (metric registration, collection)
- `src/ports/driven/` — Outbound ports (exporters)
- `src/adapters/prometheus/` — Prometheus exporter adapter
- `src/adapters/json/` — JSON exporter adapter
- `src/application/` — CQRS commands and queries

## Architecture

- Hexagonal/Clean Architecture: Domain → Application → Ports → Adapters
- Domain is pure — no external dependencies
- Metric types are composable via a registry pattern
- Prometheus exporter is the primary export target

## Build and Test

```bash
cargo build --release
cargo test
cargo bench
cargo mutants
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Kilo Integration

Rig ID: 89600299-c8b5-48cc-9967-6174106aa477
Town: 78a8d430-a206-4a25-96c0-5cd9f5caf984

Work is tracked via AgilePlus beads. Use gt_prime, gt_done, gt_checkpoint, gt_escalate as needed.

## Child Agent Usage

Use child agents for discovery/verification waves when feasible. Keep edits constrained to the smallest needed file set. Sync updates to this document when behavior changes.
