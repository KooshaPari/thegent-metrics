//! # Driven Ports (Secondary Ports)
//!
//! Interfaces that the domain defines and infrastructure must implement.

use crate::domain::entities::{Counter, Gauge, Histogram, Summary};
use crate::domain::events::MetricsEvent;

/// Port for counter operations
pub trait CounterPort {
    /// Increment counter by delta
    fn inc(&self, delta: u64);

    /// Get current value
    fn get(&self) -> u64;

    /// Get the counter entity
    fn as_entity(&self) -> &Counter;
}

/// Port for gauge operations
pub trait GaugePort {
    /// Set gauge value
    fn set(&self, value: f64);

    /// Add to gauge
    fn add(&self, delta: f64);

    /// Get current value
    fn get(&self) -> f64;

    /// Get the gauge entity
    fn as_entity(&self) -> &Gauge;
}

/// Port for histogram operations
pub trait HistogramPort {
    /// Record a value
    fn record(&self, value: u64);

    /// Get count
    fn count(&self) -> u64;

    /// Get percentile
    fn percentile(&self, p: f64) -> f64;

    /// Get the histogram entity
    fn as_entity(&self) -> &Histogram;
}

/// Port for summary operations
pub trait SummaryPort {
    /// Observe a value
    fn observe(&self, value: f64);

    /// Get count
    fn count(&self) -> u64;

    /// Get mean
    fn mean(&self) -> f64;

    /// Get the summary entity
    fn as_entity(&self) -> &Summary;
}

/// Port for metrics registry
pub trait MetricsPort {
    /// Register a counter
    fn register_counter(&self, counter: Counter) -> Result<(), String>;

    /// Register a gauge
    fn register_gauge(&self, gauge: Gauge) -> Result<(), String>;

    /// Register a histogram
    fn register_histogram(&self, histogram: Histogram) -> Result<(), String>;

    /// Register a summary
    fn register_summary(&self, summary: Summary) -> Result<(), String>;

    /// Get counter by name
    fn get_counter(&self, name: &str) -> Option<Counter>;

    /// Get gauge by name
    fn get_gauge(&self, name: &str) -> Option<Gauge>;

    /// Get histogram by name
    fn get_histogram(&self, name: &str) -> Option<Histogram>;

    /// Get summary by name
    fn get_summary(&self, name: &str) -> Option<Summary>;

    /// List all metric names
    fn list_counters(&self) -> Vec<String>;
    fn list_gauges(&self) -> Vec<String>;
    fn list_histograms(&self) -> Vec<String>;
    fn list_summaries(&self) -> Vec<String>;
}

/// Port for event publishing
pub trait EventPort {
    /// Publish a metric event
    fn publish(&mut self, event: MetricsEvent) -> Result<(), String>;

    /// Get events since timestamp
    fn get_events_since(&self, timestamp: std::time::SystemTime) -> Vec<MetricsEvent>;
}
