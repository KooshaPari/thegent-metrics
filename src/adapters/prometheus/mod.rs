//! # Prometheus Adapter
//!
//! Exports metrics in Prometheus text format.

use crate::domain::entities::{Counter, Gauge, Histogram, Summary};
use crate::ports::driven::MetricsPort;
use std::fmt::Write;

/// Prometheus formatter for metrics
pub struct PrometheusFormatter;

impl PrometheusFormatter {
    /// Format a counter in Prometheus format
    pub fn format_counter(counter: Counter) -> String {
        format!("{} {}\n", counter.name(), counter.get())
    }

    /// Format a gauge in Prometheus format
    pub fn format_gauge(gauge: Gauge) -> String {
        format!("{} {}\n", gauge.name(), gauge.get())
    }

    /// Format a histogram in Prometheus format
    pub fn format_histogram(histogram: Histogram) -> String {
        let mut output = String::new();
        let name = histogram.name();

        // Skip base histogram name
        let base_name = name.as_str().trim_end_matches("_bucket");

        writeln!(output, "{}_count {}", base_name, histogram.count()).unwrap();
        writeln!(output, "{}_sum {}", base_name, histogram.sum()).unwrap();
        writeln!(output, "{}_min {}", base_name, histogram.min()).unwrap();
        writeln!(output, "{}_max {}", base_name, histogram.max()).unwrap();
        writeln!(output, "{}_mean {}", base_name, histogram.mean()).unwrap();
        writeln!(output, "{}_p50 {}", base_name, histogram.p50()).unwrap();
        writeln!(output, "{}_p90 {}", base_name, histogram.p90()).unwrap();
        writeln!(output, "{}_p99 {}", base_name, histogram.p99()).unwrap();

        output
    }

    /// Format a summary in Prometheus format
    pub fn format_summary(summary: Summary) -> String {
        let mut output = String::new();
        let name = summary.name();

        writeln!(output, "{}_count {}", name, summary.count()).unwrap();
        writeln!(output, "{}_sum {}", name, summary.sum()).unwrap();
        writeln!(output, "{}_mean {}", name, summary.mean()).unwrap();

        output
    }

    /// Format entire registry in Prometheus format
    pub fn format_registry<T: MetricsPort>(registry: &T) -> String {
        let mut output = String::new();

        for name in registry.list_counters() {
            if let Some(counter) = registry.get_counter(&name) {
                output.push_str(&Self::format_counter(counter));
            }
        }

        for name in registry.list_gauges() {
            if let Some(gauge) = registry.get_gauge(&name) {
                output.push_str(&Self::format_gauge(gauge));
            }
        }

        for name in registry.list_histograms() {
            if let Some(histogram) = registry.get_histogram(&name) {
                output.push_str(&Self::format_histogram(histogram));
            }
        }

        for name in registry.list_summaries() {
            if let Some(summary) = registry.get_summary(&name) {
                output.push_str(&Self::format_summary(summary));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_counter() {
        let counter = Counter::with_value("requests_total", 42);
        let output = PrometheusFormatter::format_counter(counter);
        assert_eq!(output, "requests_total 42\n");
    }

    #[test]
    fn test_format_gauge() {
        let gauge = Gauge::with_value("memory_usage_bytes", 1024.0);
        let output = PrometheusFormatter::format_gauge(gauge);
        assert_eq!(output, "memory_usage_bytes 1024\n");
    }
}
