//! ada-compression: Log truncation and summarization.

pub mod extract;
pub mod diff_summary;
pub mod manager;
pub mod report_gen;

pub use report_gen::ReportGenerator;
pub use extract::ErrorExtractor;
pub use diff_summary::DiffSummarizer;
