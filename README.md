# thegent-metrics

Metrics collection and observability library for agent orchestration.

## Features

- Prometheus-compatible metrics
- Counter, Gauge, Histogram, Summary
- Resource tracking
- Performance monitoring

## Architecture

Based on thegent's observability infrastructure.

## Consolidation notes

- This crate is the best candidate for the **canonical observability core** in the ecosystem.
- `phenotype-metrics` overlaps on counter/gauge/timer registry behavior and should be treated as a smaller legacy/simple registry surface.
- `phenotype-gauge` should remain separate because its metrics are for xDD/testing and benchmark reporting, not runtime telemetry.

If shared abstractions are extracted, keep the domain model and exporters here, then let simpler registries reuse the shared core rather than copying it.

## License

MIT
