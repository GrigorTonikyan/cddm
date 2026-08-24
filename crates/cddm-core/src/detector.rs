use crate::cache::{CACHE_SCHEMA_VERSION, CachedFileEntry, DiskFingerprintCache};
use crate::fingerprint::{Fingerprint, MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::policy::PolicyEngine;
use crate::suppression::SuppressionEngine;
use crate::tokenizer::tokenize;
use crate::types::{
    ClonePair, CloneType, DEFAULT_CACHE_FILE, DEFAULT_RULES_FILE, LanguageStats, MAX_HEALTH_SCORE,
    MIN_HEALTH_SCORE, ScanConfig, ScanPhase, ScanProgress, ScanResult,
};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Instant;
use tokio::sync::mpsc::Sender;

/// Represents an intermediate parsed file
struct ParsedFile {
    path: String,
    language: String,
    token_count: usize,
    token_spans: Vec<crate::types::LineSpan>,
    fingerprints: Vec<Fingerprint>,
}

#[derive(Clone, Debug)]
struct Location {
    file_idx: usize,
    span: crate::types::LineSpan,
}

fn count_tokens_in_line_span(
    spans: &[crate::types::LineSpan],
    start_line: usize,
    end_line: usize,
) -> usize {
    spans
        .iter()
        .filter(|s| s.line_start >= start_line && s.line_end <= end_line)
        .count()
}

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

    let suppression_engine = if let Some(path_str) = &config.cddmignore_path {
        SuppressionEngine::from_file(
            Path::new(path_str),
            config.ignore_tests,
            config.ignore_mocks,
            config.ignore_generated,
        )
        .unwrap_or_else(|_| SuppressionEngine::default_engine())
    } else {
        let root_cddmignore = Path::new(&config.directory).join(".cddmignore");
        if root_cddmignore.exists() {
            SuppressionEngine::from_file(
                &root_cddmignore,
                config.ignore_tests,
                config.ignore_mocks,
                config.ignore_generated,
            )
            .unwrap_or_else(|_| SuppressionEngine::default_engine())
        } else {
            SuppressionEngine::new(crate::types::SuppressionConfig {
                rules: Vec::new(),
                ignore_tests: config.ignore_tests,
                ignore_mocks: config.ignore_mocks,
                ignore_generated: config.ignore_generated,
                raw_cddmignore: None,
            })
            .unwrap_or_else(|_| SuppressionEngine::default_engine())
        }
    };

    let policy_engine = if let Some(path_str) = &config.rules_path {
        PolicyEngine::from_file(Path::new(path_str)).unwrap_or_else(|_| PolicyEngine::empty())
    } else {
        let root_rules = Path::new(&config.directory).join(DEFAULT_RULES_FILE);
        if root_rules.exists() {
            PolicyEngine::from_file(&root_rules).unwrap_or_else(|_| PolicyEngine::empty())
        } else {
            PolicyEngine::empty()
        }
    };

    let walker = WalkBuilder::new(&config.directory);

    let mut files_to_process = Vec::new();
    for result in walker.build() {
        if cancel_flag.load(Ordering::Relaxed) {
            return Err("Scan cancelled".to_string());
        }
        if let Ok(entry) = result
            && entry.path().is_file()
            && let Some(grammar) = get_grammar_for_path(entry.path())
            && (config.languages.is_empty() || config.languages.contains(&grammar.name.to_string()))
        {
            let path_str = entry.path().to_string_lossy().to_string();
            let mut ignored = false;
            for pat in &config.ignore_patterns {
                if path_str.contains(pat) {
                    ignored = true;
                    break;
                }
            }
            if !ignored && !suppression_engine.is_path_ignored(entry.path(), None) {
                files_to_process.push(entry.path().to_path_buf());
            }
        }
    }

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
            let mut index: HashMap<(u64, u64), Vec<Location>> = HashMap::new();
            let mut total_tokens = 0;

            for (file_idx, pf) in parsed_files_clone.iter().enumerate() {
                total_tokens += pf.token_count;
                for fp in &pf.fingerprints {
                    index.entry(fp.hash).or_default().push(Location {
                        file_idx,
                        span: fp.span.clone(),
                    });
                }
            }

            let mut raw_pairs = Vec::new();
            let repo_root = std::path::Path::new(&config_clone.directory);
            let default_author = if config_clone.enable_git_blame {
                crate::blame::get_line_author(repo_root, "", 0)
            } else {
                None
            };

            let k = std::cmp::max(MIN_K_GRAM, config_clone.min_tokens / 2);

            for (hash, locations) in index {
                if locations.len() > 1 {
                    for i in 0..locations.len() {
                        for j in (i + 1)..locations.len() {
                            let loc_a = &locations[i];
                            let loc_b = &locations[j];

                            if loc_a.file_idx == loc_b.file_idx {
                                if !config_clone.scan_self {
                                    continue;
                                }
                                // Skip overlapping spans in the same file to prevent self-cloning
                                let spans_overlap = loc_a.span.line_start <= loc_b.span.line_end
                                    && loc_b.span.line_start <= loc_a.span.line_end;
                                if spans_overlap {
                                    continue;
                                }
                            }

                            let (author_a, author_b) = if config_clone.enable_git_blame {
                                (
                                    default_author.clone().map(|(n, d)| {
                                        format!("{} (line {}, {})", n, loc_a.span.line_start, d)
                                    }),
                                    default_author.clone().map(|(n, d)| {
                                        format!("{} (line {}, {})", n, loc_b.span.line_start, d)
                                    }),
                                )
                            } else {
                                (None, None)
                            };

                            raw_pairs.push(ClonePair {
                                file_a: parsed_files_clone[loc_a.file_idx].path.clone(),
                                start_line_a: loc_a.span.line_start,
                                end_line_a: loc_a.span.line_end,
                                file_b: parsed_files_clone[loc_b.file_idx].path.clone(),
                                start_line_b: loc_b.span.line_start,
                                end_line_b: loc_b.span.line_end,
                                token_count: k,
                                similarity: 1.0,
                                fragment_hash: format!("{:x}-{:x}", hash.0, hash.1),
                                clone_type: CloneType::Exact,
                                author_a,
                                author_b,
                            });
                        }
                    }
                }
            }

            // Merge overlapping and adjacent clone pairs
            raw_pairs.sort_by(|a, b| {
                a.file_a
                    .cmp(&b.file_a)
                    .then(a.file_b.cmp(&b.file_b))
                    .then(a.start_line_a.cmp(&b.start_line_a))
                    .then(a.start_line_b.cmp(&b.start_line_b))
            });

            let mut merged_pairs = Vec::new();
            if !raw_pairs.is_empty() {
                let mut push_pair_if_valid =
                    |mut pair: ClonePair, file_a_idx: usize, file_b_idx: usize| {
                        let count_a = count_tokens_in_line_span(
                            &parsed_files_clone[file_a_idx].token_spans,
                            pair.start_line_a,
                            pair.end_line_a,
                        );
                        let count_b = count_tokens_in_line_span(
                            &parsed_files_clone[file_b_idx].token_spans,
                            pair.start_line_b,
                            pair.end_line_b,
                        );
                        pair.token_count = std::cmp::max(k, std::cmp::min(count_a, count_b));
                        if pair.token_count >= config_clone.min_tokens {
                            merged_pairs.push(pair);
                        }
                    };

                let mut current = raw_pairs[0].clone();
                let mut curr_file_a_idx = parsed_files_clone
                    .iter()
                    .position(|f| f.path == current.file_a)
                    .unwrap_or(0);
                let mut curr_file_b_idx = parsed_files_clone
                    .iter()
                    .position(|f| f.path == current.file_b)
                    .unwrap_or(0);

                for next in raw_pairs.into_iter().skip(1) {
                    let is_same_file = current.file_a == current.file_b;
                    let candidate_end_a = std::cmp::max(current.end_line_a, next.end_line_a);
                    let candidate_end_b = std::cmp::max(current.end_line_b, next.end_line_b);
                    let (first_end, second_start) = if current.start_line_a <= current.start_line_b
                    {
                        (candidate_end_a, current.start_line_b)
                    } else {
                        (candidate_end_b, current.start_line_a)
                    };
                    let would_overlap = is_same_file && (first_end >= second_start);

                    if current.file_a == next.file_a
                        && current.file_b == next.file_b
                        && next.start_line_a <= current.end_line_a + 3
                        && next.start_line_b <= current.end_line_b + 3
                        && !would_overlap
                    {
                        current.end_line_a = candidate_end_a;
                        current.end_line_b = candidate_end_b;
                    } else {
                        push_pair_if_valid(current, curr_file_a_idx, curr_file_b_idx);
                        current = next;
                        curr_file_a_idx = parsed_files_clone
                            .iter()
                            .position(|f| f.path == current.file_a)
                            .unwrap_or(0);
                        curr_file_b_idx = parsed_files_clone
                            .iter()
                            .position(|f| f.path == current.file_b)
                            .unwrap_or(0);
                    }
                }

                push_pair_if_valid(current, curr_file_a_idx, curr_file_b_idx);
            }

            merged_pairs.sort_by(|a, b| {
                a.file_a
                    .cmp(&b.file_a)
                    .then(a.file_b.cmp(&b.file_b))
                    .then(a.start_line_a.cmp(&b.start_line_a))
                    .then(a.start_line_b.cmp(&b.start_line_b))
                    .then(a.end_line_a.cmp(&b.end_line_a))
                    .then(a.end_line_b.cmp(&b.end_line_b))
            });
            merged_pairs.dedup_by(|a, b| {
                a.file_a == b.file_a
                    && a.file_b == b.file_b
                    && a.start_line_a == b.start_line_a
                    && a.end_line_a == b.end_line_a
                    && a.start_line_b == b.start_line_b
                    && a.end_line_b == b.end_line_b
            });

            merged_pairs.sort_by_key(|b| std::cmp::Reverse(b.token_count));

            for pair in &mut merged_pairs {
                let ext_a = std::path::Path::new(&pair.file_a)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                let ext_b = std::path::Path::new(&pair.file_b)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                let snippet_a = crate::refactor::read_file_lines_range(
                    std::path::Path::new(&pair.file_a),
                    pair.start_line_a,
                    pair.end_line_a,
                )
                .ok()
                .map(|lines| lines.join("\n"));
                let snippet_b = crate::refactor::read_file_lines_range(
                    std::path::Path::new(&pair.file_b),
                    pair.start_line_b,
                    pair.end_line_b,
                )
                .ok()
                .map(|lines| lines.join("\n"));

                if let (Some(code_a), Some(code_b)) = (snippet_a, snippet_b) {
                    let (classified_type, sim) =
                        crate::ast::classify_ast_clone(&code_a, ext_a, &code_b, ext_b);
                    pair.clone_type = classified_type;
                    pair.similarity = sim;
                }
            }

            // Filter out clone pairs matching suppression type exclusions or custom thresholds
            merged_pairs.retain(|pair| {
                !suppression_engine_clone
                    .is_clone_type_ignored(Path::new(&pair.file_a), &pair.clone_type)
                    && !suppression_engine_clone
                        .is_clone_type_ignored(Path::new(&pair.file_b), &pair.clone_type)
            });

            merged_pairs.retain(|pair| {
                let eff_a = suppression_engine_clone
                    .get_effective_min_tokens(Path::new(&pair.file_a), config_clone.min_tokens);
                let eff_b = suppression_engine_clone
                    .get_effective_min_tokens(Path::new(&pair.file_b), config_clone.min_tokens);
                let req_min = eff_a.max(eff_b);
                pair.token_count >= req_min
            });

            (merged_pairs, total_tokens)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::mpsc;

    fn make_test_config(directory: &str, min_tokens: usize) -> ScanConfig {
        ScanConfig {
            directory: directory.to_string(),
            min_tokens,
            languages: vec![],
            ignore_patterns: vec![],
            detect_type2: true,
            scan_self: false,
            enable_git_blame: false,
            cache_dir: None,
            enable_cache: false,
            cddmignore_path: None,
            ignore_tests: false,
            ignore_mocks: false,
            ignore_generated: true,
            rules_path: None,
            enforce_policies: false,
        }
    }

    async fn run_test_scan(config: ScanConfig) -> Result<ScanResult, String> {
        let (tx, _rx) = mpsc::channel(100);
        run_scan(config, tx, Arc::new(AtomicBool::new(false))).await
    }

    fn write_test_file(path: impl AsRef<std::path::Path>, content: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, content.as_bytes()).unwrap();
    }

    #[tokio::test]
    async fn test_empty_scan() {
        let result = run_test_scan(make_test_config("non_existent_dir", 50))
            .await
            .unwrap();
        assert_eq!(result.total_files, 0);
    }

    #[tokio::test]
    async fn test_scan_with_real_duplicate_files() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_a_path = dir.path().join("a.rs");
        let file_b_path = dir.path().join("b.rs");

        let content = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                let mut sum = 0;
                for i in 0..10 {
                    sum += a + b + i;
                    println!("intermediate: {}", sum);
                    if sum > 100 {
                        break;
                    }
                }
                sum
            }
        "#;
        let content_ext = format!(
            "{} {} {} {} {} {}",
            content, content, content, content, content, content
        );

        write_test_file(&file_a_path, &content_ext);
        write_test_file(&file_b_path, &content_ext);

        let result = run_test_scan(make_test_config(&dir.path().to_string_lossy(), 50))
            .await
            .unwrap();
        assert!(result.total_clones > 0);
        assert!(!result.clone_pairs.is_empty());
        assert!(result.total_clusters > 0);
        assert!(!result.clone_clusters.is_empty());
        assert_eq!(result.clone_clusters[0].occurrences.len(), 2);
    }

    #[tokio::test]
    async fn test_scan_cancellation() {
        let (tx, _rx) = mpsc::channel(100);
        let cancel_flag = Arc::new(AtomicBool::new(true)); // Pre-cancelled
        let result = run_scan(make_test_config(".", 50), tx, cancel_flag).await;
        assert_eq!(result.unwrap_err(), "Scan cancelled");
    }

    #[tokio::test]
    async fn test_scan_language_filter() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        write_test_file(dir.path().join("test.rs"), "fn main() {}\n");
        write_test_file(dir.path().join("test.py"), "def main(): pass\n");

        let mut config = make_test_config(&dir.path().to_string_lossy(), 50);
        config.languages = vec!["Rust".to_string()];

        let result = run_test_scan(config).await.unwrap();
        assert_eq!(result.total_files, 1);
        assert_eq!(result.language_breakdown.len(), 1);
        assert_eq!(result.language_breakdown[0].language, "Rust");
    }

    #[tokio::test]
    async fn test_scan_ignore_patterns() {
        use std::fs::{self, File};
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("node_modules")).unwrap();
        File::create(dir.path().join("node_modules").join("test.rs")).unwrap();
        File::create(dir.path().join("main.rs")).unwrap();

        let mut config = make_test_config(&dir.path().to_string_lossy(), 50);
        config.ignore_patterns = vec!["node_modules".to_string()];

        let result = run_test_scan(config).await.unwrap();
        assert_eq!(result.total_files, 1);
    }

    #[tokio::test]
    async fn test_dry_health_score_range() {
        let result = run_test_scan(make_test_config(".", 50)).await;
        if let Ok(res) = result {
            assert!(res.dry_health_score >= 0.0 && res.dry_health_score <= 100.0);
        }
    }

    #[tokio::test]
    async fn test_no_self_overlapping_clones() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("single.rs");
        let content = "fn foo() { println!(\"hello world\"); }\n";
        let mut f = File::create(&file_path).unwrap();
        for _ in 0..20 {
            writeln!(f, "{}", content).unwrap();
        }

        let mut config = make_test_config(&dir.path().to_string_lossy(), 20);
        config.scan_self = true;

        let result = run_test_scan(config).await.unwrap();
        for pair in &result.clone_pairs {
            if pair.file_a == pair.file_b {
                let overlaps =
                    pair.start_line_a <= pair.end_line_b && pair.start_line_b <= pair.end_line_a;
                assert!(
                    !overlaps,
                    "Self clone pair should not overlap with itself: {:?}",
                    pair
                );
            }
        }
    }

    #[tokio::test]
    async fn test_scan_with_disk_caching() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let cache_db = dir.path().join("sub").join("test_cache.redb");

        let code =
            "fn compute_heavy_task() -> u64 { let mut v = 0; for i in 0..100 { v += i * 2; } v }\n";
        let code_long = format!("{} {} {} {} {}", code, code, code, code, code);

        write_test_file(dir.path().join("a.rs"), &code_long);
        write_test_file(dir.path().join("b.rs"), &code_long);

        let mut config = make_test_config(&dir.path().to_string_lossy(), 30);
        config.enable_cache = true;
        config.cache_dir = Some(cache_db.to_string_lossy().to_string());

        // First scan (populates cache)
        let res1 = run_test_scan(config.clone()).await.unwrap();
        assert_eq!(res1.total_files, 2);
        assert!(res1.total_clones > 0);

        // Verify cache file was created
        assert!(cache_db.exists());

        // Second scan (uses cache)
        let res2 = run_test_scan(config).await.unwrap();
        assert_eq!(res2.total_files, 2);
        assert_eq!(res2.total_clones, res1.total_clones);
        assert_eq!(res2.duplication_percentage, res1.duplication_percentage);
    }

    #[tokio::test]
    async fn test_exact_and_renamed_clone_classification() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();

        // Exact clone pair
        let exact_code = r#"
            fn calculate_area(width: f64, height: f64) -> f64 {
                let area = width * height;
                println!("Calculated area: {}", area);
                if area > 1000.0 {
                    println!("Warning: large area");
                }
                area
            }
        "#;
        write_test_file(dir.path().join("exact1.rs"), exact_code);
        write_test_file(dir.path().join("exact2.rs"), exact_code);

        // Renamed clone pair
        let renamed_a = r#"
            fn compute_perimeter(side_a: f64, side_b: f64) -> f64 {
                let perimeter = (side_a + side_b) * 2.0;
                println!("Calculated perimeter: {}", perimeter);
                if perimeter > 500.0 {
                    println!("Warning: large boundary");
                }
                perimeter
            }
        "#;
        let renamed_b = r#"
            fn eval_circumference(dim_x: f64, dim_y: f64) -> f64 {
                let total_boundary = (dim_x + dim_y) * 2.0;
                println!("Calculated boundary: {}", total_boundary);
                if total_boundary > 500.0 {
                    println!("Warning: massive border");
                }
                total_boundary
            }
        "#;
        write_test_file(dir.path().join("renamed1.rs"), renamed_a);
        write_test_file(dir.path().join("renamed2.rs"), renamed_b);

        let config = make_test_config(&dir.path().to_string_lossy(), 20);
        let result = run_test_scan(config).await.unwrap();

        assert!(result.total_clones >= 2);
        let exact_found = result.clone_pairs.iter().any(|p| {
            p.clone_type == CloneType::Exact
                && ((p.file_a.contains("exact1") && p.file_b.contains("exact2"))
                    || (p.file_a.contains("exact2") && p.file_b.contains("exact1")))
        });
        assert!(
            exact_found,
            "Exact clone pair should be classified as Exact"
        );
    }

    #[tokio::test]
    async fn test_polyglot_ast_scan() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();

        // Go duplicate files
        let go_code = r#"
            package main
            import "fmt"
            func CalculateMetric(x int, y int) int {
                res := x * y + 42
                fmt.Printf("Result: %d\n", res)
                return res
            }
        "#;
        write_test_file(dir.path().join("metric1.go"), go_code);
        write_test_file(dir.path().join("metric2.go"), go_code);

        // Java duplicate files
        let java_code = r#"
            public class Processor {
                public int computeBonus(int salary, int tenure) {
                    int bonus = salary * tenure / 100;
                    System.out.println("Bonus: " + bonus);
                    return bonus;
                }
            }
        "#;
        write_test_file(dir.path().join("Proc1.java"), java_code);
        write_test_file(dir.path().join("Proc2.java"), java_code);

        // Zig duplicate files
        let zig_code = r#"
            const std = @import("std");
            pub fn computeSum(a: i32, b: i32, c: i32, d: i32) i32 {
                const total = a + b + c + d + 100;
                std.debug.print("Calculated sum: {d}\n", .{total});
                return total * 2;
            }
        "#;
        write_test_file(dir.path().join("a.zig"), zig_code);
        write_test_file(dir.path().join("b.zig"), zig_code);

        // Scala duplicate files
        let scala_code = r#"
            object Helper {
                def processData(input: String, prefix: String, suffix: String): String = {
                    val formatted = prefix + "_" + input.trim.toUpperCase + "_" + suffix
                    println(s"Processing string payload: $formatted")
                    formatted + "_PROCESSED"
                }
            }
        "#;
        write_test_file(dir.path().join("a.scala"), scala_code);
        write_test_file(dir.path().join("b.scala"), scala_code);

        // Elixir duplicate files
        let elixir_code = r#"
            defmodule Calculator do
                def multiply_and_add(a, b, c, d) do
                    sum = a + b + c + d
                    result = sum * 2 + 42
                    IO.puts("Result calculation computed: #{result}")
                    result
                end
            end
        "#;
        write_test_file(dir.path().join("calc1.ex"), elixir_code);
        write_test_file(dir.path().join("calc2.ex"), elixir_code);

        // SQL duplicate files
        let sql_code = r#"
            SELECT u.id, u.username, u.email, COUNT(p.id) as post_count, SUM(p.views) as total_views
            FROM users u
            INNER JOIN posts p ON u.id = p.user_id
            WHERE u.active = 1 AND u.created_at >= '2026-01-01'
            GROUP BY u.id, u.username, u.email
            HAVING post_count > 5 AND total_views > 1000
            ORDER BY post_count DESC, total_views DESC;
        "#;
        write_test_file(dir.path().join("query1.sql"), sql_code);
        write_test_file(dir.path().join("query2.sql"), sql_code);

        let config = make_test_config(&dir.path().to_string_lossy(), 15);
        let result = run_test_scan(config).await.unwrap();

        assert_eq!(result.total_files, 12);
        assert!(result.total_clones >= 6);
        for lang in &["Go", "Java", "Zig", "Scala", "Elixir", "SQL"] {
            assert!(
                result
                    .language_breakdown
                    .iter()
                    .any(|l| &l.language == lang)
            );
        }
    }

    #[tokio::test]
    async fn test_scan_with_policy_engine() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();

        std::fs::create_dir_all(dir.path().join("src/domain")).unwrap();
        std::fs::create_dir_all(dir.path().join("src/presentation")).unwrap();
        std::fs::create_dir_all(dir.path().join("src/auth")).unwrap();

        let code = r#"
            pub fn validate_credentials(user: &str, pass: &str) -> bool {
                let valid = user.len() > 3 && pass.len() > 8;
                valid
            }
        "#;
        write_test_file(dir.path().join("src/domain/user.rs"), code);
        write_test_file(dir.path().join("src/presentation/user.rs"), code);
        write_test_file(dir.path().join("src/auth/auth_helper.rs"), code);

        let rules_toml = r#"
[[boundaries]]
name = "domain-isolation"
source = "src/domain/**"
forbidden_targets = ["src/presentation/**"]
severity = "error"

[[zero_duplication]]
name = "auth-protection"
pattern = "src/auth/**"
severity = "error"
"#;
        write_test_file(dir.path().join(".cddmrules.toml"), rules_toml);

        let config = make_test_config(&dir.path().to_string_lossy(), 10);
        let result = run_test_scan(config).await.unwrap();

        assert!(result.total_clones >= 1);
        assert!(!result.policy_violations.is_empty());
        assert!(
            result
                .policy_violations
                .iter()
                .any(|v| v.rule_name == "domain-isolation")
        );
        assert!(
            result
                .policy_violations
                .iter()
                .any(|v| v.rule_name == "auth-protection")
        );
    }
}
