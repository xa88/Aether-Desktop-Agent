//! JSON-RPC over stdin/stdout IPC transport.

use crate::{ToolRequest};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

impl JsonRpcResponse {
    pub fn ok(id: u64, result: Value) -> Self {
        Self { jsonrpc: "2.0".into(), id, result: Some(result), error: None }
    }

    pub fn error(id: u64, code: i64, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError { code, message: message.to_string() }),
        }
    }
}

/// Serialize a ToolRequest as a JSON-RPC request line.
pub fn encode_request(req: &ToolRequest, id: u64) -> Result<String> {
    let rpc = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id,
        method: format!("{}/{}", req.tool, req.action),
        params: serde_json::to_value(req)?,
    };
    Ok(serde_json::to_string(&rpc)?)
}

/// Parse a line as a JSON-RPC response.
pub fn decode_response(line: &str) -> Result<JsonRpcResponse> {
    Ok(serde_json::from_str(line)?)
}
