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
    let resolved_model = model
        .or_else(|| std::env::var(ENV_CDDM_AI_MODEL).ok())
        .unwrap_or_else(|| default_model.to_string());
    let resolved_key = api_key
        .or_else(|| std::env::var(env_var).ok())
        .unwrap_or_default();
    let resolved_temp = temperature.unwrap_or(DEFAULT_TEMPERATURE);
    (resolved_model, resolved_key, resolved_temp)
}

fn normalize_chat_endpoint(endpoint: &str) -> String {
    let trimmed = endpoint.trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}

fn resolve_openai_endpoint(custom_endpoint: Option<&str>, default_endpoint: &str) -> String {
    if let Some(ep) = custom_endpoint.filter(|s| !s.is_empty()) {
        return normalize_chat_endpoint(ep);
    }
    if let Some(base) = std::env::var(ENV_OPENAI_BASE_URL)
        .or_else(|_| std::env::var(ENV_OPENAI_API_BASE))
        .ok()
        .filter(|b| !b.is_empty())
    {
        return normalize_chat_endpoint(&base);
    }
    default_endpoint.to_string()
}

fn resolve_provider_kind(configured: AiProviderKind) -> AiProviderKind {
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

struct CloudInitArgs<'a> {
    model: Option<String>,
    default_model: &'a str,
    api_key: Option<String>,
    env_key: &'a str,
    temperature: Option<f64>,
    kind: AiProviderKind,
    endpoint: Option<String>,
    client: Option<reqwest::Client>,
}

/// Cloud AI provider supporting Gemini, Claude, OpenAI, and Custom backends.
#[derive(Debug, Clone)]
pub struct CloudAiProvider {
    pub model: String,
    pub api_key: String,
    pub temperature: f64,
    pub kind: AiProviderKind,
    pub endpoint: Option<String>,
    pub client: reqwest::Client,
}

pub type GeminiProvider = CloudAiProvider;
pub type ClaudeProvider = CloudAiProvider;
pub type OpenAiProvider = CloudAiProvider;

impl CloudAiProvider {
    fn create_cloud(args: CloudInitArgs<'_>) -> Self {
        let (model, api_key, temperature) = init_provider_config(
            args.model,
            args.default_model,
            args.api_key,
            args.env_key,
            args.temperature,
        );
        Self {
            model,
            api_key,
            temperature,
            kind: args.kind,
            endpoint: args.endpoint,
            client: args.client.unwrap_or_else(|| create_http_client(None)),
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
        Self::create_cloud(CloudInitArgs {
            model,
            default_model: DEFAULT_GEMINI_MODEL,
            api_key,
            env_key: ENV_GEMINI_API_KEY,
            temperature,
            kind: AiProviderKind::Gemini,
            endpoint: None,
            client,
        })
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
        Self::create_cloud(CloudInitArgs {
            model,
            default_model: DEFAULT_CLAUDE_MODEL,
            api_key,
            env_key: ENV_ANTHROPIC_API_KEY,
            temperature,
            kind: AiProviderKind::Claude,
            endpoint: None,
            client,
        })
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
        Self::new_openai_with_endpoint_and_client(model, api_key, None, temperature, client)
    }

    pub fn new_openai_with_endpoint_and_client(
        model: Option<String>,
        api_key: Option<String>,
        endpoint: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self::create_cloud(CloudInitArgs {
            model,
            default_model: DEFAULT_OPENAI_MODEL,
            api_key,
            env_key: ENV_OPENAI_API_KEY,
            temperature,
            kind: AiProviderKind::OpenAi,
            endpoint,
            client,
        })
    }

    pub fn new_custom_with_client(
        model: Option<String>,
        api_key: Option<String>,
        endpoint: Option<String>,
        temperature: Option<f64>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self::create_cloud(CloudInitArgs {
            model,
            default_model: DEFAULT_OPENAI_MODEL,
            api_key,
            env_key: ENV_OPENAI_API_KEY,
            temperature,
            kind: AiProviderKind::Custom,
            endpoint,
            client,
        })
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
                let url =
                    resolve_openai_endpoint(self.endpoint.as_deref(), DEFAULT_OPENAI_ENDPOINT);
                execute_http_chat(&self.client, &url, &headers, &payload, |val| {
                    val.get("choices")?
                        .get(0)?
                        .get("message")?
                        .get("content")?
                        .as_str()
                })
                .await
            }
            AiProviderKind::Custom => {
                let payload = chat_message_payload(&self.model, self.temperature, prompt, None);
                let url =
                    resolve_openai_endpoint(self.endpoint.as_deref(), DEFAULT_CUSTOM_ENDPOINT);
                let auth_hdr = format!("{BEARER_PREFIX}{}", self.api_key);
                let mut headers = Vec::new();
                if !self.api_key.is_empty() {
                    headers.push((HEADER_AUTHORIZATION, auth_hdr.as_str()));
                }
                execute_http_chat(&self.client, &url, &headers, &payload, |val| {
                    val.get("choices")?
                        .get(0)?
                        .get("message")?
                        .get("content")?
                        .as_str()
                })
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
