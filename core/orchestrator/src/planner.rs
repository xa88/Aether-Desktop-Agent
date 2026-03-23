use crate::intent::{Intent, IntentMatcher};

pub struct GoalDecomposer;

impl GoalDecomposer {
    /// Breaks a complex goal into smaller sub-tasks.
    pub fn decompose(goal: &str, budget_s: u64) -> Vec<String> {
        let intent = IntentMatcher::match_text(goal);
        let goal_lower = goal.to_lowercase();
        
        // Phase 7: Intent-based Template Mapping
        match intent {
            Intent::Dev if goal_lower.contains("scaffold") || goal_lower.contains("vite") => {
                return vec![
                    "fs_mkdir execute path='project-root'".to_string(),
                    "shell_run execute command='npm create vite@latest . -- --template react'".to_string(),
                    "shell_run execute command='npm install'".to_string(),
                    "test_run execute command='npm run build'".to_string(),
                ];
            }
            Intent::Dev if goal_lower.contains("git") && goal_lower.contains("conflict") => {
                return vec![
                    "git_status execute".to_string(),
                    "git_diff execute".to_string(),
                    "shell_run execute command='git mergetool --tool=vimdiff'".to_string(),
                    "git_commit execute message='Resolved merge conflicts'".to_string(),
                ];
            }
            Intent::SysOp if goal_lower.contains("port") && goal_lower.contains("busy") => {
                return vec![
                    "shell_run execute command='netstat -ano | findstr :8080'".to_string(),
                    "shell_run execute command='taskkill /F /PID <PID_FROM_PREV_STEP>'".to_string(),
                ];
            }
            _ => {}
        }

        let mut sub_goals = Vec::new();
        if goal.contains(";") {
            for part in goal.split(";") {
                sub_goals.push(part.trim().to_string());
            }
        } else if budget_s > 3600 {
            sub_goals.push(format!("Research and design for: {}", goal));
            sub_goals.push(format!("Implement: {}", goal));
            sub_goals.push(format!("Verify and test: {}", goal));
        } else {
            sub_goals.push(goal.to_string());
        }
        sub_goals
    }
}
