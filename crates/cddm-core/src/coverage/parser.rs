#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::types::{CoverageFormat, CoverageReport};

/// Automatically detect format and parse coverage file from disk.
pub fn load_coverage_report(path: &Path, format: CoverageFormat) -> Result<CoverageReport, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read coverage report '{}': {}", path.display(), e))?;
    parse_coverage_data(&content, format)
}

/// Parse coverage content given an explicit or auto-detected format.
pub fn parse_coverage_data(
    content: &str,
    format: CoverageFormat,
) -> Result<CoverageReport, String> {
    let target_format = match format {
        CoverageFormat::Auto => detect_coverage_format(content),
        f => f,
    };

    match target_format {
        CoverageFormat::Lcov | CoverageFormat::Auto => parse_lcov(content),
        CoverageFormat::Cobertura => parse_cobertura(content),
        CoverageFormat::Istanbul => parse_istanbul_json(content),
    }
}

/// Heuristically detect coverage format from content header.
fn detect_coverage_format(content: &str) -> CoverageFormat {
    let trimmed = content.trim_start();
    if trimmed.starts_with("<?xml") || trimmed.starts_with("<coverage") {
        CoverageFormat::Cobertura
    } else if trimmed.starts_with('{') {
        CoverageFormat::Istanbul
    } else {
        CoverageFormat::Lcov
    }
}

/// Parse standard LCOV (info) tracefile format.
pub fn parse_lcov(content: &str) -> Result<CoverageReport, String> {
    let mut file_line_hits: HashMap<String, HashMap<usize, u64>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut total_hits: u64 = 0;
    let mut total_lines: usize = 0;

    for line in content.lines() {
        let line = line.trim();
        if let Some(path) = line.strip_prefix("SF:") {
            let norm_path = normalize_path(path);
            current_file = Some(norm_path);
        } else if let Some(da) = line.strip_prefix("DA:") {
            if let Some(ref file_path) = current_file {
                let mut parts = da.split(',');
                if let (Some(line_str), Some(hits_str)) = (parts.next(), parts.next()) {
                    let parsed_line = line_str.parse::<usize>().ok();
                    let parsed_hits = hits_str.parse::<u64>().ok();
                    if let (Some(line_num), Some(hits)) = (parsed_line, parsed_hits) {
                        let entry = file_line_hits.entry(file_path.clone()).or_default();
                        entry.insert(line_num, hits);
                        total_hits += hits;
                        total_lines += 1;
                    }
                }
            }
        } else if line == "end_of_record" {
            current_file = None;
        }
    }

    Ok(CoverageReport {
        file_line_hits,
        total_hits,
        total_instrumented_lines: total_lines,
    })
}

/// Parse Cobertura XML coverage format.
pub fn parse_cobertura(content: &str) -> Result<CoverageReport, String> {
    let mut file_line_hits: HashMap<String, HashMap<usize, u64>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut total_hits: u64 = 0;
    let mut total_lines: usize = 0;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.contains("<class ") && trimmed.contains("filename=\"") {
            if let Some(start) = trimmed.find("filename=\"") {
                let rest = &trimmed[start + 10..];
                if let Some(end) = rest.find('"') {
                    let path = &rest[..end];
                    current_file = Some(normalize_path(path));
                }
            }
        } else if trimmed.contains("<line ")
            && trimmed.contains("number=\"")
            && trimmed.contains("hits=\"")
        {
            let line_num = extract_attr_usize(trimmed, "number=\"");
            let hits = extract_attr_u64(trimmed, "hits=\"");
            if let (Some(file_path), Some(l), Some(h)) = (&current_file, line_num, hits) {
                let entry = file_line_hits.entry(file_path.clone()).or_default();
                entry.insert(l, h);
                total_hits += h;
                total_lines += 1;
            }
        }
    }

    Ok(CoverageReport {
        file_line_hits,
        total_hits,
        total_instrumented_lines: total_lines,
    })
}

fn extract_attr_usize(tag: &str, attr_prefix: &str) -> Option<usize> {
    let start = tag.find(attr_prefix)?;
    let rest = &tag[start + attr_prefix.len()..];
    let end = rest.find('"')?;
    rest[..end].parse::<usize>().ok()
}

fn extract_attr_u64(tag: &str, attr_prefix: &str) -> Option<u64> {
    let start = tag.find(attr_prefix)?;
    let rest = &tag[start + attr_prefix.len()..];
    let end = rest.find('"')?;
    rest[..end].parse::<u64>().ok()
}

/// Parse Istanbul JSON coverage output format.
pub fn parse_istanbul_json(content: &str) -> Result<CoverageReport, String> {
    let val: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| format!("Invalid Istanbul JSON coverage: {}", e))?;

    let mut file_line_hits: HashMap<String, HashMap<usize, u64>> = HashMap::new();
    let mut total_hits: u64 = 0;
    let mut total_lines: usize = 0;

    if let Some(obj) = val.as_object() {
        for (raw_path, file_val) in obj {
            let norm_path = normalize_path(raw_path);
            let mut line_hits: HashMap<usize, u64> = HashMap::new();

            // Istanbul format stores statementMap + s (statement hits)
            if let (Some(stmt_map), Some(s_hits)) = (
                file_val.get("statementMap").and_then(|v| v.as_object()),
                file_val.get("s").and_then(|v| v.as_object()),
            ) {
                for (stmt_id, loc) in stmt_map {
                    if let Some(start_line) = loc
                        .get("start")
                        .and_then(|st| st.get("line"))
                        .and_then(|l| l.as_u64())
                    {
                        let hits = s_hits.get(stmt_id).and_then(|h| h.as_u64()).unwrap_or(0);
                        let l_usize = start_line as usize;
                        let cur = line_hits.entry(l_usize).or_insert(0);
                        *cur += hits;
                        total_hits += hits;
                        total_lines += 1;
                    }
                }
            }

            file_line_hits.insert(norm_path, line_hits);
        }
    }

    Ok(CoverageReport {
        file_line_hits,
        total_hits,
        total_instrumented_lines: total_lines,
    })
}

/// Normalize path for cross-platform and relative matching.
pub fn normalize_path(path: &str) -> String {
    let replaced = path.replace('\\', "/");
    let trimmed = replaced.trim_start_matches("./");
    trimmed.to_string()
}
