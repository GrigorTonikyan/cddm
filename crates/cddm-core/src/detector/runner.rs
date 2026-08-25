#![forbid(unsafe_code)]

use super::discovery::{discover_candidate_files, init_policy_engine, init_suppression_engine};
use super::indexer::index_and_match_clone_pairs;
use super::types::ParsedFile;
use crate::cache::{CACHE_SCHEMA_VERSION, CachedFileEntry, DiskFingerprintCache};
use crate::fingerprint::{MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{
    DEFAULT_CACHE_FILE, LanguageStats, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, ScanConfig, ScanPhase,
    ScanProgress, ScanResult,
};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tokio::sync::mpsc::Sender;

/// Runs the complete code clone detection process.
pub async fn run_scan(
    config: ScanConfig,
    progress_tx: Sender<ScanProgress>,
    cancel_flag: Arc<AtomicBool>,
) -> Result<ScanResult, String> {
    let start_time = Instant::now();
    let scan_id = uuid::Uuid::new_v4().to_string();

    let _ = progress_tx
        .send(ScanProgress {
            scan_id: scan_id.clone(),
            phase: ScanPhase::Discovery,
            files_processed: 0,
            total_files: 0,
            progress: 0.0,
            message: "Discovering files...".to_string(),
        })
        .await;

    let suppression_engine = init_suppression_engine(&config);
    let policy_engine = init_policy_engine(&config);

    let files_to_process = discover_candidate_files(&config, &suppression_engine, &cancel_flag)?;
    let total_files = files_to_process.len();

    if total_files == 0 {
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

    let _ = progress_tx
        .send(ScanProgress {
            scan_id: scan_id.clone(),
            phase: ScanPhase::Tokenization,
            files_processed: 0,
            total_files,
            progress: 0.1,
            message: format!("Tokenizing {} files...", total_files),
        })
        .await;

    if cancel_flag.load(Ordering::Relaxed) {
        return Err("Scan cancelled".to_string());
    }

    let disk_cache = if config.enable_cache {
        let cache_path = config
            .cache_dir
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| Path::new(&config.directory).join(DEFAULT_CACHE_FILE));
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

        tokio::task::spawn_blocking(move || {
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

                    if let Some(entry) = disk_cache_clone.is_file_valid(&path_str, mtime, size) {
                        cached_parsed.push(ParsedFile {
                            path: path_str,
                            language: entry.language,
                            token_count: entry.token_count,
                            token_spans: entry.token_spans,
                            fingerprints: entry.fingerprints,
                        });
                        continue;
                    }
                }
                files_to_tokenize.push(path);
            }

            let newly_parsed_and_entries: Vec<(ParsedFile, CachedFileEntry)> = files_to_tokenize
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
                    let content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

                    let tokens = tokenize(&content, grammar, config_clone.detect_type2);
                    let token_count = tokens.len();
                    let token_spans: Vec<_> = tokens.iter().map(|(_, span)| span.clone()).collect();

                    let k = std::cmp::max(MIN_K_GRAM, config_clone.min_tokens / 2);
                    let w = k + WINDOW_OFFSET;
                    let fingerprints = winnow(&tokens, k, w);

                    let path_str = path.to_string_lossy().to_string();
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
        .await
        .unwrap()
    };

    let _ = progress_tx
        .send(ScanProgress {
            scan_id: scan_id.clone(),
            phase: ScanPhase::AstAnalysis,
            files_processed: total_files,
            total_files,
            progress: 0.35,
            message: "Analyzing AST subtrees & structural patterns...".to_string(),
        })
        .await;

    if cancel_flag.load(Ordering::Relaxed) {
        return Err("Scan cancelled".to_string());
    }

    let _ = progress_tx
        .send(ScanProgress {
            scan_id: scan_id.clone(),
            phase: ScanPhase::Indexing,
            files_processed: total_files,
            total_files,
            progress: 0.5,
            message: "Indexing fingerprints...".to_string(),
        })
        .await;

    if cancel_flag.load(Ordering::Relaxed) {
        return Err("Scan cancelled".to_string());
    }

    let (merged_pairs, total_tokens) = {
        let config_clone = config.clone();
        let parsed_files_clone = Arc::clone(&parsed_files);
        let suppression_engine_clone = suppression_engine.clone();

        tokio::task::spawn_blocking(move || {
            index_and_match_clone_pairs(
                &parsed_files_clone,
                &config_clone,
                &suppression_engine_clone,
            )
        })
        .await
        .unwrap()
    };

    let mut lang_stats_map: HashMap<String, LanguageStats> = HashMap::new();
    for pf in parsed_files.iter() {
        let stats = lang_stats_map
            .entry(pf.language.clone())
            .or_insert(LanguageStats {
                language: pf.language.clone(),
                files: 0,
                tokens: 0,
                clones: 0,
            });
        stats.files += 1;
        stats.tokens += pf.token_count;
    }

    let mut cross_module_count = 0;
    for pair in &merged_pairs {
        let norm_a = pair.file_a.replace('\\', "/");
        let norm_b = pair.file_b.replace('\\', "/");
        let parent_a = Path::new(&norm_a).parent().unwrap_or(Path::new(""));
        let parent_b = Path::new(&norm_b).parent().unwrap_or(Path::new(""));
        if parent_a != parent_b {
            cross_module_count += 1;
        }
    }

    let language_breakdown: Vec<LanguageStats> = lang_stats_map.into_values().collect();
    let total_duplicated_tokens: usize = merged_pairs.iter().map(|p| p.token_count).sum();
    let duplication_percentage = if total_tokens > 0 {
        ((total_duplicated_tokens as f64) / (total_tokens as f64) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let cross_module_ratio = if !merged_pairs.is_empty() {
        cross_module_count as f64 / merged_pairs.len() as f64
    } else {
        0.0
    };
    let duplication_weight = 1.5 * (1.0 + 0.3 * cross_module_ratio);
    let dry_health_score = (MAX_HEALTH_SCORE - duplication_percentage * duplication_weight)
        .clamp(MIN_HEALTH_SCORE, MAX_HEALTH_SCORE);

    let _ = progress_tx
        .send(ScanProgress {
            scan_id: scan_id.clone(),
            phase: ScanPhase::Complete,
            files_processed: total_files,
            total_files,
            progress: 1.0,
            message: "Scan complete.".to_string(),
        })
        .await;

    let clone_clusters = crate::cluster::cluster_clone_pairs(&merged_pairs);
    let total_clusters = clone_clusters.len();

    let mut scan_result = ScanResult {
        scan_id,
        total_files,
        total_tokens,
        total_clones: merged_pairs.len(),
        total_clusters,
        duplication_percentage,
        dry_health_score,
        clone_pairs: merged_pairs,
        clone_clusters,
        duration_ms: start_time.elapsed().as_millis() as u64,
        language_breakdown,
        policy_violations: Vec::new(),
    };

    let policy_eval = policy_engine.evaluate(&scan_result);
    scan_result.policy_violations = policy_eval.violations;

    Ok(scan_result)
}
