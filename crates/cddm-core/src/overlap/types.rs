#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Recommendation for an existing standard or community package replacement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecommendedLibrary {
    /// Programming language (e.g. "rust", "typescript", "python", "go", "java", "csharp")
    pub language: String,
    /// Package/crate name (e.g. "itertools", "lodash-es", "tokio-util", "hex", "slug")
    pub package_name: String,
    /// CLI package installation command (e.g. "cargo add itertools", "bun add lodash-es")
    pub install_command: String,
    /// Idiomatic code snippet demonstrating library usage
    pub replacement_snippet: String,
}

/// A canonical standard algorithm or utility pattern commonly reinvented in codebases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemAlgorithm {
    /// Identifier name (e.g. "Array Chunking", "String Slugify", "Debounce Timer")
    pub name: String,
    /// Category domain (e.g. "Collections", "Text & Strings", "Async & Timing", "Encoding")
    pub category: String,
    /// Description of the utility
    pub description: String,
    /// Identifier keywords and AST tokens that indicate this algorithm
    pub canonical_keywords: Vec<String>,
    /// Language-specific package recommendations
    pub recommendations: Vec<RecommendedLibrary>,
}

/// A detected custom code snippet that reimplements a well-known ecosystem library utility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlapMatch {
    /// Algorithm name matched
    pub algorithm_name: String,
    /// Category domain
    pub category: String,
    /// File containing the reimplemented utility
    pub file_path: String,
    /// Function or method name
    pub function_name: String,
    /// 1-based start and end lines
    pub line_span: (usize, usize),
    /// Match confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Original code snippet
    pub snippet: String,
    /// Recommended ecosystem package replacement
    pub recommended_library: RecommendedLibrary,
}

/// Aggregated result of scanning a workspace for ecosystem library overlap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlapScanResult {
    /// Discovered library overlap matches
    pub matches: Vec<OverlapMatch>,
    /// Total code files inspected
    pub total_files_scanned: usize,
    /// Total function bodies analyzed
    pub scanned_functions: usize,
    /// Informational summary message
    pub summary: String,
}
