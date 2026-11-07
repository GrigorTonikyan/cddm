#![forbid(unsafe_code)]

use super::compiled::path_matches_glob;
use crate::ast::parser::{get_tree_sitter_language, parse_ast_tree};
use crate::types::{PolicyViolation, ProhibitedPatternRule};
use ignore::WalkBuilder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use tree_sitter::StreamingIterator;

/// Compiled prohibited pattern matcher.
#[derive(Clone, Debug)]
pub struct CompiledProhibitedPattern {
    pub rule: ProhibitedPatternRule,
    pub include_matcher: Option<Gitignore>,
    pub exclude_matcher: Option<Gitignore>,
    pub regex: Option<Regex>,
}

impl CompiledProhibitedPattern {
    /// Compiles a ProhibitedPatternRule into a CompiledProhibitedPattern.
    pub fn compile(rule: &ProhibitedPatternRule) -> Result<Self, String> {
        let include_matcher =
            if !rule.include.is_empty() {
                let mut b = GitignoreBuilder::new("");
                for inc in &rule.include {
                    let _ = b.add_line(None, inc);
                }
                Some(b.build().map_err(|e| {
                    format!("Failed to compile include globs for '{}': {e}", rule.name)
                })?)
            } else {
                None
            };

        let exclude_matcher =
            if !rule.exclude.is_empty() {
                let mut b = GitignoreBuilder::new("");
                for exc in &rule.exclude {
                    let _ = b.add_line(None, exc);
                }
                Some(b.build().map_err(|e| {
                    format!("Failed to compile exclude globs for '{}': {e}", rule.name)
                })?)
            } else {
                None
            };

        let regex = if rule.pattern_type.eq_ignore_ascii_case("regex")
            || rule.pattern_type.trim().is_empty()
        {
            let re = Regex::new(&rule.pattern)
                .map_err(|e| format!("Failed to compile regex pattern for '{}': {e}", rule.name))?;
            Some(re)
        } else {
            None
        };

        Ok(Self {
            rule: rule.clone(),
            include_matcher,
            exclude_matcher,
            regex,
        })
    }

    /// Tests if a file path is eligible for this prohibited pattern rule.
    pub fn matches_path(&self, rel_path: &Path) -> bool {
        if let Some(ref inc) = self.include_matcher
            && !path_matches_glob(inc, rel_path)
        {
            return false;
        }
        if let Some(ref exc) = self.exclude_matcher
            && path_matches_glob(exc, rel_path)
        {
            return false;
        }
        true
    }
}

fn is_system_ignored_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "node_modules"
            | "target"
            | "dist"
            | "out"
            | "coverage"
            | ".vscode"
            | ".cddm"
            | "packaging"
            | ".cache"
            | ".vite"
            | ".vite-hooks"
    )
}

/// Evaluates all compiled prohibited pattern rules against files under `root_dir`.
pub fn evaluate_prohibited_patterns(
    root_dir: &Path,
    patterns: &[CompiledProhibitedPattern],
) -> Vec<PolicyViolation> {
    if patterns.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();
    let walker = WalkBuilder::new(root_dir)
        .hidden(true)
        .git_ignore(true)
        .build();

    let mut visited_paths = HashSet::new();

    for entry_res in walker {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let is_in_ignored = path.components().any(|c| {
            c.as_os_str()
                .to_str()
                .map(is_system_ignored_dir)
                .unwrap_or(false)
        });
        if is_in_ignored {
            continue;
        }

        let rel_path = path.strip_prefix(root_dir).unwrap_or(path);
        let rel_str = rel_path.to_string_lossy().replace('\\', "/");
        if visited_paths.contains(&rel_str) {
            continue;
        }
        visited_paths.insert(rel_str.clone());

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        for pattern in patterns {
            if !pattern.matches_path(rel_path) {
                continue;
            }

            if pattern.rule.pattern_type.eq_ignore_ascii_case("ast") {
                // AST evaluation via Tree-sitter Query
                if let Some(lang) = get_tree_sitter_language(extension)
                    && let Ok(query) = tree_sitter::Query::new(&lang, &pattern.rule.pattern)
                    && let Some(tree) = parse_ast_tree(&content, extension)
                {
                    let mut cursor = tree_sitter::QueryCursor::new();
                    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

                    while let Some(m) = matches.next() {
                        for capture in m.captures {
                            let node = capture.node;
                            let start_line = node.start_position().row + 1;
                            let end_line = node.end_position().row + 1;
                            let start_byte = node.start_byte();
                            let end_byte = node.end_byte().min(content.len());
                            let snippet = content
                                .get(start_byte..end_byte)
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| {
                                    content
                                        .lines()
                                        .nth(node.start_position().row)
                                        .unwrap_or("")
                                        .trim()
                                        .to_string()
                                });

                            violations.push(PolicyViolation {
                                rule_name: pattern.rule.name.clone(),
                                rule_type: "prohibited_pattern".to_string(),
                                severity: pattern.rule.severity,
                                message: format!(
                                    "Prohibited AST pattern '{}' detected in '{}' at line {start_line}",
                                    pattern.rule.name, rel_str
                                ),
                                file_a: rel_str.clone(),
                                start_line_a: start_line,
                                end_line_a: end_line,
                                file_b: None,
                                start_line_b: None,
                                end_line_b: None,
                                cluster_id: None,
                                token_count: snippet.split_whitespace().count().max(1),
                                snippet: Some(snippet),
                                remediation: pattern.rule.remediation.clone(),
                            });
                        }
                    }
                }
            } else if let Some(ref re) = pattern.regex {
                // Regex evaluation line by line
                for (line_idx, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        let line_no = line_idx + 1;
                        let snippet = line.trim().to_string();

                        violations.push(PolicyViolation {
                            rule_name: pattern.rule.name.clone(),
                            rule_type: "prohibited_pattern".to_string(),
                            severity: pattern.rule.severity,
                            message: format!(
                                "Prohibited pattern '{}' matched in '{}' at line {line_no}: {line}",
                                pattern.rule.name, rel_str
                            ),
                            file_a: rel_str.clone(),
                            start_line_a: line_no,
                            end_line_a: line_no,
                            file_b: None,
                            start_line_b: None,
                            end_line_b: None,
                            cluster_id: None,
                            token_count: snippet.split_whitespace().count().max(1),
                            snippet: Some(snippet),
                            remediation: pattern.rule.remediation.clone(),
                        });
                    }
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicySeverity;

    #[test]
    fn test_compile_regex_pattern() {
        let rule = ProhibitedPatternRule {
            name: "no-console-log".to_string(),
            description: Some("Disallow console.log".to_string()),
            pattern: r"console\.log".to_string(),
            pattern_type: "regex".to_string(),
            include: vec!["**/*.ts".to_string()],
            exclude: vec!["**/*.test.ts".to_string()],
            severity: PolicySeverity::Error,
            remediation: Some("Use logger".to_string()),
        };
        let compiled = CompiledProhibitedPattern::compile(&rule).expect("compile failed");
        assert!(compiled.matches_path(Path::new("src/index.ts")));
        assert!(!compiled.matches_path(Path::new("src/index.test.ts")));
    }

    #[test]
    fn test_ast_query_pattern_evaluation() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let root = temp_dir.path();

        let source_file = root.join("sample.rs");
        std::fs::write(&source_file, "fn dangerous() { let x = 42; }").expect("write failed");

        let rule = ProhibitedPatternRule {
            name: "no-dangerous-fn-ast".to_string(),
            description: Some("Disallow functions named dangerous via AST".to_string()),
            pattern: "(function_item name: (identifier) @name (#eq? @name \"dangerous\"))"
                .to_string(),
            pattern_type: "ast".to_string(),
            include: vec!["*.rs".to_string()],
            exclude: vec![],
            severity: PolicySeverity::Error,
            remediation: Some("Rename function to safe name".to_string()),
        };

        let compiled = CompiledProhibitedPattern::compile(&rule).expect("compile failed");
        let violations = evaluate_prohibited_patterns(root, &[compiled]);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_name, "no-dangerous-fn-ast");
        assert_eq!(violations[0].rule_type, "prohibited_pattern");
        assert_eq!(violations[0].start_line_a, 1);
        assert!(violations[0].snippet.is_some());
    }
}
