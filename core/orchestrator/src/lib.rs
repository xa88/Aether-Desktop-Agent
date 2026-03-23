use std::sync::Arc;
use serde::{Serialize, Deserialize};
use ada_tool_api::router::ToolRouter;
use ada_llm::{LlmProvider, PlanContext, PromptBuilder, LocalTemplateEngine, IntentTaxonomy};
use ada_executor::{PlanValidator, PlanSplitter};
use ada_cache::CacheManager;
use ada_compression::manager::CompressionManager;
pub mod intent;
pub mod planner;
pub mod escalation;
pub mod swarm;
pub mod cluster;
pub mod memory;

use tracing::{info, warn, debug};
use crate::escalation::EscalationEngine;
use crate::swarm::SwarmManager;
use crate::cluster::ClusterManager;
use crate::memory::MemoryManager;

pub struct Orchestrator {
    pub router: Arc<ToolRouter>,
    pub director_llm: Arc<dyn LlmProvider>,
    pub worker_llm: Arc<dyn LlmProvider>,
    pub validator: PlanValidator,
    pub cache: Option<CacheManager>,
    pub escalation_engine: EscalationEngine,
    pub template_engine: LocalTemplateEngine,
    pub compression_manager: CompressionManager,
    pub swarm_manager: SwarmManager,
    pub cluster_manager: ClusterManager,
    pub memory_manager: MemoryManager,
    pub cloud_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBundle {
    pub audit_log: String,
    pub recent_report: String,
    pub error_signatures: Vec<String>,
}

// Redundant EscalationPolicy removed

impl Orchestrator {
    pub fn new(router: Arc<ToolRouter>, director_llm: Arc<dyn LlmProvider>, worker_llm: Arc<dyn LlmProvider>, cache_path: Option<String>, cloud_enabled: bool) -> anyhow::Result<Self> {
        let validator = PlanValidator::new()?;
        let cache = cache_path.and_then(|p| CacheManager::new(p).ok());
        let escalation_engine = EscalationEngine::new(router.clone(), validator.clone(), 3);
        let template_engine = LocalTemplateEngine::new();
        let compression_manager = CompressionManager::new(4000); // 4k token threshold
        let swarm_manager = SwarmManager::new();
        let cluster_manager = ClusterManager::new();
        let memory_manager = MemoryManager::new(std::path::PathBuf::from("runs/memory"));
        
        // Start background discovery threads
        cluster_manager.start_discovery();
        cluster_manager.broadcast_presence();
        cluster_manager.start_task_listener().ok();

        Ok(Self { router, director_llm, worker_llm, validator, cache, escalation_engine, template_engine, compression_manager, swarm_manager, cluster_manager, memory_manager, cloud_enabled })
    }

    pub async fn bundle_context(&self, audit_path: &str) -> anyhow::Result<ContextBundle> {
        debug!("Bundling context for escalation...");
        let mut audit_log = std::fs::read_to_string(audit_path).unwrap_or_default();
        let recent_report = std::fs::read_to_string("runs/run_report.md").unwrap_or_default();
        
        // Phase 9: Compression if too large
        if audit_log.len() > 10000 { // Approx characters, can be more precise with tokens
            info!("Context exceeds threshold. Compressing...");
            if let Ok(compressed) = self.compression_manager.summarize_context(&audit_log).await {
                audit_log = compressed.summary;
                info!("Context compressed from {} to {} chars.", compressed.original_length, compressed.compressed_length);
            }
        }

        Ok(ContextBundle {
            audit_log,
            recent_report,
            error_signatures: vec![],
        })
    }

    pub async fn process_task(&self, goal: &str, context: PlanContext) -> anyhow::Result<()> {
        let run_id = uuid::Uuid::new_v4().to_string();
        info!("Orchestrating task: {} (Run ID: {})", goal, run_id);
        
        // Define a helper to bundle at the end of the run
        let finalize_run = |run_id: &str| {
            if let Err(e) = ada_audit::artifacts::BundleArchiver::create_bundle(run_id, std::path::Path::new("runs/bundles")) {
                warn!("Failed to create run bundle: {}", e);
            } else {
                info!("Bundle created successfully for run {}", run_id);
            }

            // Enforce the Local Storage Data Retention policy asynchronously mapping bounds
            if let Err(e) = ada_audit::retention::cleanup_old_runs("runs", Default::default()) {
                warn!("Retention Policy Cleanup failed: {}", e);
            }
        };
        
        // 1. Check for local intent
        let intent = intent::IntentMatcher::match_text(goal);
        if intent != intent::Intent::Dev {
            if let Some(local_plan) = intent::LocalPlanGenerator::generate_for_intent(intent.clone(), goal) {
                info!("Local intent matched: {:?}. Running local template.", intent);
                
                // If the plan is large, split it
                let sub_plans = if local_plan.steps.len() > 5 {
                    PlanSplitter::split(&local_plan, 5)
                } else {
                    vec![local_plan]
                };

                for sub in sub_plans {
                    let agent_id = self.swarm_manager.spawn_worker(&sub.meta.title);
                    self.swarm_manager.update_worker_status(&agent_id, crate::swarm::AgentStatus::Executing, 0.1);
                    
                    info!("Executing sub-plan: {} (Agent: {})", sub.meta.title, agent_id);
                    // Actual execution loop would go here
                    self.swarm_manager.update_worker_status(&agent_id, crate::swarm::AgentStatus::Success, 1.0);
                }
                finalize_run(&run_id);
                return Ok(());
            } else {
                // Try rendering a template for the intent if no direct plan exists
                let mut data = std::collections::HashMap::new();
                data.insert("goal".to_string(), goal.to_string());
                
                let taxonomy = match intent {
                    intent::Intent::Query => IntentTaxonomy::Research,
                    intent::Intent::SysOp => IntentTaxonomy::Optimize, // Example mapping
                    _ => IntentTaxonomy::Dev,
                };

                if let Ok(prompt) = self.template_engine.render(taxonomy, &data) {
                    debug!("Rendered local template for {:?}: {}", intent, prompt);
                    // In a real scenario, we'd feed this prompt to a specialized LLM call
                }
            }
        }

        // 0. Perpetual Memory Recall
        if let Some(exp) = self.memory_manager.recall_experience(goal) {
            info!("RECALLED EXPERIENCE: Found successful past plan for '{}'", goal);
            // In a real system, we'd inject this into the PromptBuilder as the primary few-shot example.
        }

        // 1. Check for local intent

        if !self.cloud_enabled {
            finalize_run(&run_id);
            return Err(anyhow::anyhow!("CloudDisabledError: Enterprise Policy prohibits upstream LLM execution. Local intents naturally exhausted."));
        }

        let mut attempt = 0;
        let max_attempts = 3;
        
        loop {
            attempt += 1;
            info!("Generating plan (attempt {}/{})", attempt, max_attempts);
            
            ada_audit::record_cloud_round();
            let prompt = PromptBuilder::build_plan_prompt(goal, &context);
            let mut raw_reply = self.director_llm.generate_plan(&prompt).await?;
            
            // Try to extract yaml block if it contains one
            if raw_reply.contains("```yaml") && raw_reply.contains("```") {
                if let Some(start) = raw_reply.find("```yaml") {
                    let after_start = &raw_reply[start + 7..];
                    if let Some(end) = after_start.find("```") {
                        raw_reply = after_start[..end].to_string();
                    }
                }
            } else if raw_reply.starts_with("```") {
                let after_start = &raw_reply[3..];
                if let Some(end) = after_start.find("```") {
                    raw_reply = after_start[..end].to_string();
                }
            }
            
            let raw_reply = raw_reply.trim().to_string();
            
            match self.validator.validate_raw(&raw_reply) {
                Ok(plan) => {
                    info!("Plan validated successfully: {}", plan.meta.title);
                    // TODO: Execute plan via Executor
                    finalize_run(&run_id);
                    return Ok(());
                }
                Err(e) => {
                    if self.escalation_engine.maybe_escalate(attempt, &anyhow::anyhow!(e.to_string()), goal, &context).await? {
                        let _bundle = self.bundle_context("runs/audit.jsonl").await?;
                        finalize_run(&run_id);
                        return Err(anyhow::anyhow!("Task escalated: {}", e));
                    } else {
                        warn!("Plan validation failed, retrying. Error: {}", e);
                    }
                }
            }
        }
    }
}
