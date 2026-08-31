use crate::types::{ScanResult, WorkflowPlatform};

/// Generates a GitHub, GitLab, or generic Markdown summary comment for CI/CD pull requests.
pub fn generate_pr_markdown_comment(
    scan_result: &ScanResult,
    fail_threshold: f64,
    platform: WorkflowPlatform,
) -> String {
    let mut out = String::new();
    let is_passed = scan_result.duplication_percentage <= fail_threshold;
    let status_tag = if is_passed { "[PASS]" } else { "[FAIL]" };

    out.push_str(&format!(
        "## CDDM Code De-Duplication Quality Gate {}\n\n",
        status_tag
    ));

    if is_passed {
        out.push_str(&format!(
            "> **Quality Gate Passed**: Code duplication is **{:.2}%**, which is strictly below \
             the failure threshold of **{:.2}%**.\n\n",
            scan_result.duplication_percentage, fail_threshold
        ));
    } else {
        out.push_str(&format!(
            "> **Quality Gate Failed**: Code duplication is **{:.2}%**, which exceeds the failure \
             threshold of **{:.2}%**.\n\n",
            scan_result.duplication_percentage, fail_threshold
        ));
    }

    out.push_str("### Scan Metrics Summary\n\n");
    out.push_str("| Metric | Value |\n");
    out.push_str("| :--- | :--- |\n");
    out.push_str(&format!(
        "| **DRY Health Score** | `{:.1} / 100.0` |\n",
        scan_result.dry_health_score
    ));
    out.push_str(&format!(
        "| **Duplication Rate** | `{:.2}%` (Threshold: `{:.2}%`) |\n",
        scan_result.duplication_percentage, fail_threshold
    ));
    out.push_str(&format!(
        "| **Total Scanned Files** | `{}` |\n",
        scan_result.total_files
    ));
    out.push_str(&format!(
        "| **Total Scanned Tokens** | `{}` |\n",
        scan_result.total_tokens
    ));
    out.push_str(&format!(
        "| **Clone Pairs Detected** | `{}` |\n",
        scan_result.total_clones
    ));
    out.push_str(&format!(
        "| **N-Way Clusters** | `{}` |\n\n",
        scan_result.total_clusters
    ));

    if !scan_result.clone_pairs.is_empty() {
        out.push_str("### Top Duplication Clones\n\n");
        out.push_str("| Target A | Target B | Tokens | Similarity | Classification |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");

        for pair in scan_result.clone_pairs.iter().take(5) {
            let loc_a = format!("{}:{}-{}", pair.file_a, pair.start_line_a, pair.end_line_a);
            let loc_b = format!("{}:{}-{}", pair.file_b, pair.start_line_b, pair.end_line_b);
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{:.1}%` | `{:?}` |\n",
                loc_a,
                loc_b,
                pair.token_count,
                pair.similarity * 100.0,
                pair.clone_type
            ));
        }
        out.push('\n');
    }

    if !scan_result.policy_violations.is_empty() {
        out.push_str("### Architectural Policy Violations\n\n");
        out.push_str("| Rule | Type | Severity | Location | Message |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");

        for v in scan_result.policy_violations.iter().take(5) {
            let loc = format!("{}:{}-{}", v.file_a, v.start_line_a, v.end_line_a);
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} |\n",
                v.rule_name, v.rule_type, v.severity, loc, v.message
            ));
        }
        out.push('\n');
    }

    out.push_str("### Developer Action Guide\n\n");
    out.push_str("To reproduce this analysis locally:\n");
    out.push_str("```bash\n");
    out.push_str(&format!(
        "cargo run -p cddm-cli -- scan . --min-tokens 50 --fail-threshold {:.1}\n",
        fail_threshold
    ));
    out.push_str("```\n\n");

    out.push_str("To synthesize automated refactoring patches:\n");
    out.push_str("```bash\n");
    out.push_str("cargo run -p cddm-cli -- refactor --pair 1 --prompt\n");
    out.push_str("```\n\n");

    match platform {
        WorkflowPlatform::Gitea => {
            out.push_str("---\n*Generated automatically by [CDDM (Code De-Duplication Meister)](https://git.gt-web-dev.com/gt-dev/cddm) Gitea Action.*\n");
        }
        WorkflowPlatform::GitHub => {
            out.push_str("---\n*Generated automatically by [CDDM (Code De-Duplication Meister)](https://git.gt-web-dev.com/gt-dev/cddm) GitHub Action.*\n");
        }
        WorkflowPlatform::GitLab => {
            out.push_str("---\n*Generated automatically by [CDDM (Code De-Duplication Meister)](https://git.gt-web-dev.com/gt-dev/cddm) GitLab CI Pipeline.*\n");
        }
        WorkflowPlatform::Azure => {
            out.push_str("---\n*Generated automatically by [CDDM (Code De-Duplication Meister)](https://git.gt-web-dev.com/gt-dev/cddm) Azure DevOps Pipeline.*\n");
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ClonePair, CloneType};

    #[test]
    fn test_generate_pr_markdown_comment_passed() {
        let scan_result = ScanResult {
            scan_id: "test-scan-1".to_string(),
            total_files: 42,
            total_tokens: 10000,
            total_clones: 2,
            total_clusters: 1,
            duplication_percentage: 5.0,
            dry_health_score: 95.0,
            clone_pairs: vec![],
            clone_clusters: vec![],
            duration_ms: 120,
            language_breakdown: vec![],
            policy_violations: vec![],
        };

        let comment = generate_pr_markdown_comment(&scan_result, 15.0, WorkflowPlatform::GitHub);
        assert!(comment.contains("[PASS]"));
        assert!(comment.contains("Quality Gate Passed"));
        assert!(comment.contains("95.0 / 100.0"));
        assert!(comment.contains("GitHub Action"));
    }

    #[test]
    fn test_generate_pr_markdown_comment_failed_with_clones() {
        let scan_result = ScanResult {
            scan_id: "test-scan-2".to_string(),
            total_files: 100,
            total_tokens: 50000,
            total_clones: 1,
            total_clusters: 1,
            duplication_percentage: 24.0,
            dry_health_score: 76.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/a.rs".to_string(),
                start_line_a: 10,
                end_line_a: 30,
                file_b: "src/b.rs".to_string(),
                start_line_b: 15,
                end_line_b: 35,
                token_count: 85,
                similarity: 1.0,
                fragment_hash: "hash123".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            }],
            clone_clusters: vec![],
            duration_ms: 250,
            language_breakdown: vec![],
            policy_violations: vec![],
        };

        let comment = generate_pr_markdown_comment(&scan_result, 15.0, WorkflowPlatform::GitLab);
        assert!(comment.contains("[FAIL]"));
        assert!(comment.contains("Quality Gate Failed"));
        assert!(comment.contains("src/a.rs:10-30"));
        assert!(comment.contains("src/b.rs:15-35"));
        assert!(comment.contains("GitLab CI Pipeline"));
    }
}
