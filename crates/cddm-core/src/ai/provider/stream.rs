#![forbid(unsafe_code)]

use std::pin::Pin;
use tokio_stream::Stream;

/// Individual token or diff chunk result yielded by the provider stream.
pub type BoxStreamItem = Result<String, String>;

/// Boxed pinned asynchronous stream of AI provider token chunks.
pub type BoxAiStream = Pin<Box<dyn Stream<Item = BoxStreamItem> + Send>>;

/// Parses an OpenAI / vLLM / LocalAI Server-Sent Event data chunk.
pub fn parse_openai_sse_chunk(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(data) = trimmed.strip_prefix("data: ") {
        let payload = data.trim();
        if payload == "[DONE]" {
            return None;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) {
            return val
                .get("choices")
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("delta"))
                .and_then(|d| d.get("content"))
                .and_then(|s| s.as_str())
                .map(ToString::to_string);
        }
    }
    None
}

/// Parses an Anthropic Claude Server-Sent Event data chunk.
pub fn parse_claude_sse_chunk(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(data) = trimmed.strip_prefix("data: ") {
        let payload = data.trim();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) {
            // Check for content_block_delta text event
            if let Some(delta) = val.get("delta")
                && let Some(text) = delta.get("text").and_then(|s| s.as_str())
            {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// Parses a Google Gemini Server-Sent Event data chunk.
pub fn parse_gemini_sse_chunk(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(data) = trimmed.strip_prefix("data: ") {
        let payload = data.trim();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(payload) {
            return val
                .get("candidates")
                .and_then(|c| c.get(0))
                .and_then(|c| c.get("content"))
                .and_then(|c| c.get("parts"))
                .and_then(|p| p.get(0))
                .and_then(|p| p.get("text"))
                .and_then(|s| s.as_str())
                .map(ToString::to_string);
        }
    }
    None
}

/// Parses an Ollama streaming newline-delimited JSON chunk.
pub fn parse_ollama_chunk(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return val
            .get("response")
            .and_then(|r| r.as_str())
            .map(ToString::to_string);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_openai_sse_chunk() {
        let line = r#"data: {"choices":[{"delta":{"content":"+fn helper() {"}}]}"#;
        assert_eq!(
            parse_openai_sse_chunk(line),
            Some("+fn helper() {".to_string())
        );

        let done = "data: [DONE]";
        assert_eq!(parse_openai_sse_chunk(done), None);

        let non_data = ": ping";
        assert_eq!(parse_openai_sse_chunk(non_data), None);
    }

    #[test]
    fn test_parse_claude_sse_chunk() {
        let line = r#"data: {"type":"content_block_delta","delta":{"type":"text_delta","text":"println!();"}}"#;
        assert_eq!(
            parse_claude_sse_chunk(line),
            Some("println!();".to_string())
        );

        let ping = "data: {}";
        assert_eq!(parse_claude_sse_chunk(ping), None);
    }

    #[test]
    fn test_parse_gemini_sse_chunk() {
        let line = r#"data: {"candidates":[{"content":{"parts":[{"text":"return true;"}]}}]}"#;
        assert_eq!(
            parse_gemini_sse_chunk(line),
            Some("return true;".to_string())
        );

        let empty = "data: {}";
        assert_eq!(parse_gemini_sse_chunk(empty), None);
    }

    #[test]
    fn test_parse_ollama_chunk() {
        let line = r#"{"model":"qwen2.5-coder","response":"pub fn test()","done":false}"#;
        assert_eq!(parse_ollama_chunk(line), Some("pub fn test()".to_string()));

        let empty = "   ";
        assert_eq!(parse_ollama_chunk(empty), None);
    }
}
