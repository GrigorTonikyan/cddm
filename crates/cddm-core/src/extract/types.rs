#![forbid(unsafe_code)]

use crate::types::{CloneLocation, InferredParameter};
use serde::{Deserialize, Serialize};

/// Target packaging strategy for code extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExtractTargetKind {
    /// Automatically determine based on repository structure (e.g. Cargo crate or submodule).
    #[default]
    Auto,
    /// Create a brand-new standalone workspace crate or package.
    NewCrate,
    /// Create a new shared module file within an existing source tree.
    NewModule,
    /// Append or export into an existing module.
    ExistingModule,
}

/// Request payload to generate an automated shared extraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractRequest {
    /// List of clone occurrence locations to extract.
    pub occurrences: Vec<CloneLocation>,
    /// Target destination path or crate name (e.g., `crates/shared_utils` or `src/common/utils.rs`).
    pub target_path: String,
    /// Custom function or helper name to extract.
    pub custom_function_name: Option<String>,
    /// Target packaging kind.
    #[serde(default)]
    pub target_kind: ExtractTargetKind,
    /// Custom parameter names override.
    pub custom_parameter_names: Option<Vec<String>>,
    /// Whether to generate unit tests for the extracted helper.
    #[serde(default)]
    pub generate_tests: bool,
    /// Whether to generate performance benchmarks for the extracted helper.
    #[serde(default)]
    pub generate_benchmarks: bool,
    /// Whether to perform a dry-run without writing to disk.
    #[serde(default)]
    pub dry_run: bool,
}

/// Represents a newly generated or modified file as part of the extraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedFile {
    /// Path of the generated or modified file relative to workspace root.
    pub file_path: String,
    /// Generated file content.
    pub content: String,
    /// Whether this file is newly created.
    pub is_new: bool,
}

/// Represents an update to a workspace or package manifest (e.g. Cargo.toml or package.json).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestUpdate {
    /// Path to the manifest file relative to workspace root.
    pub manifest_path: String,
    /// Package/crate name that was added or referenced.
    pub dependency_name: String,
    /// Unified diff or change description for the manifest.
    pub diff_preview: String,
    /// Updated content of the manifest file.
    pub updated_content: String,
}

/// Represents an occurrence file call-site rewrite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallerRewrite {
    /// Path to the caller file.
    pub file_path: String,
    /// Import statement injected at the top of the file.
    pub injected_import: Option<String>,
    /// Full transformed source code with duplicate body replaced by helper invocation.
    pub rewritten_content: String,
    /// Unified diff for the caller file.
    pub diff_patch: String,
}

/// Aggregated result of generating or applying a shared crate/module extraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractResult {
    /// Name of the extracted helper function.
    pub function_name: String,
    /// Target destination path.
    pub target_path: String,
    /// Target packaging kind applied.
    pub target_kind: ExtractTargetKind,
    /// Extracted helper function signature.
    pub helper_signature: String,
    /// Inferred parameter metadata.
    pub inferred_parameters: Vec<InferredParameter>,
    /// List of newly generated files (e.g. `Cargo.toml`, `src/lib.rs`, `index.ts`).
    pub generated_files: Vec<ExtractedFile>,
    /// Synthesized unit test files verifying extracted functionality.
    #[serde(default)]
    pub test_files: Vec<ExtractedFile>,
    /// Synthesized micro-benchmark files verifying extracted performance.
    #[serde(default)]
    pub benchmark_files: Vec<ExtractedFile>,
    /// Manifest modifications across workspace root and caller packages.
    pub manifest_updates: Vec<ManifestUpdate>,
    /// Occurrence file rewrites and injected imports.
    pub caller_rewrites: Vec<CallerRewrite>,
    /// Estimated total lines of duplicate code eliminated.
    pub total_lines_saved: usize,
    /// Whether the extraction generated syntactically valid code.
    pub syntax_valid: bool,
    /// Informational message or status description.
    pub message: String,
}
