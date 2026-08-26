#![forbid(unsafe_code)]

use crate::formatters::print_console_report;
use crate::serve::{DEFAULT_HOST_IP, build_app};
use crate::types::OutputFormat;
use cddm_core::{CddmWatcher, ScanConfig, WatchDeltaReport, run_scan};
use std::net::SocketAddr;
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
    serve_port: Option<u16>,
    open_browser: bool,
    format: OutputFormat,
    cross_language: bool,
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
        cross_language,
    };

    // Optionally spawn embedded Axum WebUI server
    if let Some(port) = serve_port {
        let (state, app) = build_app();
        *state.current_config.write().await = config.clone();

        let addr = SocketAddr::from((DEFAULT_HOST_IP, port));
        let server_url = format!("http://localhost:{}", port);
        println!("CDDM Studio WebUI active at {}", server_url);

        if open_browser {
            let _ = opener::open(&server_url);
        }

        tokio::spawn(async move {
            if let Ok(listener) = tokio::net::TcpListener::bind(addr).await {
                let _ = axum::serve(listener, app).await;
            }
        });
    }

    if format == OutputFormat::Console {
        println!(
            "CDDM Watcher active on '{}' (debounce: {}ms)",
            directory.display(),
            debounce_ms
        );
        println!("Performing initial baseline scan...\n");
    }

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let mut previous_result = match run_scan(config.clone(), tx, cancel_flag).await {
        Ok(res) => {
            if format == OutputFormat::Console {
                print_console_report(&res);
            } else if format == OutputFormat::Json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else if format == OutputFormat::Ndjson {
                println!("{}", serde_json::to_string(&res)?);
            }
            Some(res)
        }
        Err(err) => {
            eprintln!("Initial scan failed: {}", err);
            None
        }
    };

    let watcher = CddmWatcher::watch_directory(&directory)?;
    if format == OutputFormat::Console {
        println!("\nWatching for workspace changes... Press Ctrl+C to exit.\n");
    }

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
                    let delta = WatchDeltaReport::compute(
                        previous_result.as_ref(),
                        &new_res,
                        &changed,
                        duration,
                    );

                    match format {
                        OutputFormat::Console => {
                            let delta_sign = if delta.score_delta > 0.0 {
                                format!("(+{:.1}%)", delta.score_delta)
                            } else if delta.score_delta < 0.0 {
                                format!("({:.1}%)", delta.score_delta)
                            } else {
                                "(+0.0%)".to_string()
                            };

                            println!(
                                "[WATCH] {} file(s) modified | Scanned in {}ms | DRY Health: \
                                 {:.1}% {} | Clones: {} (delta: {:+}) | Clusters: {}",
                                changed.len(),
                                duration,
                                new_res.dry_health_score,
                                delta_sign,
                                new_res.total_clones,
                                delta.clone_count_delta,
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
                        }
                        OutputFormat::Ndjson => {
                            println!("{}", serde_json::to_string(&delta)?);
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&delta)?);
                        }
                        OutputFormat::Markdown => {
                            println!(
                                "| {} | {}ms | {:.1}% | {} | {} |",
                                changed.len(),
                                duration,
                                new_res.dry_health_score,
                                new_res.total_clones,
                                new_res.total_clusters
                            );
                        }
                        OutputFormat::Sarif => {}
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
