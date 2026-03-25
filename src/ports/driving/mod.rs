//! # Driving Ports (Primary Ports)
//!
//! Interfaces for CLI and HTTP interfaces.

use crate::domain::entities::{Counter, Gauge, Histogram};

/// CLI interface for metrics
pub trait CliPort {
    /// Display counter value
    fn display_counter(&self, counter: &Counter);

    /// Display gauge value
    fn display_gauge(&self, gauge: &Gauge);

    /// Display histogram summary
    fn display_histogram(&self, histogram: &Histogram);

    /// Display error
    fn display_error(&self, error: &str);

    /// Display metrics snapshot
    fn display_snapshot(&self, snapshot: &MetricsSnapshot);
}

/// Snapshot of all metrics
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub counters: Vec<CounterSnapshot>,
    pub gauges: Vec<GaugeSnapshot>,
}

#[derive(Debug, Clone)]
pub struct CounterSnapshot {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Clone)]
pub struct GaugeSnapshot {
    pub name: String,
    pub value: f64,
}

/// HTTP/REST interface for metrics
pub trait HttpPort {
    /// Handle GET /metrics
    fn handle_get_metrics(&self) -> MetricsSnapshot;

    /// Handle POST /metrics/counter/{name}
    fn handle_inc_counter(&self, name: &str, delta: u64) -> Result<(), String>;

    /// Handle POST /metrics/gauge/{name}
    fn handle_set_gauge(&self, name: &str, value: f64) -> Result<(), String>;

    /// Handle POST /metrics/histogram/{name}
    fn handle_record_histogram(&self, name: &str, value: u64) -> Result<(), String>;
}
