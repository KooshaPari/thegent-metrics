//! # Value Objects
//!
//! Immutable objects defined by their attributes.

use std::fmt;
use std::sync::Arc;

/// Metric name value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetricName {
    name: Arc<String>,
}

impl MetricName {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty(), "Metric name cannot be empty");
        assert!(!name.contains(' '), "Metric name cannot contain spaces");
        Self {
            name: Arc::new(name),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Metric value for gauges
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaugeValue(pub f64);

impl GaugeValue {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

/// Metric value for counters
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CounterValue(pub u64);

impl CounterValue {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

/// Histogram bucket boundary
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BucketBound(pub f64);

impl BucketBound {
    pub fn new(bound: f64) -> Self {
        Self(bound)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

/// Default histogram buckets for latency metrics
pub fn default_latency_buckets() -> Vec<BucketBound> {
    vec![
        BucketBound::new(0.005),
        BucketBound::new(0.01),
        BucketBound::new(0.025),
        BucketBound::new(0.05),
        BucketBound::new(0.1),
        BucketBound::new(0.25),
        BucketBound::new(0.5),
        BucketBound::new(1.0),
        BucketBound::new(2.5),
        BucketBound::new(5.0),
        BucketBound::new(10.0),
    ]
}
