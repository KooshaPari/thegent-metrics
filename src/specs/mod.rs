//! # Specifications (SpecDD)
//!
//! Formal specifications for domain behavior.

/// Counter Specification
///
/// ## Invariants
/// - Value is always >= 0
/// - Increment is atomic
///
/// ## Properties
/// - P1: inc(d) by n times results in value == initial + n * d
/// - P2: Clone shares same underlying value
pub struct CounterSpec {}

/// Gauge Specification
///
/// ## Invariants
/// - Value can be any f64
///
/// ## Properties
/// - P1: set(v) results in value == v
/// - P2: add(d) results in value += d
/// - P3: sub(d) results in value -= d
pub struct GaugeSpec {}

/// Histogram Specification
///
/// ## Invariants
/// - Count is number of recorded values
/// - Percentiles are monotonic
///
/// ## Properties
/// - P1: record(v) increments count by 1
/// - P2: p50 <= p90 <= p99
/// - P3: min <= mean <= max
pub struct HistogramSpec {}
