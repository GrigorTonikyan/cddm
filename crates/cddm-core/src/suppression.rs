use crate::types::{CloneType, SuppressionConfig, SuppressionDirective, SuppressionRule};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::Path;

/// Engine managing `.cddmignore` rule parsing, inline code directives, and AST-aware suppression.
#[derive(Clone, Debug)]
pub struct SuppressionEngine {
    /// Active suppression configuration
    config: SuppressionConfig,
    /// Compiled gitignore matcher for ignored paths
    gitignore: Gitignore,
    /// Threshold overrides by pattern: matcher -> min_tokens
    threshold_overrides: Vec<(Gitignore, usize)>,
    /// Clone type exclusions by pattern: matcher -> Vec<CloneType>
    type_filters: Vec<(Gitignore, Vec<CloneType>)>,
    /// Cache of parsed inline directives per file: path -> Vec<SuppressionDirective>
    inline_directives_cache: HashMap<String, Vec<SuppressionDirective>>,
}

impl SuppressionEngine {
    /// Constructs a new SuppressionEngine with the given configuration.
    pub fn new(mut config: SuppressionConfig) -> Result<Self, String> {
        let mut builder = GitignoreBuilder::new("");
        let mut threshold_overrides = Vec::new();
        let mut type_filters = Vec::new();

        for rule in &config.rules {
            if rule.rule_type == "ignore" {
                let _ = builder.add_line(None, &rule.pattern);
            } else if rule.rule_type == "threshold" {
                if let Some(tokens) = rule.min_tokens {
                    let mut b = GitignoreBuilder::new("");
                    if b.add_line(None, &rule.pattern).is_ok()
                        && let Ok(gi) = b.build()
                    {
                        threshold_overrides.push((gi, tokens));
                    }
                }
            } else if rule.rule_type == "type_filter" && !rule.ignored_clone_types.is_empty() {
                let mut b = GitignoreBuilder::new("");
                if b.add_line(None, &rule.pattern).is_ok()
                    && let Ok(gi) = b.build()
                {
                    type_filters.push((gi, rule.ignored_clone_types.clone()));
                }
            }
        }

        let gitignore = builder
            .build()
            .map_err(|e| format!("Failed to compile .cddmignore pattern set: {e}"))?;

        // Normalize raw content
        if config.raw_cddmignore.is_none() {
            config.raw_cddmignore = Some(Self::format_rules_to_raw(&config.rules));
        }

        Ok(Self {
            config,
            gitignore,
            threshold_overrides,
            type_filters,
            inline_directives_cache: HashMap::new(),
        })
    }

    /// Creates a default suppression engine with standard defaults.
    pub fn default_engine() -> Self {
        Self::with_options(false, false, true)
    }

    /// Creates a suppression engine with specified category flags.
    pub fn with_options(ignore_tests: bool, ignore_mocks: bool, ignore_generated: bool) -> Self {
        Self::new(SuppressionConfig {
            rules: Vec::new(),
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            raw_cddmignore: None,
        })
        .unwrap_or_else(|_| Self {
            config: SuppressionConfig {
                rules: Vec::new(),
                ignore_tests,
                ignore_mocks,
                ignore_generated,
                raw_cddmignore: None,
            },
            gitignore: GitignoreBuilder::new("").build().unwrap(),
            threshold_overrides: Vec::new(),
            type_filters: Vec::new(),
            inline_directives_cache: HashMap::new(),
        })
    }

    /// Loads suppression configuration from a `.cddmignore` file on disk.
    pub fn from_file(
        file_path: &Path,
        ignore_tests: bool,
        ignore_mocks: bool,
        ignore_generated: bool,
    ) -> Result<Self, String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            format!(
                "Failed to read .cddmignore at '{}': {e}",
                file_path.display()
            )
        })?;
        Self::from_str(&content, ignore_tests, ignore_mocks, ignore_generated)
    }

    /// Parses suppression configuration from a string.
    pub fn from_str(
        content: &str,
        ignore_tests: bool,
        ignore_mocks: bool,
        ignore_generated: bool,
    ) -> Result<Self, String> {
        let rules = Self::parse_cddmignore_content(content);
        Self::new(SuppressionConfig {
            rules,
            ignore_tests,
            ignore_mocks,
            ignore_generated,
            raw_cddmignore: Some(content.to_string()),
        })
    }

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
                    let types = Self::parse_clone_types(types_str);

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

    fn parse_clone_types(types_str: &str) -> Vec<CloneType> {
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

    fn format_rules_to_raw(rules: &[SuppressionRule]) -> String {
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

    /// Checks if a file path is ignored by path rules, test/mock filters, or generated headers.
    pub fn is_path_ignored(&self, path: &Path, content: Option<&str>) -> bool {
        // 1. Check gitignore match
        if self.gitignore.matched(path, false).is_ignore() {
            return true;
        }

        // 2. Check test files filter
        if self.config.ignore_tests && Self::is_test_path(path) {
            return true;
        }

        // 3. Check mock files filter
        if self.config.ignore_mocks && Self::is_mock_path(path) {
            return true;
        }

        // 4. Check auto-generated header
        if self.config.ignore_generated
            && let Some(text) = content
            && Self::is_generated_header(text)
        {
            return true;
        }

        false
    }

    /// Determines if a file is a test file based on conventions.
    pub fn is_test_path(path: &Path) -> bool {
        let p = path.to_string_lossy().replace('\\', "/").to_lowercase();
        p.contains("/tests/")
            || p.contains("/test/")
            || p.contains("/__tests__/")
            || p.contains(".test.")
            || p.contains(".spec.")
            || p.contains("_test.")
            || p.contains("_spec.")
            || p.ends_with("test.rs")
    }

    /// Determines if a file is a mock or test fixture.
    pub fn is_mock_path(path: &Path) -> bool {
        let p = path.to_string_lossy().replace('\\', "/").to_lowercase();
        p.contains("/mocks/")
            || p.contains("/mock/")
            || p.contains("/__mocks__/")
            || p.contains("/fixtures/")
            || p.contains(".mock.")
            || p.contains("_mock.")
    }

    /// Inspects the top lines of a file to detect common automated code generation headers.
    pub fn is_generated_header(content: &str) -> bool {
        for line in content.lines().take(25) {
            let lower = line.to_lowercase();
            if lower.contains("@generated")
                || lower.contains("do not edit")
                || lower.contains("<auto-generated")
                || lower.contains("code generated by")
                || lower.contains("this file was generated")
                || lower.contains("this file is generated")
                || lower.contains("autogenerated by")
                || lower.contains("automatically generated")
            {
                return true;
            }
        }
        false
    }

    /// Parses inline suppression directives from source code.
    pub fn parse_inline_directives(file_path: &str, content: &str) -> Vec<SuppressionDirective> {
        let mut directives = Vec::new();
        let mut current_block_start: Option<usize> = None;

        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            if trimmed.contains("cddm:ignore-start")
                || trimmed.contains("cddm-ignore-start")
                || trimmed.contains("cddm:allow-start")
            {
                current_block_start = Some(line_num);
            } else if trimmed.contains("cddm:ignore-end")
                || trimmed.contains("cddm-ignore-end")
                || trimmed.contains("cddm:allow-end")
            {
                if let Some(start) = current_block_start {
                    directives.push(SuppressionDirective {
                        file_path: file_path.to_string(),
                        start_line: start,
                        end_line: line_num,
                        directive_type: "ignore_block".to_string(),
                        reason: None,
                    });
                    current_block_start = None;
                }
            } else if trimmed.contains("cddm:ignore")
                || trimmed.contains("cddm-ignore")
                || trimmed.contains("#[cddm(allow)]")
                || trimmed.contains("#[cddm(allow_duplication)]")
                || trimmed.contains("@cddm_ignore")
            {
                directives.push(SuppressionDirective {
                    file_path: file_path.to_string(),
                    start_line: line_num,
                    end_line: line_num + 1,
                    directive_type: "ignore_line".to_string(),
                    reason: None,
                });
            }
        }

        if let Some(start) = current_block_start {
            directives.push(SuppressionDirective {
                file_path: file_path.to_string(),
                start_line: start,
                end_line: content.lines().count(),
                directive_type: "ignore_block".to_string(),
                reason: None,
            });
        }

        directives
    }

    /// Caches inline directives for a file.
    pub fn register_file_directives(&mut self, file_path: &str, content: &str) {
        let directives = Self::parse_inline_directives(file_path, content);
        if !directives.is_empty() {
            self.inline_directives_cache
                .insert(file_path.to_string(), directives);
        }
    }

    /// Checks if a specific line span in a file is suppressed.
    pub fn is_span_suppressed(&self, file_path: &str, start_line: usize, end_line: usize) -> bool {
        if let Some(directives) = self.inline_directives_cache.get(file_path) {
            for d in directives {
                if start_line <= d.end_line && end_line >= d.start_line {
                    return true;
                }
            }
        }
        false
    }

    /// Gets the effective minimum token threshold for a file path, taking overrides into account.
    pub fn get_effective_min_tokens(&self, path: &Path, default_min: usize) -> usize {
        for (gi, min) in &self.threshold_overrides {
            if gi.matched(path, false).is_ignore() {
                return *min;
            }
        }
        default_min
    }

    /// Checks if a clone type is excluded for a given file path.
    pub fn is_clone_type_ignored(&self, path: &Path, clone_type: &CloneType) -> bool {
        for (gi, types) in &self.type_filters {
            if gi.matched(path, false).is_ignore() && types.contains(clone_type) {
                return true;
            }
        }
        false
    }

    /// Returns a reference to the active configuration.
    pub fn config(&self) -> &SuppressionConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_cddmignore_rules() {
        let content = r#"
# General ignore
tests/**
src/fixtures/**

# Threshold override
[threshold] benchmarks/** min_tokens=150

# Clone type filter
[type-filter] legacy/** ignore=Exact,Renamed
"#;
        let rules = SuppressionEngine::parse_cddmignore_content(content);
        assert_eq!(rules.len(), 4);
        assert_eq!(rules[0].pattern, "tests/**");
        assert_eq!(rules[0].rule_type, "ignore");

        assert_eq!(rules[2].pattern, "benchmarks/**");
        assert_eq!(rules[2].rule_type, "threshold");
        assert_eq!(rules[2].min_tokens, Some(150));

        assert_eq!(rules[3].pattern, "legacy/**");
        assert_eq!(rules[3].rule_type, "type_filter");
        assert_eq!(
            rules[3].ignored_clone_types,
            vec![CloneType::Exact, CloneType::Renamed]
        );
    }

    #[test]
    fn test_is_path_ignored_glob() {
        let content = "tests/**\n*.generated.ts\n";
        let engine = SuppressionEngine::from_str(content, false, false, false).unwrap();

        assert!(engine.is_path_ignored(Path::new("tests/unit/test_app.rs"), None));
        assert!(engine.is_path_ignored(Path::new("src/models/user.generated.ts"), None));
        assert!(!engine.is_path_ignored(Path::new("src/models/user.ts"), None));
    }

    #[test]
    fn test_is_generated_header() {
        let generated_code = r#"// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
//  protoc-gen-go v1.28.0
package models
"#;
        assert!(SuppressionEngine::is_generated_header(generated_code));

        let normal_code = r#"// Copyright 2026 Grigor Tonikyan
package models

func Calculate() int { return 42 }
"#;
        assert!(!SuppressionEngine::is_generated_header(normal_code));
    }

    #[test]
    fn test_inline_suppression_directives() {
        let code = r#"
fn a() {
    // cddm:ignore
    let dup_code = 123;
}

// cddm:ignore-start
fn b() {
    let block_dup = 456;
}
// cddm:ignore-end

fn c() {
    let clean = 789;
}
"#;
        let directives = SuppressionEngine::parse_inline_directives("src/a.rs", code);
        assert_eq!(directives.len(), 2);

        let mut engine = SuppressionEngine::default_engine();
        engine.register_file_directives("src/a.rs", code);

        assert!(engine.is_span_suppressed("src/a.rs", 3, 4));
        assert!(engine.is_span_suppressed("src/a.rs", 8, 10));
        assert!(!engine.is_span_suppressed("src/a.rs", 14, 16));
    }

    #[test]
    fn test_threshold_and_type_filter_overrides() {
        let content = r#"
[threshold] benches/** min_tokens=200
[type-filter] legacy/** ignore=Exact
"#;
        let engine = SuppressionEngine::from_str(content, false, false, false).unwrap();

        assert_eq!(
            engine.get_effective_min_tokens(Path::new("benches/bench.rs"), 50),
            200
        );
        assert_eq!(
            engine.get_effective_min_tokens(Path::new("src/main.rs"), 50),
            50
        );

        assert!(engine.is_clone_type_ignored(Path::new("legacy/old.rs"), &CloneType::Exact));
        assert!(!engine.is_clone_type_ignored(Path::new("legacy/old.rs"), &CloneType::NearMiss));
    }

    #[test]
    fn test_from_file_lifecycle() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".cddmignore");
        let content = SuppressionEngine::generate_default_cddmignore();
        std::fs::write(&file_path, content).unwrap();

        let engine = SuppressionEngine::from_file(&file_path, true, true, true).unwrap();
        assert!(engine.is_path_ignored(Path::new("tests/app_test.rs"), None));
        assert!(engine.is_path_ignored(Path::new("src/fixtures/mock.json"), None));
    }
}
