#![forbid(unsafe_code)]

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
            Ok("--- a/test.rs\n+++ b/test.rs\n@@ -1,3 +1,3 @@\n-old();\n+new();\n".to_string())
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
            model: model.unwrap_or_else(|| "llama3".to_string()),
            endpoint: endpoint.unwrap_or_else(|| "http://localhost:11434".to_string()),
            temperature: temperature.unwrap_or(0.2),
        }
    }
}

fn execute_curl_post(
    url: &str,
    extra_headers: &[(&str, &str)],
    payload: &serde_json::Value,
) -> Result<String, String> {
    let mut cmd = std::process::Command::new("curl");
    cmd.args([
        "-s",
        "-X",
        "POST",
        url,
        "-H",
        "Content-Type: application/json",
    ]);
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

        let url = format!("{}/api/generate", self.endpoint.trim_end_matches('/'));
        let resp_str = execute_curl_post(&url, &[], &payload)?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(resp) = val.get("response").and_then(|r| r.as_str())
        {
            return Ok(resp.to_string());
        }
        Ok(resp_str)
    }
}

/// Google Gemini provider.
#[derive(Debug, Clone)]
pub struct GeminiProvider {
    pub model: String,
    pub api_key: String,
    pub temperature: f64,
}

impl GeminiProvider {
    pub fn new(model: Option<String>, api_key: Option<String>, temperature: Option<f64>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "gemini-1.5-pro".to_string()),
            api_key: api_key
                .or_else(|| std::env::var("GEMINI_API_KEY").ok())
                .unwrap_or_default(),
            temperature: temperature.unwrap_or(0.2),
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("Gemini API key not provided or set in GEMINI_API_KEY".to_string());
        }

        let payload = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }],
            "generationConfig": {
                "temperature": self.temperature,
            }
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let resp_str = execute_curl_post(&url, &[], &payload)?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(text) = val
                .get("candidates")
                .and_then(|c| c.get(0))
                .and_then(|c0| c0.get("content"))
                .and_then(|cnt| cnt.get("parts"))
                .and_then(|p| p.get(0))
                .and_then(|p0| p0.get("text"))
                .and_then(|t| t.as_str())
        {
            return Ok(text.to_string());
        }
        Ok(resp_str)
    }
}

/// Anthropic Claude provider.
#[derive(Debug, Clone)]
pub struct ClaudeProvider {
    pub model: String,
    pub api_key: String,
    pub temperature: f64,
}

impl ClaudeProvider {
    pub fn new(model: Option<String>, api_key: Option<String>, temperature: Option<f64>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string()),
            api_key: api_key
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                .unwrap_or_default(),
            temperature: temperature.unwrap_or(0.2),
        }
    }
}

#[async_trait]
impl AiProvider for ClaudeProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("Anthropic API key not provided or set in ANTHROPIC_API_KEY".to_string());
        }

        let payload = json!({
            "model": self.model,
            "max_tokens": 4096,
            "temperature": self.temperature,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let url = "https://api.anthropic.com/v1/messages";
        let headers = [
            ("x-api-key", self.api_key.as_str()),
            ("anthropic-version", "2023-06-01"),
        ];

        let resp_str = execute_curl_post(url, &headers, &payload)?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(text) = val
                .get("content")
                .and_then(|c| c.get(0))
                .and_then(|c0| c0.get("text"))
                .and_then(|t| t.as_str())
        {
            return Ok(text.to_string());
        }
        Ok(resp_str)
    }
}

/// OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    pub model: String,
    pub api_key: String,
    pub temperature: f64,
}

impl OpenAiProvider {
    pub fn new(model: Option<String>, api_key: Option<String>, temperature: Option<f64>) -> Self {
        Self {
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
            api_key: api_key
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .unwrap_or_default(),
            temperature: temperature.unwrap_or(0.2),
        }
    }
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    async fn complete_prompt(&self, prompt: &str) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("OpenAI API key not provided or set in OPENAI_API_KEY".to_string());
        }

        let payload = json!({
            "model": self.model,
            "temperature": self.temperature,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let url = "https://api.openai.com/v1/chat/completions";
        let auth_hdr = format!("Bearer {}", self.api_key);
        let headers = [("Authorization", auth_hdr.as_str())];

        let resp_str = execute_curl_post(url, &headers, &payload)?;
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
            && let Some(text) = val
                .get("choices")
                .and_then(|c| c.get(0))
                .and_then(|c0| c0.get("message"))
                .and_then(|m| m.get("content"))
                .and_then(|cnt| cnt.as_str())
        {
            return Ok(text.to_string());
        }
        Ok(resp_str)
    }
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
        AiProviderKind::Gemini => Box::new(GeminiProvider::new(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::Claude => Box::new(ClaudeProvider::new(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::OpenAi => Box::new(OpenAiProvider::new(
            config.model.clone(),
            config.api_key.clone(),
            config.temperature,
        )),
        AiProviderKind::Custom => Box::new(MockAiProvider::new(None)),
    }
}
