mod credentials;

use std::env;
use std::fs;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use async_trait::async_trait;

use ada_tool_api::{ToolError, ToolRequest};
use ada_tool_api::router::{ToolRouter, ToolHandler};
use ada_executor::{Executor, PlanValidator};
use ada_executor::runner::ExecutorConfig;
use ada_sandbox::{SandboxManager, providers::mock::MockSandboxProvider};

// Adapters
use ada_adapter_fs::handler::FsHandler;
use ada_adapter_shell::handler::ShellHandler;
use ada_adapter_git::handler::GitHandler;
use ada_adapter_search::handler::SearchHandler;
use ada_adapter_search::web_handler::WebSearchHandler;

// A small shim to bridge PlanStep `type` (e.g. fs_mkdir) to adapter `action` (e.g. mkdir), 
// because Executor hardcodes action="execute".
struct Shim {
    inner: Arc<dyn ToolHandler>,
    action: String,
}

#[async_trait]
impl ToolHandler for Shim {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        let mut cloned = req.clone();
        cloned.action = self.action.clone();
        self.inner.handle(&cloned).await
    }
}

fn shim(inner: Arc<dyn ToolHandler>, action: &str) -> Arc<dyn ToolHandler> {
    Arc::new(Shim {
        inner,
        action: action.to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting Aether Desktop Agent CLI (ADA)");

    // 2. Parse Args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ada-cli <path-to-plan.yaml> | listen | goal \"<text>\" | tts \"<text>\" | credentials <action> <key>");
        std::process::exit(1);
    }

    if args[1] == "listen" {
        info!("Voice: Starting transcription engine...");
        let asr = ada_adapter_voice::asr::WhisperEngine::new("base.en")?;
        let text = asr.transcribe(&[0.1; 16000]).await?; // Simulated 1s of audio
        println!("TRANSCRIPT: {}", text);
        return Ok(());
    }

    if args[1] == "tts" && args.len() > 2 {
        let text = &args[2];
        let provider = ada_adapter_voice::tts::SystemTtsProvider;
        use ada_adapter_voice::tts::TtsProvider;
        provider.speak(text).await?;
        return Ok(());
    }

    if args[1] == "credentials" && args.len() > 3 {
        // ... (existing credentials logic)
        let manager = credentials::CredentialManager::new();
        let action = &args[2];
        let key = &args[3];
        if action == "set" && args.len() > 4 {
            manager.set_secret(key, &args[4])?;
            println!("Secret set successfully: {}", key);
        } else if action == "get" {
            let secret = manager.get_secret(key)?;
            println!("{}", secret);
        } else if action == "delete" {
            manager.delete_secret(key)?;
            println!("Secret deleted: {}", key);
        }
        return Ok(());
    }

    if args[1] == "goal" && args.len() > 2 {
        let goal = &args[2];
        info!("Autonomous Goal: {}", goal);
        
        let mut router = ToolRouter::new();
        register_all_adapters(&mut router);

        let provider = Arc::new(ada_llm::providers::openai_compat::OpenAiCompatProvider::new(
            config.base_url.clone(),
            config.api_key.clone(),
            config.model.clone(),
        ));

        let orchestrator = ada_orchestrator::Orchestrator::new(
            router,
            provider.clone(), // Director
            provider,         // Worker
            Some("cache.db".to_string()),
            true,
        )?;

        let context = ada_llm::PlanContext {
            workspace_path: ".".into(),
            os: "Windows 11 / Desktop".into(),
            max_steps: 15,
            constraints: vec!["No destructive deletes without backup".into()],
            state: vec![],
            failures: vec![],
            diff_summary: vec![],
            details: vec![],
        };

        orchestrator.process_task(goal, context).await?;
        return Ok(());
    }

    let plan_path = &args[1];
    info!("Loading plan from: {}", plan_path);

    // 3. Load Plan YAML
    let plan_yaml = fs::read_to_string(plan_path)?;
    let validator = PlanValidator::new()?;
    let plan = validator.validate_raw(&plan_yaml)?;
    
    info!("Successfully parsed plan: {}", plan.meta.title);

    // 4. Setup Router & Adapters
    let mut router = ToolRouter::new();
    register_all_adapters(&mut router);
    
    let router = router; // Make immutable

    // 5. Execute
    let config = ExecutorConfig {
        plan_path: plan_path.clone(),
        audit_path: "runs/audit.jsonl".into(),
        cache_path: Some("runs/cache.db".into()),
        dry_run: false,
    };
    
    // Create runs directory if it doesn't exist to avoid audit log failure
    let _ = fs::create_dir_all("runs");
    
    // Setup Sandbox Architecture
    let sandbox_provider = Arc::new(MockSandboxProvider); // Using mock for now until platform-specific dispatch
    let sandbox = SandboxManager::new(sandbox_provider, "host".into());

    info!("Execution pipeline ready. Starting executor...");
    let executor = Executor::new(config, router, sandbox);
    let summary = executor.run().await?;
    
    info!("Plan Execution Finished. Run ID: {}. Steps OK: {}, Failed: {}", summary.run_id, summary.steps_ok, summary.steps_fail);

    Ok(())
}

fn register_all_adapters(router: &mut ToolRouter) {
    // Parse Enterprise Policies
    let profile = std::fs::read_to_string("profile.yaml")
        .map(|s| serde_yaml::from_str::<ada_policy::profile::EnterpriseProfile>(&s).unwrap_or_default())
        .unwrap_or_default();

    let allowed_paths: Vec<String> = if profile.restricted_paths.is_empty() {
        vec![".".into()]
    } else {
        profile.restricted_paths
    };

    let fs_adapter = Arc::new(FsHandler::new(
        allowed_paths,
        ada_policy::guard::default_blocked_patterns()
    ));
    router.register("fs_mkdir",       "execute", shim(Arc::clone(&fs_adapter) as Arc<dyn ToolHandler>, "mkdir"));
    router.register("fs_delete",      "execute", shim(Arc::clone(&fs_adapter) as Arc<dyn ToolHandler>, "delete"));
    router.register("fs_write",       "execute", shim(Arc::clone(&fs_adapter) as Arc<dyn ToolHandler>, "write"));
    router.register("fs_apply_patch", "execute", shim(Arc::clone(&fs_adapter) as Arc<dyn ToolHandler>, "apply_patch"));
    
    let shell_adapter = Arc::new(ShellHandler::new(1024));
    router.register("shell_run",  "execute", shim(Arc::clone(&shell_adapter) as Arc<dyn ToolHandler>, "run"));
    router.register("test_run",   "execute", shim(Arc::clone(&shell_adapter) as Arc<dyn ToolHandler>, "run"));
    
    let git_adapter = Arc::new(GitHandler::new());
    router.register("git_status",   "execute", shim(Arc::clone(&git_adapter) as Arc<dyn ToolHandler>, "status"));
    router.register("git_diff",     "execute", shim(Arc::clone(&git_adapter) as Arc<dyn ToolHandler>, "diff"));
    router.register("git_commit",   "execute", shim(Arc::clone(&git_adapter) as Arc<dyn ToolHandler>, "commit"));
    router.register("git_checkout", "execute", shim(Arc::clone(&git_adapter) as Arc<dyn ToolHandler>, "checkout"));
    router.register("git_stash",    "execute", shim(Arc::clone(&git_adapter) as Arc<dyn ToolHandler>, "stash"));
    
    let local_search = Arc::new(SearchHandler::new());
    router.register("local_search", "execute", shim(Arc::clone(&local_search) as Arc<dyn ToolHandler>, "handle"));

    // Phase 8: Web Research
    let tavily = Arc::new(WebSearchHandler::new("MOCK_KEY".into(), "tavily".into()));
    router.register("web_search_tavily", "execute", shim(Arc::clone(&tavily) as Arc<dyn ToolHandler>, "handle"));
    
    let serper = Arc::new(WebSearchHandler::new("MOCK_KEY".into(), "serper".into()));
    router.register("web_search_serper", "execute", shim(Arc::clone(&serper) as Arc<dyn ToolHandler>, "handle"));
}
