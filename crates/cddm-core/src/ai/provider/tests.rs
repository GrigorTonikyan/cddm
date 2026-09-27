#![forbid(unsafe_code)]

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

    let claude = CloudAiProvider::new_claude_with_client(None, None, None, Some(client.clone()));
    assert_eq!(claude.model, DEFAULT_CLAUDE_MODEL);
    assert_eq!(claude.kind, AiProviderKind::Claude);

    let openai = CloudAiProvider::new_openai_with_client(None, None, None, Some(client.clone()));
    assert_eq!(openai.model, DEFAULT_OPENAI_MODEL);
    assert_eq!(openai.kind, AiProviderKind::OpenAi);

    let custom = CloudAiProvider::new_custom_with_client(
        Some("deepseek-coder".into()),
        None,
        Some("http://localhost:8000/v1".into()),
        None,
        Some(client),
    );
    assert_eq!(custom.model, "deepseek-coder");
    assert_eq!(custom.kind, AiProviderKind::Custom);
    assert_eq!(custom.endpoint.as_deref(), Some("http://localhost:8000/v1"));
}

#[test]
fn test_resolve_openai_endpoint() {
    assert_eq!(
        resolve_openai_endpoint(Some("http://localhost:11434/v1"), DEFAULT_OPENAI_ENDPOINT),
        "http://localhost:11434/v1/chat/completions"
    );
    assert_eq!(
        resolve_openai_endpoint(
            Some("http://localhost:11434/v1/chat/completions"),
            DEFAULT_OPENAI_ENDPOINT
        ),
        "http://localhost:11434/v1/chat/completions"
    );
    assert_eq!(
        resolve_openai_endpoint(None, DEFAULT_OPENAI_ENDPOINT),
        DEFAULT_OPENAI_ENDPOINT
    );
}

#[test]
fn test_normalize_chat_endpoint() {
    assert_eq!(
        normalize_chat_endpoint("http://localhost:8000/v1"),
        "http://localhost:8000/v1/chat/completions"
    );
    assert_eq!(
        normalize_chat_endpoint("http://localhost:8000/v1/chat/completions"),
        "http://localhost:8000/v1/chat/completions"
    );
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

#[test]
fn test_resolve_provider_kind_default_and_env() {
    assert_eq!(
        resolve_provider_kind(AiProviderKind::Mock),
        AiProviderKind::Mock
    );
    assert_eq!(
        resolve_provider_kind(AiProviderKind::Custom),
        AiProviderKind::Custom
    );
}
