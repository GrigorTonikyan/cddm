#![forbid(unsafe_code)]

pub mod cloud;
pub mod ollama;
pub mod stream;

pub use cloud::*;
pub use ollama::*;
pub use stream::*;

use super::constants::*;
use super::http::*;
use super::types::{AiProviderConfig, AiProviderKind};
use async_trait::async_trait;
use std::fmt::Debug;

/// Universal asynchronous interface for AI LLM providers.
#[async_trait]
pub trait AiProvider: Debug + Send + Sync {
    /// Dispatches a prompt to the AI model and retrieves the completed response string.
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String>;

    /// Dispatches a prompt to the AI model and retrieves an asynchronous stream of tokens.
    async fn stream_prompt(&self, prompt: &str) -> Result<BoxAiStream, String>;
}

/// Mock AI provider for hermetic testing and offline validation.
#[derive(Debug, Clone)]
pub struct MockAiProvider {
    /// Canned response to return if configured
    pub canned_response: Option<String>,
}

impl MockAiProvider {
    pub fn new(canned_response: Option<String>) -> Self {
        Self { canned_response }
    }
}

#[async_trait]
impl AiProvider for MockAiProvider {
    async fn complete_prompt(&self, _prompt: &str) -> Result<String, String> {
        if let Some(resp) = &self.canned_response {
            Ok(resp.clone())
        } else {
            Ok(DEFAULT_MOCK_DIFF_RESPONSE.to_string())
        }
    }

    async fn stream_prompt(&self, _prompt: &str) -> Result<BoxAiStream, String> {
        let text = self
            .canned_response
            .clone()
            .unwrap_or_else(|| DEFAULT_MOCK_DIFF_RESPONSE.to_string());
        let chunks: Vec<Result<String, String>> = text
            .split_inclusive('\n')
            .map(|s| Ok(s.to_string()))
            .collect();
        Ok(Box::pin(tokio_stream::iter(chunks)))
    }
}

pub fn resolve_provider_kind(configured: AiProviderKind) -> AiProviderKind {
    if let Ok(env_provider) = std::env::var(ENV_CDDM_AI_PROVIDER) {
        match env_provider.to_lowercase().trim() {
            "gemini" => return AiProviderKind::Gemini,
            "claude" => return AiProviderKind::Claude,
            "openai" => return AiProviderKind::OpenAi,
            "ollama" => return AiProviderKind::Ollama,
            "custom" => return AiProviderKind::Custom,
            "mock" => return AiProviderKind::Mock,
            _ => {}
        }
    }
    configured
}

/// Constructs an AI provider instance from a configuration object.
pub fn create_ai_provider(config: &AiProviderConfig) -> Box<dyn AiProvider> {
    let client = create_http_client(config.timeout_secs);
    let provider = resolve_provider_kind(config.provider);
    let model = config
        .model
        .clone()
        .or_else(|| std::env::var(ENV_CDDM_AI_MODEL).ok());

    match provider {
        AiProviderKind::Mock => Box::new(MockAiProvider::new(model)),
        AiProviderKind::Ollama => Box::new(OllamaProvider::with_client(
            model,
            config.endpoint.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Gemini => Box::new(CloudAiProvider::new_gemini_with_client(
            model,
            config.api_key.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Claude => Box::new(CloudAiProvider::new_claude_with_client(
            model,
            config.api_key.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::OpenAi => Box::new(CloudAiProvider::new_openai_with_endpoint_and_client(
            model,
            config.api_key.clone(),
            config.endpoint.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Custom => Box::new(CloudAiProvider::new_custom_with_client(
            model,
            config.api_key.clone(),
            config.endpoint.clone(),
            config.temperature,
            Some(client),
        )),
    }
}

#[cfg(test)]
mod tests;
