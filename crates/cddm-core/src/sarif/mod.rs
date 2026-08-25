#![forbid(unsafe_code)]

pub mod builder;
pub mod types;

pub use builder::{
    build_rule_catalog, generate_sarif_json, generate_sarif_report, make_sarif_rule,
};
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        ClonePair, CloneType, LanguageStats, PolicySeverity, PolicyViolation, ScanResult,
    };

    fn make_sample_scan_result() -> ScanResult {
        ScanResult {
            scan_id: "test-scan-001".to_string(),
            total_files: 2,
            total_tokens: 150,
            total_clones: 1,
            total_clusters: 1,
            duplication_percentage: 25.0,
            dry_health_score: 85.0,
            clone_pairs: vec![ClonePair {
                file_a: "src/auth/login.rs".to_string(),
                start_line_a: 10,
                end_line_a: 25,
                file_b: "src/auth/register.rs".to_string(),
                start_line_b: 30,
                end_line_b: 45,
                token_count: 50,
                similarity: 1.0,
                fragment_hash: "abcd1234ef567890".to_string(),
                clone_type: CloneType::Exact,
                author_a: Some("Alice <alice@example.com>".to_string()),
                author_b: Some("Bob <bob@example.com>".to_string()),
            }],
            clone_clusters: vec![],
            duration_ms: 42,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 2,
                tokens: 150,
                clones: 1,
            }],
            policy_violations: Vec::new(),
        }
    }

    #[test]
    fn test_sarif_report_generation() {
        let scan_result = make_sample_scan_result();
        let report = generate_sarif_report(&scan_result);

        assert_eq!(report.schema, SARIF_SCHEMA_URI);
        assert_eq!(report.version, SARIF_VERSION);
        assert_eq!(report.runs.len(), 1);

        let run = &report.runs[0];
        assert_eq!(run.tool.driver.name, TOOL_NAME);
        assert_eq!(run.tool.driver.rules.len(), 7);
        assert_eq!(run.results.len(), 1);

        let result = &run.results[0];
        assert_eq!(result.rule_id, sarif_rules::RULE_ID_EXACT);
        assert_eq!(result.level, sarif_severity::WARNING);
        assert_eq!(result.locations.len(), 1);
        assert_eq!(
            result.locations[0].physical_location.artifact_location.uri,
            "src/auth/login.rs"
        );
        assert_eq!(result.locations[0].physical_location.region.start_line, 10);
        assert_eq!(result.locations[0].physical_location.region.end_line, 25);

        assert_eq!(result.related_locations.len(), 1);
        assert_eq!(
            result.related_locations[0]
                .physical_location
                .artifact_location
                .uri,
            "src/auth/register.rs"
        );
        assert_eq!(
            result.related_locations[0]
                .physical_location
                .region
                .start_line,
            30
        );
        assert_eq!(
            result.related_locations[0]
                .physical_location
                .region
                .end_line,
            45
        );

        assert_eq!(
            result.properties.author_a.as_deref(),
            Some("Alice <alice@example.com>")
        );
        assert_eq!(
            result.properties.author_b.as_deref(),
            Some("Bob <bob@example.com>")
        );
        assert_eq!(result.properties.token_count, 50);
        assert_eq!(result.properties.similarity, 1.0);
    }

    #[test]
    fn test_all_clone_types_mapped() {
        let mut scan_result = make_sample_scan_result();
        scan_result.clone_pairs = vec![
            ClonePair {
                file_a: "a.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "b.rs".to_string(),
                start_line_b: 1,
                end_line_b: 10,
                token_count: 30,
                similarity: 1.0,
                fragment_hash: "hash1".to_string(),
                clone_type: CloneType::Exact,
                author_a: None,
                author_b: None,
            },
            ClonePair {
                file_a: "c.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "d.rs".to_string(),
                start_line_b: 1,
                end_line_b: 10,
                token_count: 30,
                similarity: 0.9,
                fragment_hash: "hash2".to_string(),
                clone_type: CloneType::Renamed,
                author_a: None,
                author_b: None,
            },
            ClonePair {
                file_a: "e.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "f.rs".to_string(),
                start_line_b: 1,
                end_line_b: 10,
                token_count: 30,
                similarity: 0.8,
                fragment_hash: "hash3".to_string(),
                clone_type: CloneType::NearMiss,
                author_a: None,
                author_b: None,
            },
            ClonePair {
                file_a: "g.rs".to_string(),
                start_line_a: 1,
                end_line_a: 10,
                file_b: "h.rs".to_string(),
                start_line_b: 1,
                end_line_b: 10,
                token_count: 30,
                similarity: 0.7,
                fragment_hash: "hash4".to_string(),
                clone_type: CloneType::Semantic,
                author_a: None,
                author_b: None,
            },
        ];

        let report = generate_sarif_report(&scan_result);
        let results = &report.runs[0].results;
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].rule_id, sarif_rules::RULE_ID_EXACT);
        assert_eq!(results[1].rule_id, sarif_rules::RULE_ID_RENAMED);
        assert_eq!(results[2].rule_id, sarif_rules::RULE_ID_NEAR_MISS);
        assert_eq!(results[3].rule_id, sarif_rules::RULE_ID_SEMANTIC);
    }

    #[test]
    fn test_sarif_policy_violations_mapping() {
        let mut scan_result = make_sample_scan_result();
        scan_result.clone_pairs = Vec::new();
        scan_result.policy_violations = vec![
            PolicyViolation {
                rule_name: "no-domain-web".to_string(),
                rule_type: "boundary".to_string(),
                severity: PolicySeverity::Error,
                file_a: "domain.rs".to_string(),
                start_line_a: 10,
                end_line_a: 20,
                file_b: Some("web.rs".to_string()),
                start_line_b: Some(30),
                end_line_b: Some(40),
                message: "Boundary violation".to_string(),
                cluster_id: None,
                token_count: 50,
            },
            PolicyViolation {
                rule_name: "strict-core".to_string(),
                rule_type: "zero_duplication".to_string(),
                severity: PolicySeverity::Warning,
                file_a: "core.rs".to_string(),
                start_line_a: 5,
                end_line_a: 15,
                file_b: None,
                start_line_b: None,
                end_line_b: None,
                message: "Zero dup violation".to_string(),
                cluster_id: None,
                token_count: 25,
            },
        ];

        let report = generate_sarif_report(&scan_result);
        let results = &report.runs[0].results;
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].rule_id, sarif_rules::RULE_ID_BOUNDARY);
        assert_eq!(results[0].level, sarif_severity::ERROR);
        assert_eq!(results[1].rule_id, sarif_rules::RULE_ID_ZERO_DUP);
        assert_eq!(results[1].level, sarif_severity::WARNING);
    }

    #[test]
    fn test_sarif_json_serde_roundtrip() {
        let scan_result = make_sample_scan_result();
        let sarif_val = generate_sarif_json(&scan_result);
        assert!(sarif_val.is_object());
        assert_eq!(sarif_val["version"], SARIF_VERSION);

        let json_str = serde_json::to_string(&sarif_val).unwrap();
        let deserialized: SarifReport = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.version, SARIF_VERSION);
    }
}
