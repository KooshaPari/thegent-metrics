# FUNCTIONAL_REQUIREMENTS — thegent-metrics

## FR-INSTR-001: Command Counter
**SHALL** increment `thegent_command_total{command, outcome}` counter on each command completion.
Traces to: E1.1

## FR-INSTR-002: Latency Histogram
**SHALL** record `thegent_command_duration_ms{command}` histogram with buckets [10, 50, 100, 500, 1000, 5000].
Traces to: E1.2

## FR-INSTR-003: Resource Metrics
**SHALL** record `thegent_memory_bytes` (heap alloc) and `thegent_goroutines` gauge per command invocation.
Traces to: E1.4

## FR-OTEL-001: MeterProvider Init
**SHALL** initialize OTel MeterProvider on first use; support `OTEL_EXPORTER_OTLP_ENDPOINT` env override.
Traces to: E2.1

## FR-OTEL-002: Prometheus Exporter
**SHALL** expose Prometheus metrics on `http://localhost:<port>/metrics`; default port 9090, configurable.
Traces to: E2.2

## FR-OTEL-003: OTLP Export
**SHALL** support OTLP gRPC export to configurable endpoint with configurable export interval (default 30s).
Traces to: E2.3

## FR-CLI-001: Auto-Instrumentation Middleware
**SHALL** provide `thegent.Use(metrics.Middleware())` that wraps all commands without per-command changes.
Traces to: E3.1

## FR-CLI-002: Zero-Config Default
**SHALL** function with no config file; Prometheus exporter starts automatically on first command run.
Traces to: E3.2

## FR-CLI-003: Disable Flag
**SHALL** skip all instrumentation when `THEGENT_METRICS_DISABLED=1` or `--no-metrics` flag is set; no performance overhead.
Traces to: E3.2
