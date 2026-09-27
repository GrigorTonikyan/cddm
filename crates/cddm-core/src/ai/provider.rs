#![forbid(unsafe_code)]

use super::constants::*;
use super::http::*;
use super::types::{AiProviderConfig, AiProviderKind};
use async_trait::async_trait;
use serde_json::json;
use std::fmt::Debug;

/// Universal asynchronous interface for AI LLM providers.
#[async_trait]
pub trait AiProvider: Debug + Send + Sync {
    /// Dispatches a prompt to the AI model and retrieves the completed response string.
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String>;
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
}

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
        Self {
            model: model.unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_string()),
            endpoint: endpoint.unwrap_or_else(|| DEFAULT_OLLAMA_ENDPOINT.to_string()),
            temperature: temperature.unwrap_or(DEFAULT_TEMPERATURE),
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
        let resp_str = execute_http_post(&self.client, &url, &[], &payload).await?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(resp) = val.get("response").and_then(|r| r.as_str())
        {
            return Ok(resp.to_string());
        }
        Ok(resp_str)
    }
}

fn init_provider_config(
    model: Option<String>,
    default_model: &str,
    api_key: Option<String>,
    env_var: &str,
    temperature: Option<f64>,
) -> (String, String, f64) {
    (
        model.unwrap_or_else(|| default_model.to_string()),
        api_key
            .or_else(|| std::env::var(env_var).ok())
            .unwrap_or_default(),
        temperature.unwrap_or(DEFAULT_TEMPERATURE),
    )
}

/// Cloud AI provider supporting Gemini, Claude, and OpenAI backends.
#[derive(Debug, Clone)]
pub struct CloudAiProvider {
    pub model: String,
    pub api_key: String,
    pub temperature: f64,
    pub kind: AiProviderKind,
    pub client: reqwest::Client,
}

pub type GeminiProvider = CloudAiProvider;
pub type ClaudeProvider = CloudAiProvider;
pub type OpenAiProvider = CloudAiProvider;

impl CloudAiProvider {
    fn create_cloud(
        model: Option<String>,
        default_model: &str,
        api_key: Option<String>,
        env_key: &str,
        temperature: Option<f64>,
        kind: AiProviderKind,
        client: Option<reqwest::Client>,
    ) -> Self {
        let (model, api_key, temperature) =
            init_provider_config(model, default_model, api_key, env_key, temperature);
        Self {
            model,
            api_key,
            temperature,
            kind,
            client: client.unwrap_or_else(|| create_http_client(None)),
        }
    }

    pub fn new_gemini(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::new_gemini_with_client(model, api_key, temperature, None)
    }

    pub fn new_gemini_with_client(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_GEMINI_MODEL,
            api_key,
            ENV_GEMINI_API_KEY,
            temperature,
            AiProviderKind::Gemini,
            client,
        )
    }

    pub fn new_claude(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::new_claude_with_client(model, api_key, temperature, None)
    }

    pub fn new_claude_with_client(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_CLAUDE_MODEL,
            api_key,
            ENV_ANTHROPIC_API_KEY,
            temperature,
            AiProviderKind::Claude,
            client,
        )
    }

    pub fn new_openai(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::new_openai_with_client(model, api_key, temperature, None)
    }

    pub fn new_openai_with_client(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_OPENAI_MODEL,
            api_key,
            ENV_OPENAI_API_KEY,
            temperature,
            AiProviderKind::OpenAi,
            client,
        )
    }
}

#[async_trait]
impl AiProvider for CloudAiProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        match self.kind {
            AiProviderKind::Gemini => {
                if self.api_key.is_empty() {
                    return Err(format!(
                        "Gemini API key not provided or set in {ENV_GEMINI_API_KEY}"
                    ));
                }
                let payload = json!({
                    "contents": [{"parts": [{"text": prompt}]}],
                    "generationConfig": {"temperature": self.temperature}
                });
                let url = GEMINI_API_ENDPOINT_TEMPLATE
                    .replacen("{}", &self.model, 1)
                    .replacen("{}", &self.api_key, 1);
                post_and_extract(&self.client, &url, &[], &payload, |val| {
                    val.get("candidates")?
                        .get(0)?
                        .get("content")?
                        .get("parts")?
                        .get(0)?
                        .get("text")?
                        .as_str()
                })
                .await
            }
            AiProviderKind::Claude => {
                if self.api_key.is_empty() {
                    return Err(format!(
                        "Anthropic API key not provided or set in {ENV_ANTHROPIC_API_KEY}"
                    ));
                }
                let payload = chat_message_payload(
                    &self.model,
                    self.temperature,
                    prompt,
                    Some(DEFAULT_CLAUDE_MAX_TOKENS),
                );
                let headers = [
                    (HEADER_ANTHROPIC_API_KEY, self.api_key.as_str()),
                    (HEADER_ANTHROPIC_VERSION, ANTHROPIC_API_VERSION),
                ];
                execute_http_chat(
                    &self.client,
                    DEFAULT_CLAUDE_ENDPOINT,
                    &headers,
                    &payload,
                    |val| val.get("content")?.get(0)?.get("text")?.as_str(),
                )
                .await
            }
            AiProviderKind::OpenAi => {
                if self.api_key.is_empty() {
                    return Err(format!(
                        "OpenAI API key not provided or set in {ENV_OPENAI_API_KEY}"
                    ));
                }
                let payload = chat_message_payload(&self.model, self.temperature, prompt, None);
                let auth_hdr = format!("{BEARER_PREFIX}{}", self.api_key);
                let headers = [(HEADER_AUTHORIZATION, auth_hdr.as_str())];
                execute_http_chat(
                    &self.client,
                    DEFAULT_OPENAI_ENDPOINT,
                    &headers,
                    &payload,
                    |val| {
                        val.get("choices")?
                            .get(0)?
                            .get("message")?
                            .get("content")?
                            .as_str()
                    },
                )
                .await
            }
            _ => Err("Unsupported cloud provider backend".to_string()),
        }
    }
}

fn chat_message_payload(
    model: &str,
    temperature: f64,
    prompt: &str,
    max_tokens: Option<usize>,
) -> serde_json::Value {
    let mut payload = json!({
        "model": model,
        "temperature": temperature,
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });
    if let Some(mt) = max_tokens
        && let Some(obj) = payload.as_object_mut()
    {
        obj.insert("max_tokens".to_string(), json!(mt));
    }
    payload
}

/// Constructs an AI provider instance from a configuration object.
pub fn create_ai_provider(config: &AiProviderConfig) -> Box<dyn AiProvider> {
    let client = create_http_client(config.timeout_secs);
    match config.provider {
        AiProviderKind::Mock => Box::new(MockAiProvider::new(config.model.clone())),
        AiProviderKind::Ollama => Box::new(OllamaProvider::with_client(
            config.model.clone(),
            config.endpoint.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Gemini => Box::new(CloudAiProvider::new_gemini_with_client(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Claude => Box::new(CloudAiProvider::new_claude_with_client(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::OpenAi => Box::new(CloudAiProvider::new_openai_with_client(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
            Some(client),
        )),
        AiProviderKind::Custom => Box::new(MockAiProvider::new(None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_hermetic() {
        let provider = MockAiProvider::new(None);
        let resp = provider.complete_prompt("test").await.unwrap();
        assert_eq!(resp, DEFAULT_MOCK_DIFF_RESPONSE);

        let custom = MockAiProvider::new(Some("custom patch".to_string()));
        let resp2 = custom.complete_prompt("test").await.unwrap();
        assert_eq!(resp2, "custom patch");
    }

    #[tokio::test]
    async fn test_provider_constructors_with_client() {
        let client = create_http_client(Some(10));
        let ollama = OllamaProvider::with_client(None, None, None, Some(client.clone()));
        assert_eq!(ollama.model, DEFAULT_OLLAMA_MODEL);
        assert_eq!(ollama.endpoint, DEFAULT_OLLAMA_ENDPOINT);

        let gemini = CloudAiProvider::new_gemini_with_client(
            Some("gemini-test".into()),
            Some("key".into()),
            Some(0.5),
            Some(client.clone()),
        );
        assert_eq!(gemini.model, "gemini-test");
        assert_eq!(gemini.kind, AiProviderKind::Gemini);

        let claude =
            CloudAiProvider::new_claude_with_client(None, None, None, Some(client.clone()));
        assert_eq!(claude.model, DEFAULT_CLAUDE_MODEL);
        assert_eq!(claude.kind, AiProviderKind::Claude);

        let openai = CloudAiProvider::new_openai_with_client(None, None, None, Some(client));
        assert_eq!(openai.model, DEFAULT_OPENAI_MODEL);
        assert_eq!(openai.kind, AiProviderKind::OpenAi);
    }

    #[tokio::test]
    async fn test_cloud_provider_missing_key_errors() {
        let gemini = CloudAiProvider::new_gemini(None, Some("".into()), None);
        let err = gemini.complete_prompt("hello").await.unwrap_err();
        assert!(err.contains("Gemini API key not provided"));

        let claude = CloudAiProvider::new_claude(None, Some("".into()), None);
        let err = claude.complete_prompt("hello").await.unwrap_err();
        assert!(err.contains("Anthropic API key not provided"));

        let openai = CloudAiProvider::new_openai(None, Some("".into()), None);
        let err = openai.complete_prompt("hello").await.unwrap_err();
        assert!(err.contains("OpenAI API key not provided"));
    }

    #[tokio::test]
    async fn test_ollama_unreachable_endpoint_graceful_error() {
        let ollama = OllamaProvider::new(
            Some("test-model".into()),
            Some("http://127.0.0.1:59999".into()),
            None,
        );
        let err = ollama.complete_prompt("hello").await.unwrap_err();
        assert!(err.contains("HTTP request error") || err.contains("connection"));
    }
}
