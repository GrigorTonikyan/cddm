#![forbid(unsafe_code)]

use super::clone::{CloneCluster, CloneType};
use super::policy::{PolicyEvaluationResult, PolicySeverity};
use super::scan::{LanguageStats, ScanResult};
use serde::{Deserialize, Serialize};

/// Compact cluster summary containing only essential occurrence locations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompactClusterSummary {
    pub cluster_id: usize,
    pub clone_type: CloneType,
    pub token_count: usize,
    pub occurrences_count: usize,
    pub primary_files: Vec<String>,
}

impl CompactClusterSummary {
    pub fn from_cluster(cluster: &CloneCluster) -> Self {
        let mut files: Vec<String> = cluster.occurrences.iter().map(|o| o.file.clone()).collect();
        files.sort();
        files.dedup();

        Self {
            cluster_id: cluster.id,
            clone_type: cluster.clone_type.clone(),
            token_count: cluster.token_count,
            occurrences_count: cluster.occurrences.len(),
            primary_files: files,
        }
    }
}

/// Compact representation of scan results designed to minimize context token usage for AI agents.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompactScanResult {
    pub summary_mode: bool,
    pub scan_id: String,
    pub total_files: usize,
    pub total_tokens: usize,
    pub total_clones: usize,
    pub total_clusters: usize,
    pub duplication_percentage: f64,
    pub dry_health_score: f64,
    pub duration_ms: u64,
    pub language_breakdown: Vec<LanguageStats>,
    pub policy_violations_count: usize,
    pub top_clusters: Vec<CompactClusterSummary>,
}

impl CompactScanResult {
    pub fn from_scan_result(res: &ScanResult, top_n: usize) -> Self {
        let mut sorted_clusters = res.clone_clusters.clone();
        sorted_clusters.sort_by(|a, b| {
            let weight_b = b.token_count * b.occurrences.len();
            let weight_a = a.token_count * a.occurrences.len();
            weight_b.cmp(&weight_a)
        });

        let top_clusters = sorted_clusters
            .into_iter()
            .take(top_n)
            .map(|c| CompactClusterSummary::from_cluster(&c))
            .collect();

        Self {
            summary_mode: true,
            scan_id: res.scan_id.clone(),
            total_files: res.total_files,
            total_tokens: res.total_tokens,
            total_clones: res.total_clones,
            total_clusters: res.total_clusters,
            duplication_percentage: res.duplication_percentage,
            dry_health_score: res.dry_health_score,
            duration_ms: res.duration_ms,
            language_breakdown: res.language_breakdown.clone(),
            policy_violations_count: res.policy_violations.len(),
            top_clusters,
        }
    }
}

/// Compact representation of a single architectural policy violation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompactPolicyViolation {
    pub rule_name: String,
    pub severity: PolicySeverity,
    pub file_a: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_b: Option<String>,
    pub message: String,
}

/// Compact representation of policy evaluation results.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompactPolicyResult {
    pub summary_mode: bool,
    pub passed: bool,
    pub total_violations: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub top_violations: Vec<CompactPolicyViolation>,
}

impl CompactPolicyResult {
    pub fn from_evaluation(eval: &PolicyEvaluationResult, top_n: usize) -> Self {
        let top_violations = eval
            .violations
            .iter()
            .take(top_n)
            .map(|v| CompactPolicyViolation {
                rule_name: v.rule_name.clone(),
                severity: v.severity,
                file_a: v.file_a.clone(),
                file_b: v.file_b.clone(),
                message: v.message.clone(),
            })
            .collect();

        Self {
            summary_mode: true,
            passed: eval.passed,
            total_violations: eval.total_violations,
            error_count: eval.error_count,
            warning_count: eval.warning_count,
            info_count: eval.info_count,
            top_violations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_scan_result_from_scan_result() {
        let mut scan_res = ScanResult {
            total_files: 10,
            total_tokens: 5000,
            total_clones: 2,
            total_clusters: 1,
            duplication_percentage: 4.5,
            dry_health_score: 95.5,
            ..Default::default()
        };

        let cluster = CloneCluster {
            id: 1,
            clone_type: CloneType::Exact,
            token_count: 100,
            similarity: 1.0,
            fragment_hash: "test_hash".to_string(),
            occurrences: vec![
                crate::types::CloneLocation {
                    file: "a.rs".to_string(),
                    start_line: 1,
                    end_line: 10,
                    author: None,
                },
                crate::types::CloneLocation {
                    file: "b.rs".to_string(),
                    start_line: 1,
                    end_line: 10,
                    author: None,
                },
            ],
        };
        scan_res.clone_clusters.push(cluster);

        let compact = CompactScanResult::from_scan_result(&scan_res, 5);
        assert!(compact.summary_mode);
        assert_eq!(compact.total_files, 10);
        assert_eq!(compact.top_clusters.len(), 1);
        assert_eq!(compact.top_clusters[0].primary_files, vec!["a.rs", "b.rs"]);
    }

    #[test]
    fn test_compact_policy_result() {
        let eval = PolicyEvaluationResult {
            passed: true,
            total_violations: 1,
            error_count: 0,
            warning_count: 1,
            info_count: 0,
            violations: vec![crate::types::PolicyViolation {
                rule_name: "test-rule".to_string(),
                rule_type: "boundary".to_string(),
                severity: PolicySeverity::Warning,
                message: "violation".to_string(),
                file_a: "src/a.rs".to_string(),
                start_line_a: 1,
                end_line_a: 5,
                file_b: None,
                start_line_b: None,
                end_line_b: None,
                cluster_id: None,
                token_count: 50,
            }],
        };

        let compact = CompactPolicyResult::from_evaluation(&eval, 5);
        assert!(compact.summary_mode);
        assert!(compact.passed);
        assert_eq!(compact.total_violations, 1);
        assert_eq!(compact.top_violations.len(), 1);
        assert_eq!(compact.top_violations[0].rule_name, "test-rule");
    }
}
