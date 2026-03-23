//! ada-executor binary entry point.

use ada_executor::{runner::{Executor, ExecutorConfig}};
use ada_tool_api::router::ToolRouter;
use ada_sandbox::{SandboxManager, providers::mock::MockSandboxProvider};

use tracing_subscriber::EnvFilter;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ada-executor", about = "ADA Plan Executor")]
struct Args {
    #[arg(short, long, default_value = "plan.yaml")]
    plan: String,

    #[arg(short, long, default_value = "runs/audit.jsonl")]
    audit: String,

    #[arg(short, long, default_value = "runs/cache.db")]
    cache: String,

    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("ada=info".parse().unwrap()))
        .init();

    let args = Args::parse();

    // Ensure runs/ directory exists
    std::fs::create_dir_all("runs")?;

    let router = ToolRouter::new();
    // TODO: register adapters (fs, git, shell, test, search) here

    let config = ExecutorConfig {
        plan_path: args.plan,
        audit_path: args.audit,
        cache_path: Some(args.cache),
        dry_run: args.dry_run,
    };
    let sandbox_provider = std::sync::Arc::new(MockSandboxProvider);
    let sandbox = SandboxManager::new(sandbox_provider, "host".into());

    let executor = Executor::new(config, router, sandbox);
    let summary = executor.run().await?;

    println!("=== ADA Run Complete ===");
    println!("Run ID:   {}", summary.run_id);
    println!("Title:    {}", summary.title);
    println!("Steps OK: {} | Failed: {}", summary.steps_ok, summary.steps_fail);
    println!("Elapsed:  {}s", summary.elapsed_s);
    if let Some(hint) = summary.escalate_hint {
        println!("Escalate: {hint}");
    }

    // Flush cache metrics
    let _ = ada_cache::metrics::flush_metrics("runs/metrics.json");

    Ok(())
}
