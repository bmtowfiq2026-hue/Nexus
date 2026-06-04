pub mod openai;
pub mod anthropic;
pub mod ollama;
pub mod demo;
pub mod openai_compat;

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

pub const PROVIDERS: &[(&str, &str, &str)] = &[
    ("openai",      "OpenAI",           "https://api.openai.com/v1"),
    ("anthropic",   "Anthropic",        "https://api.anthropic.com/v1"),
    ("google",      "Google Gemini",    "https://generativelanguage.googleapis.com/v1beta"),
    ("deepseek",    "DeepSeek",         "https://api.deepseek.com/v1"),
    ("groq",        "Groq",             "https://api.groq.com/openai/v1"),
    ("together",    "Together AI",      "https://api.together.xyz/v1"),
    ("fireworks",   "Fireworks AI",     "https://api.fireworks.ai/inference/v1"),
    ("openrouter",  "OpenRouter",       "https://openrouter.ai/api/v1"),
    ("perplexity",  "Perplexity",       "https://api.perplexity.ai"),
    ("mistral",     "Mistral AI",       "https://api.mistral.ai/v1"),
    ("cohere",      "Cohere",           "https://api.cohere.ai/v1"),
    ("ai21",        "AI21 Labs",        "https://api.ai21.com/studio/v1"),
    ("replicate",   "Replicate",        "https://api.replicate.com/v1"),
    ("huggingface", "HuggingFace",      "https://api-inference.huggingface.co/v1"),
    ("lmstudio",    "LM Studio",        "http://localhost:1234/v1"),
    ("localai",     "LocalAI",          "http://localhost:8080/v1"),
    ("textgen",     "oobabooga",        "http://localhost:5000/v1"),
    ("ollama",      "Ollama",           "http://localhost:11434"),
    ("together",    "Together AI",      "https://api.together.xyz/v1"),
    ("deepinfra",   "DeepInfra",        "https://api.deepinfra.com/v1/openai"),
];

pub const OPENAI_COMPAT_PROVIDERS: &[(&str, &str, &str, &str)] = &[
    ("google",      "Google Gemini",    "https://generativelanguage.googleapis.com/v1beta/openai",   "gemini-2.0-flash"),
    ("deepseek",    "DeepSeek",         "https://api.deepseek.com/v1",                               "deepseek-chat"),
    ("groq",        "Groq",             "https://api.groq.com/openai/v1",                            "llama3-70b-8192"),
    ("together",    "Together AI",      "https://api.together.xyz/v1",                               "mistralai/Mixtral-8x22B-Instruct-v0.1"),
    ("fireworks",   "Fireworks AI",     "https://api.fireworks.ai/inference/v1",                     "accounts/fireworks/models/llama-v3p1-70b-instruct"),
    ("openrouter",  "OpenRouter",       "https://openrouter.ai/api/v1",                              "openai/gpt-4o"),
    ("perplexity",  "Perplexity",       "https://api.perplexity.ai",                                 "sonar-pro"),
    ("mistral",     "Mistral AI",       "https://api.mistral.ai/v1",                                 "mistral-large-latest"),
    ("cohere",      "Cohere",           "https://api.cohere.ai/v1",                                  "command-r-plus"),
    ("ai21",        "AI21 Labs",        "https://api.ai21.com/studio/v1",                            "jamba-1.5-large"),
    ("replicate",   "Replicate",        "https://api.replicate.com/v1",                              "meta/meta-llama-3.1-405b-instruct"),
    ("huggingface", "HuggingFace",      "https://api-inference.huggingface.co/v1",                   "meta-llama/Meta-Llama-3.1-70B-Instruct"),
    ("cerebras",    "Cerebras",         "https://api.cerebras.ai/v1",                                "cerebras/Llama3.1-70B"),
    ("xai",         "xAI (Grok)",       "https://api.x.ai/v1",                                       "grok-2"),
    ("lmstudio",    "LM Studio",        "http://localhost:1234/v1",                                  "local-model"),
    ("localai",     "LocalAI",          "http://localhost:8080/v1",                                  "gpt-4"),
    ("textgen",     "oobabooga",        "http://localhost:5000/v1",                                  "model"),
    ("deepinfra",   "DeepInfra",        "https://api.deepinfra.com/v1/openai",                       "mistralai/Mixtral-8x22B-Instruct-v0.1"),
    ("sambanova",   "SambaNova",        "https://api.sambanova.ai/v1",                               "Meta-Llama-3.1-70B-Instruct"),
    ("anyscale",    "Anyscale",         "https://api.endpoints.anyscale.com/v1",                      "meta-llama/Llama-2-70b-chat-hf"),
];
