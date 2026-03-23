//! ada-audit: Append-only JSONL audit writer + artifacts.json + run_report.md generator.

pub mod events;
pub mod metrics;
pub mod artifacts;
pub mod report;
pub mod writer;
pub mod retention;
pub mod telemetry;

pub use events::*;
pub use metrics::*;
pub use writer::AuditWriter;
pub use telemetry::{TelemetryClient, TelemetryConfig};
