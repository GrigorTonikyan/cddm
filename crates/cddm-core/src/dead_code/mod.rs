#![forbid(unsafe_code)]

pub mod detector;
pub mod static_analyzer;
pub mod types;

pub use detector::run_dead_code_detection;
pub use static_analyzer::analyze_static_dead_code;
pub use types::{DeadCodeConfig, DeadCodeItem, DeadCodeKind, DeadCodeSummary};
