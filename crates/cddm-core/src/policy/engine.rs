#![forbid(unsafe_code)]

use super::budgets::{CompiledTokenBudget, evaluate_token_budgets};
use super::compiled::{CompiledBoundary, CompiledLimit, CompiledZeroDuplication};
use super::emoji::evaluate_zero_emoji;
use super::eval::evaluate_scan_policies;
use super::patterns::{CompiledProhibitedPattern, evaluate_prohibited_patterns};
use crate::types::{PolicyConfig, PolicyEvaluationResult, PolicySeverity, ScanResult};
use ignore::gitignore::GitignoreBuilder;
use std::path::Path;

/// Engine managing `.cddmrules.toml` architectural policy parsing, pattern compilation, and evaluation.
#[derive(Clone, Debug)]
pub struct PolicyEngine {
    /// Active policy configuration
    config: PolicyConfig,
    /// Compiled boundary matchers
    compiled_boundaries: Vec<CompiledBoundary>,
    /// Compiled zero duplication matchers
    compiled_zero_duplication: Vec<CompiledZeroDuplication>,
    /// Compiled limit matchers
    compiled_limits: Vec<CompiledLimit>,
    /// Compiled prohibited pattern matchers
    compiled_prohibited_patterns: Vec<CompiledProhibitedPattern>,
    /// Compiled token budget matchers
    compiled_token_budgets: Vec<CompiledTokenBudget>,
}

impl PolicyEngine {
    /// Constructs a new PolicyEngine from a PolicyConfig.
    pub fn new(mut config: PolicyConfig) -> Result<Self, String> {
        let mut compiled_boundaries = Vec::new();
        for rule in &config.boundaries {
            let mut sb = GitignoreBuilder::new("");
            let _ = sb.add_line(None, &rule.source);
            let source_matcher = sb
                .build()
                .map_err(|e| format!("Failed to compile boundary source '{}': {e}", rule.source))?;

            let mut target_matchers = Vec::new();
            for target in &rule.forbidden_targets {
                let mut tb = GitignoreBuilder::new("");
                let _ = tb.add_line(None, target);
                let tm = tb
                    .build()
                    .map_err(|e| format!("Failed to compile boundary target '{target}': {e}"))?;
                target_matchers.push(tm);
            }

            compiled_boundaries.push(CompiledBoundary {
                rule: rule.clone(),
                source_matcher,
                target_matchers,
            });
        }

        let mut compiled_zero_duplication = Vec::new();
        for rule in &config.zero_duplication {
            let mut b = GitignoreBuilder::new("");
            let _ = b.add_line(None, &rule.pattern);
            let matcher = b.build().map_err(|e| {
                format!(
                    "Failed to compile zero_duplication pattern '{}': {e}",
                    rule.pattern
                )
            })?;

            compiled_zero_duplication.push(CompiledZeroDuplication {
                rule: rule.clone(),
                matcher,
            });
        }

        let mut compiled_limits = Vec::new();
        for rule in &config.limits {
            let mut b = GitignoreBuilder::new("");
            let _ = b.add_line(None, &rule.pattern);
            let matcher = b
                .build()
                .map_err(|e| format!("Failed to compile limit pattern '{}': {e}", rule.pattern))?;

            compiled_limits.push(CompiledLimit {
                rule: rule.clone(),
                matcher,
            });
        }

        let mut compiled_prohibited_patterns = Vec::new();
        for rule in &config.prohibited_patterns {
            let cp = CompiledProhibitedPattern::compile(rule)?;
            compiled_prohibited_patterns.push(cp);
        }

        let mut compiled_token_budgets = Vec::new();
        for rule in &config.token_budgets {
            let tb = CompiledTokenBudget::compile(rule)?;
            compiled_token_budgets.push(tb);
        }

        if config.raw_toml.is_none() {
            config.raw_toml = toml::to_string_pretty(&config).ok();
        }

        Ok(Self {
            config,
            compiled_boundaries,
            compiled_zero_duplication,
            compiled_limits,
            compiled_prohibited_patterns,
            compiled_token_budgets,
        })
    }

    /// Creates an empty policy engine with no rules.
    pub fn empty() -> Self {
        Self {
            config: PolicyConfig::default(),
            compiled_boundaries: Vec::new(),
            compiled_zero_duplication: Vec::new(),
            compiled_limits: Vec::new(),
            compiled_prohibited_patterns: Vec::new(),
            compiled_token_budgets: Vec::new(),
        }
    }

    /// Returns a reference to the active policy configuration.
    pub fn config(&self) -> &PolicyConfig {
        &self.config
    }

    /// Loads policies from a `.cddmrules.toml` file on disk.
    pub fn from_file(file_path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(file_path).map_err(|e| {
            format!(
                "Failed to read .cddmrules.toml at '{}': {e}",
                file_path.display()
            )
        })?;
        content.parse::<Self>()
    }

    /// Parses raw TOML text into a `PolicyConfig` record.
    pub fn parse_rules_content(content: &str) -> Result<PolicyConfig, String> {
        toml::from_str::<PolicyConfig>(content)
            .map_err(|e| format!("Failed to parse .cddmrules.toml: {e}"))
    }

    /// Evaluates all active policy rules against a scan result.
    pub fn evaluate(&self, scan_result: &ScanResult) -> PolicyEvaluationResult {
        let mut eval = evaluate_scan_policies(
            scan_result,
            &self.compiled_boundaries,
            &self.compiled_zero_duplication,
            &self.compiled_limits,
        );

        let target_dir_str = scan_result.target_directory.as_deref().unwrap_or(".");
        let target_path = Path::new(target_dir_str);

        if target_path.exists() {
            // 4. Prohibited pattern rules
            if !self.compiled_prohibited_patterns.is_empty() {
                let pattern_violations =
                    evaluate_prohibited_patterns(target_path, &self.compiled_prohibited_patterns);
                eval.violations.extend(pattern_violations);
            }

            // 5. Zero-emoji sensor
            if self.config.zero_emojis == Some(true) {
                let emoji_violations = evaluate_zero_emoji(target_path);
                eval.violations.extend(emoji_violations);
            }

            // 6. Token budget rules
            if !self.compiled_token_budgets.is_empty() {
                let budget_violations =
                    evaluate_token_budgets(target_path, &self.compiled_token_budgets);
                eval.violations.extend(budget_violations);
            }

            // Recalculate aggregation metrics
            eval.total_violations = eval.violations.len();
            eval.error_count = eval
                .violations
                .iter()
                .filter(|v| v.severity == PolicySeverity::Error)
                .count();
            eval.warning_count = eval
                .violations
                .iter()
                .filter(|v| v.severity == PolicySeverity::Warning)
                .count();
            eval.info_count = eval
                .violations
                .iter()
                .filter(|v| v.severity == PolicySeverity::Info)
                .count();
            eval.passed = eval.error_count == 0;
        }

        eval
    }

    /// Serializes the current policy configuration back to a valid TOML string.
    pub fn to_toml_string(&self) -> Result<String, String> {
        if let Some(ref raw) = self.config.raw_toml {
            Ok(raw.clone())
        } else {
            toml::to_string_pretty(&self.config).map_err(|e| e.to_string())
        }
    }

    /// Generates a starter `.cddmrules.toml` template.
    pub fn starter_rules_toml() -> &'static str {
        Self::generate_starter_template()
    }

    /// Generates a starter `.cddmrules.toml` template.
    pub fn generate_starter_template() -> &'static str {
        r#"# CDDM Architectural Rules & Boundary Policy Configuration
# Schema Reference: docs/ARCHITECTURE.md

# [[boundaries]]
# Enforces that code from a source layer or domain cannot be duplicated in target modules.
# [[boundaries]]
# name = "domain-isolation"
# description = "Domain core logic must not be duplicated into presentation or infrastructure layers"
# source = "src/domain/**"
# forbidden_targets = ["src/presentation/**", "src/infra/**"]
# severity = "error" # "error" | "warning" | "info"

# [[zero_duplication]]
# Enforces 0% clone duplication within security-critical or sensitive modules.
# [[zero_duplication]]
# name = "auth-security-zone"
# description = "Authentication and cryptography modules must have zero code duplication"
# pattern = "src/auth/**"
# severity = "error"

# [[limits]]
# Imposes maximum token limits and cluster occurrence counts on specific subsystems.
# [[limits]]
# name = "api-cluster-limit"
# description = "API handlers must not exceed 100 duplicate tokens or 3 multi-site occurrences"
# pattern = "src/api/**"
# max_tokens = 100
# max_occurrences = 3
# severity = "warning"

# [[prohibited_patterns]]
# Enforces prohibited code patterns via Regex or Tree-sitter AST queries.
# [[prohibited_patterns]]
# name = "no-console-log"
# description = "Disallow debugging console.log statements in production code"
# pattern = "console\\.log"
# pattern_type = "regex" # "regex" | "ast"
# include = ["src/**/*.ts", "src/**/*.tsx"]
# exclude = ["src/**/*.test.ts"]
# severity = "error"
# remediation = "Use structured logger or remove debug console statements"

# Enforce zero-emoji policy across all repository files
# zero_emojis = true

# [[token_budgets]]
# Constrains maximum token counts per file or total tokens across a glob pattern.
# [[token_budgets]]
# name = "agent-rules-budget"
# description = "Agent rule files must not exceed token budgets to prevent LLM context bloat"
# pattern = ".agents/rules/*.md"
# max_file_tokens = 2500
# max_total_tokens = 20000
# severity = "error"
"#
    }
}

impl std::str::FromStr for PolicyEngine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::parse_rules_content(s)?;
        config.raw_toml = Some(s.to_string());
        Self::new(config)
    }
}
