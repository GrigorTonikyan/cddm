pub mod ast;
pub mod blame;
pub mod cache;
pub mod detector;
pub mod fingerprint;
pub mod grammar;
pub mod tokenizer;
pub mod types;
pub mod watcher;

pub use detector::run_scan;
pub use types::{
    ClonePair, CloneType, DEFAULT_DIRECTORY, DEFAULT_IGNORE_PATTERNS, DEFAULT_MIN_TOKENS,
    LanguageStats, LineSpan, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, NormalizedToken, ScanConfig,
    ScanPhase, ScanProgress, ScanResult,
};
