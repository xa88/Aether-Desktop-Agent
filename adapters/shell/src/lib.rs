//! Shell adapter: runs commands with timeout + kill-process-tree + output capture.

pub mod handler;
pub use handler::ShellHandler;
