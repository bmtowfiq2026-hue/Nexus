use crate::Result;
use crate::providers::{CalledTool, Provider, ProviderConfig, ProviderRequest, ProviderResponse, TokenUsage};
use async_trait::async_trait;
use futures::stream::BoxStream;
use futures::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OpenAICompatProvider {
    name: String,
    display_name: String,
    config: ProviderConfig,
    client: Client,
}

impl OpenAICompatProvider {
    pub fn new(name: &str, display_name: &str, base_url: &str, model: &str, api_key: Option<String>) -> Self {
        let mut extra = HashMap::new();
        extra.insert("base_url".into(), Value::String(base_url.into()));
        Self {
            name: name.to_string(),
            display_name: display_name.to_string(),
            config: ProviderConfig {
                api_key,
                base_url: Some(base_url.into()),
                model: model.into(),
                max_tokens: 4096,
                temperature: 0.7,
                extra,
            },
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    fn get_api_key(&self) -> String {
        if let Some(ref key) = self.config.api_key {
            if !key.is_empty() {
                return key.clone();
            }
        }
        std::env::var(format!("{}_API_KEY", self.name.to_uppercase()))
            .or_else(|_| std::env::var(format!("{}_API_KEY", self.display_name.to_uppercase().replace(' ', "_"))))
            .unwrap_or_default()
    }
}

#[async_trait]
impl Provider for OpenAICompatProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn complete(&self, request: ProviderRequest) -> Result<ProviderResponse> {
        let api_key = self.get_api_key();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

        let mut messages = Vec::new();
        messages.push(json!({
            "role": "system",
            "content": request.system_prompt
        }));
        for turn in &request.messages {
            messages.push(json!({
                "role": match turn.role { crate::agent::session::TurnRole::Assistant => "assistant", _ => "user" },
                "content": turn.content
            }));
        }
        messages.push(json!({
            "role": "user",
            "content": request.user_message
        }));

        let mut body = json!({
            "model": self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature,
        });

        if !request.tools.is_empty() {
            let tools: Vec<Value> = request.tools.iter().map(|t| json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
            })).collect();
            body["tools"] = json!(tools);
            body["tool_choice"] = json!("auto");
        }

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::NexusError::Provider(format!("Request failed: {}", e)))?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(crate::NexusError::Provider(format!(
                "{} API error ({}): {}", self.display_name, status, text
            )));
        }

        let result: Value = serde_json::from_str(&text)
            .map_err(|e| crate::NexusError::Provider(format!("Parse error: {}", e)))?;

        let choice = result["choices"][0].clone();
        let content = choice["message"]["content"].as_str().unwrap_or("").to_string();
        let message = &choice["message"];

        let mut tool_calls = Vec::new();
        if let Some(calls) = message["tool_calls"].as_array() {
            for call in calls {
                if call["type"] == "function" {
                    tool_calls.push(CalledTool {
                        id: call["id"].as_str().unwrap_or("").to_string(),
                        name: call["function"]["name"].as_str().unwrap_or("").to_string(),
                        arguments: serde_json::from_str(
                            call["function"]["arguments"].as_str().unwrap_or("{}")
                        ).unwrap_or(json!({})),
                    });
                }
            }
        }

        let usage = result["usage"].clone();
        let usage = TokenUsage {
            input_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            output_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as u32,
        };

        Ok(ProviderResponse { content, tool_calls, usage })
    }

    async fn stream_complete(&self, request: ProviderRequest) -> Result<BoxStream<'static, Result<String>>> {
        let api_key = self.get_api_key();
        let base_url = self.config.base_url.as_deref().unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

        let mut messages = vec![
            json!({"role": "system", "content": request.system_prompt})
        ];
        for turn in &request.messages {
            let role = match turn.role { crate::agent::session::TurnRole::Assistant => "assistant", _ => "user" };
            messages.push(json!({"role": role, "content": turn.content}));
        }
        messages.push(json!({"role": "user", "content": request.user_message}));

        let body = json!({
            "model": self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature,
            "stream": true,
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| crate::NexusError::Provider(format!("Stream request failed: {}", e)))?;

        let stream = response.bytes_stream().map(|chunk| {
            chunk
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
                .map_err(|e| crate::NexusError::Provider(format!("Stream error: {}", e)))
        });

        let boxed: BoxStream<'static, Result<String>> = stream.boxed();
        Ok(boxed)
    }
}
