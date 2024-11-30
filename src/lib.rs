pub mod mcp_types;
use std::{error::Error, fmt};

pub use mcp_types::prelude::*;
use serde::{Deserialize, Serialize};

pub use schemars::JsonSchema;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Error for McpError {}
impl fmt::Display for McpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MCP error {}: {}", self.code, self.message)
    }
}


pub trait Request {
    fn method(&self) -> &str;
    fn params(&self) -> Option<serde_json::Value>;
}

pub trait Notification {
    fn method(&self) -> &str;
    fn params(&self) -> Option<serde_json::Value>;
}

pub trait Result {
    fn result(&self) -> McpError;
}