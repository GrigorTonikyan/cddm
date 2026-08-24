use crate::types::{ClonePair, CloneType, PolicySeverity, PolicyViolation, ScanResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Official OASIS SARIF 2.1.0 JSON Schema URI.
pub const SARIF_SCHEMA_URI: &str =
    "https://docs.oasis-open.org/sarif/sarif/v2.1.0/os/schemas/sarif-schema-2.1.0.json";

/// Supported SARIF specification version.
pub const SARIF_VERSION: &str = "2.1.0";

/// Static analysis tool display name.
pub const TOOL_NAME: &str = "CDDM";

/// Tool project homepage and information URI.
pub const TOOL_INFORMATION_URI: &str = "https://github.com/GrigorTonikyan/cddm";

/// SARIF Rule identifiers for duplicate code classifications.
pub mod sarif_rules {
    pub const RULE_ID_EXACT: &str = "CDDM001";
    pub const RULE_ID_RENAMED: &str = "CDDM002";
    pub const RULE_ID_NEAR_MISS: &str = "CDDM003";
    pub const RULE_ID_SEMANTIC: &str = "CDDM004";
    pub const RULE_ID_BOUNDARY: &str = "CDDM_BOUNDARY";
    pub const RULE_ID_ZERO_DUP: &str = "CDDM_ZERO_DUP";
    pub const RULE_ID_LIMIT: &str = "CDDM_LIMIT";
}

/// SARIF taxonomy tags for static analysis categorizations.
pub mod sarif_tags {
    pub const MAINTAINABILITY: &str = "maintainability";
    pub const QUALITY: &str = "quality";
    pub const DUPLICATION: &str = "duplication";
}

/// SARIF problem severity descriptors for GitHub Code Scanning.
pub mod sarif_severity {
    pub const WARNING: &str = "warning";
    pub const ERROR: &str = "error";
    pub const RECOMMENDATION: &str = "recommendation";
}

/// SARIF precision descriptors for GitHub Code Scanning.
pub mod sarif_precision {
    pub const VERY_HIGH: &str = "very-high";
}

/// Top-level OASIS SARIF v2.1.0 Report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifReport {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

/// A single execution run of the static analysis tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub invocations: Vec<SarifInvocation>,
    pub results: Vec<SarifResult>,
}

/// Tool metadata descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

/// Analysis driver details and rule catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifDriver {
    pub name: String,
    #[serde(rename = "semanticVersion")]
    pub semantic_version: String,
    #[serde(rename = "informationUri")]
    pub information_uri: String,
    pub rules: Vec<SarifRule>,
}

/// Rule reporting descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMessage,
    #[serde(rename = "fullDescription")]
    pub full_description: SarifMessage,
    #[serde(rename = "defaultConfiguration")]
    pub default_configuration: SarifDefaultConfiguration,
    pub help: SarifMultiformatMessage,
    pub properties: SarifRuleProperties,
}

/// Multi-format message container supporting plain text and markdown.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifMultiformatMessage {
    pub text: String,
    pub markdown: String,
}

/// Default configuration for a rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifDefaultConfiguration {
    pub level: String,
}

/// Rule properties for GitHub Code Scanning integration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifRuleProperties {
    pub tags: Vec<String>,
    pub precision: String,
    #[serde(rename = "problem.severity")]
    pub problem_severity: String,
}

/// Tool execution invocation record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifInvocation {
    #[serde(rename = "executionSuccessful")]
    pub execution_successful: bool,
    #[serde(rename = "toolExecutionNotifications")]
    pub tool_execution_notifications: Vec<String>,
}

/// A specific finding emitted by the analysis tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    #[serde(rename = "ruleIndex")]
    pub rule_index: usize,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
    #[serde(rename = "relatedLocations", skip_serializing_if = "Vec::is_empty")]
    pub related_locations: Vec<SarifRelatedLocation>,
    #[serde(
        rename = "partialFingerprints",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub partial_fingerprints: HashMap<String, String>,
    pub properties: SarifResultProperties,
}

/// Plaintext message container.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifMessage {
    pub text: String,
}

/// Primary finding location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

/// Physical file path and line region.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

/// Relative artifact URI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifArtifactLocation {
    pub uri: String,
    #[serde(rename = "uriBaseId", skip_serializing_if = "Option::is_none")]
    pub uri_base_id: Option<String>,
}

/// Source line and column boundary region.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "endLine")]
    pub end_line: usize,
}

/// Secondary/counterpart location for clone pairs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SarifRelatedLocation {
    pub id: usize,
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
    pub message: SarifMessage,
}

/// Additional metadata properties attached to a result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifResultProperties {
    #[serde(rename = "tokenCount")]
    pub token_count: usize,
    pub similarity: f64,
    #[serde(rename = "cloneType")]
    pub clone_type: String,
    #[serde(rename = "fragmentHash")]
    pub fragment_hash: String,
    #[serde(rename = "authorA", skip_serializing_if = "Option::is_none")]
    pub author_a: Option<String>,
    #[serde(rename = "authorB", skip_serializing_if = "Option::is_none")]
    pub author_b: Option<String>,
}

/// Helper to construct a standard SARIF rule descriptor.
fn make_sarif_rule(
    id: &str,
    name: &str,
    short_desc: &str,
    full_desc: &str,
    help_text: &str,
    help_md: &str,
) -> SarifRule {
    SarifRule {
        id: id.to_string(),
        name: name.to_string(),
        short_description: SarifMessage {
            text: short_desc.to_string(),
        },
        full_description: SarifMessage {
            text: full_desc.to_string(),
        },
        default_configuration: SarifDefaultConfiguration {
            level: sarif_severity::WARNING.to_string(),
        },
        help: SarifMultiformatMessage {
            text: help_text.to_string(),
            markdown: help_md.to_string(),
        },
        properties: SarifRuleProperties {
            tags: vec![
                sarif_tags::MAINTAINABILITY.to_string(),
                sarif_tags::QUALITY.to_string(),
                sarif_tags::DUPLICATION.to_string(),
            ],
            precision: sarif_precision::VERY_HIGH.to_string(),
            problem_severity: sarif_severity::WARNING.to_string(),
        },
    }
}

/// Build the standard rule catalog for CDDM duplicate classifications.
fn build_rule_catalog() -> Vec<SarifRule> {
    vec![
        make_sarif_rule(
            sarif_rules::RULE_ID_EXACT,
            "ExactCodeClone",
            "Exact duplicate code clone (Type 1)",
            "Exact duplicate code fragment with identical tokens and structure detected across \
             files or functions.",
            "Extract common code into a shared helper function or utility module.",
            "### Remediation\n\nExtract this identical logic into a shared helper function, \
             module, or common crate to preserve DRY modularity and prevent divergent changes.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_RENAMED,
            "RenamedIdentifierClone",
            "Renamed identifier code clone (Type 2)",
            "Structurally identical code fragment with renamed identifiers, variables, or \
             literals detected.",
            "Parameterize the varying identifiers into function arguments of a shared helper.",
            "### Remediation\n\nParameterize varying identifiers and literal values into \
             arguments of a unified function or generic template.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_NEAR_MISS,
            "NearMissCodeClone",
            "Near-miss code clone (Type 3)",
            "Similar code fragment with small statement variations or inserted/deleted lines \
             detected.",
            "Consolidate variations using strategy callbacks or template methods.",
            "### Remediation\n\nRefactor differing statements into strategy closures, callbacks, \
             or template hooks to unify the overarching workflow.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_SEMANTIC,
            "SemanticCodeClone",
            "Semantic AST code clone (Type 4)",
            "Semantically equivalent logic performing identical operations with different \
             syntactic patterns.",
            "Standardize on a single canonical architectural pattern across modules.",
            "### Remediation\n\nStandardize divergent implementations on a single canonical \
             architecture or standard library primitive.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_BOUNDARY,
            "ArchitectureBoundaryViolation",
            "Architecture Boundary Policy Violation",
            "Code duplication detected across disallowed architectural domain boundaries.",
            "Refactor shared logic into an approved common module to preserve boundary \
             encapsulation.",
            "### Remediation\n\nMove common functionality into a designated shared kernel or \
             domain module rather than duplicating logic across isolation boundaries.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_ZERO_DUP,
            "ZeroDuplicationZoneViolation",
            "Zero Duplication Zone Policy Violation",
            "Code duplication detected inside a protected security or critical zero-duplication \
             path.",
            "Eliminate duplication within the protected module to maintain strict maintainability \
             and security standards.",
            "### Remediation\n\nRefactor and unify code within this security-critical subsystem \
             to guarantee zero copy-paste logic.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_LIMIT,
            "CloneLimitPolicyViolation",
            "Clone Limit Policy Violation",
            "Code clone exceeds configured maximum token limit or cluster occurrence count.",
            "Consolidate large or highly duplicated clusters into reusable abstractions.",
            "### Remediation\n\nExtract and parameterize this large or frequently repeated code \
             fragment to reduce duplication density.",
        ),
    ]
}

/// Computes a stable Blake3 line hash for SARIF partial fingerprints.
fn compute_line_hash(file_path: &str, start_line: usize, end_line: usize) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(file_path.as_bytes());
    hasher.update(&start_line.to_le_bytes());
    hasher.update(&end_line.to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

/// Maps a CloneType to its rule ID and index in the rule catalog.
fn map_clone_type_to_rule(clone_type: &CloneType) -> (&'static str, usize) {
    match clone_type {
        CloneType::Exact => (sarif_rules::RULE_ID_EXACT, 0),
        CloneType::Renamed => (sarif_rules::RULE_ID_RENAMED, 1),
        CloneType::NearMiss => (sarif_rules::RULE_ID_NEAR_MISS, 2),
        CloneType::Semantic => (sarif_rules::RULE_ID_SEMANTIC, 3),
    }
}

/// Converts a ClonePair into a standard SARIF Result.
fn map_clone_pair_to_sarif_result(pair: &ClonePair) -> SarifResult {
    let (rule_id, rule_index) = map_clone_type_to_rule(&pair.clone_type);

    let message_text = format!(
        "Duplicate code fragment ({} tokens, {:.1}% similarity) detected in {} (lines {}-{}) \
         matching counterpart in {} (lines {}-{}).",
        pair.token_count,
        pair.similarity * 100.0,
        pair.file_a,
        pair.start_line_a,
        pair.end_line_a,
        pair.file_b,
        pair.start_line_b,
        pair.end_line_b,
    );

    let primary_location = SarifLocation {
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: pair.file_a.clone(),
                uri_base_id: Some("%SRCROOT%".to_string()),
            },
            region: SarifRegion {
                start_line: pair.start_line_a,
                end_line: pair.end_line_a,
            },
        },
    };

    let related_location = SarifRelatedLocation {
        id: 1,
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: pair.file_b.clone(),
                uri_base_id: Some("%SRCROOT%".to_string()),
            },
            region: SarifRegion {
                start_line: pair.start_line_b,
                end_line: pair.end_line_b,
            },
        },
        message: SarifMessage {
            text: "Counterpart duplicate code fragment location".to_string(),
        },
    };

    let mut partial_fingerprints = HashMap::new();
    partial_fingerprints.insert(
        "primaryLocationLineHash".to_string(),
        compute_line_hash(&pair.file_a, pair.start_line_a, pair.end_line_a),
    );

    SarifResult {
        rule_id: rule_id.to_string(),
        rule_index,
        level: sarif_severity::WARNING.to_string(),
        message: SarifMessage { text: message_text },
        locations: vec![primary_location],
        related_locations: vec![related_location],
        partial_fingerprints,
        properties: SarifResultProperties {
            token_count: pair.token_count,
            similarity: pair.similarity,
            clone_type: format!("{:?}", pair.clone_type),
            fragment_hash: pair.fragment_hash.clone(),
            author_a: pair.author_a.clone(),
            author_b: pair.author_b.clone(),
        },
    }
}

/// Converts a PolicyViolation into a standard SARIF Result.
fn map_policy_violation_to_sarif_result(v: &PolicyViolation) -> SarifResult {
    let (rule_id, rule_index) = match v.rule_type.as_str() {
        "boundary" => (sarif_rules::RULE_ID_BOUNDARY, 4),
        "zero_duplication" => (sarif_rules::RULE_ID_ZERO_DUP, 5),
        _ => (sarif_rules::RULE_ID_LIMIT, 6),
    };

    let level = match v.severity {
        PolicySeverity::Error => sarif_severity::ERROR.to_string(),
        PolicySeverity::Warning => sarif_severity::WARNING.to_string(),
        PolicySeverity::Info => sarif_severity::RECOMMENDATION.to_string(),
    };

    let primary_location = SarifLocation {
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: v.file_a.clone(),
                uri_base_id: Some("%SRCROOT%".to_string()),
            },
            region: SarifRegion {
                start_line: v.start_line_a,
                end_line: v.end_line_a,
            },
        },
    };

    let mut related_locations = Vec::new();
    if let (Some(file_b), Some(start_b), Some(end_b)) = (&v.file_b, v.start_line_b, v.end_line_b) {
        related_locations.push(SarifRelatedLocation {
            id: 1,
            physical_location: SarifPhysicalLocation {
                artifact_location: SarifArtifactLocation {
                    uri: file_b.clone(),
                    uri_base_id: Some("%SRCROOT%".to_string()),
                },
                region: SarifRegion {
                    start_line: start_b,
                    end_line: end_b,
                },
            },
            message: SarifMessage {
                text: "Offending counterpart location".to_string(),
            },
        });
    }

    let mut partial_fingerprints = HashMap::new();
    partial_fingerprints.insert(
        "primaryLocationLineHash".to_string(),
        compute_line_hash(&v.file_a, v.start_line_a, v.end_line_a),
    );

    SarifResult {
        rule_id: rule_id.to_string(),
        rule_index,
        level,
        message: SarifMessage {
            text: v.message.clone(),
        },
        locations: vec![primary_location],
        related_locations,
        partial_fingerprints,
        properties: SarifResultProperties {
            token_count: v.token_count,
            similarity: 1.0,
            clone_type: v.rule_type.clone(),
            fragment_hash: String::new(),
            author_a: None,
            author_b: None,
        },
    }
}

/// Generates a fully compliant OASIS SARIF v2.1.0 report struct from a CDDM ScanResult.
pub fn generate_sarif_report(result: &ScanResult) -> SarifReport {
    let rules = build_rule_catalog();
    let mut sarif_results: Vec<SarifResult> = result
        .clone_pairs
        .iter()
        .map(map_clone_pair_to_sarif_result)
        .collect();

    for violation in &result.policy_violations {
        sarif_results.push(map_policy_violation_to_sarif_result(violation));
    }

    SarifReport {
        schema: SARIF_SCHEMA_URI.to_string(),
        version: SARIF_VERSION.to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: TOOL_NAME.to_string(),
                    semantic_version: env!("CARGO_PKG_VERSION").to_string(),
                    information_uri: TOOL_INFORMATION_URI.to_string(),
                    rules,
                },
            },
            invocations: vec![SarifInvocation {
                execution_successful: true,
                tool_execution_notifications: Vec::new(),
            }],
            results: sarif_results,
        }],
    }
}

/// Generates a serde_json::Value representation of the SARIF 2.1.0 report.
pub fn generate_sarif_json(result: &ScanResult) -> serde_json::Value {
    let report = generate_sarif_report(result);
    serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ClonePair, CloneType, LanguageStats, ScanResult};

    fn create_test_scan_result() -> ScanResult {
        ScanResult {
            scan_id: "test-scan-123".to_string(),
            total_files: 5,
            total_tokens: 1200,
            total_clones: 2,
            total_clusters: 2,
            duplication_percentage: 8.5,
            dry_health_score: 91.5,
            policy_violations: vec![],
            clone_pairs: vec![
                ClonePair {
                    file_a: "src/auth/login.rs".to_string(),
                    start_line_a: 10,
                    end_line_a: 25,
                    file_b: "src/auth/register.rs".to_string(),
                    start_line_b: 15,
                    end_line_b: 30,
                    token_count: 65,
                    similarity: 1.0,
                    fragment_hash: "hash_exact_123".to_string(),
                    clone_type: CloneType::Exact,
                    author_a: Some("Author A".to_string()),
                    author_b: Some("Author B".to_string()),
                },
                ClonePair {
                    file_a: "src/api/user.rs".to_string(),
                    start_line_a: 40,
                    end_line_a: 60,
                    file_b: "src/api/admin.rs".to_string(),
                    start_line_b: 45,
                    end_line_b: 65,
                    token_count: 80,
                    similarity: 0.95,
                    fragment_hash: "hash_renamed_456".to_string(),
                    clone_type: CloneType::Renamed,
                    author_a: None,
                    author_b: None,
                },
            ],
            clone_clusters: vec![],
            duration_ms: 42,
            language_breakdown: vec![LanguageStats {
                language: "Rust".to_string(),
                files: 5,
                tokens: 1200,
                clones: 2,
            }],
        }
    }

    #[test]
    fn test_sarif_report_generation() {
        let scan_result = create_test_scan_result();
        let sarif = generate_sarif_report(&scan_result);

        assert_eq!(sarif.schema, SARIF_SCHEMA_URI);
        assert_eq!(sarif.version, SARIF_VERSION);
        assert_eq!(sarif.runs.len(), 1);

        let run = &sarif.runs[0];
        assert_eq!(run.tool.driver.name, TOOL_NAME);
        assert_eq!(run.tool.driver.rules.len(), 7);
        assert_eq!(run.results.len(), 2);

        // Verify result 1 (Exact clone)
        let res1 = &run.results[0];
        assert_eq!(res1.rule_id, sarif_rules::RULE_ID_EXACT);
        assert_eq!(res1.rule_index, 0);
        assert_eq!(
            res1.locations[0].physical_location.artifact_location.uri,
            "src/auth/login.rs"
        );
        assert_eq!(res1.locations[0].physical_location.region.start_line, 10);
        assert_eq!(res1.locations[0].physical_location.region.end_line, 25);
        assert_eq!(
            res1.related_locations[0]
                .physical_location
                .artifact_location
                .uri,
            "src/auth/register.rs"
        );
        assert_eq!(res1.properties.token_count, 65);
        assert_eq!(res1.properties.clone_type, "Exact");
        assert!(
            res1.partial_fingerprints
                .contains_key("primaryLocationLineHash")
        );

        // Verify result 2 (Renamed clone)
        let res2 = &run.results[1];
        assert_eq!(res2.rule_id, sarif_rules::RULE_ID_RENAMED);
        assert_eq!(res2.rule_index, 1);
        assert_eq!(
            res2.locations[0].physical_location.artifact_location.uri,
            "src/api/user.rs"
        );
        assert_eq!(res2.properties.token_count, 80);
        assert_eq!(res2.properties.clone_type, "Renamed");
    }

    #[test]
    fn test_sarif_json_serde_roundtrip() {
        let scan_result = create_test_scan_result();
        let sarif_json = generate_sarif_json(&scan_result);

        assert_eq!(sarif_json["version"], SARIF_VERSION);
        assert_eq!(sarif_json["runs"][0]["tool"]["driver"]["name"], TOOL_NAME);
        assert_eq!(
            sarif_json["runs"][0]["results"].as_array().unwrap().len(),
            2
        );

        let json_str = serde_json::to_string(&sarif_json).unwrap();
        let deserialized: SarifReport = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.runs.len(), 1);
        assert_eq!(deserialized.runs[0].results.len(), 2);
    }

    #[test]
    fn test_sarif_policy_violations_mapping() {
        let mut scan_result = create_test_scan_result();
        scan_result.policy_violations = vec![
            PolicyViolation {
                rule_name: "domain-isolation".to_string(),
                rule_type: "boundary".to_string(),
                severity: PolicySeverity::Error,
                message: "Boundary violation between domain and presentation".to_string(),
                file_a: "src/domain/user.rs".to_string(),
                start_line_a: 10,
                end_line_a: 20,
                file_b: Some("src/presentation/user.rs".to_string()),
                start_line_b: Some(30),
                end_line_b: Some(40),
                cluster_id: None,
                token_count: 50,
            },
            PolicyViolation {
                rule_name: "auth-zero-dup".to_string(),
                rule_type: "zero_duplication".to_string(),
                severity: PolicySeverity::Error,
                message: "Zero duplication violation in auth".to_string(),
                file_a: "src/auth/token.rs".to_string(),
                start_line_a: 5,
                end_line_a: 15,
                file_b: None,
                start_line_b: None,
                end_line_b: None,
                cluster_id: None,
                token_count: 45,
            },
        ];

        let sarif = generate_sarif_report(&scan_result);
        assert_eq!(sarif.runs[0].results.len(), 4); // 2 clone pairs + 2 policy violations
        let boundary_res = sarif.runs[0]
            .results
            .iter()
            .find(|r| r.rule_id == sarif_rules::RULE_ID_BOUNDARY)
            .expect("Boundary SARIF result not found");
        assert_eq!(boundary_res.level, "error");
        assert_eq!(
            boundary_res.locations[0]
                .physical_location
                .artifact_location
                .uri,
            "src/domain/user.rs"
        );
    }

    #[test]
    fn test_all_clone_types_mapped() {
        let types = [
            (CloneType::Exact, sarif_rules::RULE_ID_EXACT, 0),
            (CloneType::Renamed, sarif_rules::RULE_ID_RENAMED, 1),
            (CloneType::NearMiss, sarif_rules::RULE_ID_NEAR_MISS, 2),
            (CloneType::Semantic, sarif_rules::RULE_ID_SEMANTIC, 3),
        ];

        for (clone_type, expected_rule, expected_idx) in types {
            let (rule_id, idx) = map_clone_type_to_rule(&clone_type);
            assert_eq!(rule_id, expected_rule);
            assert_eq!(idx, expected_idx);
        }
    }
}
