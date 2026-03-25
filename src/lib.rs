//! # thegent-metrics
//!
//! High-performance metrics collection for multi-agent orchestration.
//!
//! ## Architecture
//!
//! This crate follows **Hexagonal Architecture** (Ports & Adapters) with **Clean Architecture** layers.
//!
//! ## xDD Methodologies Applied
//!
//! - **TDD**: Tests written first
//! - **DDD**: Bounded contexts for metrics types
//! - **SOLID**: Single responsibility per module
//! - **CQRS**: Separate command and query interfaces
//! - **EDA**: Domain events for metric changes
//! - **TraceDD**: Trace identifiers on all operations

pub mod domain;
pub mod application;
pub mod ports;
pub mod adapters;

// Re-export for convenience
pub use domain::entities::*;
pub use domain::value_objects::*;
pub use domain::events::*;
pub use application::commands::*;
pub use application::queries::*;
pub use application::use_cases::*;
pub use ports::driven::MetricsPort;
pub use ports::driven::CounterPort;
pub use ports::driven::GaugePort;
pub use ports::driven::HistogramPort;
