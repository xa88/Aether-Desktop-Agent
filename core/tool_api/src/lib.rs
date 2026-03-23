//! ada-tool-api: Defines the unified Tool API types for ADA.
//! All tool calls between Core and Adapters flow through these types.

pub mod ipc;
pub mod router;
pub mod types;

pub use types::*;
