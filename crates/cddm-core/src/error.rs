#![forbid(unsafe_code)]

use thiserror::Error;

/// Root error hierarchy for all CDDM core domain operations.
#[derive(Debug, Error)]
pub enum CddmError {
    #[error("Scan error: {0}")]
    Scan(#[from] ScanError),

    #[error("AST parsing error: {0}")]
    Ast(#[from] AstParseError),

    #[error("Refactoring error: {0}")]
    Refactor(#[from] RefactorError),

    #[error("Policy violation: {0}")]
    Policy(#[from] PolicyViolationError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("Shared module extraction error: {0}")]
    Extraction(#[from] ExtractionError),

    #[error("Neural embedding error: {0}")]
    Neural(#[from] NeuralError),

    #[error("{0}")]
    General(String),
}

/// Errors occurring during repository traversal and clone detection scanning.
#[derive(Debug, Error)]
pub enum ScanError {
    #[error("I/O error during scan: {0}")]
    Io(#[from] std::io::Error),

    #[error("Directory not found or inaccessible: '{0}'")]
    InvalidDirectory(String),

    #[error("Programming language '{0}' is not supported for AST clone detection")]
    LanguageNotSupported(String),

    #[error("Policy enforcement failure: {0}")]
    PolicyViolation(String),

    #[error("Scan operation was cancelled by user")]
    Cancelled,
}

/// Errors occurring during Tree-sitter CST parsing or AST hashing.
#[derive(Debug, Error)]
pub enum AstParseError {
    #[error("Grammar not found for extension: '{0}'")]
    LanguageNotSupported(String),

    #[error("Syntax parsing failed for file: '{0}'")]
    SyntaxError(String),

    #[error("Incremental tree edit failed: {0}")]
    TreeEditFailed(String),
}

/// Errors occurring during clone refactoring synthesis and patch generation.
#[derive(Debug, Error)]
pub enum RefactorError {
    #[error("Clone pair or cluster '{0}' not found in scan results")]
    CloneNotFound(String),

    #[error("Failed to synthesize invariant refactoring patch: {0}")]
    PatchSynthesisFailed(String),

    #[error("Git branch operation failed: {0}")]
    GitBranchError(String),

    #[error("Test verification runner failed: {0}")]
    VerificationFailed(String),

    #[error("AI provider error: {0}")]
    AiProviderError(String),
}

/// Errors occurring during architectural boundary and anti-duplication policy checks.
#[derive(Debug, Error)]
pub enum PolicyViolationError {
    #[error("Boundary rule '{rule}' violated between '{source_layer}' and '{target_layer}'")]
    BoundaryBreach {
        source_layer: String,
        target_layer: String,
        rule: String,
    },

    #[error("Zero-duplication rule violated in module '{path}' with {tokens} duplicate tokens")]
    ZeroDuplicationViolation { path: String, tokens: usize },

    #[error("Token limit exceeded in '{path}': {actual} tokens (maximum allowed: {limit})")]
    LimitExceeded {
        path: String,
        actual: usize,
        limit: usize,
    },
}

/// Errors occurring in the persistent Redb cache or .cddmpack archives.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Disk cache storage error: {0}")]
    DiskError(String),

    #[error("Cache pack checksum mismatch: expected {expected}, actual {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Corrupted cache archive: {0}")]
    CorruptedData(String),
}

/// Errors occurring during automated shared module extraction.
#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("Workspace manifest not found in: '{0}'")]
    ManifestNotFound(String),

    #[error("Failed to parse workspace manifest '{0}': {1}")]
    ManifestParseFailed(String, String),

    #[error("Target destination crate or package '{0}' already exists")]
    TargetAlreadyExists(String),

    #[error("AST caller callsite rewriting failed: {0}")]
    RewriteFailed(String),
}

/// Errors occurring in the neural code embedding and HNSW vector index.
#[derive(Debug, Error)]
pub enum NeuralError {
    #[error("Vector dimension mismatch: expected {expected}, actual {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Attempted to operate on an empty embedding vector")]
    EmptyVector,

    #[error("HNSW graph index error: {0}")]
    IndexError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_and_conversion() {
        let scan_err = ScanError::InvalidDirectory("non_existent_dir".to_string());
        let cddm_err: CddmError = scan_err.into();
        assert!(cddm_err.to_string().contains("non_existent_dir"));

        let policy_err = PolicyViolationError::ZeroDuplicationViolation {
            path: "src/auth".to_string(),
            tokens: 150,
        };
        let cddm_policy: CddmError = policy_err.into();
        assert!(cddm_policy.to_string().contains("src/auth"));
        assert!(cddm_policy.to_string().contains("150"));
    }
}
