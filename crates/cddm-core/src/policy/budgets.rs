#![forbid(unsafe_code)]

use super::compiled::{add_glob_pattern_lines, path_matches_glob};
use crate::grammar::get_grammar_for_path;
use crate::suppression::directives::{is_mock_path, is_test_path};
use crate::tokenizer::tokenize;
use crate::types::{PolicyEvaluationOptions, PolicyViolation, TokenBudgetRule};
use ignore::WalkBuilder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Compiled token / line budget matcher.
#[derive(Clone, Debug)]
pub struct CompiledTokenBudget {
    pub rule: TokenBudgetRule,
    pub matcher: Gitignore,
    pub exclude_matcher: Option<Gitignore>,
}

impl CompiledTokenBudget {
    /// Compiles a TokenBudgetRule into a CompiledTokenBudget.
    pub fn compile(rule: &TokenBudgetRule) -> Result<Self, String> {
        let mut b = GitignoreBuilder::new("");
        let mut pattern_count = 0usize;
        if !rule.pattern.trim().is_empty() {
            add_glob_pattern_lines(&mut b, &rule.pattern);
            pattern_count += 1;
        }
        for p in &rule.patterns {
            if !p.trim().is_empty() {
                add_glob_pattern_lines(&mut b, p);
                pattern_count += 1;
            }
        }
        if pattern_count == 0 {
            return Err(format!(
                "Budget rule '{}' must define 'pattern' or non-empty 'patterns'",
                rule.name
            ));
        }

        let matcher = b.build().map_err(|e| {
            format!(
                "Failed to compile token/line budget pattern '{}': {e}",
                rule.pattern
            )
        })?;

        let exclude_matcher = if !rule.exclude.is_empty() {
            let mut eb = GitignoreBuilder::new("");
            for exc in &rule.exclude {
                if !exc.trim().is_empty() {
                    add_glob_pattern_lines(&mut eb, exc);
                }
            }
            Some(eb.build().map_err(|e| {
                format!(
                    "Failed to compile exclude pattern for budget '{}': {e}",
                    rule.name
                )
            })?)
        } else {
            None
        };

        Ok(Self {
            rule: rule.clone(),
            matcher,
            exclude_matcher,
        })
    }

    fn display_pattern(&self) -> String {
        if !self.rule.pattern.trim().is_empty() {
            self.rule.pattern.clone()
        } else {
            self.rule.patterns.join(", ")
        }
    }
}

/// Counts tokens in a file. If recognized programming grammar is available, uses normalized token stream;
/// otherwise, uses whitespace-delimited word tokens.
pub fn count_file_tokens(path: &Path, content: &str) -> usize {
    if let Some(grammar) = get_grammar_for_path(path) {
        let tokens = tokenize(content, grammar, false);
        tokens.len()
    } else {
        content.split_whitespace().count()
    }
}

/// Counts lines in file content (`0` for empty files, otherwise at least `1`).
pub fn count_content_lines(content: &str) -> usize {
    if content.is_empty() {
        0
    } else {
        content.lines().count().max(1)
    }
}

/// Evaluates token and line budgets across all matching files in `root_dir` using default options.
pub fn evaluate_token_budgets(
    root_dir: &Path,
    budgets: &[CompiledTokenBudget],
) -> Vec<PolicyViolation> {
    evaluate_token_budgets_with_options(
        root_dir,
        budgets,
        None,
        None,
        None,
        &PolicyEvaluationOptions::default(),
    )
}

/// Evaluates token and line budgets across all matching files in `root_dir` with explicit inclusion options.
pub fn evaluate_token_budgets_with_options(
    root_dir: &Path,
    budgets: &[CompiledTokenBudget],
    config_ignore_tests: Option<bool>,
    config_ignore_mocks: Option<bool>,
    config_include_ignored: Option<bool>,
    options: &PolicyEvaluationOptions,
) -> Vec<PolicyViolation> {
    if budgets.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();

    for budget in budgets {
        let eff_ignore_tests = options
            .force_ignore_tests
            .or(budget.rule.ignore_tests)
            .or(config_ignore_tests)
            .unwrap_or(options.ignore_tests);
        let eff_ignore_mocks = options
            .force_ignore_mocks
            .or(budget.rule.ignore_mocks)
            .or(config_ignore_mocks)
            .unwrap_or(options.ignore_mocks);
        let eff_include_ignored = options
            .force_include_ignored
            .or(budget.rule.include_ignored)
            .or(config_include_ignored)
            .unwrap_or(options.include_ignored);

        let respect_ignore = !eff_include_ignored;
        let mut total_tokens = 0usize;
        let mut matching_file_count = 0usize;

        let walker = WalkBuilder::new(root_dir)
            .hidden(false)
            .parents(true)
            .git_ignore(respect_ignore)
            .git_global(respect_ignore)
            .git_exclude(respect_ignore)
            .ignore(respect_ignore)
            .require_git(false)
            .filter_entry(|entry| {
                if entry.file_type().is_some_and(|ft| ft.is_dir())
                    && let Some(name) = entry.file_name().to_str()
                {
                    return name != ".git" && name != ".cddm";
                }
                true
            })
            .build();

        let needs_token_count =
            budget.rule.max_file_tokens.is_some() || budget.rule.max_total_tokens.is_some();

        for entry_res in walker {
            let entry = match entry_res {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let rel_path = path.strip_prefix(root_dir).unwrap_or(path);
            if !path_matches_glob(&budget.matcher, rel_path) {
                continue;
            }
            if let Some(ref exc) = budget.exclude_matcher
                && path_matches_glob(exc, rel_path)
            {
                continue;
            }
            if eff_ignore_tests && is_test_path(rel_path) {
                continue;
            }
            if eff_ignore_mocks && is_mock_path(rel_path) {
                continue;
            }

            let rel_str = rel_path.to_string_lossy().replace('\\', "/");
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let line_count = count_content_lines(&content);
            let file_tokens = if needs_token_count {
                count_file_tokens(path, &content)
            } else {
                0
            };

            total_tokens += file_tokens;
            matching_file_count += 1;

            if let Some(max_lines) = budget.rule.max_file_lines
                && line_count > max_lines
            {
                violations.push(PolicyViolation {
                    rule_name: budget.rule.name.clone(),
                    rule_type: "line_budget".to_string(),
                    severity: budget.rule.severity,
                    message: format!(
                        "Line budget '{}' violated: file '{rel_str}' has {line_count} lines, exceeding max_file_lines limit of {max_lines}",
                        budget.rule.name
                    ),
                    file_a: rel_str.clone(),
                    start_line_a: 1,
                    end_line_a: line_count.max(1),
                    file_b: None,
                    start_line_b: None,
                    end_line_b: None,
                    cluster_id: None,
                    token_count: file_tokens,
                    line_count: Some(line_count),
                    max_lines: Some(max_lines),
                    snippet: Some(format!("{line_count} / {max_lines} lines")),
                    remediation: budget.rule.remediation.clone().or_else(|| {
                        Some(format!(
                            "Decompose '{rel_str}' ({line_count} lines) into modular submodules to stay within the {max_lines}-line ceiling"
                        ))
                    }),
                });
            }

            if let Some(max_file) = budget.rule.max_file_tokens
                && file_tokens > max_file
            {
                violations.push(PolicyViolation {
                    rule_name: budget.rule.name.clone(),
                    rule_type: "token_budget".to_string(),
                    severity: budget.rule.severity,
                    message: format!(
                        "Token budget '{}' violated: file '{rel_str}' has {file_tokens} tokens, exceeding max_file_tokens limit of {max_file}",
                        budget.rule.name
                    ),
                    file_a: rel_str.clone(),
                    start_line_a: 1,
                    end_line_a: line_count.max(1),
                    file_b: None,
                    start_line_b: None,
                    end_line_b: None,
                    cluster_id: None,
                    token_count: file_tokens,
                    line_count: Some(line_count),
                    max_lines: budget.rule.max_file_lines,
                    snippet: None,
                    remediation: budget.rule.remediation.clone().or_else(|| {
                        Some(format!(
                            "Reduce content in '{rel_str}' to stay within limit of {max_file} tokens"
                        ))
                    }),
                });
            }
        }

        if let Some(max_total) = budget.rule.max_total_tokens
            && total_tokens > max_total
        {
            let pattern_label = budget.display_pattern();
            violations.push(PolicyViolation {
                rule_name: budget.rule.name.clone(),
                rule_type: "token_budget".to_string(),
                severity: budget.rule.severity,
                message: format!(
                    "Token budget '{}' violated: combined total of {total_tokens} tokens across {matching_file_count} files exceeds max_total_tokens limit of {max_total}",
                    budget.rule.name
                ),
                file_a: pattern_label.clone(),
                start_line_a: 1,
                end_line_a: 1,
                file_b: None,
                start_line_b: None,
                end_line_b: None,
                cluster_id: None,
                token_count: total_tokens,
                line_count: None,
                max_lines: None,
                snippet: None,
                remediation: budget.rule.remediation.clone().or_else(|| {
                    Some(format!(
                        "Reduce combined total tokens across files matching '{pattern_label}' to stay within limit of {max_total}"
                    ))
                }),
            });
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicySeverity;

    #[test]
    fn test_count_file_tokens_text() {
        let content = "The quick brown fox jumps over the lazy dog";
        assert_eq!(count_file_tokens(Path::new("test.txt"), content), 9);
    }

    #[test]
    fn test_compile_token_budget() {
        let rule = TokenBudgetRule {
            name: "test-budget".to_string(),
            description: None,
            pattern: "docs/*.md".to_string(),
            max_file_tokens: Some(100),
            max_total_tokens: Some(500),
            severity: PolicySeverity::Error,
            ..Default::default()
        };
        let compiled = CompiledTokenBudget::compile(&rule).expect("compile failed");
        assert_eq!(compiled.rule.name, "test-budget");
    }

    #[test]
    fn test_max_file_lines_budget_and_test_inclusion() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::create_dir_all(root.join("tests")).unwrap();

        let big_code = (1..=12)
            .map(|i| format!("let x_{i} = {i};"))
            .collect::<Vec<_>>()
            .join("\n");
        let small_code = "let ok = 1;\nlet ok2 = 2;\n";

        std::fs::write(root.join("src/big.rs"), &big_code).unwrap();
        std::fs::write(root.join("src/small.rs"), small_code).unwrap();
        std::fs::write(root.join("tests/big_test.rs"), &big_code).unwrap();

        let rule = TokenBudgetRule {
            name: "max-5-lines".to_string(),
            pattern: "**/*.{rs,ts}".to_string(),
            max_file_lines: Some(5),
            severity: PolicySeverity::Error,
            ..Default::default()
        };
        let compiled = CompiledTokenBudget::compile(&rule).expect("compile");

        // Excluding tests when ignore_tests=true
        let opts_ignore = PolicyEvaluationOptions {
            ignore_tests: true,
            ..Default::default()
        };
        let v_ignore = evaluate_token_budgets_with_options(
            root,
            std::slice::from_ref(&compiled),
            None,
            None,
            None,
            &opts_ignore,
        );
        assert_eq!(v_ignore.len(), 1);
        assert_eq!(v_ignore[0].rule_type, "line_budget");
        assert_eq!(v_ignore[0].file_a, "src/big.rs");
        assert_eq!(v_ignore[0].line_count, Some(12));
        assert_eq!(v_ignore[0].max_lines, Some(5));

        // Including tests when force_ignore_tests=Some(false)
        let opts_include = PolicyEvaluationOptions {
            ignore_tests: false,
            force_ignore_tests: Some(false),
            ..Default::default()
        };
        let v_include =
            evaluate_token_budgets_with_options(root, &[compiled], None, None, None, &opts_include);
        assert_eq!(v_include.len(), 2);
        assert!(v_include.iter().any(|v| v.file_a == "tests/big_test.rs"));
    }
}
