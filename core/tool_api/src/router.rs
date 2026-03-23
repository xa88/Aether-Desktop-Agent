//! Tool Router: registers named handlers and dispatches ToolRequests.

use crate::{ToolError, ToolErrorCode, ToolRequest, ToolResponse};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Any async function that handles a tool call.
#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError>;
}

/// Central dispatcher: maps "tool/action" -> handler.
pub struct ToolRouter {
    handlers: HashMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRouter {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    pub fn register(&mut self, tool: &str, action: &str, handler: Arc<dyn ToolHandler>) {
        self.handlers.insert(format!("{}/{}", tool, action), handler);
    }

    pub async fn dispatch(&self, req: &ToolRequest) -> ToolResponse {
        let key = format!("{}/{}", req.tool, req.action);
        let start = Instant::now();
        match self.handlers.get(&key) {
            Some(h) => match h.handle(req).await {
                Ok(v) => ToolResponse::ok(&req.id, v, start.elapsed().as_millis() as u64),
                Err(e) => ToolResponse::err(&req.id, e, start.elapsed().as_millis() as u64),
            },
            None => ToolResponse::err(
                &req.id,
                ToolError {
                    code: ToolErrorCode::NotFound,
                    message: format!("No handler for '{}'", key),
                    detail: None,
                },
                start.elapsed().as_millis() as u64,
            ),
        }
    }
}

impl Default for ToolRouter {
    fn default() -> Self {
        Self::new()
    }
}
