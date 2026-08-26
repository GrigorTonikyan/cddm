#![forbid(unsafe_code)]

use crate::formatters::{print_console_report, print_markdown_report, print_sarif_report};
use crate::types::OutputFormat;
use cddm_core::{PolicySeverity, ScanConfig, run_scan};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

#[allow(clippy::too_many_arguments)]
pub async fn run_scan_command(
    directory: PathBuf,
    min_tokens: usize,
    format: OutputFormat,
    fail_threshold: Option<f64>,
    languages: Vec<String>,
    ignore: Vec<String>,
    git_blame: bool,
    cache_dir: Option<PathBuf>,
    no_cache: bool,
    clear_cache: bool,
    cddmignore: Option<PathBuf>,
    ignore_tests: bool,
    ignore_mocks: bool,
    ignore_generated: bool,
    rules: Option<PathBuf>,
    enforce_policies: bool,
    cross_language: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());

    if clear_cache {
        let path_to_clear = cache_dir
            .clone()
            .unwrap_or_else(|| directory.join(cddm_core::DEFAULT_CACHE_FILE));
        if path_to_clear.exists() {
            let _ = fs::remove_file(&path_to_clear);
            eprintln!("Cleared cache database at '{}'", path_to_clear.display());
        }
    }

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
        cross_language,
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

    let result = run_scan(config, tx, cancel_flag).await?;

    match format {
        OutputFormat::Console => print_console_report(&result),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Markdown => print_markdown_report(&result),
        OutputFormat::Sarif => print_sarif_report(&result)?,
    }

    if let Some(threshold) = fail_threshold
        && result.duplication_percentage > threshold
    {
        eprintln!(
            "Error: Duplication percentage {:.2}% exceeds failure threshold {:.2}%",
            result.duplication_percentage, threshold
        );
        std::process::exit(1);
    }

    if enforce_policies
        && result
            .policy_violations
            .iter()
            .any(|v| v.severity == PolicySeverity::Error)
    {
        eprintln!(
            "Error: Architectural policy violations detected ({} violation(s)).",
            result.policy_violations.len()
        );
        std::process::exit(1);
    }

    Ok(())
}
