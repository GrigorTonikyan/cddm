#![forbid(unsafe_code)]

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

pub mod sarif_tags {
    pub const MAINTAINABILITY: &str = "maintainability";
    pub const QUALITY: &str = "quality";
    pub const DUPLICATION: &str = "duplication";
}

pub mod sarif_severity {
    pub const WARNING: &str = "warning";
    pub const ERROR: &str = "error";
    pub const RECOMMENDATION: &str = "recommendation";
}

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
