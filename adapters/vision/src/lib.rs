//! ada-adapter-vision: Vision fallback for UI elements (OCR/Screenshot matching).

use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest};
use ada_tool_api::router::{ToolHandler, ToolRouter};
use ada_core_ui::Rect;
use async_trait::async_trait;
pub mod sensing;
pub mod interaction; // Added this based on OSInteractionHandler
use crate::sensing::ScreenCaptureHandler;
use crate::interaction::OSInteractionHandler;
use std::sync::Arc;
use tracing::{info, warn};

pub struct VisionHandler {
    pub router: Arc<ToolRouter>,
}

impl VisionHandler {
    pub fn new() -> Self {
        let mut router = ToolRouter::new();
        
        router.register("vision", "capture_screen", Arc::new(ScreenCaptureHandler));
        router.register("vision", "click", Arc::new(OSInteractionHandler));
        router.register("vision", "type", Arc::new(OSInteractionHandler));

        Self { router: Arc::new(router) }
    }

    /// MVP Mock: Simulate OCR scanning the screen returning a center coordinate finding
    async fn mock_ocr_locate(&self, expected_text: &str) -> anyhow::Result<Rect> {
        info!("Vision: Scanning screen for '{}'...", expected_text);
        
        // Simulating 500ms delay for OCR processing
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        if expected_text.contains("fail") {
            anyhow::bail!("OCR Failed to locate text on screen");
        }

        // Return a mock bounding box
        Ok(Rect {
            x: 150.0,
            y: 200.0,
            width: 100.0,
            height: 35.0,
        })
    }
}

#[async_trait]
impl ToolHandler for VisionHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        info!("vision/{}: Request received via Vision fallback", req.action);

        if req.action == "locate_text" {
            if let Some(text) = req.args["text"].as_str() {
                match self.mock_ocr_locate(text).await {
                    Ok(bounds) => {
                        return Ok(serde_json::json!({
                            "found": true,
                            "bounds": bounds,
                            "confidence": 0.85,
                        }));
                    }
                    Err(e) => {
                        warn!("Vision locating failed: {}", e);
                        return Err(ToolError {
                            code: ToolErrorCode::NotFound,
                            message: format!("Vision text locate failed: {}", e),
                            detail: None,
                        });
                    }
                }
            } else {
                return Err(ToolError {
                    code: ToolErrorCode::InvalidArgs,
                    message: "Missing 'text' parameter for OCR locate".into(),
                    detail: None,
                });
            }
        }

        if req.action == "screenshot" {
            info!("Vision: Taking full screen snapshot.");
            // We would write image logic here
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            return Ok(serde_json::json!({
                "path": "runs/latest_screenshot.png",
                "width": 1920,
                "height": 1080
            }));
        }

        Err(ToolError {
            code: ToolErrorCode::InvalidArgs,
            message: format!("Unsupported vision action: {}", req.action),
            detail: None,
        })
    }
}
