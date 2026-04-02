//! # Domain Entities
//!
//! Core business objects with identity.

use super::value_objects::MetricName;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Counter Entity - Thread-safe incrementing counter
#[derive(Debug, Clone)]
pub struct Counter {
    /// Unique metric name
    pub name: MetricName,
    /// Atomic counter value
    value: Arc<AtomicU64>,
}

impl Counter {
    /// Create a new counter with initial value 0.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: MetricName::new(name),
            value: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Create a counter with initial value.
    pub fn with_value(name: impl Into<String>, initial: u64) -> Self {
        Self {
            name: MetricName::new(name),
            value: Arc::new(AtomicU64::new(initial)),
        }
    }

    /// Increment the counter by delta.
    pub fn inc(&self, delta: u64) {
        self.value.fetch_add(delta, Ordering::Relaxed);
    }

    /// Get current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Get the metric name.
    pub fn name(&self) -> &MetricName {
        &self.name
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Gauge Entity - Thread-safe numeric gauge using Mutex for f64
#[derive(Debug, Clone)]
pub struct Gauge {
    /// Unique metric name
    pub name: MetricName,
    /// Mutex-protected gauge value (f64)
    value: Arc<Mutex<f64>>,
}

impl Gauge {
    /// Create a new gauge with initial value 0.0.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: MetricName::new(name),
            value: Arc::new(Mutex::new(0.0)),
        }
    }

    /// Create a gauge with initial value.
    pub fn with_value(name: impl Into<String>, initial: f64) -> Self {
        Self {
            name: MetricName::new(name),
            value: Arc::new(Mutex::new(initial)),
        }
    }

    /// Set the gauge value.
    pub fn set(&self, val: f64) {
        *self.value.lock() = val;
    }

    /// Add to the gauge value.
    pub fn add(&self, delta: f64) {
        let mut v = self.value.lock();
        *v += delta;
    }

    /// Subtract from the gauge value.
    pub fn sub(&self, delta: f64) {
        let mut v = self.value.lock();
        *v -= delta;
    }

    /// Get current value.
    pub fn get(&self) -> f64 {
        *self.value.lock()
    }

    /// Get the metric name.
    pub fn name(&self) -> &MetricName {
        &self.name
    }
}

/// Histogram Entity - Thread-safe histogram with percentiles
#[derive(Debug, Clone)]
pub struct Histogram {
    /// Unique metric name
    pub name: MetricName,
    /// Internal histogram data
    data: Arc<HistogramData>,
}

#[derive(Debug, Clone)]
struct HistogramData {
    values: Arc<parking_lot::Mutex<Vec<u64>>>,
    count: Arc<AtomicU64>,
    sum: Arc<AtomicU64>,
    min: Arc<AtomicU64>,
    max: Arc<AtomicU64>,
}

impl Histogram {
    /// Create a new histogram.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: MetricName::new(name),
            data: Arc::new(HistogramData {
                values: Arc::new(parking_lot::Mutex::new(Vec::with_capacity(1000))),
                count: Arc::new(AtomicU64::new(0)),
                sum: Arc::new(AtomicU64::new(0)),
                min: Arc::new(AtomicU64::new(u64::MAX)),
                max: Arc::new(AtomicU64::new(0)),
            }),
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &MetricName {
        &self.name
    }

    /// Record a value in the histogram.
    pub fn record(&self, value: u64) {
        let mut values = self.data.values.lock();
        values.push(value);
        values.sort_unstable();

        self.data.count.fetch_add(1, Ordering::Relaxed);
        self.data.sum.fetch_add(value, Ordering::Relaxed);

        // Update min with atomic operations
        loop {
            let current_min = self.data.min.load(Ordering::Relaxed);
            if value >= current_min
                || self
                    .data
                    .min
                    .compare_exchange(current_min, value, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
            {
                break;
            }
        }

        // Update max with atomic operations
        loop {
            let current_max = self.data.max.load(Ordering::Relaxed);
            if value <= current_max
                || self
                    .data
                    .max
                    .compare_exchange(current_max, value, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
            {
                break;
            }
        }
    }

    /// Get the count of recorded values.
    pub fn count(&self) -> u64 {
        self.data.count.load(Ordering::Relaxed)
    }

    /// Get the sum of recorded values.
    pub fn sum(&self) -> u64 {
        self.data.sum.load(Ordering::Relaxed)
    }

    /// Get the minimum value.
    pub fn min(&self) -> u64 {
        let min = self.data.min.load(Ordering::Relaxed);
        if min == u64::MAX {
            0
        } else {
            min
        }
    }

    /// Get the maximum value.
    pub fn max(&self) -> u64 {
        self.data.max.load(Ordering::Relaxed)
    }

    /// Get the mean value.
    pub fn mean(&self) -> f64 {
        let count = self.count();
        if count == 0 {
            0.0
        } else {
            self.sum() as f64 / count as f64
        }
    }

    /// Get a percentile value.
    pub fn percentile(&self, p: f64) -> f64 {
        let values = self.data.values.lock();
        if values.is_empty() {
            return 0.0;
        }

        let idx = ((p / 100.0) * values.len() as f64) as usize;
        let idx = idx.min(values.len() - 1);
        values[idx] as f64
    }

    /// Get p50 (median).
    pub fn p50(&self) -> f64 {
        self.percentile(50.0)
    }

    /// Get p90.
    pub fn p90(&self) -> f64 {
        self.percentile(90.0)
    }

    /// Get p95.
    pub fn p99(&self) -> f64 {
        self.percentile(99.0)
    }

    /// Get p99.9.
    pub fn p999(&self) -> f64 {
        self.percentile(99.9)
    }
}

/// Summary Entity - Provides count, sum, and optionally quantiles
#[derive(Debug, Clone)]
pub struct Summary {
    pub name: MetricName,
    count: Arc<AtomicU64>,
    sum: Arc<parking_lot::Mutex<f64>>,
}

impl Summary {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: MetricName::new(name),
            count: Arc::new(AtomicU64::new(0)),
            sum: Arc::new(parking_lot::Mutex::new(0.0)),
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &MetricName {
        &self.name
    }

    pub fn observe(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        *self.sum.lock() += value;
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn sum(&self) -> f64 {
        *self.sum.lock()
    }

    pub fn mean(&self) -> f64 {
        let c = self.count();
        if c == 0 {
            0.0
        } else {
            self.sum() / c as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic() {
        let counter = Counter::new("requests");
        assert_eq!(counter.get(), 0);
        counter.inc(1);
        assert_eq!(counter.get(), 1);
        counter.inc(5);
        assert_eq!(counter.get(), 6);
    }

    #[test]
    fn test_counter_clone_shares_state() {
        let counter = Counter::new("test");
        let clone = counter.clone();
        counter.inc(10);
        assert_eq!(clone.get(), 10);
    }

    #[test]
    fn test_gauge_basic() {
        let gauge = Gauge::new("memory");
        assert_eq!(gauge.get(), 0.0);
        gauge.set(1024.0);
        assert_eq!(gauge.get(), 1024.0);
        gauge.add(512.0);
        assert_eq!(gauge.get(), 1536.0);
        gauge.sub(256.0);
        assert_eq!(gauge.get(), 1280.0);
    }

    #[test]
    fn test_histogram_basic() {
        let hist = Histogram::new("latency");
        hist.record(5);
        hist.record(15);
        hist.record(25);
        assert_eq!(hist.count(), 3);
        assert!((hist.p50() - 15.0).abs() < f64::EPSILON);
        assert_eq!(hist.min(), 5);
        assert_eq!(hist.max(), 25);
    }

    #[test]
    fn test_histogram_empty() {
        let hist = Histogram::new("empty");
        assert_eq!(hist.count(), 0);
        assert_eq!(hist.p50(), 0.0);
        assert_eq!(hist.mean(), 0.0);
    }

    #[test]
    fn test_histogram_percentiles() {
        let hist = Histogram::new("latency");
        for i in 1..=100 {
            hist.record(i);
        }
        let p50 = hist.p50();
        let p90 = hist.p90();
        let p99 = hist.p99();
        assert!((50.0..=51.0).contains(&p50));
        assert!((90.0..=91.0).contains(&p90));
        assert!((99.0..=100.0).contains(&p99));
    }

    #[test]
    fn test_summary_basic() {
        let summary = Summary::new("request_duration");
        summary.observe(10.0);
        summary.observe(20.0);
        summary.observe(30.0);
        assert_eq!(summary.count(), 3);
        assert!((summary.sum() - 60.0).abs() < f64::EPSILON);
        assert!((summary.mean() - 20.0).abs() < f64::EPSILON);
    }
}
