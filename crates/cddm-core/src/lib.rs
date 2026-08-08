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
    ClonePair, CloneType, LanguageStats, LineSpan, NormalizedToken, ScanConfig, ScanProgress,
    ScanResult,
};
