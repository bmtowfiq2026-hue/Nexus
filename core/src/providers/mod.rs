pub mod openai;
pub mod anthropic;
pub mod ollama;

use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type ToolName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequest {
    pub system_prompt: String,
    pub messages: Vec<super::agent::session::Turn>,
    pub user_message: String,
    pub tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub content: String,
    pub tool_calls: Vec<CalledTool>,
    pub usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalledTool {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub extra: HashMap<String, Value>,
}

#[async_trait]
pub trait Provider: std::fmt::Debug {
    fn name(&self) -> &str;
    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse>;
    async fn stream_complete(
        &self,
        request: ProviderRequest,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>>;
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
            model: String::from("gpt-4o"),
            max_tokens: 4096,
            temperature: 0.7,
            extra: HashMap::new(),
        }
    }
}
