#![forbid(unsafe_code)]

use super::compiled::{
    CompiledBoundary, CompiledLimit, CompiledZeroDuplication, path_matches_glob,
};
use crate::types::{PolicyEvaluationResult, PolicySeverity, PolicyViolation, ScanResult};
use std::path::Path;

/// Evaluates all active policy rules against a scan result.
pub fn evaluate_scan_policies(
    scan_result: &ScanResult,
    compiled_boundaries: &[CompiledBoundary],
    compiled_zero_duplication: &[CompiledZeroDuplication],
    compiled_limits: &[CompiledLimit],
) -> PolicyEvaluationResult {
    let mut violations = Vec::new();

    // 1. Evaluate Boundary Rules across clone pairs
    for pair in &scan_result.clone_pairs {
        let path_a = Path::new(&pair.file_a);
        let path_b = Path::new(&pair.file_b);

        for boundary in compiled_boundaries {
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
                                "Architecture boundary '{}' violated: duplication across source \
                                 '{}' ({}) and target '{}' ({})",
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
                                "Architecture boundary '{}' violated: duplication across source \
                                 '{}' ({}) and target '{}' ({})",
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
        for zero_dup in compiled_zero_duplication {
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
        for limit in compiled_limits {
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
                        "Limit policy '{}' violated: token count {} exceeds maximum allowed limit \
                         {}",
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
            for limit in compiled_limits {
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
                        let end_line = cluster.occurrences.first().map(|o| o.end_line).unwrap_or(1);

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
