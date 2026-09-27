#![forbid(unsafe_code)]

use crate::ai::constants::*;
use crate::ai::http::*;
use crate::ai::provider::AiProvider;
use crate::ai::provider::stream::{
    BoxAiStream, parse_claude_sse_chunk, parse_gemini_sse_chunk, parse_openai_sse_chunk,
};
use crate::ai::types::AiProviderKind;
use async_trait::async_trait;
use serde_json::json;

#[derive(Debug)]
pub struct CloudInitArgs<'a> {
    pub model: Option<String>,
    pub default_model: &'a str,
    pub api_key: Option<String>,
    pub env_key: &'a str,
    pub temperature: Option<f64>,
    pub kind: AiProviderKind,
    pub endpoint: Option<String>,
    pub client: Option<reqwest::Client>,
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
    pub fn create_cloud(args: CloudInitArgs<'_>) -> Self {
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

pub fn init_provider_config(
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

pub fn normalize_chat_endpoint(endpoint: &str) -> String {
    let trimmed = endpoint.trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/chat/completions")
    }
}

pub fn resolve_openai_endpoint(custom_endpoint: Option<&str>, default_endpoint: &str) -> String {
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

pub fn chat_message_payload(
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

#[async_trait]
impl AiProvider for CloudAiProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        let empty_headers: &[(&str, &str)] = &[];
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
                post_and_extract(&self.client, &url, empty_headers, &payload, |val| {
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

    async fn stream_prompt(&self, prompt: &str) -> Result<BoxAiStream, String> {
        let empty_headers: &[(&str, &str)] = &[];
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
                let url = format!(
                    "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
                    self.model, self.api_key
                );
                execute_http_chat_stream(
                    &self.client,
                    &url,
                    empty_headers,
                    &payload,
                    parse_gemini_sse_chunk,
                )
                .await
            }
            AiProviderKind::Claude => {
                if self.api_key.is_empty() {
                    return Err(format!(
                        "Anthropic API key not provided or set in {ENV_ANTHROPIC_API_KEY}"
                    ));
                }
                let mut payload = chat_message_payload(
                    &self.model,
                    self.temperature,
                    prompt,
                    Some(DEFAULT_CLAUDE_MAX_TOKENS),
                );
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("stream".to_string(), json!(true));
                }
                let headers = [
                    (HEADER_ANTHROPIC_API_KEY, self.api_key.as_str()),
                    (HEADER_ANTHROPIC_VERSION, ANTHROPIC_API_VERSION),
                ];
                execute_http_chat_stream(
                    &self.client,
                    DEFAULT_CLAUDE_ENDPOINT,
                    &headers,
                    &payload,
                    parse_claude_sse_chunk,
                )
                .await
            }
            AiProviderKind::OpenAi => {
                if self.api_key.is_empty() {
                    return Err(format!(
                        "OpenAI API key not provided or set in {ENV_OPENAI_API_KEY}"
                    ));
                }
                let mut payload = chat_message_payload(&self.model, self.temperature, prompt, None);
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("stream".to_string(), json!(true));
                }
                let auth_hdr = format!("{BEARER_PREFIX}{}", self.api_key);
                let headers = [(HEADER_AUTHORIZATION, auth_hdr.as_str())];
                let url =
                    resolve_openai_endpoint(self.endpoint.as_deref(), DEFAULT_OPENAI_ENDPOINT);
                execute_http_chat_stream(
                    &self.client,
                    &url,
                    &headers,
                    &payload,
                    parse_openai_sse_chunk,
                )
                .await
            }
            AiProviderKind::Custom => {
                let mut payload = chat_message_payload(&self.model, self.temperature, prompt, None);
                if let Some(obj) = payload.as_object_mut() {
                    obj.insert("stream".to_string(), json!(true));
                }
                let url =
                    resolve_openai_endpoint(self.endpoint.as_deref(), DEFAULT_CUSTOM_ENDPOINT);
                let auth_hdr = format!("{BEARER_PREFIX}{}", self.api_key);
                let mut headers = Vec::new();
                if !self.api_key.is_empty() {
                    headers.push((HEADER_AUTHORIZATION, auth_hdr.as_str()));
                }
                execute_http_chat_stream(
                    &self.client,
                    &url,
                    &headers,
                    &payload,
                    parse_openai_sse_chunk,
                )
                .await
            }
            _ => Err("Unsupported cloud provider streaming backend".to_string()),
        }
    }
}
