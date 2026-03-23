//! Report Generation logic.

use crate::extract::ErrorExtractor;
use crate::diff_summary::DiffSummarizer;

pub struct ReportGenerator {
    extractor: ErrorExtractor,
}

impl ReportGenerator {
    pub fn new() -> Self {
        Self { extractor: ErrorExtractor::new() }
    }

    pub fn generate_report(&self, title: &str, log: &str, diff: &str) -> String {
        let signatures = self.extractor.extract_signatures(log);
        let clusters = self.extractor.cluster_errors(signatures);
        let diff_summary = DiffSummarizer::summarize(diff);

        let mut report = format!("# Run Report: {}\n\n", title);
        
        report.push_str("## Diff Summary\n");
        report.push_str(&format!("{}\n\n", diff_summary));

        report.push_str("## Top Errors\n");
        if clusters.is_empty() {
            report.push_str("No major errors detected.\n");
        } else {
            for (sig, count) in clusters.iter().take(3) {
                report.push_str(&format!("- **{}** (occurred {} times)\n", sig, count));
            }
        }

        report.push_str("\n## Log Snippets (Tail)\n");
        let tail: Vec<&str> = log.lines().rev().take(20).collect::<Vec<_>>().into_iter().rev().collect();
        report.push_str("```\n");
        report.push_str(&tail.join("\n"));
        report.push_str("\n```\n");

        report
    }
}
