//! Git adapter: status/diff/stash/commit/checkout via CLI subprocess.

pub mod handler;
pub use handler::GitHandler;
