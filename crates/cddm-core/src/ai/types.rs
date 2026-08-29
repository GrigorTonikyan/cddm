#![forbid(unsafe_code)]

use crate::types::CloneLocation;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported AI LLM provider backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AiProviderKind {
    Gemini,
    Claude,
    OpenAi,
    Ollama,
    #[default]
    Mock,
    Custom,
}

/// Configuration settings for connecting to an AI provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AiProviderConfig {
    /// Target provider backend
    pub provider: AiProviderKind,
    /// Model identifier (e.g. "gemini-2.5-pro", "claude-3-7-sonnet", "gpt-4.5-preview", "qwen2.5-coder")
    pub model: Option<String>,
    /// Secret API key for authentication
    pub api_key: Option<String>,
    /// Custom HTTP API endpoint URL (e.g. "http://localhost:11434" for Ollama)
    pub endpoint: Option<String>,
    /// Temperature parameter for generation (0.0 to 1.0)
    pub temperature: Option<f64>,
    /// Request timeout in seconds
    pub timeout_secs: Option<u64>,
}

/// Log record for a single iteration in the autonomous healing loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealIterationLog {
    /// 1-based iteration counter
    pub iteration: usize,
    /// Prompt dispatched to the AI provider
    pub prompt: String,
    /// Raw response patch or content returned by the provider
    pub response_patch: String,
    /// Whether the synthesized patch was applied cleanly to the workspace
    pub patch_applied: bool,
    /// Whether workspace tests passed after applying the patch
    pub test_passed: bool,
    /// Compilation or test failure diagnostics fed back to the provider for repair
    pub error_feedback: Option<String>,
}

/// Request parameters for initiating an autonomous refactoring healing loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealRefactorRequest {
    /// Optional target clone cluster ID
    pub cluster_id: Option<usize>,
    /// Optional target clone pair ID
    pub pair_id: Option<usize>,
    /// Explicit occurrence locations
    pub occurrences: Vec<CloneLocation>,
    /// Suggested function name for the extracted shared helper
    pub function_name: Option<String>,
    /// Destination module path for the extracted helper
    pub target_module: Option<String>,
    /// Custom architectural guidelines or constraints
    pub custom_instructions: Option<String>,
    /// AI provider connection settings
    pub provider_config: AiProviderConfig,
    /// Maximum healing repair iterations (default: DEFAULT_HEAL_ITERATIONS)
    pub max_iterations: usize,
    /// Optional Git branch name to commit passing refactoring to
    pub apply_branch: Option<String>,
    /// Whether to run workspace tests to verify refactoring
    pub verify: bool,
    /// Custom test command (e.g. "cargo test", "bun test")
    pub test_cmd: Option<String>,
    /// Root directory path of workspace
    pub workspace_root: Option<PathBuf>,
}

impl Default for HealRefactorRequest {
    fn default() -> Self {
        Self {
            cluster_id: None,
            pair_id: None,
            occurrences: Vec::new(),
            function_name: None,
            target_module: None,
            custom_instructions: None,
            provider_config: AiProviderConfig::default(),
            max_iterations: super::constants::DEFAULT_HEAL_ITERATIONS,
            apply_branch: None,
            verify: false,
            test_cmd: None,
            workspace_root: None,
        }
    }
}

/// Final outcome of an autonomous self-healing refactoring session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealRefactorResult {
    /// Whether the healing loop successfully synthesized a passing refactoring
    pub success: bool,
    /// Total number of iterations performed
    pub iterations_run: usize,
    /// Final unified Git diff patch applied
    pub final_patch: String,
    /// List of file paths modified during refactoring
    pub modified_files: Vec<String>,
    /// Name of created Git branch if branch application was requested
    pub branch_created: Option<String>,
    /// Detailed step-by-step iteration logs
    pub iterations: Vec<HealIterationLog>,
    /// Human-readable summary message
    pub message: String,
}
