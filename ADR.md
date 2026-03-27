# ADR — thegent-metrics

## ADR-001: OpenTelemetry as Foundation
**Status:** Accepted
**Context:** Metrics library choice affects consumer lock-in.
**Decision:** Use OpenTelemetry Go SDK as the instrumentation layer. Prometheus and OTLP are exporters, not the core API.
**Rationale:** OTel is vendor-neutral; consumers can swap exporters without changing instrumentation code.

## ADR-002: Prometheus as Default Exporter
**Status:** Accepted
**Context:** Most Phenotype infra uses Prometheus + Grafana stack.
**Decision:** Default exporter is Prometheus (`prometheus/client_golang`). OTLP is opt-in via config.
**Rationale:** Zero-config developer experience; Prometheus is universally available in local dev.

## ADR-003: Lazy Exporter Start
**Status:** Accepted
**Context:** Starting an HTTP server at process init is surprising for a CLI tool.
**Decision:** The Prometheus HTTP server starts on the first instrumented command invocation, not at import time.
**Rationale:** Avoids port conflicts on tools that run in scripts without needing metrics.

## ADR-004: No-Op on Disable
**Status:** Accepted
**Context:** CI pipelines and scripts should not incur metrics overhead.
**Decision:** When disabled, all metric calls are routed to a no-op implementation with zero allocation. Check is done once at init, not per call.
**Rationale:** Performance correctness; disabled path must have zero cost.
