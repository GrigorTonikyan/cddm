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
    detect_type3: bool,
    threads: Option<usize>,
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

    let config = build_cli_scan_config(
        &directory,
        min_tokens,
        languages,
        ignore,
        git_blame,
        cache_path,
        !no_cache,
        cddmignore,
        ignore_tests,
        ignore_mocks,
        ignore_generated,
        rules,
        enforce_policies,
        cross_language,
        detect_type3,
        threads,
    );

    let (tx, rx) = mpsc::channel::<cddm_core::ScanProgress>(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));

    if format == OutputFormat::Console {
        spawn_console_progress_printer(rx);
    }

    let result = run_scan(config, tx, cancel_flag).await?;

    match format {
        OutputFormat::Console => print_console_report(&result),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Markdown => print_markdown_report(&result),
        OutputFormat::Sarif => print_sarif_report(&result)?,
        OutputFormat::Ndjson => println!("{}", serde_json::to_string(&result)?),
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

/// Constructs a standardized ScanConfig from CLI flag inputs.
#[allow(clippy::too_many_arguments)]
pub fn build_cli_scan_config(
    directory: &std::path::Path,
    min_tokens: usize,
    languages: Vec<String>,
    ignore: Vec<String>,
    git_blame: bool,
    cache_path: Option<String>,
    enable_cache: bool,
    cddmignore: Option<PathBuf>,
    ignore_tests: bool,
    ignore_mocks: bool,
    ignore_generated: bool,
    rules: Option<PathBuf>,
    enforce_policies: bool,
    cross_language: bool,
    detect_type3: bool,
    threads: Option<usize>,
) -> ScanConfig {
    ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages,
        ignore_patterns: if ignore.is_empty() {
            ScanConfig::default().ignore_patterns
        } else {
            ignore
        },
        detect_type2: true,
        detect_type3,
        scan_self: true,
        enable_git_blame: git_blame,
        cache_dir: cache_path,
        enable_cache,
        cddmignore_path: cddmignore.map(|p| p.to_string_lossy().to_string()),
        ignore_tests,
        ignore_mocks,
        ignore_generated,
        rules_path: rules.map(|p| p.to_string_lossy().to_string()),
        enforce_policies,
        cross_language,
        threads,
    }
}

/// Spawns a background task to print real-time scan progress to stderr in console mode.
pub fn spawn_console_progress_printer(mut rx: mpsc::Receiver<cddm_core::ScanProgress>) {
    tokio::spawn(async move {
        let mut last_pct = u32::MAX;
        let mut last_phase = cddm_core::ScanPhase::Discovery;

        while let Some(progress) = rx.recv().await {
            let pct = (progress.progress * 100.0) as u32;
            if pct != last_pct || progress.phase != last_phase {
                last_pct = pct;
                last_phase = progress.phase;
                eprintln!("[{}] {}% - {}", progress.phase, pct, progress.message);
            }
        }
    });
}
