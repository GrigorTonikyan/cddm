#![forbid(unsafe_code)]

use super::discovery::{discover_candidate_files, init_policy_engine, init_suppression_engine};
use super::indexer::index_and_match_clone_pairs;
use super::types::ParsedFile;
use crate::cache::{
    CACHE_SCHEMA_VERSION, CachedFileEntry, DiskFingerprintCache, resolve_default_cache_path,
};
use crate::fingerprint::{MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{ScanConfig, ScanPhase, ScanProgress, ScanResult};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::sync::mpsc::Sender;

struct ProgressTracker {
    scan_id: String,
    phase: RwLock<ScanPhase>,
    files_processed: AtomicUsize,
    total_files: AtomicUsize,
    progress_scaled: AtomicU64,
    message: RwLock<String>,
    done: AtomicBool,
}

fn execute_in_thread_pool<F, R>(threads: Option<usize>, f: F) -> R
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    if let Some(num_threads) = threads.filter(|&n| n > 0)
        && let Ok(pool) = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
    {
        return pool.install(f);
    }
    f()
}

/// Runs the complete code clone detection process with real-time granular progress updates.
pub async fn run_scan(
    config: ScanConfig,
    progress_tx: Sender<ScanProgress>,
    cancel_flag: Arc<AtomicBool>,
) -> Result<ScanResult, String> {
    let start_time = Instant::now();
    let scan_id = uuid::Uuid::new_v4().to_string();

    let tracker = Arc::new(ProgressTracker {
        scan_id: scan_id.clone(),
        phase: RwLock::new(ScanPhase::Discovery),
        files_processed: AtomicUsize::new(0),
        total_files: AtomicUsize::new(0),
        progress_scaled: AtomicU64::new(0),
        message: RwLock::new("Discovering files...".to_string()),
        done: AtomicBool::new(false),
    });

    tracing::info!(
        scan_id = %scan_id,
        directory = %config.directory,
        min_tokens = config.min_tokens,
        "Starting CDDM polyglot code deduplication scan..."
    );

    let suppression_engine = init_suppression_engine(&config);
    let policy_engine = init_policy_engine(&config);

    let files_to_process = discover_candidate_files(&config, &suppression_engine, &cancel_flag)?;
    let total_files = files_to_process.len();
    tracker.total_files.store(total_files, Ordering::Relaxed);
    tracing::debug!(
        total_files = total_files,
        "Discovered candidate files for analysis"
    );

    if total_files == 0 {
        let _ = progress_tx
            .send(ScanProgress {
                scan_id: scan_id.clone(),
                phase: ScanPhase::Complete,
                files_processed: 0,
                total_files: 0,
                progress: 1.0,
                message: "No files found to scan.".to_string(),
            })
            .await;

        return Ok(ScanResult {
            scan_id,
            total_files: 0,
            total_tokens: 0,
            total_clones: 0,
            total_clusters: 0,
            duplication_percentage: 0.0,
            dry_health_score: 100.0,
            clone_pairs: vec![],
            clone_clusters: vec![],
            duration_ms: start_time.elapsed().as_millis() as u64,
            language_breakdown: vec![],
            policy_violations: vec![],
        });
    }

    // Spawn background monitor emitting smooth progress updates every 25ms
    let monitor_tx = progress_tx.clone();
    let monitor_tracker = Arc::clone(&tracker);
    let monitor_handle = tokio::spawn(async move {
        let mut last_progress = -1.0;
        let mut last_files = usize::MAX;
        let mut last_msg = String::new();

        while !monitor_tracker.done.load(Ordering::Relaxed) {
            tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

            let cur_files = monitor_tracker.files_processed.load(Ordering::Relaxed);
            let cur_total = monitor_tracker.total_files.load(Ordering::Relaxed);
            let cur_progress =
                monitor_tracker.progress_scaled.load(Ordering::Relaxed) as f64 / 10000.0;
            let cur_phase = *monitor_tracker.phase.read().unwrap();
            let cur_msg = monitor_tracker.message.read().unwrap().clone();

            if (cur_progress - last_progress).abs() > 0.0002
                || cur_files != last_files
                || cur_msg != last_msg
            {
                last_progress = cur_progress;
                last_files = cur_files;
                last_msg = cur_msg.clone();

                let _ = monitor_tx
                    .send(ScanProgress {
                        scan_id: monitor_tracker.scan_id.clone(),
                        phase: cur_phase,
                        files_processed: cur_files,
                        total_files: cur_total,
                        progress: cur_progress.clamp(0.0, 1.0),
                        message: cur_msg,
                    })
                    .await;
            }
        }
    });

    // Start Tokenization phase
    {
        *tracker.phase.write().unwrap() = ScanPhase::Tokenization;
        *tracker.message.write().unwrap() = format!("Tokenizing {} files...", total_files);
        tracker.progress_scaled.store(500, Ordering::Relaxed);
    }

    if cancel_flag.load(Ordering::Relaxed) {
        tracker.done.store(true, Ordering::Relaxed);
        let _ = monitor_handle.await;
        return Err("Scan cancelled".to_string());
    }

    let disk_cache = if config.enable_cache {
        let cache_path = config
            .cache_dir
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| resolve_default_cache_path(Path::new(&config.directory)));
        DiskFingerprintCache::open_or_create(&cache_path).unwrap_or_else(|err| {
            tracing::warn!(
                "Failed to initialize disk cache: {}; continuing in memory",
                err
            );
            DiskFingerprintCache::disabled()
        })
    } else {
        DiskFingerprintCache::disabled()
    };

    let parsed_files: Arc<Vec<ParsedFile>> = {
        let config_clone = config.clone();
        let disk_cache_clone = disk_cache.clone();
        let tracker_clone = Arc::clone(&tracker);
        let total_files_f64 = total_files as f64;

        tokio::task::spawn_blocking(move || {
            execute_in_thread_pool(config_clone.threads, || {
                let mut cached_parsed = Vec::new();
                let mut files_to_tokenize = Vec::new();

                for path in files_to_process {
                    let path_str = path.to_string_lossy().to_string();
                    if let Ok(meta) = std::fs::metadata(&path) {
                        let mtime = meta
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0);
                        let size = meta.len();

                        if let Some(entry) = disk_cache_clone.is_file_valid(&path_str, mtime, size)
                        {
                            cached_parsed.push(ParsedFile {
                                path: path_str,
                                language: entry.language,
                                token_count: entry.token_count,
                                token_spans: entry.token_spans,
                                fingerprints: entry.fingerprints,
                            });
                            let cur = tracker_clone
                                .files_processed
                                .fetch_add(1, Ordering::Relaxed)
                                + 1;
                            let p = 0.05 + 0.35 * (cur as f64 / total_files_f64);
                            tracker_clone
                                .progress_scaled
                                .store((p * 10000.0) as u64, Ordering::Relaxed);
                            continue;
                        }
                    }
                    files_to_tokenize.push(path);
                }

                let newly_parsed_and_entries: Vec<(ParsedFile, CachedFileEntry)> =
                    files_to_tokenize
                        .par_iter()
                        .filter_map(|path| {
                            let grammar = get_grammar_for_path(path)?;
                            let content = crate::io::read_file_source(path).ok()?;
                            let meta = std::fs::metadata(path).ok()?;
                            let mtime = meta
                                .modified()
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs() as i64)
                                .unwrap_or(0);
                            let file_size = meta.len();
                            let content_hash =
                                blake3::hash(content.as_bytes()).to_hex().to_string();

                            let path_str = path.to_string_lossy().to_string();
                            let mut tokens = tokenize(&content, grammar, config_clone.detect_type2);
                            let directives =
                                crate::suppression::parse_inline_directives(&path_str, &content);
                            if !directives.is_empty() {
                                tokens.retain(|(_, span)| {
                                    !directives.iter().any(|d| {
                                        span.line_start <= d.end_line
                                            && span.line_end >= d.start_line
                                    })
                                });
                            }
                            let token_count = tokens.len();
                            let token_spans: Vec<_> =
                                tokens.iter().map(|(_, span)| span.clone()).collect();

                            let k = std::cmp::max(MIN_K_GRAM, config_clone.min_tokens / 2);
                            let w = k + WINDOW_OFFSET;
                            let fingerprints = winnow(&tokens, k, w);

                            let parsed = ParsedFile {
                                path: path_str.clone(),
                                language: grammar.name.to_string(),
                                token_count,
                                token_spans: token_spans.clone(),
                                fingerprints: fingerprints.clone(),
                            };

                            let entry = CachedFileEntry {
                                schema_version: CACHE_SCHEMA_VERSION,
                                content_hash,
                                mtime_secs: mtime,
                                file_size,
                                language: grammar.name.to_string(),
                                token_count,
                                token_spans,
                                fingerprints,
                            };

                            let cur = tracker_clone
                                .files_processed
                                .fetch_add(1, Ordering::Relaxed)
                                + 1;
                            let p = 0.05 + 0.35 * (cur as f64 / total_files_f64);
                            tracker_clone
                                .progress_scaled
                                .store((p * 10000.0) as u64, Ordering::Relaxed);

                            Some((parsed, entry))
                        })
                        .collect();

                if disk_cache_clone.is_enabled() && !newly_parsed_and_entries.is_empty() {
                    let batch: Vec<(String, CachedFileEntry)> = newly_parsed_and_entries
                        .iter()
                        .map(|(p, e)| (p.path.clone(), e.clone()))
                        .collect();
                    let _ = disk_cache_clone.batch_save_entries(&batch);
                }

                let mut all_files = cached_parsed;
                for (p, _) in newly_parsed_and_entries {
                    all_files.push(p);
                }

                Arc::new(all_files)
            })
        })
        .await
        .unwrap()
    };

    if cancel_flag.load(Ordering::Relaxed) {
        tracker.done.store(true, Ordering::Relaxed);
        let _ = monitor_handle.await;
        return Err("Scan cancelled".to_string());
    }

    // Phase: AST Analysis
    {
        *tracker.phase.write().unwrap() = ScanPhase::AstAnalysis;
        *tracker.message.write().unwrap() =
            "Analyzing AST subtrees & structural patterns...".to_string();
        tracker.progress_scaled.store(4500, Ordering::Relaxed);
    }

    // Phase: Indexing
    {
        *tracker.phase.write().unwrap() = ScanPhase::Indexing;
        *tracker.message.write().unwrap() =
            "Indexing fingerprints & matching clones...".to_string();
        tracker.progress_scaled.store(5500, Ordering::Relaxed);
    }

    let (merged_pairs, total_tokens) = {
        let config_clone = config.clone();
        let parsed_files_clone = Arc::clone(&parsed_files);
        let suppression_engine_clone = suppression_engine.clone();
        let tracker_clone = Arc::clone(&tracker);

        tokio::task::spawn_blocking(move || {
            execute_in_thread_pool(config_clone.threads, || {
                let (mut pairs, total) = index_and_match_clone_pairs(
                    &parsed_files_clone,
                    &config_clone,
                    &suppression_engine_clone,
                );

                if config_clone.detect_type4 || config_clone.cross_language {
                    if let Ok(mut msg_guard) = tracker_clone.message.write() {
                        *msg_guard = "Extracting semantic control flow graphs...".to_string();
                    }
                    tracker_clone.progress_scaled.store(6500, Ordering::Relaxed);

                    let tracker_cb = Arc::clone(&tracker_clone);
                    super::semantic::evaluate_and_merge_semantic_clones(
                        &mut pairs,
                        &parsed_files_clone,
                        &config_clone,
                        &suppression_engine_clone,
                        Some(
                            move |evaluated: usize, total_candidates: usize, msg: &str| {
                                let p = 0.65
                                    + 0.25 * (evaluated as f64 / total_candidates.max(1) as f64);
                                tracker_cb
                                    .progress_scaled
                                    .store((p * 10000.0) as u64, Ordering::Relaxed);
                                if let Ok(mut msg_guard) = tracker_cb.message.write() {
                                    *msg_guard = format!(
                                        "{} ({}/{} evaluated)...",
                                        msg.trim_end_matches("..."),
                                        evaluated,
                                        total_candidates
                                    );
                                }
                            },
                        ),
                    );
                }

                (pairs, total)
            })
        })
        .await
        .unwrap()
    };

    // Phase: Scoring
    {
        *tracker.phase.write().unwrap() = ScanPhase::Scoring;
        *tracker.message.write().unwrap() =
            "Calculating DRY health score & policy evaluation...".to_string();
        tracker.progress_scaled.store(9500, Ordering::Relaxed);
    }

    let metrics = super::scoring::compute_scan_scoring(&parsed_files, &merged_pairs, total_tokens);
    let total_clusters = metrics.clone_clusters.len();

    let mut scan_result = ScanResult {
        scan_id: scan_id.clone(),
        total_files,
        total_tokens,
        total_clones: merged_pairs.len(),
        total_clusters,
        duplication_percentage: metrics.duplication_percentage,
        dry_health_score: metrics.dry_health_score,
        clone_pairs: merged_pairs,
        clone_clusters: metrics.clone_clusters,
        duration_ms: start_time.elapsed().as_millis() as u64,
        language_breakdown: metrics.language_breakdown,
        policy_violations: Vec::new(),
    };

    let policy_eval = policy_engine.evaluate(&scan_result);
    scan_result.policy_violations = policy_eval.violations;

    // Conclude scan progress
    tracker.done.store(true, Ordering::Relaxed);
    let _ = monitor_handle.await;

    let _ = progress_tx
        .send(ScanProgress {
            scan_id,
            phase: ScanPhase::Complete,
            files_processed: total_files,
            total_files,
            progress: 1.0,
            message: "Scan complete.".to_string(),
        })
        .await;

    Ok(scan_result)
}
