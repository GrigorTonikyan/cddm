#![forbid(unsafe_code)]

use super::catalog::get_canonical_algorithms;
use super::types::{OverlapMatch, OverlapScanResult, RecommendedLibrary};
use crate::semantic_graph::cfg::extract_cfgs_from_source;
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

/// Detects language from file extension.
fn detect_language_from_ext(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "go",
        "java" => "java",
        "cs" => "csharp",
        _ => "text",
    }
}

/// Scans workspace files for reimplemented ecosystem library algorithms.
pub fn scan_workspace_overlap(
    workspace_root: &Path,
    threshold: f64,
) -> Result<OverlapScanResult, String> {
    let algorithms = get_canonical_algorithms();
    let mut matches = Vec::new();
    let mut total_files_scanned = 0;
    let mut scanned_functions = 0;

    let walker = WalkBuilder::new(workspace_root)
        .hidden(true)
        .parents(true)
        .git_ignore(true)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e,
            None => continue,
        };

        let lang = detect_language_from_ext(ext);
        if lang == "text" {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        total_files_scanned += 1;
        let rel_path = path
            .strip_prefix(workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let cfgs = extract_cfgs_from_source(&rel_path, &content, lang);
        scanned_functions += cfgs.len();

        let lines: Vec<&str> = content.lines().collect();

        for cfg in &cfgs {
            let start = cfg.line_start.saturating_sub(1);
            let end = cfg.line_end.min(lines.len());
            let fn_snippet = if start < end {
                lines[start..end].join("\n")
            } else {
                String::new()
            };

            let fn_lower = cfg.function_name.to_lowercase();
            let body_lower = fn_snippet.to_lowercase();

            for algo in &algorithms {
                let mut keyword_hits = 0;
                for kw in &algo.canonical_keywords {
                    if fn_lower.contains(kw) || body_lower.contains(kw) {
                        keyword_hits += 1;
                    }
                }

                if keyword_hits == 0 {
                    continue;
                }

                let ratio = (keyword_hits as f64) / (algo.canonical_keywords.len().max(1) as f64);
                let confidence = (ratio * 1.5
                    + if fn_lower.contains(&algo.name.to_lowercase()) {
                        0.4
                    } else {
                        0.1
                    })
                .min(1.0);

                if confidence >= threshold {
                    // Pick best language recommendation or fallback
                    let rec = algo
                        .recommendations
                        .iter()
                        .find(|r| {
                            r.language == lang
                                || (lang == "javascript" && r.language == "typescript")
                        })
                        .cloned()
                        .unwrap_or_else(|| {
                            algo.recommendations.first().cloned().unwrap_or_else(|| {
                                RecommendedLibrary {
                                    language: lang.to_string(),
                                    package_name: "standard-library".to_string(),
                                    install_command: String::new(),
                                    replacement_snippet: String::new(),
                                }
                            })
                        });

                    matches.push(OverlapMatch {
                        algorithm_name: algo.name.clone(),
                        category: algo.category.clone(),
                        file_path: rel_path.clone(),
                        function_name: cfg.function_name.clone(),
                        line_span: (cfg.line_start, cfg.line_end),
                        confidence,
                        snippet: fn_snippet.clone(),
                        recommended_library: rec,
                    });
                }
            }
        }
    }

    // Sort matches by confidence descending
    matches.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let summary = format!(
        "Discovered {} ecosystem library overlap matches across {} files ({} functions analyzed)",
        matches.len(),
        total_files_scanned,
        scanned_functions
    );

    Ok(OverlapScanResult {
        matches,
        total_files_scanned,
        scanned_functions,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_scan_workspace_overlap() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let src = root.join("src");
        fs::create_dir_all(&src).unwrap();

        fs::write(
            src.join("utils.rs"),
            r#"
pub fn chunk_items(items: &[i32], chunk_size: usize) -> Vec<Vec<i32>> {
    let mut batches = Vec::new();
    for x in items.chunks(chunk_size) {
        batches.push(x.to_vec());
    }
    batches
}
"#,
        )
        .unwrap();

        let res = scan_workspace_overlap(root, 0.2).unwrap();
        assert_eq!(res.total_files_scanned, 1);
        assert!(!res.matches.is_empty());
        assert_eq!(res.matches[0].algorithm_name, "Array Chunking");
        assert_eq!(res.matches[0].recommended_library.package_name, "itertools");
    }
}
