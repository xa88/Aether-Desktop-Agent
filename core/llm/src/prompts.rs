//! Prompt Protocol: builds structured prompts for the LLM.


pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build_plan_prompt(goal: &str, context: &PlanContext) -> String {
        format!(
r###"Task: Generate a plan.yaml for the following goal.
Goal: {goal}

System Info & Context:
- Workspace: {workspace}
- OS: {os}
- Max Steps Allowed: {max_steps}

Safety & Policy Constraints:
{constraints}

Environment State:
{state}

Top Failures (if any from previous run):
{failures}

Diff Summary (Recent File Changes):
{diff_summary}

Additional Details:
{context_details}

Response Format:
You MUST output ONLY a valid YAML block containing the plan. Ensure no markdown formatting or prefacing text outside the YAML block.
```yaml
schema_version: '1.0'
meta:
  task_id: '...'
  title: '...'
  ...
steps:
  - id: 'step-1'
    type: '...'
    ...
```
"###,
            goal = goal,
            workspace = context.workspace_path,
            os = context.os,
            max_steps = context.max_steps,
            constraints = context.constraints.join("\n"),
            state = context.state.join("\n"),
            failures = context.failures.join("\n"),
            diff_summary = context.diff_summary.join("\n"),
            context_details = context.details.join("\n")
        )
    }
}

pub struct PlanContext {
    pub workspace_path: String,
    pub os: String,
    pub max_steps: usize,
    pub constraints: Vec<String>,
    pub state: Vec<String>,
    pub failures: Vec<String>,
    pub diff_summary: Vec<String>,
    pub details: Vec<String>,
}

impl Default for PlanContext {
    fn default() -> Self {
        Self {
            workspace_path: ".".into(),
            os: std::env::consts::OS.into(),
            max_steps: 10,
            constraints: vec![],
            state: vec![],
            failures: vec![],
            diff_summary: vec![],
            details: vec![],
        }
    }
}
