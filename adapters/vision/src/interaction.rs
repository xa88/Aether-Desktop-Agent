use ada_tool_api::{router::ToolHandler, ToolRequest, ToolError, ToolErrorCode};
use async_trait::async_trait;
use enigo::{Enigo, MouseControllable, KeyboardControllable, MouseButton};

pub struct OSInteractionHandler;

#[async_trait]
impl ToolHandler for OSInteractionHandler {
    async fn handle(&self, req: &ToolRequest) -> Result<serde_json::Value, ToolError> {
        match req.action.as_str() {
            "click" => {
                let x = req.args["x"].as_i64().ok_or_else(|| ToolError {
                    code: ToolErrorCode::InvalidArgs,
                    message: "Missing x".into(),
                    detail: None,
                })? as i32;
                let y = req.args["y"].as_i64().ok_or_else(|| ToolError {
                    code: ToolErrorCode::InvalidArgs,
                    message: "Missing y".into(),
                    detail: None,
                })? as i32;
                
                let mut enigo = Enigo::new();
                enigo.mouse_move_to(x, y);
                enigo.mouse_click(MouseButton::Left);
                Ok(serde_json::json!({ "clicked": { "x": x, "y": y } }))
            }
            "type" => {
                let text = req.args["text"].as_str().ok_or_else(|| ToolError {
                    code: ToolErrorCode::InvalidArgs,
                    message: "Missing text".into(),
                    detail: None,
                })?;
                
                let mut enigo = Enigo::new();
                enigo.key_sequence(text);
                Ok(serde_json::json!({ "typed": text }))
            }
            _ => Err(ToolError {
                code: ToolErrorCode::NotFound,
                message: format!("Command '{}' not found in OS interaction", req.action),
                detail: None,
            })
        }
    }
}
