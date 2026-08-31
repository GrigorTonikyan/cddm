pub mod ai;
pub mod ai_prompt;
pub mod ast;
pub mod blame;
pub mod cache;
pub mod cluster;
pub mod coverage;
pub mod cpg;
pub mod dead_code;
pub mod detector;
pub mod diff;
pub mod diff_matrix;
pub mod error;
pub mod extract;
pub mod fingerprint;
pub mod grammar;
pub mod hub;
pub mod io;
pub mod logging;
pub mod monorepo;
pub mod neural;
pub mod overlap;
pub mod policy;
pub mod pr_comment;
pub mod query;
pub mod refactor;
pub mod sarif;
pub mod semantic_graph;
pub mod service;
pub mod simd;
pub mod suppression;
pub mod timeline;
pub mod tokenizer;
pub mod types;
pub mod watcher;
pub mod workflow;

pub use ai::{
    AiProvider, AiProviderConfig, AiProviderKind, DEFAULT_CLAUDE_MAX_TOKENS, DEFAULT_CLAUDE_MODEL,
    DEFAULT_EXTRACTED_FUNCTION_NAME, DEFAULT_GEMINI_MODEL, DEFAULT_HEAL_ITERATIONS,
    DEFAULT_HEAL_LINES_SAVED_MULTIPLIER, DEFAULT_HEAL_SIMILARITY, DEFAULT_HEAL_TOKEN_COUNT,
    DEFAULT_MOCK_DIFF_RESPONSE, DEFAULT_OLLAMA_ENDPOINT, DEFAULT_OLLAMA_MODEL,
    DEFAULT_OPENAI_MODEL, DEFAULT_PROVIDER_TIMEOUT_SECS, DEFAULT_TARGET_MODULE,
    DEFAULT_TEMPERATURE, DEFAULT_VERIFY_TIMEOUT_SECS, ENV_ANTHROPIC_API_KEY, ENV_GEMINI_API_KEY,
    ENV_OPENAI_API_KEY, HealIterationLog, HealRefactorRequest, HealRefactorResult,
    MAX_HEAL_ITERATIONS, MIN_HEAL_ITERATIONS, create_ai_provider, heal_cluster_refactor,
};
pub use ai_prompt::*;
pub use cache::pack::*;
pub use cache::{
    CachedFileEntry, DiskFingerprintCache, find_workspace_root, resolve_default_cache_path,
};
pub use cluster::cluster_clone_pairs;
pub use coverage::*;
pub use cpg::{
    CodePropertyGraph, CpgEdge, CpgEdgeKind, CpgNode, CpgNodeKind, SymbolId, SymbolInterner,
    build_all_cpgs_from_source, build_cpg_from_function,
};
pub use dead_code::*;
pub use detector::run_scan;
pub use diff::{get_changed_files_between_refs, run_diff_scan};
pub use diff_matrix::*;
pub use error::*;
pub use extract::*;
pub use hub::*;
pub use io::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
pub use logging::*;
pub use monorepo::*;
pub use neural::*;
pub use overlap::*;
pub use policy::PolicyEngine;
pub use pr_comment::generate_pr_markdown_comment;
pub use query::*;
pub use refactor::*;
pub use sarif::*;
pub use semantic_graph::*;
pub use service::*;
pub use simd::compute_kgram_rolling_hashes;
pub use suppression::SuppressionEngine;
pub use timeline::collect_git_timeline;
pub use types::*;
pub use watcher::{CddmWatcher, WatchDeltaReport, WatchFileEvent};
pub use workflow::*;
