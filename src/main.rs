//! # thegent-metrics CLI
//!
//! Command-line interface for metrics collection.

use clap::{Parser, Subcommand};
use thegent_metrics::domain::entities::{Counter, Gauge, Histogram, Summary};
use thegent_metrics::ports::driven::MetricsPort;
use thegent_metrics::adapters::prometheus::PrometheusFormatter;
use thegent_metrics::adapters::json::JsonFormatter;
use std::sync::Arc;
use dashmap::DashMap;

/// Metrics registry using DashMap for thread-safety
struct Registry {
    counters: DashMap<String, Counter>,
    gauges: DashMap<String, Gauge>,
    histograms: DashMap<String, Histogram>,
    summaries: DashMap<String, Summary>,
}

impl Registry {
    fn new() -> Self {
        Self {
            counters: DashMap::new(),
            gauges: DashMap::new(),
            histograms: DashMap::new(),
            summaries: DashMap::new(),
        }
    }
}

impl MetricsPort for Registry {
    fn register_counter(&self, counter: Counter) -> Result<(), String> {
        let name = counter.name().as_str().to_string();
        if self.counters.contains_key(&name) {
            return Err(format!("Counter '{}' already exists", name));
        }
        self.counters.insert(name, counter);
        Ok(())
    }

    fn register_gauge(&self, gauge: Gauge) -> Result<(), String> {
        let name = gauge.name().as_str().to_string();
        if self.gauges.contains_key(&name) {
            return Err(format!("Gauge '{}' already exists", name));
        }
        self.gauges.insert(name, gauge);
        Ok(())
    }

    fn register_histogram(&self, histogram: Histogram) -> Result<(), String> {
        let name = histogram.name().as_str().to_string();
        if self.histograms.contains_key(&name) {
            return Err(format!("Histogram '{}' already exists", name));
        }
        self.histograms.insert(name, histogram);
        Ok(())
    }

    fn register_summary(&self, summary: Summary) -> Result<(), String> {
        let name = summary.name().as_str().to_string();
        if self.summaries.contains_key(&name) {
            return Err(format!("Summary '{}' already exists", name));
        }
        self.summaries.insert(name, summary);
        Ok(())
    }

    fn get_counter(&self, name: &str) -> Option<Counter> {
        self.counters.get(name).map(|r| r.clone())
    }

    fn get_gauge(&self, name: &str) -> Option<Gauge> {
        self.gauges.get(name).map(|r| r.clone())
    }

    fn get_histogram(&self, name: &str) -> Option<Histogram> {
        self.histograms.get(name).map(|r| r.clone())
    }

    fn get_summary(&self, name: &str) -> Option<Summary> {
        self.summaries.get(name).map(|r| r.clone())
    }

    fn list_counters(&self) -> Vec<String> {
        self.counters.iter().map(|r| r.key().clone()).collect()
    }

    fn list_gauges(&self) -> Vec<String> {
        self.gauges.iter().map(|r| r.key().clone()).collect()
    }

    fn list_histograms(&self) -> Vec<String> {
        self.histograms.iter().map(|r| r.key().clone()).collect()
    }

    fn list_summaries(&self) -> Vec<String> {
        self.summaries.iter().map(|r| r.key().clone()).collect()
    }
}

#[derive(Parser)]
#[command(name = "thegent-metrics")]
#[command(about = "High-performance metrics collection CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register a new counter
    Counter {
        name: String,
    },

    /// Register a new gauge
    Gauge {
        name: String,
    },

    /// Register a new histogram
    Histogram {
        name: String,
    },

    /// Register a new summary
    Summary {
        name: String,
    },

    /// Increment a counter
    Inc {
        name: String,
        #[arg(default_value = "1")]
        delta: u64,
    },

    /// Set a gauge value
    Set {
        name: String,
        value: f64,
    },

    /// Record a histogram value
    Record {
        name: String,
        value: u64,
    },

    /// Observe a summary value
    Observe {
        name: String,
        value: f64,
    },

    /// List all metrics
    List,

    /// Show a specific metric
    Show {
        name: String,
    },

    /// Export in Prometheus format
    Prometheus,

    /// Export in JSON format
    Json,

    /// Delete a metric
    Delete {
        name: String,
    },

    /// Clear all metrics
    Clear,
}

fn main() {
    let cli = Cli::parse();
    let registry = Arc::new(Registry::new());

    match cli.command {
        Commands::Counter { name } => {
            let counter = Counter::new(&name);
            match registry.register_counter(counter) {
                Ok(()) => println!("Registered counter: {}", name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Gauge { name } => {
            let gauge = Gauge::new(&name);
            match registry.register_gauge(gauge) {
                Ok(()) => println!("Registered gauge: {}", name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Histogram { name } => {
            let histogram = Histogram::new(&name);
            match registry.register_histogram(histogram) {
                Ok(()) => println!("Registered histogram: {}", name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Summary { name } => {
            let summary = Summary::new(&name);
            match registry.register_summary(summary) {
                Ok(()) => println!("Registered summary: {}", name),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Inc { name, delta } => {
            // For increment, we need mutable access
            if let Some(counter) = registry.get_counter(&name) {
                counter.inc(delta);
                println!("Incremented {} by {}, new value: {}", name, delta, counter.get());
            } else {
                eprintln!("Counter '{}' not found", name);
            }
        }

        Commands::Set { name, value } => {
            if let Some(gauge) = registry.get_gauge(&name) {
                gauge.set(value);
                println!("Set {} to {}", name, value);
            } else {
                eprintln!("Gauge '{}' not found", name);
            }
        }

        Commands::Record { name, value } => {
            if let Some(histogram) = registry.get_histogram(&name) {
                histogram.record(value);
                println!("Recorded {} in {}", value, name);
            } else {
                eprintln!("Histogram '{}' not found", name);
            }
        }

        Commands::Observe { name, value } => {
            if let Some(summary) = registry.get_summary(&name) {
                summary.observe(value);
                println!("Observed {} in {}", value, name);
            } else {
                eprintln!("Summary '{}' not found", name);
            }
        }

        Commands::List => {
            println!("=== Counters ===");
            for name in registry.list_counters() {
                if let Some(counter) = registry.get_counter(&name) {
                    println!("  {} = {}", name, counter.get());
                }
            }

            println!("\n=== Gauges ===");
            for name in registry.list_gauges() {
                if let Some(gauge) = registry.get_gauge(&name) {
                    println!("  {} = {}", name, gauge.get());
                }
            }

            println!("\n=== Histograms ===");
            for name in registry.list_histograms() {
                if let Some(histogram) = registry.get_histogram(&name) {
                    println!("  {}: count={}, mean={:.2}, p50={:.2}, p99={:.2}",
                        name, histogram.count(), histogram.mean(), histogram.p50(), histogram.p99());
                }
            }

            println!("\n=== Summaries ===");
            for name in registry.list_summaries() {
                if let Some(summary) = registry.get_summary(&name) {
                    println!("  {}: count={}, mean={:.2}", name, summary.count(), summary.mean());
                }
            }
        }

        Commands::Show { name } => {
            if let Some(counter) = registry.get_counter(&name) {
                println!("Counter: {}", name);
                println!("  Value: {}", counter.get());
            } else if let Some(gauge) = registry.get_gauge(&name) {
                println!("Gauge: {}", name);
                println!("  Value: {}", gauge.get());
            } else if let Some(histogram) = registry.get_histogram(&name) {
                println!("Histogram: {}", name);
                println!("  Count: {}", histogram.count());
                println!("  Sum: {}", histogram.sum());
                println!("  Min: {}", histogram.min());
                println!("  Max: {}", histogram.max());
                println!("  Mean: {:.2}", histogram.mean());
                println!("  p50: {:.2}", histogram.p50());
                println!("  p90: {:.2}", histogram.p90());
                println!("  p99: {:.2}", histogram.p99());
            } else if let Some(summary) = registry.get_summary(&name) {
                println!("Summary: {}", name);
                println!("  Count: {}", summary.count());
                println!("  Sum: {:.2}", summary.sum());
                println!("  Mean: {:.2}", summary.mean());
            } else {
                eprintln!("Metric '{}' not found", name);
            }
        }

        Commands::Prometheus => {
            println!("{}", PrometheusFormatter::format_registry(&*registry));
        }

        Commands::Json => {
            println!("{}", JsonFormatter::format(&*registry));
        }

        Commands::Delete { name } => {
            let removed = registry.counters.remove(&name).is_some()
                || registry.gauges.remove(&name).is_some()
                || registry.histograms.remove(&name).is_some()
                || registry.summaries.remove(&name).is_some();

            if removed {
                println!("Deleted: {}", name);
            } else {
                eprintln!("Metric '{}' not found", name);
            }
        }

        Commands::Clear => {
            registry.counters.clear();
            registry.gauges.clear();
            registry.histograms.clear();
            registry.summaries.clear();
            println!("Cleared all metrics");
        }
    }
}
