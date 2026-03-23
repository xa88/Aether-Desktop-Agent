//! ada-policy: path guard, command blacklist, risk-tier enforcement, secret redaction.

pub mod guard;
pub mod secrets;
pub mod profile;

pub use guard::PolicyGuard;
