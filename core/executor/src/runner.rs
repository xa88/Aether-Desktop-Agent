//! Core Executor: loads a plan.yaml and runs steps sequentially with retry/rollback/budget.

use crate::budget::BudgetTracker;
use crate::on_fail::{decide, OnFailDecision};
use crate::plan::{load_plan, Plan, PlanStep};
use ada_cache::{CacheManager, RepoFingerprint};
use ada_audit::{AuditWriter, AuditEvent, AuditActor, AuditResult, self as audit};
use ada_tool_api::{ToolRequest};
use ada_tool_api::router::ToolRouter;
use ada_sandbox::SandboxManager;
// use ada_compression::ReportGenerator;
use ada_playbooks::{PlaybookRegistry, Playbook};
use ::anyhow::{Result};
use chrono::Utc;
use sha2::{Sha256, Digest};
use std::time::Instant;
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct ExecutorConfig {
    pub plan_path: String,
    pub audit_path: String,
    pub cache_path: Option<String>,
    pub dry_run: bool,
}

pub struct Executor {
    config: ExecutorConfig,
    router: ToolRouter,
    sandbox: SandboxManager,
    cache: Option<CacheManager>,
    playbooks: PlaybookRegistry,
    gate: crate::concurrency::ResourceGate,
}

impl Executor {
    pub fn new(config: ExecutorConfig, router: ToolRouter, sandbox: SandboxManager) -> Self {
        let cache = config.cache_path.as_ref().and_then(|p| CacheManager::new(p).ok());
        let mut playbooks = PlaybookRegistry::new();
        let _ = playbooks.load_from_dir("playbooks");
        let gate = crate::concurrency::ResourceGate::new();
        Self { config, router, sandbox, cache, playbooks, gate }
    }

    pub async fn run(&self) -> Result<RunSummary> {
        let plan = load_plan(&self.config.plan_path)?;
        info!("Loaded plan '{}' ({} steps)", plan.meta.title, plan.steps.len());

        let run_id = Uuid::new_v4().to_string();
        let audit = AuditWriter::open(&self.config.audit_path).await?;
        let mut budget = BudgetTracker::new(plan.meta.budgets.clone());

        let mut steps_ok = 0usize;
        let mut steps_fail = 0usize;
        let mut escalate_hint: Option<String> = None;
        let mut full_log = String::new();

        // Check for resumed state
        let start_index = if let Some(cache) = &self.cache {
            cache.get_exec(&format!("run:{}:step", run_id))
                 .ok()
                 .flatten()
                 .and_then(|s| s.parse::<usize>().ok())
                 .unwrap_or(0)
        } else { 0 };

        for (i, step) in plan.steps.iter().enumerate() {
            if i < start_index {
                steps_ok += 1;
                continue;
            }
            // Check budget before each step and smoothly degrade
            if let Err(e) = budget.check_step() {
                warn!("Budget exceeded: {}. Auto-degrading run to PARTIAL status.", e);
                escalate_hint = Some(format!("PARTIAL: {}", e));
                break;
            }

            let result = self.run_step_with_retry(step, &plan, &run_id, &audit, &mut budget).await;
            match result {
                Ok(_) => {
                    steps_ok += 1;
                    full_log.push_str(&format!("[{}] Success\n", step.id));
                }
                Err(StepOutcome::Skipped) => { 
                    info!("Step {} skipped", step.id);
                    full_log.push_str(&format!("[{}] Skipped\n", step.id));
                }
                Err(StepOutcome::Abort(msg)) => {
                    error!("Aborting at step {}: {msg}", step.id);
                    steps_fail += 1;
                    full_log.push_str(&format!("[{}] Failed: {}\n", step.id, msg));
                    break;
                }
                Err(StepOutcome::Escalate(hint)) => {
                    info!("Escalating plan at step {}", step.id);
                    escalate_hint = hint;
                    steps_fail += 1;
                    break;
                }
                Err(StepOutcome::Rollback(to)) => {
                    if let Some(snapshot_name) = &to {
                        warn!("Rollback requested to: {}", snapshot_name);
                        let _ = self.sandbox.snapshot_restore(snapshot_name).await;
                    } else {
                        warn!("Rollback requested without target snapshot");
                    }
                    steps_fail += 1;
                    break;
                }
            }
            
            // Checkpoint progress
            if let Some(cache) = &self.cache {
                let _ = cache.set_exec(&format!("run:{}:step", run_id), &(i + 1).to_string(), 7);
            }
        }

        // Generate Advanced Run Report (Phase 7)
        let report_obj = audit::report::RunReport {
            run_id: run_id.clone(),
            title: plan.meta.title.clone(),
            execution_mode: if self.config.dry_run { "Dry Run".into() } else { "Host/Sandbox".into() },
            result: if steps_fail == 0 { "SUCCESS".into() } else { "PARTIAL/FAIL".into() },
            start: Utc::now() - chrono::Duration::try_seconds(budget.elapsed_s() as i64).unwrap_or_else(|| chrono::Duration::zero()),
            end: Utc::now(),
            steps_executed: steps_ok + steps_fail,
            cloud_llm_calls: 0, // Mocked for now
            events: vec![], // In a real scenario, we'd filter audit events here
            artifacts: audit::artifacts::ArtifactsManifest::default(),
            top_failures: if let Some(hint) = &escalate_hint {
                vec![audit::report::FailureSummary {
                    signature: "ESCALATION_REQUIRED".into(),
                    where_: "Executor:Finalize".into(),
                    key_lines: vec![],
                    what_tried: "Sequential execution of plan steps".into(),
                    suggested_fix: hint.clone(),
                }]
            } else { vec![] },
            diff_summary: vec![],
        };
        
        let report_md = report_obj.render();
        let _ = std::fs::write("runs/run_report.md", report_md);
        info!("Generated advanced run_report.md");

        // Flush Metrics
        let _ = audit::flush_run_metrics("runs/metrics_summary.json");
        info!("Generated metrics_summary.json");

        Ok(RunSummary {
            run_id,
            title: plan.meta.title.clone(),
            steps_ok,
            steps_fail,
            elapsed_s: budget.elapsed_s(),
            escalate_hint,
        })
    }

    async fn run_step_with_retry(
        &self,
        step: &PlanStep,
        _plan: &Plan,
        run_id: &str,
        audit: &AuditWriter,
        budget: &mut BudgetTracker,
    ) -> Result<(), StepOutcome> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.run_step_once(step, run_id, audit, attempt > 0).await;
            match outcome {
                Ok(_) => return Ok(()),
                Err(msg) => {
                    attempt += 1;
                    let on_fail = step.on_fail.as_ref()
                        .ok_or_else(|| StepOutcome::Abort(msg.clone()))?;

                    if budget.record_retry().is_err() {
                        return Err(StepOutcome::Abort("Retry budget exhausted".to_string()));
                    }

                    // Backoff
                    if step.retry_backoff_ms > 0 {
                        tokio::time::sleep(
                            tokio::time::Duration::from_millis(step.retry_backoff_ms)
                        ).await;
                    }

                    if let OnFailDecision::Retry { .. } = decide(on_fail, attempt) {
                         // Phase 7: Deep Self-Healing Integration
                         if let Some(_cache) = &self.cache {
                             let sig = CacheManager::normalize_error(&msg);
                             if let Some(pb) = self.playbooks.find_match(&sig) {
                                 warn!("Critical failure detected ({}). Applying Fix-it Playbook: {}.", sig, pb.id);
                                 if self.apply_playbook(pb, run_id, audit).await.is_ok() {
                                     info!("Self-healing successful. Retrying original step with corrected environment.");
                                     continue;
                                 } else {
                                     warn!("Self-healing playbook failed. Falling back to standard retry logic.");
                                 }
                             }
                         }

                         warn!("Retrying step {} (attempt {})", step.id, attempt + 1);
                         continue;
                    }
                    
                    match decide(on_fail, attempt) {
                        OnFailDecision::Retry { .. } => unreachable!(),
                        OnFailDecision::Skip => return Err(StepOutcome::Skipped),
                        OnFailDecision::Rollback { to } => return Err(StepOutcome::Rollback(to)),
                        OnFailDecision::Abort { .. } => return Err(StepOutcome::Abort(msg)),
                        OnFailDecision::Escalate { hint } => return Err(StepOutcome::Escalate(hint)),
                    }
                }
            }
        }
    }

    async fn run_step_once(
        &self,
        step: &PlanStep,
        run_id: &str,
        audit: &AuditWriter,
        is_self_heal: bool,
    ) -> Result<(), String> {
        let args_json = step.args.to_string();
        let args_hash = format!("{:x}", Sha256::digest(args_json.as_bytes()));
        let t = Instant::now();

        info!("[{}] Running step type={}", step.id, step.step_type);
        
        // Secure execution concurrency bounds to prevent CPU/IO host thrashing 
        let _permit = self.gate.acquire(&step.step_type).await;

        // RepoCache hit check
        if let Some(cache) = &self.cache {
            if let Some(fingerprint_val) = step.args.get("repo_fingerprint") {
                if let Ok(fp) = serde_json::from_value::<RepoFingerprint>(fingerprint_val.clone()) {
                    let hash = fp.compute_hash();
                    if let Ok(Some(state)) = cache.get_repo_state(&step.id, &hash) {
                        if state == "success" {
                            info!("[{}] Cache hit (RepoCache)! Skipping step.", step.id);
                            audit::record_cache_hit();
                            
                            // Emit audit event for cache hit
                            let event = AuditEvent {
                                ts: Utc::now(),
                                run_id: run_id.to_string(),
                                step_id: step.id.clone(),
                                actor: AuditActor::Executor,
                                tool: step.step_type.clone(),
                                action: "execute".to_string(),
                                args_hash: format!("{:x}", Sha256::digest(step.args.to_string().as_bytes())),
                                result: AuditResult::Success,
                                duration_ms: 0,
                                risk_tier: step.risk_tier.clone().unwrap_or_else(|| "t0".to_string()),
                                is_cached: true,
                                is_self_heal: false,
                                redactions: vec![],
                                artifacts: vec![],
                            };
                            let _ = audit.write(&event).await;
                            
                            return Ok(());
                        }
                    }
                }
            }
        }

        let (success, reason): (bool, Option<String>) = if self.config.dry_run {
            info!("[{}] DRY RUN - skipping actual execution", step.id);
            (true, None)
        } else {
            let req = ToolRequest {
                id: Uuid::new_v4().to_string(),
                run_id: run_id.to_string(),
                step_id: step.id.clone(),
                tool: step.step_type.clone(),
                action: "execute".to_string(),
                args: step.args.clone(),
                risk_tier: ada_tool_api::RiskTier::default(),
                timeout_ms: step.timeout_s * 1000,
                cwd: step.cwd.clone(),
                env: step.env.clone(),
            };
            
            // Special handling for snapshot steps
            if step.step_type == "snapshot_create" {
                if let Some(name) = step.args["name"].as_str() {
                    match self.sandbox.snapshot_create(name).await {
                        Ok(_) => (true, None),
                        Err(e) => (false, Some(e.to_string())),
                    }
                } else {
                    (false, Some("Missing snapshot 'name' arg".to_string()))
                }
            } else if step.step_type == "snapshot_restore" {
                if let Some(name) = step.args["name"].as_str() {
                    match self.sandbox.snapshot_restore(name).await {
                        Ok(_) => (true, None),
                        Err(e) => (false, Some(e.to_string())),
                    }
                } else {
                    (false, Some("Missing snapshot 'name' arg".to_string()))
                }
            } else {
                let resp = self.sandbox.dispatch(&req, &self.router).await;
                if resp.success {
                    (true, None)
                } else {
                    let msg = resp.error.map(|e| e.message).unwrap_or_default();
                    (false, Some(msg))
                }
            }
        };

        let duration_ms = t.elapsed().as_millis() as u64;
        audit::record_step_duration(step.id.clone(), duration_ms, step.step_type.clone());

        let event = AuditEvent {
            ts: Utc::now(),
            run_id: run_id.to_string(),
            step_id: step.id.clone(),
            actor: AuditActor::Executor,
            tool: step.step_type.clone(),
            action: "execute".to_string(),
            args_hash,
            result: if success {
                AuditResult::Success
            } else {
                AuditResult::Failure { reason: reason.clone().unwrap_or_else(|| "unknown".to_string()) }
            },
            duration_ms,
            risk_tier: step.risk_tier.clone().unwrap_or_else(|| "t0".to_string()),
            is_cached: false, // Inverted logic: if we reached here, it wasn't cached
            is_self_heal,
            redactions: vec![],
            artifacts: vec![],
        };
        let _ = audit.write(&event).await;

        if success {
            // Update RepoCache on success if fingerprint provided
            if let Some(cache) = &self.cache {
                if let Some(fingerprint_val) = step.args.get("repo_fingerprint") {
                    if let Ok(fp) = serde_json::from_value::<RepoFingerprint>(fingerprint_val.clone()) {
                        let _ = cache.set_repo_state(&step.id, &fp.compute_hash(), "success");
                    }
                }
            }
            Ok(())
        } else {
            Err(reason.unwrap_or_else(|| "step failed".to_string()))
        }
    }

    async fn apply_playbook(&self, pb: &Playbook, run_id: &str, audit: &AuditWriter) -> Result<()> {
        for (i, step_val) in pb.fix_steps.iter().enumerate() {
            let step: PlanStep = serde_json::from_value::<PlanStep>(step_val.clone())?;
            info!("  -> [Playbook {}] Running fix step {}/{}", pb.id, i+1, pb.fix_steps.len());
            audit::record_self_heal();
            self.run_step_once(&step, run_id, audit, true).await.map_err(|e| anyhow::anyhow!(e))?;
        }
        
        for (i, step_val) in pb.check_steps.iter().enumerate() {
            let step: PlanStep = serde_json::from_value::<PlanStep>(step_val.clone())?;
            info!("  -> [Playbook {}] Running check step {}/{}", pb.id, i+1, pb.check_steps.len());
            self.run_step_once(&step, run_id, audit, true).await.map_err(|e| anyhow::anyhow!(e))?;
        }
        
        Ok(())
    }
}

#[derive(Debug)]
enum StepOutcome {
    Skipped,
    Abort(String),
    Rollback(Option<String>),
    Escalate(Option<String>),
}

#[derive(Debug)]
pub struct RunSummary {
    pub run_id: String,
    pub title: String,
    pub steps_ok: usize,
    pub steps_fail: usize,
    pub elapsed_s: u64,
    pub escalate_hint: Option<String>,
}
