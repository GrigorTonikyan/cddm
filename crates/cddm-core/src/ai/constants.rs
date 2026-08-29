#![forbid(unsafe_code)]

macro_rules! define_ai_str_constants {
    ($($name:ident => $val:expr),* $(,)?) => {
        $( pub const $name: &str = $val; )*
    };
}

define_ai_str_constants! {
    DEFAULT_GEMINI_MODEL => "gemini-2.5-pro",
    DEFAULT_CLAUDE_MODEL => "claude-3-7-sonnet",
    DEFAULT_OPENAI_MODEL => "gpt-4.5-preview",
    DEFAULT_OLLAMA_MODEL => "qwen2.5-coder",
    DEFAULT_OLLAMA_ENDPOINT => "http://localhost:11434",
    GEMINI_API_ENDPOINT_TEMPLATE => "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
    DEFAULT_CLAUDE_ENDPOINT => "https://api.anthropic.com/v1/messages",
    DEFAULT_OPENAI_ENDPOINT => "https://api.openai.com/v1/chat/completions",
    OLLAMA_GENERATE_PATH => "api/generate",
    ENV_GEMINI_API_KEY => "GEMINI_API_KEY",
    ENV_ANTHROPIC_API_KEY => "ANTHROPIC_API_KEY",
    ENV_OPENAI_API_KEY => "OPENAI_API_KEY",
    HEADER_CONTENT_TYPE => "Content-Type",
    CONTENT_TYPE_JSON => "application/json",
    HEADER_ANTHROPIC_API_KEY => "x-api-key",
    HEADER_ANTHROPIC_VERSION => "anthropic-version",
    ANTHROPIC_API_VERSION => "2023-06-01",
    HEADER_AUTHORIZATION => "Authorization",
    BEARER_PREFIX => "Bearer ",
    CURL_COMMAND => "curl",
}

/// Default generation temperature for deterministic refactoring.
pub const DEFAULT_TEMPERATURE: f64 = 0.2;

/// Default maximum output token limit for Claude refactoring responses.
pub const DEFAULT_CLAUDE_MAX_TOKENS: usize = 4096;

/// Default HTTP request timeout in seconds for AI provider calls.
pub const DEFAULT_PROVIDER_TIMEOUT_SECS: u64 = 60;

/// Default canned patch response for hermetic mock provider testing.
pub const DEFAULT_MOCK_DIFF_RESPONSE: &str =
    "--- a/test.rs\n+++ b/test.rs\n@@ -1,3 +1,3 @@\n-old();\n+new();\n";

/// Minimum allowed iterations in autonomous healing loop.
pub const MIN_HEAL_ITERATIONS: usize = 1;

/// Maximum allowed iterations in autonomous healing loop.
pub const MAX_HEAL_ITERATIONS: usize = 10;

/// Default number of iterations in autonomous healing loop.
pub const DEFAULT_HEAL_ITERATIONS: usize = 3;

/// Default function name for extracted shared abstraction helper.
pub const DEFAULT_EXTRACTED_FUNCTION_NAME: &str = "extracted_shared_helper";

/// Default target destination module path for extracted helpers.
pub const DEFAULT_TARGET_MODULE: &str = "src/utils.rs";

/// Default baseline similarity target for AI refactoring prompt synthesis.
pub const DEFAULT_HEAL_SIMILARITY: f64 = 0.95;

/// Default token count threshold for AI refactoring prompt synthesis.
pub const DEFAULT_HEAL_TOKEN_COUNT: usize = 100;

/// Multiplier to estimate lines of code saved per occurrence in prompt.
pub const DEFAULT_HEAL_LINES_SAVED_MULTIPLIER: usize = 10;

/// Default timeout in seconds for test suite verification runs during healing.
pub const DEFAULT_VERIFY_TIMEOUT_SECS: u64 = 30;
