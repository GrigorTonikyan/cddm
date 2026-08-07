pub mod types;
pub mod grammar;
pub mod tokenizer;
pub mod fingerprint;
pub mod detector;
pub mod blame;
pub mod ast;
pub mod cache;
pub mod watcher;

pub use detector::run_scan;
pub use types::{ClonePair, CloneType, LanguageStats, LineSpan, NormalizedToken, ScanConfig, ScanProgress, ScanResult};
