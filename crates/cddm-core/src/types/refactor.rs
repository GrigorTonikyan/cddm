#![forbid(unsafe_code)]

use super::clone::CloneLocation;
use serde::{Deserialize, Serialize};

/// Request payload for synthesizing customized cluster refactoring preview in the sandbox.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RefactorSandboxRequest {
    /// Target 1-based cluster ID
    pub cluster_id: Option<usize>,
    /// Explicit list of occurrences to refactor
    pub occurrences: Vec<CloneLocation>,
    /// Custom function name to extract (e.g. "validate_user_credentials")
    pub custom_function_name: Option<String>,
    /// Target destination module/file path for extracted helper
    pub target_module_path: Option<String>,
    /// Custom parameter names to use in extracted function signature
    pub custom_parameter_names: Option<Vec<String>>,
}

/// Structured result of customized refactoring synthesis in the sandbox.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RefactorSandboxResult {
    /// Cluster ID if applicable
    pub cluster_id: Option<usize>,
    /// Name of the extracted helper function
    pub function_name: String,
    /// Target file path where extracted function resides
    pub target_module_path: String,
    /// Unified diff patch
    pub unified_patch: String,
    /// Total lines saved across all sites
    pub total_lines_saved: usize,
    /// Number of call sites refactored
    pub sites_count: usize,
    /// List of affected distinct file paths
    pub affected_files: Vec<String>,
}

/// Request payload for applying refactoring patch directly with Git branch creation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApplyRefactorBranchRequest {
    /// Unified patch content to apply
    pub patch: String,
    /// Name of the git branch to create (e.g. "cddm/refactor-cluster-1")
    pub branch_name: Option<String>,
    /// Commit message if committing
    pub commit_message: Option<String>,
    /// Whether to create a dedicated Git branch before applying
    pub create_branch: bool,
}

/// Result of applying refactoring patch with Git branch creation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApplyRefactorBranchResult {
    /// Whether patch applied successfully
    pub success: bool,
    /// Name of the branch created, if applicable
    pub branch_created: Option<String>,
    /// List of modified file paths
    pub modified_files: Vec<String>,
    /// Total hunks applied
    pub hunks_applied: usize,
    /// Informational message
    pub message: String,
}

/// Represents an inferred parameter with extracted name and language-specific type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InferredParameter {
    /// Synthesized or customized parameter name
    pub name: String,
    /// Inferred language-appropriate type (e.g. &str, number, int)
    pub inferred_type: String,
    /// Original occurrence values across clone fragments
    pub original_values: Vec<String>,
}

/// Represents a source file rewritten via AST node substitution.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AstRewrittenFile {
    /// Target file path
    pub file_path: String,
    /// Original line count before rewrite
    pub original_line_count: usize,
    /// New line count after rewrite
    pub new_line_count: usize,
    /// Number of clone call sites replaced
    pub call_sites_count: usize,
    /// Full transformed file source code
    pub rewritten_source: String,
    /// Any import/use statements added
    pub imports_added: Vec<String>,
}

/// Complete result of an AST-native refactoring transformation across multiple files.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AstRewriteResult {
    /// Cluster ID if applicable
    pub cluster_id: Option<usize>,
    /// Extracted helper function name
    pub function_name: String,
    /// Destination module or file path
    pub target_module_path: String,
    /// Full generated function signature header
    pub helper_signature: String,
    /// Full generated helper function code block
    pub helper_function_code: String,
    /// Inferred parameters
    pub inferred_parameters: Vec<InferredParameter>,
    /// Transformed files
    pub rewritten_files: Vec<AstRewrittenFile>,
    /// Synthesized unified diff patch for preview
    pub unified_patch: String,
    /// Estimated net lines of code saved
    pub total_lines_saved: usize,
    /// Whether the rewritten code parses cleanly into the target AST grammar
    pub syntax_valid: bool,
}

/// Request to run test suite verification on refactored workspace or branch.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VerifyRefactorRequest {
    /// Workspace root directory
    pub directory: String,
    /// Optional custom test command (e.g. "cargo test", "bun test", "pytest")
    pub test_command: Option<String>,
    /// Optional branch to test against
    pub branch_name: Option<String>,
    /// Timeout in seconds (default: 60)
    pub timeout_seconds: Option<u64>,
}

/// Result of running closed-loop test suite verification.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct VerifyRefactorResult {
    /// Whether test suite exited with code 0
    pub success: bool,
    /// Process exit code
    pub exit_code: i32,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Exact command line executed
    pub command_executed: String,
    /// Trailing stdout output snippet
    pub stdout_snippet: String,
    /// Trailing stderr output snippet
    pub stderr_snippet: String,
    /// Status message
    pub message: String,
}
