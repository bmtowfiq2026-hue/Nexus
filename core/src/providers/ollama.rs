use super::{Provider, ProviderConfig, ProviderRequest, ProviderResponse, TokenUsage};
use crate::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct OllamaProvider {
    config: ProviderConfig,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse> {
        let messages = [serde_json::json!({
            "role": "system",
            "content": request.system_prompt
        })].iter().cloned().chain(request.messages.iter().map(|t| {
            let role = match t.role {
                crate::agent::session::TurnRole::User => "user",
                crate::agent::session::TurnRole::Assistant => "assistant",
                _ => "user",
            };
            serde_json::json!({
                "role": role,
                "content": t.content
            })
        })).collect::<Vec<_>>();

        let body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens
            }
        });

        let base_url = self.config.base_url.as_deref().unwrap_or("http://localhost:11434");
        let response = self
            .client
            .post(format!("{}/api/chat", base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| crate::NexusError::Provider(e.to_string()))?;

        let content = data["message"]["content"].as_str().unwrap_or("").to_string();

        Ok(ProviderResponse {
            content,
            tool_calls: Vec::new(),
            usage: TokenUsage {
                input_tokens: 0,
                output_tokens: 0,
                total_tokens: 0,
            },
        })
    }

    async fn stream_complete(
        &self,
        _request: ProviderRequest,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>> {
        Err(crate::NexusError::Provider("Streaming not yet implemented".to_string()))
    }
}
