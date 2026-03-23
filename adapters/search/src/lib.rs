//! Search adapter: wraps ripgrep for structured match output.

pub mod handler;
pub mod web_handler;
pub use handler::SearchHandler;
