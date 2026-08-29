#![forbid(unsafe_code)]

use super::constants::*;
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
}

impl OllamaProvider {
    pub fn new(model: Option<String>, endpoint: Option<String>, temperature: Option<f64>) -> Self {
        Self {
            model: model.unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_string()),
            endpoint: endpoint.unwrap_or_else(|| DEFAULT_OLLAMA_ENDPOINT.to_string()),
            temperature: temperature.unwrap_or(DEFAULT_TEMPERATURE),
        }
    }
}

fn execute_curl_post(
    url: &str,
    extra_headers: &[(&str, &str)],
    payload: &serde_json::Value,
) -> Result<String, String> {
    let mut cmd = std::process::Command::new(CURL_COMMAND);
    let content_type_hdr = format!("{}: {}", HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON);
    cmd.args(["-s", "-X", "POST", url, "-H", &content_type_hdr]);
    for (k, v) in extra_headers {
        cmd.args(["-H", &format!("{}: {}", k, v)]);
    }
    cmd.args(["-d", &payload.to_string()]);

    match cmd.output() {
        Ok(out) if out.status.success() => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        Ok(out) => Err(format!(
            "HTTP request failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )),
        Err(e) => Err(format!("Failed to execute curl: {}", e)),
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
        let resp_str = execute_curl_post(&url, &[], &payload)?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(resp) = val.get("response").and_then(|r| r.as_str())
        {
            return Ok(resp.to_string());
        }
        Ok(resp_str)
    }
}

fn post_and_extract(
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
    extract: impl FnOnce(&serde_json::Value) -> Option<&str>,
) -> Result<String, String> {
    let resp_str = execute_curl_post(url, headers, payload)?;
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
        && let Some(text) = extract(&val)
    {
        return Ok(text.to_string());
    }
    Ok(resp_str)
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
    ) -> Self {
        let (model, api_key, temperature) =
            init_provider_config(model, default_model, api_key, env_key, temperature);
        Self {
            model,
            api_key,
            temperature,
            kind,
        }
    }

    pub fn new_gemini(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_GEMINI_MODEL,
            api_key,
            ENV_GEMINI_API_KEY,
            temperature,
            AiProviderKind::Gemini,
        )
    }

    pub fn new_claude(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_CLAUDE_MODEL,
            api_key,
            ENV_ANTHROPIC_API_KEY,
            temperature,
            AiProviderKind::Claude,
        )
    }

    pub fn new_openai(
        model: Option<String>,
        api_key: Option<String>,
        temperature: Option<f64>,
    ) -> Self {
        Self::create_cloud(
            model,
            DEFAULT_OPENAI_MODEL,
            api_key,
            ENV_OPENAI_API_KEY,
            temperature,
            AiProviderKind::OpenAi,
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
                post_and_extract(&url, &[], &payload, |val| {
                    val.get("candidates")?
                        .get(0)?
                        .get("content")?
                        .get("parts")?
                        .get(0)?
                        .get("text")?
                        .as_str()
                })
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
                execute_http_chat(DEFAULT_CLAUDE_ENDPOINT, &headers, &payload, |val| {
                    val.get("content")?.get(0)?.get("text")?.as_str()
                })
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
                execute_http_chat(DEFAULT_OPENAI_ENDPOINT, &headers, &payload, |val| {
                    val.get("choices")?
                        .get(0)?
                        .get("message")?
                        .get("content")?
                        .as_str()
                })
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

fn execute_http_chat(
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
    extract_fn: impl Fn(&serde_json::Value) -> Option<&str>,
) -> Result<String, String> {
    post_and_extract(url, headers, payload, extract_fn)
}

/// Constructs an AI provider instance from a configuration object.
pub fn create_ai_provider(config: &AiProviderConfig) -> Box<dyn AiProvider> {
    match config.provider {
        AiProviderKind::Mock => Box::new(MockAiProvider::new(config.model.clone())),
        AiProviderKind::Ollama => Box::new(OllamaProvider::new(
            config.model.clone(),
            config.endpoint.clone(),
            config.temperature,
        )),
        AiProviderKind::Gemini => Box::new(CloudAiProvider::new_gemini(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::Claude => Box::new(CloudAiProvider::new_claude(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::OpenAi => Box::new(CloudAiProvider::new_openai(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::Custom => Box::new(MockAiProvider::new(None)),
    }
}
