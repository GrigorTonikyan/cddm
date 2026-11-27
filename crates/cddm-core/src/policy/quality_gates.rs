#![forbid(unsafe_code)]

use crate::dead_code::{DeadCodeConfig, DeadCodeSummary, run_dead_code_detection};
use crate::error::CddmError;
use crate::types::{
    DeadCodeGateResult, DeadCodePolicy, PolicyConfig, PolicyEvaluationResult, PolicySeverity,
    PolicyViolation, ScanConfig, ScanPolicy, ScanResult,
};

pub const RULE_TYPE_QUALITY_GATE: &str = "quality_gate";
pub const RULE_TYPE_DEAD_CODE: &str = "dead_code";
pub const RULE_MAX_DUPLICATION: &str = "max_duplication_pct";
pub const RULE_MIN_DRY_SCORE: &str = "min_dry_score";
pub const RULE_DEAD_CODE_MAX_ITEMS: &str = "dead_code_max_items";
pub const MAX_PERCENTAGE: f64 = 100.0;

/// Validates that a percentage threshold is finite and within `0.0..=100.0`.
pub fn validate_percentage(name: &str, value: f64) -> Result<f64, String> {
    if value.is_nan() || !(0.0..=MAX_PERCENTAGE).contains(&value) {
        return Err(format!(
            "Invalid '{name}' threshold {value}: must be between 0.0 and {MAX_PERCENTAGE}"
        ));
    }
    Ok(value)
}

/// Threshold settings for evaluating scan-wide duplication and DRY score gates.
#[derive(Clone, Debug, PartialEq)]
pub struct QualityGateThresholds {
    pub max_duplication_pct: Option<f64>,
    pub min_dry_score: Option<f64>,
    pub severity: PolicySeverity,
    pub remediation: Option<String>,
}

impl From<&ScanPolicy> for QualityGateThresholds {
    fn from(policy: &ScanPolicy) -> Self {
        Self {
            max_duplication_pct: policy.max_duplication_pct,
            min_dry_score: policy.min_dry_score,
            severity: policy.severity.unwrap_or(PolicySeverity::Error),
            remediation: policy.remediation.clone(),
        }
    }
}

impl QualityGateThresholds {
    /// Returns `true` when neither threshold is configured.
    pub fn is_empty(&self) -> bool {
        self.max_duplication_pct.is_none() && self.min_dry_score.is_none()
    }
}

/// Evaluates quality gate thresholds (`max_duplication_pct`, `min_dry_score`) against a `ScanResult`.
pub fn evaluate_quality_gates(
    result: &ScanResult,
    thresholds: &QualityGateThresholds,
) -> Vec<PolicyViolation> {
    let mut violations = Vec::new();
    let target = result
        .target_directory
        .as_deref()
        .unwrap_or(".")
        .to_string();

    if let Some(max_dup) = thresholds.max_duplication_pct
        && result.duplication_percentage > max_dup
    {
        violations.push(PolicyViolation {
            rule_name: RULE_MAX_DUPLICATION.to_string(),
            rule_type: RULE_TYPE_QUALITY_GATE.to_string(),
            severity: thresholds.severity,
            message: format!(
                "Duplication {:.2}% exceeds maximum {:.2}%",
                result.duplication_percentage, max_dup
            ),
            file_a: target.clone(),
            remediation: thresholds.remediation.clone().or_else(|| {
                Some(format!(
                    "Reduce code duplication to <= {max_dup:.2}% before merging"
                ))
            }),
            ..Default::default()
        });
    }

    if let Some(min_dry) = thresholds.min_dry_score
        && result.dry_health_score < min_dry
    {
        violations.push(PolicyViolation {
            rule_name: RULE_MIN_DRY_SCORE.to_string(),
            rule_type: RULE_TYPE_QUALITY_GATE.to_string(),
            severity: thresholds.severity,
            message: format!(
                "DRY Health Score {:.2} is below minimum {:.2}",
                result.dry_health_score, min_dry
            ),
            file_a: target,
            remediation: thresholds.remediation.clone().or_else(|| {
                Some(format!(
                    "Improve DRY Health Score to >= {min_dry:.2} by extracting shared abstractions"
                ))
            }),
            ..Default::default()
        });
    }

    violations
}

/// Resolved dead-code failure gate specification.
#[derive(Clone, Debug, PartialEq)]
pub struct DeadCodeGate {
    pub max_items: usize,
    pub severity: PolicySeverity,
    pub remediation: Option<String>,
}

impl DeadCodeGate {
    /// Builds a gate from explicit options (`fail_on_findings` / `max_items`).
    pub fn from_options(fail_on_findings: Option<bool>, max_items: Option<usize>) -> Option<Self> {
        let enabled = fail_on_findings == Some(true) || max_items.is_some();
        if !enabled {
            return None;
        }
        Some(Self {
            max_items: max_items.unwrap_or(0),
            severity: PolicySeverity::Error,
            remediation: None,
        })
    }

    /// Builds a gate from a `[dead_code]` policy section when enabled.
    pub fn from_policy(policy: &DeadCodePolicy) -> Option<Self> {
        let enabled = policy.fail_on_findings == Some(true) || policy.max_items.is_some();
        if !enabled {
            return None;
        }
        Some(Self {
            max_items: policy.max_items.unwrap_or(0),
            severity: policy.severity.unwrap_or(PolicySeverity::Error),
            remediation: policy.remediation.clone(),
        })
    }

    /// Resolves the effective gate, giving explicit CLI/request arguments precedence over `[dead_code]`.
    pub fn resolve(
        cli_fail_on_findings: bool,
        cli_max_items: Option<usize>,
        policy: Option<&DeadCodePolicy>,
    ) -> Option<Self> {
        if cli_fail_on_findings || cli_max_items.is_some() {
            let mut gate = Self::from_options(Some(cli_fail_on_findings), cli_max_items)?;
            if let Some(p) = policy {
                if let Some(sev) = p.severity {
                    gate.severity = sev;
                }
                gate.remediation = p.remediation.clone();
            }
            return Some(gate);
        }
        policy.and_then(Self::from_policy)
    }
}

/// Computes and attaches `DeadCodeGateResult` onto `summary`.
pub fn apply_dead_code_gate(summary: &mut DeadCodeSummary, gate: &DeadCodeGate) {
    let total = summary.total_dead_items;
    let passed = total <= gate.max_items;
    let message = if passed {
        format!(
            "Dead code gate passed: {} item(s) within maximum {}",
            total, gate.max_items
        )
    } else {
        format!(
            "Dead code gate failed: {} item(s) exceed maximum {}",
            total, gate.max_items
        )
    };
    summary.gate = Some(DeadCodeGateResult {
        passed,
        total_dead_items: total,
        max_items: gate.max_items,
        message,
    });
}

/// Converts a failed `DeadCodeSummary` + `DeadCodeGate` into a `PolicyViolation` if exceeded.
pub fn dead_code_gate_violation(
    summary: &DeadCodeSummary,
    gate: &DeadCodeGate,
) -> Option<PolicyViolation> {
    if summary.total_dead_items <= gate.max_items {
        return None;
    }
    let primary_file = summary
        .items
        .first()
        .map(|item| item.file_path.clone())
        .unwrap_or_else(|| ".".to_string());
    let start_line = summary
        .items
        .first()
        .map(|item| item.line_start)
        .unwrap_or(1);
    let end_line = summary
        .items
        .first()
        .map(|item| item.line_end)
        .unwrap_or(start_line);

    Some(PolicyViolation {
        rule_name: RULE_DEAD_CODE_MAX_ITEMS.to_string(),
        rule_type: RULE_TYPE_DEAD_CODE.to_string(),
        severity: gate.severity,
        message: format!(
            "Dead code findings ({}) exceed maximum allowed ({})",
            summary.total_dead_items, gate.max_items
        ),
        file_a: primary_file,
        start_line_a: start_line,
        end_line_a: end_line,
        remediation: gate.remediation.clone().or_else(|| {
            Some("Remove unreferenced functions, unreachable blocks, and dead clones".to_string())
        }),
        ..Default::default()
    })
}

/// Runs dead-code detection when `[dead_code]` gate is configured in `PolicyConfig`.
pub async fn evaluate_dead_code_policy(
    directory: &str,
    config: &PolicyConfig,
    include_ignored: bool,
) -> Result<Vec<PolicyViolation>, CddmError> {
    let Some(ref dc_policy) = config.dead_code else {
        return Ok(Vec::new());
    };
    let Some(gate) = DeadCodeGate::from_policy(dc_policy) else {
        return Ok(Vec::new());
    };

    let dc_config = DeadCodeConfig {
        directory: directory.to_string(),
        min_tokens: dc_policy
            .min_tokens
            .unwrap_or_else(|| DeadCodeConfig::default().min_tokens),
        static_only: dc_policy.static_only.unwrap_or(false),
        include_ignored: Some(include_ignored),
        ..Default::default()
    };

    let summary = run_dead_code_detection(dc_config).await?;
    Ok(dead_code_gate_violation(&summary, &gate)
        .into_iter()
        .collect())
}

impl PolicyConfig {
    /// Returns effective `ignore_tests`, checking top-level field then `[scan].ignore_tests`.
    pub fn effective_ignore_tests(&self) -> Option<bool> {
        self.ignore_tests
            .or_else(|| self.scan.as_ref().and_then(|s| s.ignore_tests))
    }

    /// Returns effective `ignore_mocks`, checking top-level field then `[scan].ignore_mocks`.
    pub fn effective_ignore_mocks(&self) -> Option<bool> {
        self.ignore_mocks
            .or_else(|| self.scan.as_ref().and_then(|s| s.ignore_mocks))
    }

    /// Extracts configured quality gate thresholds from `[scan]` when non-empty.
    pub fn quality_gates(&self) -> Option<QualityGateThresholds> {
        let scan = self.scan.as_ref()?;
        let thresholds = QualityGateThresholds::from(scan);
        if thresholds.is_empty() {
            None
        } else {
            Some(thresholds)
        }
    }
}

impl ScanPolicy {
    /// Applies configured Type-3/Type-4 detection overrides onto a `ScanConfig`.
    pub fn apply_to(&self, cfg: &mut ScanConfig) {
        if let Some(t3) = self.detect_type3 {
            cfg.detect_type3 = t3;
        }
        if let Some(t4) = self.detect_type4 {
            cfg.detect_type4 = t4;
        }
    }
}

impl PolicyEvaluationResult {
    /// Appends additional policy violations and recomputes aggregate counts and `passed`.
    pub fn push_violations(&mut self, extra: Vec<PolicyViolation>) {
        self.violations.extend(extra);
        self.recount();
    }

    /// Recomputes `total_violations`, severity counts, and `passed` from `self.violations`.
    pub fn recount(&mut self) {
        self.total_violations = self.violations.len();
        self.error_count = self
            .violations
            .iter()
            .filter(|v| v.severity == PolicySeverity::Error)
            .count();
        self.warning_count = self
            .violations
            .iter()
            .filter(|v| v.severity == PolicySeverity::Warning)
            .count();
        self.info_count = self
            .violations
            .iter()
            .filter(|v| v.severity == PolicySeverity::Info)
            .count();
        self.passed = self.error_count == 0;
    }
}
