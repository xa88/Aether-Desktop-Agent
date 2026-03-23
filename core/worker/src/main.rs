use axum::{
    routing::{post, get},
    Router, Json, extract::Path,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, Level};
use tracing_subscriber;
use ada_tool_api::{ToolRequest, ToolResponse, ToolError};
use ada_tool_api::router::{ToolRouter, ToolHandler};
use ada_adapter_shell::handler::ShellHandler;
use ada_adapter_fs::handler::FsHandler;
use std::sync::Arc;
use async_trait::async_trait;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/api/v1/tools/execute", post(handle_tool_execute))
        .route("/api/v1/jobs", post(handle_job_submit))
        .route("/api/v1/jobs/:id/bundle", get(handle_job_bundle));

    let addr = "0.0.0.0:3000";
    info!("ADA Remote Worker listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ----------------------------------------------------------------------
// 1. Tool Call Granular Endpoints (Remote Sandbox Target)
// ----------------------------------------------------------------------

struct Shim {
    inner: Arc<dyn ToolHandler>,
    action: String,
}

#[async_trait]
impl ToolHandler for Shim {
    async fn handle(&self, req: &ToolRequest) -> Result<Value, ToolError> {
        let mut cloned = req.clone();
        cloned.action = self.action.clone();
        self.inner.handle(&cloned).await
    }
}

async fn handle_tool_execute(Json(req): Json<ToolRequest>) -> Json<ToolResponse> {
    info!("Offloaded execution received: {} -> {}", req.tool, req.action);
    
    let mut router = ToolRouter::new();
    let fs_adapter = Arc::new(FsHandler::new(vec![".".into()], vec![]));
    let shell_adapter = Arc::new(ShellHandler::new(1024));
    
    router.register("fs_mkdir", "execute", Arc::new(Shim { inner: fs_adapter.clone(), action: "mkdir".into() }));
    router.register("shell_run", "execute", Arc::new(Shim { inner: shell_adapter.clone(), action: "run".into() }));
    
    let response = router.dispatch(&req).await;
    Json(response)
}

// ----------------------------------------------------------------------
// 2. Offloaded Complete Plan Execution Endpoints 
// ----------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JobSubmitRequest {
    plan_yaml: String,
}

#[derive(Serialize)]
struct JobSubmitResponse {
    run_id: String,
    status: String,
}

async fn handle_job_submit(Json(_req): Json<JobSubmitRequest>) -> Json<JobSubmitResponse> {
    let run_id = uuid::Uuid::new_v4().to_string();
    info!("Batch Plan job received. Run: {}", run_id);
    
    // In actual implementation, we'd spawn a tokio thread pulling up `ada_executor::Executor`, 
    // running the yaml, mapping it to MockSandbox, and triggering `BundleArchiver`.
    // Returning dummy accepted for now.
    
    Json(JobSubmitResponse {
        run_id,
        status: "Accepted".into(),
    })
}

async fn handle_job_bundle(Path(id): Path<String>) -> (axum::http::StatusCode, Vec<u8>) {
    info!("Requested Bundle for Job: {}", id);
    // Dummy zip representation
    (axum::http::StatusCode::OK, vec![0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
}
