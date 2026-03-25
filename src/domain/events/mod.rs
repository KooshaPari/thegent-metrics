//! # Domain Events
//!
//! Immutable events representing metric state changes.

use std::time::SystemTime;

/// Domain events for the metrics bounded context
#[derive(Debug, Clone)]
pub enum MetricsEvent {
    /// Counter was incremented
    CounterIncremented {
        name: String,
        delta: u64,
        new_value: u64,
        timestamp: SystemTime,
    },

    /// Counter was reset
    CounterReset {
        name: String,
        timestamp: SystemTime,
    },

    /// Gauge was set
    GaugeSet {
        name: String,
        value: f64,
        timestamp: SystemTime,
    },

    /// Histogram value was recorded
    HistogramRecorded {
        name: String,
        value: u64,
        timestamp: SystemTime,
    },

    /// Summary value was observed
    SummaryObserved {
        name: String,
        value: f64,
        timestamp: SystemTime,
    },
}

impl MetricsEvent {
    /// Get the timestamp of the event.
    pub fn timestamp(&self) -> SystemTime {
        match self {
            MetricsEvent::CounterIncremented { timestamp, .. } => *timestamp,
            MetricsEvent::CounterReset { timestamp, .. } => *timestamp,
            MetricsEvent::GaugeSet { timestamp, .. } => *timestamp,
            MetricsEvent::HistogramRecorded { timestamp, .. } => *timestamp,
            MetricsEvent::SummaryObserved { timestamp, .. } => *timestamp,
        }
    }

    /// Get the metric name.
    pub fn metric_name(&self) -> &str {
        match self {
            MetricsEvent::CounterIncremented { name, .. } => name,
            MetricsEvent::CounterReset { name, .. } => name,
            MetricsEvent::GaugeSet { name, .. } => name,
            MetricsEvent::HistogramRecorded { name, .. } => name,
            MetricsEvent::SummaryObserved { name, .. } => name,
        }
    }
}
