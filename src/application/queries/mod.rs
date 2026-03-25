//! # Queries (CQRS)
//!
//! Operations that read state without side effects.

/// Query to get a counter value
#[derive(Debug, Clone)]
pub struct GetCounterQuery {
    pub name: String,
}

/// Query to get a gauge value
#[derive(Debug, Clone)]
pub struct GetGaugeQuery {
    pub name: String,
}

/// Query to get histogram stats
#[derive(Debug, Clone)]
pub struct GetHistogramQuery {
    pub name: String,
}

/// Query to list all metrics
#[derive(Debug, Clone)]
pub struct ListMetricsQuery {}
