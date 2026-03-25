//! # Commands (CQRS)
//!
//! Operations that change state.

/// Command to increment a counter
#[derive(Debug, Clone)]
pub struct IncCounterCommand {
    pub name: String,
    pub delta: u64,
}

/// Command to reset a counter
#[derive(Debug, Clone)]
pub struct ResetCounterCommand {
    pub name: String,
}

/// Command to set a gauge
#[derive(Debug, Clone)]
pub struct SetGaugeCommand {
    pub name: String,
    pub value: f64,
}

/// Command to add to a gauge
#[derive(Debug, Clone)]
pub struct AddGaugeCommand {
    pub name: String,
    pub delta: f64,
}

/// Command to record a histogram value
#[derive(Debug, Clone)]
pub struct RecordHistogramCommand {
    pub name: String,
    pub value: u64,
}

/// Command to observe a summary value
#[derive(Debug, Clone)]
pub struct ObserveSummaryCommand {
    pub name: String,
    pub value: f64,
}
