#![forbid(unsafe_code)]

use super::compiled::path_matches_glob;
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use crate::types::{PolicyViolation, TokenBudgetRule};
use ignore::WalkBuilder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Compiled token budget matcher.
#[derive(Clone, Debug)]
pub struct CompiledTokenBudget {
    pub rule: TokenBudgetRule,
    pub matcher: Gitignore,
}

impl CompiledTokenBudget {
    /// Compiles a TokenBudgetRule into a CompiledTokenBudget.
    pub fn compile(rule: &TokenBudgetRule) -> Result<Self, String> {
        let mut b = GitignoreBuilder::new("");
        let _ = b.add_line(None, &rule.pattern);
        let matcher = b.build().map_err(|e| {
            format!(
                "Failed to compile token budget pattern '{}': {e}",
                rule.pattern
            )
        })?;

        Ok(Self {
            rule: rule.clone(),
            matcher,
        })
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

/// Evaluates token budgets across all matching files in `root_dir`.
pub fn evaluate_token_budgets(
    root_dir: &Path,
    budgets: &[CompiledTokenBudget],
) -> Vec<PolicyViolation> {
    if budgets.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();

    for budget in budgets {
        let mut total_tokens = 0usize;
        let mut matching_file_count = 0usize;

        let walker = WalkBuilder::new(root_dir)
            .hidden(false)
            .git_ignore(true)
            .build();

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

            let rel_str = rel_path.to_string_lossy().replace('\\', "/");
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let file_tokens = count_file_tokens(path, &content);
            let line_count = content.lines().count().max(1);

            total_tokens += file_tokens;
            matching_file_count += 1;

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
                    end_line_a: line_count,
                    file_b: None,
                    start_line_b: None,
                    end_line_b: None,
                    cluster_id: None,
                    token_count: file_tokens,
                    snippet: None,
                    remediation: Some(format!(
                        "Reduce content in '{rel_str}' to stay within limit of {max_file} tokens"
                    )),
                });
            }
        }

        if let Some(max_total) = budget.rule.max_total_tokens
            && total_tokens > max_total
        {
            violations.push(PolicyViolation {
                    rule_name: budget.rule.name.clone(),
                    rule_type: "token_budget".to_string(),
                    severity: budget.rule.severity,
                    message: format!(
                        "Token budget '{}' violated: combined total of {total_tokens} tokens across {matching_file_count} files exceeds max_total_tokens limit of {max_total}",
                        budget.rule.name
                    ),
                    file_a: budget.rule.pattern.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    file_b: None,
                    start_line_b: None,
                    end_line_b: None,
                    cluster_id: None,
                    token_count: total_tokens,
                    snippet: None,
                    remediation: Some(format!(
                        "Reduce combined total tokens across files matching '{}' to stay within limit of {max_total}",
                        budget.rule.pattern
                    )),
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
        };
        let compiled = CompiledTokenBudget::compile(&rule).expect("compile failed");
        assert_eq!(compiled.rule.name, "test-budget");
    }
}
