use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainReq {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub name: String,
    pub working_dir: String,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOutput {
    pub env_var: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokeCheck {
    pub name: String,
    pub command: Vec<String>,
    pub expected_exit_code: i32,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRecipe {
    pub version: String,
    pub kind: String,
    pub id: String,
    pub description: String,
    pub platforms: Vec<String>,
    pub required_toolchains: Vec<ToolchainReq>,
    pub steps: Vec<RecipeStep>,
    pub outputs: Vec<RecipeOutput>,
    #[serde(default)]
    pub smoke_checks: Vec<SmokeCheck>,
}

pub struct RecipeRunner;

impl RecipeRunner {
    pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<BuildRecipe> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&content)?)
    }

    pub fn execute(recipe: &BuildRecipe) -> anyhow::Result<()> {
        info!("Executing Build Recipe: {} ({})", recipe.id, recipe.description);
        
        // Mock Execution Step
        for step in &recipe.steps {
            info!("Running Step [{}]: {:?}", step.name, step.command);
            // In a real execution, we would use std::process::Command
            // Setup working_dir and execute.
        }

        // Output Collection
        for output in &recipe.outputs {
            info!("Registered Output {}: {}", output.env_var, output.path);
        }

        // Smoke Checks
        for check in &recipe.smoke_checks {
            info!("Running Smoke Check [{}]: {:?}", check.name, check.command);
            // In a real execution, we would invoke and assert check.expected_exit_code
        }

        info!("Recipe {} executed successfully.", recipe.id);
        Ok(())
    }
}
