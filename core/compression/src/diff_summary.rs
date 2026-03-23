//! Diff Summarization logic.

pub struct DiffSummarizer;

impl DiffSummarizer {
    pub fn summarize(diff: &str) -> String {
        if diff.is_empty() {
            return "No changes.".to_string();
        }

        let mut modified: Vec<&str> = Vec::new();
        let mut _added: Vec<&str> = Vec::new();
        let mut _deleted: Vec<&str> = Vec::new();

        for line in diff.lines() {
            if line.starts_with("--- ") { continue; }
            if line.starts_with("+++ ") {
                let file = line[4..].trim();
                if file == "/dev/null" { continue; }
                modified.push(file);
            }
        }

        // Extremely simplified MVP: just list the files
        let mut summary = String::new();
        if !modified.is_empty() {
            summary.push_str(&format!("Modified: {}", modified.join(", ")));
        }
        
        if summary.is_empty() { "Substantial changes detected.".to_string() } else { summary }
    }
}
