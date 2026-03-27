# PRD — thegent-metrics

## Overview
thegent-metrics provides metrics and telemetry instrumentation for the thegent CLI framework. It instruments CLI command execution, latency, error rates, and resource usage using OpenTelemetry, with Prometheus exposition and optional OTLP export.

## Epics

### E1 — Metrics Instrumentation
**E1.1** Command execution counter: track invocations per command with outcome labels (success/error).
**E1.2** Command latency histogram: record execution duration per command in milliseconds.
**E1.3** Error rate metric: derived from counter labels; exported as gauge.
**E1.4** Resource metrics: memory usage, goroutine count, CPU time per command invocation.

### E2 — OpenTelemetry Integration
**E2.1** OTel SDK setup: initialize MeterProvider from config (OTLP endpoint, export interval).
**E2.2** Prometheus exporter: expose `/metrics` on configurable port for Prometheus scrape.
**E2.3** OTLP gRPC export: push metrics to remote collector (Grafana Cloud, self-hosted).
**E2.4** Context propagation: carry trace context through CLI command invocations.

### E3 — CLI Integration
**E3.1** Middleware for thegent CLI framework that auto-instruments all registered commands.
**E3.2** Zero-config default: works out of the box with in-process Prometheus exporter.
**E3.3** Config file support: `thegent-metrics.yaml` for endpoint, sampling, export interval.

### E4 — Dashboards and Alerting
**E4.1** Pre-built Grafana dashboard JSON for thegent command metrics.
**E4.2** Alert rules: command error rate > 5%, p99 latency > 5s.

## Acceptance Criteria
- Zero metrics overhead when telemetry is disabled (flag or env var).
- Prometheus endpoint starts on first command invocation, not at process start.
- All OTel SDK errors are logged to stderr; they do not crash the CLI.
