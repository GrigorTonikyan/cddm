pub mod ast;
pub mod blame;
pub mod cache;
pub mod detector;
pub mod fingerprint;
pub mod grammar;
pub mod refactor;
pub mod sarif;
pub mod tokenizer;
pub mod types;
pub mod watcher;

pub use detector::run_scan;
pub use refactor::{
    ParameterDifference, RefactorSuggestion, analyze_clone_refactoring,
    analyze_snippets_refactoring,
};
pub use sarif::{SarifReport, generate_sarif_json, generate_sarif_report};
pub use types::{
    ClonePair, CloneType, DEFAULT_DIRECTORY, DEFAULT_IGNORE_PATTERNS, DEFAULT_MIN_TOKENS,
    LanguageStats, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, NormalizedToken, ScanConfig,
    ScanPhase, ScanProgress, ScanResult,
};
