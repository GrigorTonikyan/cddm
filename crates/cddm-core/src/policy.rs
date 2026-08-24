use crate::types::{
    BoundaryRule, LimitRule, PolicyConfig, PolicyEvaluationResult, PolicySeverity, PolicyViolation,
    ScanResult, ZeroDuplicationRule,
};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

#[derive(Clone, Debug)]
struct CompiledBoundary {
    rule: BoundaryRule,
    source_matcher: Gitignore,
    target_matchers: Vec<Gitignore>,
}

#[derive(Clone, Debug)]
struct CompiledZeroDuplication {
    rule: ZeroDuplicationRule,
    matcher: Gitignore,
}

#[derive(Clone, Debug)]
struct CompiledLimit {
    rule: LimitRule,
    matcher: Gitignore,
}

fn path_matches_glob(matcher: &Gitignore, path: &Path) -> bool {
    let norm = path.to_string_lossy().replace('\\', "/");
    let p = Path::new(&norm);
    if matcher.matched(p, false).is_ignore() {
        return true;
    }
    let components: Vec<&str> = norm.split('/').collect();
    for i in 0..components.len() {
        let sub = components[i..].join("/");
        if matcher.matched(Path::new(&sub), false).is_ignore() {
            return true;
        }
    }
    false
}

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

        if config.raw_toml.is_none() {
            config.raw_toml = toml::to_string_pretty(&config).ok();
        }

        Ok(Self {
            config,
            compiled_boundaries,
            compiled_zero_duplication,
            compiled_limits,
        })
    }

    /// Creates an empty policy engine with no rules.
    pub fn empty() -> Self {
        Self {
            config: PolicyConfig::default(),
            compiled_boundaries: Vec::new(),
            compiled_zero_duplication: Vec::new(),
            compiled_limits: Vec::new(),
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
}

impl std::str::FromStr for PolicyEngine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::parse_rules_content(s)?;
        config.raw_toml = Some(s.to_string());
        Self::new(config)
    }
}

impl PolicyEngine {
    /// Evaluates all active policy rules against a scan result.
    pub fn evaluate(&self, scan_result: &ScanResult) -> PolicyEvaluationResult {
        let mut violations = Vec::new();

        // 1. Evaluate Boundary Rules across clone pairs
        for pair in &scan_result.clone_pairs {
            let path_a = Path::new(&pair.file_a);
            let path_b = Path::new(&pair.file_b);

            for boundary in &self.compiled_boundaries {
                let a_matches_src = path_matches_glob(&boundary.source_matcher, path_a);
                let b_matches_src = path_matches_glob(&boundary.source_matcher, path_b);

                if a_matches_src {
                    for (idx, target_matcher) in boundary.target_matchers.iter().enumerate() {
                        if path_matches_glob(target_matcher, path_b) {
                            let target_pattern = boundary
                                .rule
                                .forbidden_targets
                                .get(idx)
                                .map(|s| s.as_str())
                                .unwrap_or("forbidden target");
                            violations.push(PolicyViolation {
                                rule_name: boundary.rule.name.clone(),
                                rule_type: "boundary".to_string(),
                                severity: boundary.rule.severity,
                                message: format!(
                                    "Architecture boundary '{}' violated: duplication across \
                                     source '{}' ({}) and target '{}' ({})",
                                    boundary.rule.name,
                                    boundary.rule.source,
                                    pair.file_a,
                                    target_pattern,
                                    pair.file_b
                                ),
                                file_a: pair.file_a.clone(),
                                start_line_a: pair.start_line_a,
                                end_line_a: pair.end_line_a,
                                file_b: Some(pair.file_b.clone()),
                                start_line_b: Some(pair.start_line_b),
                                end_line_b: Some(pair.end_line_b),
                                cluster_id: None,
                                token_count: pair.token_count,
                            });
                        }
                    }
                } else if b_matches_src {
                    for (idx, target_matcher) in boundary.target_matchers.iter().enumerate() {
                        if path_matches_glob(target_matcher, path_a) {
                            let target_pattern = boundary
                                .rule
                                .forbidden_targets
                                .get(idx)
                                .map(|s| s.as_str())
                                .unwrap_or("forbidden target");
                            violations.push(PolicyViolation {
                                rule_name: boundary.rule.name.clone(),
                                rule_type: "boundary".to_string(),
                                severity: boundary.rule.severity,
                                message: format!(
                                    "Architecture boundary '{}' violated: duplication across \
                                     source '{}' ({}) and target '{}' ({})",
                                    boundary.rule.name,
                                    boundary.rule.source,
                                    pair.file_b,
                                    target_pattern,
                                    pair.file_a
                                ),
                                file_a: pair.file_a.clone(),
                                start_line_a: pair.start_line_a,
                                end_line_a: pair.end_line_a,
                                file_b: Some(pair.file_b.clone()),
                                start_line_b: Some(pair.start_line_b),
                                end_line_b: Some(pair.end_line_b),
                                cluster_id: None,
                                token_count: pair.token_count,
                            });
                        }
                    }
                }
            }

            // 2. Evaluate Zero Duplication Rules
            for zero_dup in &self.compiled_zero_duplication {
                let a_matches = path_matches_glob(&zero_dup.matcher, path_a);
                let b_matches = path_matches_glob(&zero_dup.matcher, path_b);

                if a_matches || b_matches {
                    let offending_file = if a_matches {
                        &pair.file_a
                    } else {
                        &pair.file_b
                    };
                    violations.push(PolicyViolation {
                        rule_name: zero_dup.rule.name.clone(),
                        rule_type: "zero_duplication".to_string(),
                        severity: zero_dup.rule.severity,
                        message: format!(
                            "Zero duplication policy '{}' violated in protected path '{}' ({})",
                            zero_dup.rule.name, zero_dup.rule.pattern, offending_file
                        ),
                        file_a: pair.file_a.clone(),
                        start_line_a: pair.start_line_a,
                        end_line_a: pair.end_line_a,
                        file_b: Some(pair.file_b.clone()),
                        start_line_b: Some(pair.start_line_b),
                        end_line_b: Some(pair.end_line_b),
                        cluster_id: None,
                        token_count: pair.token_count,
                    });
                }
            }

            // 3. Evaluate Limits on Clone Pairs
            for limit in &self.compiled_limits {
                let a_matches = path_matches_glob(&limit.matcher, path_a);
                let b_matches = path_matches_glob(&limit.matcher, path_b);

                if (a_matches || b_matches)
                    && let Some(max_tokens) = limit.rule.max_tokens
                    && pair.token_count > max_tokens
                {
                    violations.push(PolicyViolation {
                        rule_name: limit.rule.name.clone(),
                        rule_type: "limit".to_string(),
                        severity: limit.rule.severity,
                        message: format!(
                            "Limit policy '{}' violated: token count {} exceeds maximum allowed \
                             limit {}",
                            limit.rule.name, pair.token_count, max_tokens
                        ),
                        file_a: pair.file_a.clone(),
                        start_line_a: pair.start_line_a,
                        end_line_a: pair.end_line_a,
                        file_b: Some(pair.file_b.clone()),
                        start_line_b: Some(pair.start_line_b),
                        end_line_b: Some(pair.end_line_b),
                        cluster_id: None,
                        token_count: pair.token_count,
                    });
                }
            }
        }

        // 4. Evaluate Cluster-Level Limits
        for cluster in &scan_result.clone_clusters {
            let mut cluster_matches = false;
            for occ in &cluster.occurrences {
                let path = Path::new(&occ.file);
                for limit in &self.compiled_limits {
                    if path_matches_glob(&limit.matcher, path) {
                        cluster_matches = true;
                        if let Some(max_occ) = limit.rule.max_occurrences
                            && cluster.occurrences.len() > max_occ
                        {
                            let primary_file = cluster
                                .occurrences
                                .first()
                                .map(|o| o.file.clone())
                                .unwrap_or_default();
                            let start_line = cluster
                                .occurrences
                                .first()
                                .map(|o| o.start_line)
                                .unwrap_or(1);
                            let end_line =
                                cluster.occurrences.first().map(|o| o.end_line).unwrap_or(1);

                            violations.push(PolicyViolation {
                                rule_name: limit.rule.name.clone(),
                                rule_type: "limit".to_string(),
                                severity: limit.rule.severity,
                                message: format!(
                                    "Limit policy '{}' violated: cluster #{} has {} occurrences, \
                                     exceeding limit {}",
                                    limit.rule.name,
                                    cluster.id,
                                    cluster.occurrences.len(),
                                    max_occ
                                ),
                                file_a: primary_file,
                                start_line_a: start_line,
                                end_line_a: end_line,
                                file_b: None,
                                start_line_b: None,
                                end_line_b: None,
                                cluster_id: Some(cluster.id),
                                token_count: cluster.token_count,
                            });
                        }
                    }
                }
                if cluster_matches {
                    break;
                }
            }
        }

        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;

        for v in &violations {
            match v.severity {
                PolicySeverity::Error => error_count += 1,
                PolicySeverity::Warning => warning_count += 1,
                PolicySeverity::Info => info_count += 1,
            }
        }

        let passed = error_count == 0;
        let total_violations = violations.len();

        PolicyEvaluationResult {
            passed,
            total_violations,
            error_count,
            warning_count,
            info_count,
            violations,
        }
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
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CloneLocation, ClonePair, CloneType};

    #[test]
    fn test_policy_engine_parse_and_evaluate_boundary() {
        let toml_content = r#"
[[boundaries]]
name = "domain-isolation"
description = "No domain clones in presentation"
source = "src/domain/**"
forbidden_targets = ["src/presentation/**"]
severity = "error"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test".to_string(),
            total_files: 2,
            total_tokens: 500,
            total_clones: 1,
            total_clusters: 0,
            duplication_percentage: 10.0,
            dry_health_score: 90.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/domain/user.rs".to_string(),
                start_line_a: 10,
                end_line_a: 30,
                file_b: "src/presentation/user.rs".to_string(),
                start_line_b: 15,
                end_line_b: 35,
                token_count: 65,
                similarity: 1.0,
                fragment_hash: "hash1".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: Vec::new(),
            duration_ms: 50,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1);
        assert_eq!(eval.violations.len(), 1);
        assert_eq!(eval.violations[0].rule_name, "domain-isolation");
        assert_eq!(eval.violations[0].rule_type, "boundary");
    }

    #[test]
    fn test_policy_engine_zero_duplication_rule() {
        let toml_content = r#"
[[zero_duplication]]
name = "auth-clean"
pattern = "src/auth/**"
severity = "error"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-zd".to_string(),
            total_files: 2,
            total_tokens: 500,
            total_clones: 1,
            total_clusters: 0,
            duplication_percentage: 5.0,
            dry_health_score: 95.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/auth/token.rs".to_string(),
                start_line_a: 1,
                end_line_a: 20,
                file_b: "src/utils/token.rs".to_string(),
                start_line_b: 1,
                end_line_b: 20,
                token_count: 55,
                similarity: 1.0,
                fragment_hash: "hash2".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: Vec::new(),
            duration_ms: 20,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
        };

        let eval = engine.evaluate(&scan_result);
        assert!(!eval.passed);
        assert_eq!(eval.error_count, 1);
        assert_eq!(eval.violations[0].rule_name, "auth-clean");
    }

    #[test]
    fn test_policy_engine_limit_rule() {
        let toml_content = r#"
[[limits]]
name = "max-api-tokens"
pattern = "src/api/**"
max_tokens = 60
max_occurrences = 2
severity = "warning"
"#;
        let engine = toml_content
            .parse::<PolicyEngine>()
            .expect("Failed to parse policy TOML");

        let scan_result = ScanResult {
            scan_id: "test-limit".to_string(),
            total_files: 3,
            total_tokens: 1000,
            total_clones: 1,
            total_clusters: 1,
            duplication_percentage: 8.0,
            dry_health_score: 92.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/api/handler1.rs".to_string(),
                start_line_a: 10,
                end_line_a: 50,
                file_b: "src/api/handler2.rs".to_string(),
                start_line_b: 10,
                end_line_b: 50,
                token_count: 80, // Exceeds 60
                similarity: 1.0,
                fragment_hash: "hash3".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: vec![crate::types::CloneCluster {
                id: 1,
                clone_type: CloneType::Exact,
                token_count: 80,
                similarity: 1.0,
                fragment_hash: "hash3".to_string(),
                occurrences: vec![
                    CloneLocation {
                        file: "src/api/handler1.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                    CloneLocation {
                        file: "src/api/handler2.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                    CloneLocation {
                        file: "src/api/handler3.rs".to_string(),
                        start_line: 10,
                        end_line: 50,
                        author: None,
                    },
                ],
            }],
            duration_ms: 30,
            language_breakdown: Vec::new(),
            policy_violations: Vec::new(),
        };

        let eval = engine.evaluate(&scan_result);
        assert!(eval.passed); // Warnings only
        assert_eq!(eval.warning_count, 2); // 1 for pair token_count, 1 for cluster max_occurrences
        assert_eq!(eval.error_count, 0);
    }
}
