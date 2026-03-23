use crate::plan::Plan;

pub struct PlanSplitter;

impl PlanSplitter {
    /// Splits a large plan into multiple sub-plans based on a step limit.
    pub fn split(plan: &Plan, max_steps_per_plan: usize) -> Vec<Plan> {
        let mut sub_plans = Vec::new();
        let chunks = plan.steps.chunks(max_steps_per_plan);

        for (i, chunk) in chunks.enumerate() {
            let mut sub_plan = plan.clone();
            sub_plan.meta.task_id = format!("{}-part-{}", plan.meta.task_id, i + 1);
            sub_plan.meta.title = format!("{} (Part {})", plan.meta.title, i + 1);
            sub_plan.steps = chunk.to_vec();
            sub_plans.push(sub_plan);
        }

        sub_plans
    }

    /// Splits a plan at specific step IDs.
    pub fn split_at(plan: &Plan, step_ids: Vec<String>) -> Vec<Plan> {
        let mut sub_plans = Vec::new();
        let mut current_steps = Vec::new();
        let mut part = 1;

        for step in &plan.steps {
            current_steps.push(step.clone());
            if step_ids.contains(&step.id) {
                let mut sub_plan = plan.clone();
                sub_plan.meta.task_id = format!("{}-part-{}", plan.meta.task_id, part);
                sub_plan.meta.title = format!("{} (Part {})", plan.meta.title, part);
                sub_plan.steps = current_steps;
                sub_plans.push(sub_plan);
                
                current_steps = Vec::new();
                part += 1;
            }
        }

        if !current_steps.is_empty() {
            let mut sub_plan = plan.clone();
            sub_plan.meta.task_id = format!("{}-part-{}", plan.meta.task_id, part);
            sub_plan.meta.title = format!("{} (Part {})", plan.meta.title, part);
            sub_plan.steps = current_steps;
            sub_plans.push(sub_plan);
        }

        sub_plans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::{PlanMeta, Budgets, ExecutionMode, PlanStep};
    use std::collections::HashMap;

    fn mock_plan(steps_count: usize) -> Plan {
        let mut steps = Vec::new();
        for i in 0..steps_count {
            steps.push(PlanStep {
                id: format!("step_{}", i),
                step_type: "shell_run".to_string(),
                risk_tier: None,
                timeout_s: 60,
                retries: 0,
                retry_backoff_ms: 0,
                cwd: None,
                env: HashMap::new(),
                args: serde_json::json!({"command": "echo hello"}),
                on_fail: None,
            });
        }

        Plan {
            schema_version: "1.0".to_string(),
            meta: PlanMeta {
                task_id: "test-task".to_string(),
                title: "Test Plan".to_string(),
                created_at: "2024-01-01".to_string(),
                requested_by: "test".to_string(),
                workspace: None,
                execution_mode: ExecutionMode::Host,
                budgets: Budgets {
                    max_wall_time_s: 3600,
                    max_steps: 100,
                    max_cmd_output_kb: 512,
                    max_artifacts_mb: 500,
                    max_retries_total: 20,
                },
            },
            artifacts_to_produce: Vec::new(),
            rollback: None,
            safety: None,
            steps,
            checks: Vec::new(),
        }
    }

    #[test]
    fn test_split_by_count() {
        let plan = mock_plan(12);
        let sub_plans = PlanSplitter::split(&plan, 5);
        assert_eq!(sub_plans.len(), 3);
        assert_eq!(sub_plans[0].steps.len(), 5);
        assert_eq!(sub_plans[1].steps.len(), 5);
        assert_eq!(sub_plans[2].steps.len(), 2);
        assert!(sub_plans[0].meta.task_id.ends_with("-part-1"));
    }

    #[test]
    fn test_split_at_id() {
        let plan = mock_plan(5);
        let sub_plans = PlanSplitter::split_at(&plan, vec!["step_2".to_string()]);
        assert_eq!(sub_plans.len(), 2);
        assert_eq!(sub_plans[0].steps.len(), 3); // step_0, step_1, step_2
        assert_eq!(sub_plans[1].steps.len(), 2); // step_3, step_4
    }
}
