#![forbid(unsafe_code)]

use crate::types::{CloneType, SuppressionRule};

/// Parses raw `.cddmignore` text into a list of `SuppressionRule` records.
pub fn parse_cddmignore_content(content: &str) -> Vec<SuppressionRule> {
    let mut rules = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let line_number = idx + 1;

        // Check for [threshold] pattern min_tokens=N
        if let Some(rest) = trimmed.strip_prefix("[threshold]") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                let pattern = parts[0].to_string();
                let min_tokens = parts[1]
                    .strip_prefix("min_tokens=")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(50);

                rules.push(SuppressionRule {
                    pattern,
                    rule_type: "threshold".to_string(),
                    min_tokens: Some(min_tokens),
                    ignored_clone_types: Vec::new(),
                    line_number,
                });
                continue;
            }
        }

        // Check for [type-filter] pattern ignore=Exact,Renamed
        if let Some(rest) = trimmed.strip_prefix("[type-filter]") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                let pattern = parts[0].to_string();
                let types_str = parts[1].strip_prefix("ignore=").unwrap_or("");
                let types = parse_clone_types(types_str);

                rules.push(SuppressionRule {
                    pattern,
                    rule_type: "type_filter".to_string(),
                    min_tokens: None,
                    ignored_clone_types: types,
                    line_number,
                });
                continue;
            }
        }

        // Standard ignore pattern
        rules.push(SuppressionRule {
            pattern: trimmed.to_string(),
            rule_type: "ignore".to_string(),
            min_tokens: None,
            ignored_clone_types: Vec::new(),
            line_number,
        });
    }

    rules
}

pub fn parse_clone_types(types_str: &str) -> Vec<CloneType> {
    types_str
        .split(',')
        .filter_map(|s| match s.trim().to_lowercase().as_str() {
            "exact" | "type1" => Some(CloneType::Exact),
            "renamed" | "type2" => Some(CloneType::Renamed),
            "nearmiss" | "type3" => Some(CloneType::NearMiss),
            "semantic" | "type4" => Some(CloneType::Semantic),
            _ => None,
        })
        .collect()
}

pub fn format_rules_to_raw(rules: &[SuppressionRule]) -> String {
    let mut lines = Vec::new();
    lines.push("# CDDM Code De-Duplication Suppression Rules (.cddmignore)".to_string());
    for r in rules {
        if r.rule_type == "ignore" {
            lines.push(r.pattern.clone());
        } else if r.rule_type == "threshold" {
            lines.push(format!(
                "[threshold] {} min_tokens={}",
                r.pattern,
                r.min_tokens.unwrap_or(50)
            ));
        } else if r.rule_type == "type_filter" {
            let type_names: Vec<&str> = r
                .ignored_clone_types
                .iter()
                .map(|t| match t {
                    CloneType::Exact => "Exact",
                    CloneType::Renamed => "Renamed",
                    CloneType::NearMiss => "NearMiss",
                    CloneType::Semantic => "Semantic",
                })
                .collect();
            lines.push(format!(
                "[type-filter] {} ignore={}",
                r.pattern,
                type_names.join(",")
            ));
        }
    }
    lines.join("\n")
}

/// Generates a turnkey standard `.cddmignore` template file content.
pub fn generate_default_cddmignore() -> String {
    r#"# CDDM Code De-Duplication Meister — Suppression Rules (.cddmignore)
# Global path ignore patterns (globs):
node_modules/**
target/**
dist/**
build/**
.git/**
.logs/**
.cddm/**
.vite-hooks/**

# Generated code files:
*.generated.*
*.pb.go
*_pb2.py
*.g.dart

# Custom min-token thresholds for specific directories:
# [threshold] tests/** min_tokens=100
# [threshold] fixtures/** min_tokens=80

# Exclude specific clone types from detection:
# [type-filter] src/mocks/** ignore=Exact,Renamed
"#
    .to_string()
}
