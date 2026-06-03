use super::{Provider, ProviderConfig, ProviderRequest, ProviderResponse, TokenUsage};
use crate::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct OpenAIProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse> {
        let api_key = self.config.api_key.as_deref().unwrap_or_default();

        let messages = build_messages(&request);

        let tools = request.tools.iter().map(|t| {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters
                }
            })
        }).collect::<Vec<_>>();

        let body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "tools": if tools.is_empty() { serde_json::Value::Null } else { serde_json::Value::Array(tools) },
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature,
        });

        let response = self
            .client
            .post(format!(
                "{}/chat/completions",
                self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1")
            ))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let content = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = data["usage"].as_object().map(|u| TokenUsage {
            input_tokens: u["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: u["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: u["total_tokens"].as_u64().unwrap_or(0) as u32,
        }).unwrap_or(TokenUsage {
            input_tokens: 0,
            output_tokens: 0,
            total_tokens: 0,
        });

        Ok(ProviderResponse {
            content,
            tool_calls: Vec::new(),
            usage,
        })
    }

    async fn stream_complete(
        &self,
        _request: ProviderRequest,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>> {
        Err(crate::NexusError::Provider("Streaming not yet implemented".to_string()))
    }
}

fn build_messages(request: &ProviderRequest) -> Vec<serde_json::Value> {
    let mut messages = vec![
        serde_json::json!({
            "role": "system",
            "content": request.system_prompt
        })
    ];

    for turn in &request.messages {
        let role = match turn.role {
            crate::agent::session::TurnRole::User => "user",
            crate::agent::session::TurnRole::Assistant => "assistant",
            crate::agent::session::TurnRole::System => "system",
            crate::agent::session::TurnRole::Tool => "tool",
        };
        messages.push(serde_json::json!({
            "role": role,
            "content": turn.content
        }));
    }

    messages.push(serde_json::json!({
        "role": "user",
        "content": request.user_message
    }));

    messages
}
