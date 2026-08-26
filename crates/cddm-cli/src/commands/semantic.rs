use crate::formatters::format_semantic_report;
use crate::types::OutputFormat;
use cddm_core::{DEFAULT_MIN_TOKENS, ScanConfig, scan_cross_language_workspace};
use std::path::PathBuf;

/// Executes the dedicated `cddm semantic` CLI command to analyze cross-language clones.
pub fn run_semantic_command(
    directory: PathBuf,
    threshold: f64,
    min_tokens: usize,
    format: OutputFormat,
    languages: Vec<String>,
    ignore: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens: if min_tokens == 0 {
            DEFAULT_MIN_TOKENS
        } else {
            min_tokens
        },
        languages,
        ignore_patterns: if ignore.is_empty() {
            ScanConfig::default().ignore_patterns
        } else {
            ignore
        },
        detect_type2: true,
        scan_self: true,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: true,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
        cross_language: true,
    };

    let pairs = scan_cross_language_workspace(&config, threshold)
        .map_err(|e| format!("Semantic cross-language scan failed: {}", e))?;

    let report = format_semantic_report(&pairs, format, threshold);
    println!("{}", report);

    Ok(())
}
