#![forbid(unsafe_code)]

use super::types::*;
use crate::types::{ClonePair, CloneType, PolicySeverity, PolicyViolation, ScanResult};
use std::collections::HashMap;

/// Helper to construct a standard SARIF rule descriptor.
pub fn make_sarif_rule(
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
pub fn build_rule_catalog() -> Vec<SarifRule> {
    vec![
        make_sarif_rule(
            sarif_rules::RULE_ID_EXACT,
            "ExactDuplicateCode",
            "Exact duplicate code clone detected (Type-1)",
            "CDDM detected an identical copy-paste code clone across source files. Exact clones \
             should be eliminated by extracting shared helper functions, utility modules, or \
             reusable traits.",
            "Eliminate identical duplication by extracting a shared function or constant.",
            "### Recommendation\nExtract the identical code into a single reusable helper \
             function or module to adhere to DRY principles.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_RENAMED,
            "RenamedDuplicateCode",
            "Renamed identifier duplicate code clone detected (Type-2)",
            "CDDM detected a structurally identical code clone with renamed identifiers or \
             modified literal values. Parameterize the differing variables to extract a unified \
             abstraction.",
            "Refactor by parameterizing renamed variables into a single function.",
            "### Recommendation\nParameterize the variable differences and consolidate both \
             fragments into a parameterized helper function.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_NEAR_MISS,
            "NearMissDuplicateCode",
            "Near-miss duplicate code clone detected (Type-3)",
            "CDDM detected a near-miss duplicate clone with minor statement insertions, \
             deletions, or reorderings. Consider refactoring with the Template Method or Strategy \
             pattern.",
            "Refactor near-miss clones using higher-order functions or strategy patterns.",
            "### Recommendation\nExtract invariant statements and inject variant logic via \
             closures or polymorphic interfaces.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_SEMANTIC,
            "SemanticDuplicateCode",
            "Semantic duplicate code clone detected (Type-4)",
            "CDDM detected semantically equivalent logic implemented with divergent syntax. \
             Standardize on the idiomatic implementation.",
            "Standardize on a single canonical implementation.",
            "### Recommendation\nSelect the most performant or readable implementation and \
             replace redundant variants.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_BOUNDARY,
            "ArchitecturalBoundaryViolation",
            "Duplication across architectural boundaries is forbidden by policy",
            "CDDM detected duplicate code crossing an architectural boundary defined in \
             .cddmrules.toml.",
            "Extract shared code into a common layer or remove forbidden cross-boundary \
             dependencies.",
            "### Recommendation\nMove duplicate logic to a shared dependency layer compliant with \
             workspace architecture.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_ZERO_DUP,
            "ZeroDuplicationViolation",
            "Zero duplication policy violated in strict module",
            "CDDM detected duplicate code in a path designated as zero-duplication in \
             .cddmrules.toml.",
            "Eliminate all duplication in this critical module.",
            "### Recommendation\nRefactor immediately to maintain 100% DRY compliance in this \
             strict module.",
        ),
        make_sarif_rule(
            sarif_rules::RULE_ID_LIMIT,
            "DuplicationLimitExceeded",
            "Duplication percentage or clone count exceeds configured threshold",
            "CDDM detected that module duplication exceeded the limit set in .cddmrules.toml.",
            "Reduce duplicate tokens or clone counts below the configured ceiling.",
            "### Recommendation\nRefactor top duplicate hotspots to bring module duplication \
             within policy limits.",
        ),
    ]
}

/// Maps internal CloneType to the corresponding SARIF Rule ID and rule catalog index.
fn map_clone_type_to_rule(clone_type: &CloneType) -> (&'static str, usize) {
    match clone_type {
        CloneType::Exact => (sarif_rules::RULE_ID_EXACT, 0),
        CloneType::Renamed => (sarif_rules::RULE_ID_RENAMED, 1),
        CloneType::NearMiss => (sarif_rules::RULE_ID_NEAR_MISS, 2),
        CloneType::Semantic => (sarif_rules::RULE_ID_SEMANTIC, 3),
    }
}

/// Maps policy violation to corresponding SARIF Rule ID and rule catalog index.
fn map_policy_violation_to_rule(violation: &PolicyViolation) -> (&'static str, usize, String) {
    let level = match violation.severity {
        PolicySeverity::Error => sarif_severity::ERROR.to_string(),
        PolicySeverity::Warning => sarif_severity::WARNING.to_string(),
        PolicySeverity::Info => sarif_severity::RECOMMENDATION.to_string(),
    };

    match violation.rule_type.as_str() {
        "boundary" => (sarif_rules::RULE_ID_BOUNDARY, 4, level),
        "zero_duplication" => (sarif_rules::RULE_ID_ZERO_DUP, 5, level),
        "limit" => (sarif_rules::RULE_ID_LIMIT, 6, level),
        _ => (sarif_rules::RULE_ID_EXACT, 0, level),
    }
}

/// Converts a single ClonePair finding into a standard SARIF Result.
fn convert_clone_pair_to_sarif_result(pair: &ClonePair) -> SarifResult {
    let (rule_id, rule_index) = map_clone_type_to_rule(&pair.clone_type);

    let message_text = format!(
        "Duplicate code clone ({:?}) detected: {} tokens shared with {}:{}-{} ({:.1}% similarity).",
        pair.clone_type,
        pair.token_count,
        pair.file_b,
        pair.start_line_b,
        pair.end_line_b,
        pair.similarity * 100.0
    );

    let primary_location = SarifLocation {
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: pair.file_a.clone(),
                uri_base_id: None,
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
                uri_base_id: None,
            },
            region: SarifRegion {
                start_line: pair.start_line_b,
                end_line: pair.end_line_b,
            },
        },
        message: SarifMessage {
            text: format!(
                "Counterpart clone fragment location in {}:{}-{}",
                pair.file_b, pair.start_line_b, pair.end_line_b
            ),
        },
    };

    let mut partial_fingerprints = HashMap::new();
    partial_fingerprints.insert("fragmentHash/v1".to_string(), pair.fragment_hash.clone());

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

/// Converts a policy violation finding into a standard SARIF Result.
fn convert_policy_violation_to_sarif_result(violation: &PolicyViolation) -> SarifResult {
    let (rule_id, rule_index, level) = map_policy_violation_to_rule(violation);

    let primary_location = SarifLocation {
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: violation.file_a.clone(),
                uri_base_id: None,
            },
            region: SarifRegion {
                start_line: violation.start_line_a,
                end_line: violation.end_line_a,
            },
        },
    };

    let mut related_locations = Vec::new();
    if let (Some(fb), Some(sb), Some(eb)) = (
        &violation.file_b,
        violation.start_line_b,
        violation.end_line_b,
    ) {
        related_locations.push(SarifRelatedLocation {
            id: 1,
            physical_location: SarifPhysicalLocation {
                artifact_location: SarifArtifactLocation {
                    uri: fb.clone(),
                    uri_base_id: None,
                },
                region: SarifRegion {
                    start_line: sb,
                    end_line: eb,
                },
            },
            message: SarifMessage {
                text: format!("Counterpart boundary location in {fb}:{sb}-{eb}"),
            },
        });
    }

    SarifResult {
        rule_id: rule_id.to_string(),
        rule_index,
        level,
        message: SarifMessage {
            text: violation.message.clone(),
        },
        locations: vec![primary_location],
        related_locations,
        partial_fingerprints: HashMap::new(),
        properties: SarifResultProperties {
            token_count: 0,
            similarity: 1.0,
            clone_type: violation.rule_type.clone(),
            fragment_hash: String::new(),
            author_a: None,
            author_b: None,
        },
    }
}

/// Generates a complete OASIS SARIF v2.1.0 report object from a CDDM ScanResult.
pub fn generate_sarif_report(scan_result: &ScanResult) -> SarifReport {
    let mut results: Vec<SarifResult> = scan_result
        .clone_pairs
        .iter()
        .map(convert_clone_pair_to_sarif_result)
        .collect();

    for violation in &scan_result.policy_violations {
        results.push(convert_policy_violation_to_sarif_result(violation));
    }

    let rules = build_rule_catalog();

    let driver = SarifDriver {
        name: TOOL_NAME.to_string(),
        semantic_version: env!("CARGO_PKG_VERSION").to_string(),
        information_uri: TOOL_INFORMATION_URI.to_string(),
        rules,
    };

    let tool = SarifTool { driver };

    let invocation = SarifInvocation {
        execution_successful: true,
        tool_execution_notifications: Vec::new(),
    };

    let run = SarifRun {
        tool,
        invocations: vec![invocation],
        results,
    };

    SarifReport {
        schema: SARIF_SCHEMA_URI.to_string(),
        version: SARIF_VERSION.to_string(),
        runs: vec![run],
    }
}

/// Generates a serialized pretty-printed JSON representation of the SARIF v2.1.0 report.
pub fn generate_sarif_json(scan_result: &ScanResult) -> serde_json::Value {
    let report = generate_sarif_report(scan_result);
    serde_json::to_value(&report).unwrap_or(serde_json::Value::Null)
}
