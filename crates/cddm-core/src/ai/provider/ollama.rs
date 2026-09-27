#![forbid(unsafe_code)]

use crate::ai::constants::*;
use crate::ai::http::*;
use crate::ai::provider::AiProvider;
use crate::ai::provider::stream::{BoxAiStream, parse_ollama_chunk};
use async_trait::async_trait;
use serde_json::json;

/// Ollama local LLM provider.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    pub model: String,
    pub endpoint: String,
    pub temperature: f64,
    pub client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(model: Option<String>, endpoint: Option<String>, temperature: Option<f64>) -> Self {
        Self::with_client(model, endpoint, temperature, None)
    }

    pub fn with_client(
        model: Option<String>,
        endpoint: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        let resolved_model = model
            .or_else(|| std::env::var(ENV_CDDM_AI_MODEL).ok())
            .unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_string());
        let resolved_endpoint = endpoint.unwrap_or_else(|| DEFAULT_OLLAMA_ENDPOINT.to_string());
        let resolved_temp = temperature.unwrap_or(DEFAULT_TEMPERATURE);
        Self {
            model: resolved_model,
            endpoint: resolved_endpoint,
            temperature: resolved_temp,
            client: client.unwrap_or_else(|| create_http_client(None)),
        }
    }
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": self.temperature,
            }
        });

        let url = format!(
            "{}/{}",
            self.endpoint.trim_end_matches('/'),
            OLLAMA_GENERATE_PATH
        );
        let no_headers: &[(&str, &str)] = &[];
        let resp_str = execute_http_post(&self.client, &url, no_headers, &payload).await?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(resp) = val.get("response").and_then(|r| r.as_str())
        {
            return Ok(resp.to_string());
        }
        Ok(resp_str)
    }

    async fn stream_prompt(&self, prompt: &str) -> Result<BoxAiStream, String> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": true,
            "options": {
                "temperature": self.temperature,
            }
        });

        let url = format!(
            "{}/{}",
            self.endpoint.trim_end_matches('/'),
            OLLAMA_GENERATE_PATH
        );
        let no_headers: &[(&str, &str)] = &[];
        execute_http_chat_stream(&self.client, &url, no_headers, &payload, parse_ollama_chunk).await
    }
}
