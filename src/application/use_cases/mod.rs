//! # Use Cases
//!
//! Application services that orchestrate domain logic.

use crate::domain::entities::{Counter, Gauge, Histogram, Summary};
use crate::domain::events::MetricsEvent;
use crate::ports::driven::MetricsPort;
use std::time::SystemTime;

/// Use case for incrementing a counter
pub struct IncCounterUseCase<T: MetricsPort, E> {
    registry: T,
    events: E,
}

impl<T: MetricsPort, E: FnMut(MetricsEvent)> IncCounterUseCase<T, E> {
    pub fn new(registry: T, events: E) -> Self {
        Self { registry, events }
    }

    pub fn execute(&mut self, name: String, delta: u64) -> Result<u64, String> {
        let counter = self.registry.get_counter(&name)
            .ok_or_else(|| format!("Counter '{}' not found", name))?;

        let new_value = counter.get() + delta;
        (self.events)(MetricsEvent::CounterIncremented {
            name: name.clone(),
            delta,
            new_value,
            timestamp: SystemTime::now(),
        });

        Ok(new_value)
    }
}

/// Use case for setting a gauge
pub struct SetGaugeUseCase<T: MetricsPort, E> {
    registry: T,
    events: E,
}

impl<T: MetricsPort, E: FnMut(MetricsEvent)> SetGaugeUseCase<T, E> {
    pub fn new(registry: T, events: E) -> Self {
        Self { registry, events }
    }

    pub fn execute(&mut self, name: String, value: f64) -> Result<(), String> {
        let gauge = self.registry.get_gauge(&name)
            .ok_or_else(|| format!("Gauge '{}' not found", name))?;

        gauge.set(value);
        (self.events)(MetricsEvent::GaugeSet {
            name,
            value,
            timestamp: SystemTime::now(),
        });

        Ok(())
    }
}

/// Use case for recording a histogram value
pub struct RecordHistogramUseCase<T: MetricsPort, E> {
    registry: T,
    events: E,
}

impl<T: MetricsPort, E: FnMut(MetricsEvent)> RecordHistogramUseCase<T, E> {
    pub fn new(registry: T, events: E) -> Self {
        Self { registry, events }
    }

    pub fn execute(&mut self, name: String, value: u64) -> Result<(), String> {
        let histogram = self.registry.get_histogram(&name)
            .ok_or_else(|| format!("Histogram '{}' not found", name))?;

        histogram.record(value);
        (self.events)(MetricsEvent::HistogramRecorded {
            name,
            value,
            timestamp: SystemTime::now(),
        });

        Ok(())
    }
}
