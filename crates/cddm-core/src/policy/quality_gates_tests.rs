#![forbid(unsafe_code)]

use crate::dead_code::{DeadCodeItem, DeadCodeKind, DeadCodeSummary};
use crate::policy::PolicyEngine;
use crate::policy::quality_gates::{
    DeadCodeGate, QualityGateThresholds, RULE_MAX_DUPLICATION, RULE_MIN_DRY_SCORE,
    RULE_TYPE_DEAD_CODE, RULE_TYPE_QUALITY_GATE, apply_dead_code_gate, dead_code_gate_violation,
    evaluate_quality_gates, validate_percentage,
};
use crate::types::{
    DeadCodePolicy, PolicyConfig, PolicyEvaluationResult, PolicySeverity, PolicyViolation,
    ScanPolicy, ScanResult,
};

fn sample_scan_result(dup: f64, dry: f64) -> ScanResult {
    ScanResult {
        scan_id: "qg-test".to_string(),
        duplication_percentage: dup,
        dry_health_score: dry,
        target_directory: Some("workspace".to_string()),
        ..Default::default()
    }
}

#[test]
fn test_quality_gates_pass_fail_and_equality() {
    let thresholds = QualityGateThresholds {
        max_duplication_pct: Some(5.0),
        min_dry_score: Some(90.0),
        severity: PolicySeverity::Error,
        remediation: None,
    };

    // Exact boundary equality passes
    let equal_res = sample_scan_result(5.0, 90.0);
    assert!(evaluate_quality_gates(&equal_res, &thresholds).is_empty());

    // Strictly exceeding fails both
    let fail_res = sample_scan_result(7.2, 87.5);
    let violations = evaluate_quality_gates(&fail_res, &thresholds);
    assert_eq!(violations.len(), 2);
    assert!(
        violations
            .iter()
            .all(|v| v.rule_type == RULE_TYPE_QUALITY_GATE)
    );
    assert_eq!(violations[0].rule_name, RULE_MAX_DUPLICATION);
    assert_eq!(
        violations[0].message,
        "Duplication 7.20% exceeds maximum 5.00%"
    );
    assert_eq!(violations[1].rule_name, RULE_MIN_DRY_SCORE);
    assert_eq!(
        violations[1].message,
        "DRY Health Score 87.50 is below minimum 90.00"
    );
}

#[test]
fn test_quality_gates_severity_override() {
    let thresholds = QualityGateThresholds {
        max_duplication_pct: Some(2.0),
        min_dry_score: None,
        severity: PolicySeverity::Warning,
        remediation: Some("Clean up duplicates".to_string()),
    };
    let violations = evaluate_quality_gates(&sample_scan_result(3.5, 96.5), &thresholds);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].severity, PolicySeverity::Warning);
    assert_eq!(
        violations[0].remediation.as_deref(),
        Some("Clean up duplicates")
    );
}

#[test]
fn test_dead_code_gate_resolve_precedence() {
    let policy = DeadCodePolicy {
        fail_on_findings: Some(true),
        max_items: Some(2),
        severity: Some(PolicySeverity::Warning),
        remediation: Some("Remove dead symbols".to_string()),
        ..Default::default()
    };

    // Policy fallback when CLI flags unset
    let from_pol = DeadCodeGate::resolve(false, None, Some(&policy)).expect("expected policy gate");
    assert_eq!(from_pol.max_items, 2);
    assert_eq!(from_pol.severity, PolicySeverity::Warning);

    // Explicit CLI max_items overrides policy max_items
    let from_cli = DeadCodeGate::resolve(false, Some(5), Some(&policy)).expect("expected cli gate");
    assert_eq!(from_cli.max_items, 5);
    assert_eq!(from_cli.severity, PolicySeverity::Warning);

    // fail_on_findings=true with no max_items yields max_items=0
    let strict = DeadCodeGate::resolve(true, None, Some(&policy)).expect("expected strict gate");
    assert_eq!(strict.max_items, 0);

    // None when neither CLI nor policy enables gate
    assert!(DeadCodeGate::resolve(false, None, None).is_none());
}

#[test]
fn test_apply_dead_code_gate_pass_and_fail() {
    let mut summary = DeadCodeSummary {
        total_dead_items: 1,
        items: vec![DeadCodeItem {
            id: 1,
            file_path: "src/unused.rs".to_string(),
            symbol_name: "dead_fn".to_string(),
            kind: DeadCodeKind::UnreferencedFunction,
            line_start: 10,
            line_end: 20,
            token_count: 35,
            estimated_lines_saved: 11,
            reason: "No callers".to_string(),
            confidence: 0.95,
            package_name: None,
            is_exported: false,
            cross_package_callers: Vec::new(),
        }],
        ..Default::default()
    };

    let pass_gate = DeadCodeGate::from_options(None, Some(1)).unwrap();
    apply_dead_code_gate(&mut summary, &pass_gate);
    assert!(summary.gate.as_ref().unwrap().passed);
    assert!(dead_code_gate_violation(&summary, &pass_gate).is_none());

    let fail_gate = DeadCodeGate::from_options(Some(true), None).unwrap();
    apply_dead_code_gate(&mut summary, &fail_gate);
    let gate_res = summary.gate.as_ref().unwrap();
    assert!(!gate_res.passed);
    assert_eq!(gate_res.max_items, 0);

    let violation = dead_code_gate_violation(&summary, &fail_gate).expect("expected violation");
    assert_eq!(violation.rule_type, RULE_TYPE_DEAD_CODE);
    assert_eq!(violation.file_a, "src/unused.rs");
}

#[test]
fn test_toml_parse_scan_and_dead_code_and_invalid_threshold() {
    let toml_ok = r#"
[scan]
max_duplication_pct = 5.0
min_dry_score = 90.0
detect_type4 = true
ignore_tests = false

[dead_code]
fail_on_findings = true
max_items = 0
min_tokens = 25
static_only = true
"#;
    let engine = toml_ok
        .parse::<PolicyEngine>()
        .expect("should parse [scan] and [dead_code]");
    assert_eq!(engine.config().effective_ignore_tests(), Some(false));
    let scan_pol = engine.config().scan.as_ref().unwrap();
    assert_eq!(scan_pol.detect_type4, Some(true));

    let eval = engine.evaluate(&sample_scan_result(6.5, 85.0));
    assert!(!eval.passed);
    assert_eq!(eval.error_count, 2);

    let toml_bad = r#"
[scan]
min_dry_score = 150.0
"#;
    let err = toml_bad
        .parse::<PolicyEngine>()
        .expect_err("150.0 should be rejected");
    assert!(err.contains("150"));
    assert!(validate_percentage("x", f64::NAN).is_err());
}

#[test]
fn test_effective_ignore_precedence_and_push_violations() {
    let cfg = PolicyConfig {
        ignore_tests: Some(true),
        scan: Some(ScanPolicy {
            ignore_tests: Some(false),
            ignore_mocks: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(cfg.effective_ignore_tests(), Some(true));
    assert_eq!(cfg.effective_ignore_mocks(), Some(false));

    let mut eval = PolicyEvaluationResult {
        passed: true,
        total_violations: 0,
        error_count: 0,
        warning_count: 0,
        info_count: 0,
        violations: Vec::new(),
    };
    eval.push_violations(vec![
        PolicyViolation {
            severity: PolicySeverity::Warning,
            ..Default::default()
        },
        PolicyViolation {
            severity: PolicySeverity::Error,
            ..Default::default()
        },
    ]);
    assert!(!eval.passed);
    assert_eq!(eval.total_violations, 2);
    assert_eq!(eval.warning_count, 1);
    assert_eq!(eval.error_count, 1);
}
