//! Shared UI Intermediate Representation (UIIR) across Web and Desktop adapters.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiNode {
    pub id: String,
    pub role: String,
    pub name: String,
    pub value: Option<String>,
    pub state: Vec<String>,
    pub bounds: Option<Rect>,
    pub children: Vec<UiNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LocatorStrategy {
    AutomationId { id: String },
    RoleName { role: String, name: String },
    XPath { path: String },
    Coordinates { x: f64, y: f64 },
    OcrText { text: String },
}
