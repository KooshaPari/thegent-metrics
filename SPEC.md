# thegent-metrics Specification

## Architecture
```
┌─────────────────────────────────────────────────────┐
│            thegent-metrics (Rust)                    │
├─────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────┐ │
│  │         Metrics collection                    │ │
│  │   ┌─────────┐   ┌─────────┐   ┌────────────┐  │ │
│  │   │Counter │   │ Gauge   │   │ Histogram  │  │ │
│  │   └─────────┘   └─────────┘   └────────────┘  │ │
│  └───────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| Counter | Increment metric | `inc()`, `add()` |
| Gauge | Set metric | `set()`, `inc()`, `dec()` |
| Histogram | Duration metrics | `observe()` |

## Data Models

```rust
struct MetricData {
    name: String,
    value: f64,
    labels: HashMap<String, String>,
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Record write | <100μs |
| Query | <1ms |