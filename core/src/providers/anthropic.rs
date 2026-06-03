use super::{Provider, ProviderConfig, ProviderRequest, ProviderResponse, TokenUsage};
use crate::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct AnthropicProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse> {
        let api_key = self.config.api_key.as_deref().unwrap_or_default();

        let messages = request.messages.iter().map(|t| {
            let role = match t.role {
                crate::agent::session::TurnRole::User => "user",
                crate::agent::session::TurnRole::Assistant => "assistant",
                _ => "user",
            };
            serde_json::json!({
                "role": role,
                "content": t.content
            })
        }).collect::<Vec<_>>();

        let body = serde_json::json!({
            "model": self.config.model,
            "system": request.system_prompt,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature,
        });

        let response = self
            .client
            .post(format!(
                "{}/messages",
                self.config.base_url.as_deref().unwrap_or("https://api.anthropic.com/v1")
            ))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let content = data["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = data["usage"].as_object().map(|u| TokenUsage {
            input_tokens: u["input_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: u["output_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: (u["input_tokens"].as_u64().unwrap_or(0) + u["output_tokens"].as_u64().unwrap_or(0)) as u32,
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
