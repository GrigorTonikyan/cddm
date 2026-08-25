#![forbid(unsafe_code)]

use crate::formatters::{print_diff_console_report, print_diff_markdown_report};
use crate::types::OutputFormat;
use cddm_core::{ScanConfig, run_diff_scan};
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

#[allow(clippy::too_many_arguments)]
pub async fn run_diff_command(
    base_ref: String,
    target_ref: Option<String>,
    directory: PathBuf,
    min_tokens: usize,
    format: OutputFormat,
    fail_threshold: Option<f64>,
    languages: Vec<String>,
    ignore: Vec<String>,
    git_blame: bool,
    cache_dir: Option<PathBuf>,
    no_cache: bool,
    cddmignore: Option<PathBuf>,
    ignore_tests: bool,
    ignore_mocks: bool,
    ignore_generated: bool,
    rules: Option<PathBuf>,
    enforce_policies: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());

    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages,
        ignore_patterns: if ignore.is_empty() {
            ScanConfig::default().ignore_patterns
        } else {
            ignore
        },
        detect_type2: true,
        scan_self: true,
        enable_git_blame: git_blame,
        cache_dir: cache_path,
        enable_cache: !no_cache,
        cddmignore_path: cddmignore.map(|p| p.to_string_lossy().to_string()),
        ignore_tests,
        ignore_mocks,
        ignore_generated,
        rules_path: rules.map(|p| p.to_string_lossy().to_string()),
        enforce_policies,
    };

    let (tx, mut rx) = mpsc::channel::<cddm_core::ScanProgress>(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    if format == OutputFormat::Console {
        tokio::spawn(async move {
            while let Some(progress) = rx.recv().await {
                eprintln!(
                    "[{}] {}% - {}",
                    progress.phase,
                    (progress.progress * 100.0) as u32,
                    progress.message
                );
            }
        });
    }

    let diff_result =
        run_diff_scan(&base_ref, target_ref.as_deref(), config, tx, cancel_flag).await?;

    match format {
        OutputFormat::Console => print_diff_console_report(&diff_result),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&diff_result)?);
        }
        OutputFormat::Markdown => print_diff_markdown_report(&diff_result),
        OutputFormat::Sarif => {
            eprintln!("Warning: SARIF format for diff scanning falls back to JSON");
            println!("{}", serde_json::to_string_pretty(&diff_result)?);
        }
    }

    if let Some(threshold) = fail_threshold {
        if (diff_result.summary.new_clones as f64) > threshold {
            eprintln!(
                "Error: Introduced {} new clones, exceeding failure threshold of {:.0}",
                diff_result.summary.new_clones, threshold
            );
            std::process::exit(1);
        }
    } else if diff_result.summary.new_clones > 0 {
        eprintln!(
            "Notice: {} new clone pairs introduced in this changeset.",
            diff_result.summary.new_clones
        );
    }

    Ok(())
}
