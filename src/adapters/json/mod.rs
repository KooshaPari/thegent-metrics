//! # JSON Adapter
//!
//! Exports metrics in JSON format.

use crate::ports::driven::MetricsPort;
use serde::Serialize;

/// Snapshot of metrics for JSON export
#[derive(Debug, Serialize)]
pub struct MetricsSnapshotJson {
    pub counters: Vec<CounterJson>,
    pub gauges: Vec<GaugeJson>,
    pub histograms: Vec<HistogramJson>,
    pub summaries: Vec<SummaryJson>,
}

#[derive(Debug, Serialize)]
pub struct CounterJson {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Serialize)]
pub struct GaugeJson {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct HistogramJson {
    pub name: String,
    pub count: u64,
    pub sum: u64,
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
}

#[derive(Debug, Serialize)]
pub struct SummaryJson {
    pub name: String,
    pub count: u64,
    pub sum: f64,
    pub mean: f64,
}

/// JSON formatter for metrics
pub struct JsonFormatter;

impl JsonFormatter {
    /// Create a snapshot of the registry
    pub fn snapshot<T: MetricsPort>(registry: &T) -> MetricsSnapshotJson {
        let counters = registry
            .list_counters()
            .into_iter()
            .filter_map(|name| {
                registry.get_counter(&name).map(|c| CounterJson {
                    name,
                    value: c.get(),
                })
            })
            .collect();

        let gauges = registry
            .list_gauges()
            .into_iter()
            .filter_map(|name| {
                registry.get_gauge(&name).map(|g| GaugeJson {
                    name,
                    value: g.get(),
                })
            })
            .collect();

        let histograms = registry
            .list_histograms()
            .into_iter()
            .filter_map(|name| {
                registry.get_histogram(&name).map(|h| HistogramJson {
                    name,
                    count: h.count(),
                    sum: h.sum(),
                    min: h.min(),
                    max: h.max(),
                    mean: h.mean(),
                    p50: h.p50(),
                    p90: h.p90(),
                    p99: h.p99(),
                })
            })
            .collect();

        let summaries = registry
            .list_summaries()
            .into_iter()
            .filter_map(|name| {
                registry.get_summary(&name).map(|s| SummaryJson {
                    name,
                    count: s.count(),
                    sum: s.sum(),
                    mean: s.mean(),
                })
            })
            .collect();

        MetricsSnapshotJson {
            counters,
            gauges,
            histograms,
            summaries,
        }
    }

    /// Format as JSON string
    pub fn format<T: MetricsPort>(registry: &T) -> String {
        let snapshot = Self::snapshot(registry);
        serde_json::to_string_pretty(&snapshot).unwrap_or_default()
    }
}
