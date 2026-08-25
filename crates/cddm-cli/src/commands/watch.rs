#![forbid(unsafe_code)]

use crate::formatters::print_console_report;
use cddm_core::{CddmWatcher, ScanConfig, run_scan};
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

#[allow(clippy::too_many_arguments)]
pub async fn run_watch_command(
    directory: PathBuf,
    min_tokens: usize,
    languages: Vec<String>,
    ignore: Vec<String>,
    git_blame: bool,
    cache_dir: Option<PathBuf>,
    no_cache: bool,
    debounce_ms: u64,
    fail_threshold: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = cache_dir.as_ref().map(|p| p.to_string_lossy().to_string());
    let ignore_patterns = if ignore.is_empty() {
        ScanConfig::default().ignore_patterns
    } else {
        ignore
    };

    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages,
        ignore_patterns: ignore_patterns.clone(),
        detect_type2: true,
        scan_self: true,
        enable_git_blame: git_blame,
        cache_dir: cache_path,
        enable_cache: !no_cache,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
    };

    println!(
        "CDDM Watcher active on '{}' (debounce: {}ms)",
        directory.display(),
        debounce_ms
    );
    println!("Performing initial baseline scan...\n");

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let mut previous_result = match run_scan(config.clone(), tx, cancel_flag).await {
        Ok(res) => {
            print_console_report(&res);
            Some(res)
        }
        Err(err) => {
            eprintln!("Initial scan failed: {}", err);
            None
        }
    };

    let watcher = CddmWatcher::watch_directory(&directory)?;
    println!("\nWatching for workspace changes... Press Ctrl+C to exit.\n");

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(debounce_ms));

    loop {
        interval.tick().await;
        let changed = watcher.collect_changed_paths(&ignore_patterns);
        if !changed.is_empty() {
            let (tx_inc, _rx_inc) = mpsc::channel(100);
            let cancel = Arc::new(AtomicBool::new(false));
            let start = std::time::Instant::now();

            match run_scan(config.clone(), tx_inc, cancel).await {
                Ok(new_res) => {
                    let duration = start.elapsed().as_millis();
                    let score_delta = if let Some(ref prev) = previous_result {
                        new_res.dry_health_score - prev.dry_health_score
                    } else {
                        0.0
                    };

                    let delta_str = if score_delta > 0.0 {
                        format!("(+{:.1}%)", score_delta)
                    } else if score_delta < 0.0 {
                        format!("({:.1}%)", score_delta)
                    } else {
                        "(+0.0%)".to_string()
                    };

                    println!(
                        "[WATCH] {} file(s) modified | Scanned in {}ms | DRY Health: {:.1}% {} | \
                         Clones: {} | Clusters: {}",
                        changed.len(),
                        duration,
                        new_res.dry_health_score,
                        delta_str,
                        new_res.total_clones,
                        new_res.total_clusters
                    );

                    if let Some(threshold) = fail_threshold
                        && new_res.duplication_percentage > threshold
                    {
                        eprintln!(
                            "[WARN] Duplication {:.1}% exceeds failure threshold {:.1}%",
                            new_res.duplication_percentage, threshold
                        );
                    }

                    previous_result = Some(new_res);
                }
                Err(err) => {
                    eprintln!("[WATCH ERROR] Incremental scan failed: {}", err);
                }
            }
        }
    }
}
