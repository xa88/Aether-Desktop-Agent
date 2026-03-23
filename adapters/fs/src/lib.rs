//! Transactional filesystem adapter: read/write/mkdir/list/apply_patch with path guards.

pub mod handler;
pub use handler::FsHandler;
