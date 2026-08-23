use crate::fingerprint::{Fingerprint, MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{
    ClonePair, CloneType, LanguageStats, MAX_HEALTH_SCORE, MIN_HEALTH_SCORE, ScanConfig, ScanPhase,
    ScanProgress, ScanResult,
};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
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
            if !ignored {
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
            duplication_percentage: 0.0,
            dry_health_score: 100.0,
            clone_pairs: vec![],
            duration_ms: start_time.elapsed().as_millis() as u64,
            language_breakdown: vec![],
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

    let parsed_files: Arc<Vec<ParsedFile>> = {
        let config_clone = config.clone();
        tokio::task::spawn_blocking(move || {
            let files: Vec<ParsedFile> = files_to_process
                .par_iter()
                .filter_map(|path| {
                    let grammar = get_grammar_for_path(path)?;
                    let content = std::fs::read_to_string(path).ok()?;
                    let tokens = tokenize(&content, grammar, config_clone.detect_type2);
                    let token_count = tokens.len();
                    let token_spans: Vec<_> = tokens.iter().map(|(_, span)| span.clone()).collect();

                    let k = std::cmp::max(MIN_K_GRAM, config_clone.min_tokens / 2);
                    let w = k + WINDOW_OFFSET;
                    let fingerprints = winnow(&tokens, k, w);

                    Some(ParsedFile {
                        path: path.to_string_lossy().to_string(),
                        language: grammar.name.to_string(),
                        token_count,
                        token_spans,
                        fingerprints,
                    })
                })
                .collect();
            Arc::new(files)
        })
        .await
        .unwrap()
    };

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

    Ok(ScanResult {
        scan_id,
        total_files,
        total_tokens,
        total_clones: merged_pairs.len(),
        duplication_percentage,
        dry_health_score,
        clone_pairs: merged_pairs,
        duration_ms: start_time.elapsed().as_millis() as u64,
        language_breakdown,
    })
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
        }
    }

    async fn run_test_scan(config: ScanConfig) -> Result<ScanResult, String> {
        let (tx, _rx) = mpsc::channel(100);
        run_scan(config, tx, Arc::new(AtomicBool::new(false))).await
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
        use std::fs::File;
        use std::io::Write;
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

        let mut file_a = File::create(&file_a_path).unwrap();
        writeln!(file_a, "{}", content_ext).unwrap();

        let mut file_b = File::create(&file_b_path).unwrap();
        writeln!(file_b, "{}", content_ext).unwrap();

        let result = run_test_scan(make_test_config(&dir.path().to_string_lossy(), 50))
            .await
            .unwrap();
        assert!(result.total_clones > 0);
        assert!(!result.clone_pairs.is_empty());
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
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let mut file_rs = File::create(dir.path().join("test.rs")).unwrap();
        writeln!(file_rs, "fn main() {{}}").unwrap();
        let mut file_py = File::create(dir.path().join("test.py")).unwrap();
        writeln!(file_py, "def main(): pass").unwrap();

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
}
