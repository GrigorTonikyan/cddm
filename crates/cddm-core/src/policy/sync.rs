#![forbid(unsafe_code)]

use crate::types::{PolicyViolation, SyncRule};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Evaluates SSoT constant and version synchronization rules.
pub fn evaluate_sync_rules(root_dir: &Path, rules: &[SyncRule]) -> Vec<PolicyViolation> {
    if rules.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();

    for rule in rules {
        let source_path = root_dir.join(&rule.source_file);
        if !source_path.exists() {
            violations.push(PolicyViolation {
                rule_name: rule.name.clone(),
                rule_type: "sync_drift".to_string(),
                severity: rule.severity,
                message: format!(
                    "Source file '{}' not found for sync rule '{}'",
                    rule.source_file, rule.name
                ),
                file_a: rule.source_file.clone(),
                start_line_a: 1,
                end_line_a: 1,
                snippet: None,
                remediation: rule
                    .remediation
                    .clone()
                    .or_else(|| Some(format!("Ensure source file '{}' exists", rule.source_file))),
                ..Default::default()
            });
            continue;
        }

        let source_content = match fs::read_to_string(&source_path) {
            Ok(c) => c,
            Err(e) => {
                violations.push(PolicyViolation {
                    rule_name: rule.name.clone(),
                    rule_type: "sync_drift".to_string(),
                    severity: rule.severity,
                    message: format!(
                        "Failed to read source file '{}' for sync rule '{}': {e}",
                        rule.source_file, rule.name
                    ),
                    file_a: rule.source_file.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    snippet: None,
                    remediation: None,
                    ..Default::default()
                });
                continue;
            }
        };

        let source_extract = extract_value(
            &rule.source_file,
            &source_content,
            rule.source_key.as_deref(),
            rule.source_regex.as_deref(),
        );

        let (source_val, source_line) = match source_extract {
            Ok(res) => res,
            Err(e) => {
                violations.push(PolicyViolation {
                    rule_name: rule.name.clone(),
                    rule_type: "sync_drift".to_string(),
                    severity: rule.severity,
                    message: format!(
                        "Failed to extract source value from '{}' for sync rule '{}': {e}",
                        rule.source_file, rule.name
                    ),
                    file_a: rule.source_file.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    snippet: None,
                    remediation: rule.remediation.clone(),
                    ..Default::default()
                });
                continue;
            }
        };

        let mut targets = rule.target_files.clone();
        if let Some(ref single) = rule.target_file
            && !targets.contains(single)
        {
            targets.push(single.clone());
        }

        if targets.is_empty() {
            violations.push(PolicyViolation {
                rule_name: rule.name.clone(),
                rule_type: "sync_drift".to_string(),
                severity: rule.severity,
                message: format!("Sync rule '{}' has no target files configured", rule.name),
                file_a: rule.source_file.clone(),
                start_line_a: 1,
                end_line_a: 1,
                snippet: None,
                remediation: Some("Specify target_file or target_files in sync rule".to_string()),
                ..Default::default()
            });
            continue;
        }

        for target_rel in targets {
            let target_path = root_dir.join(&target_rel);
            if !target_path.exists() {
                violations.push(PolicyViolation {
                    rule_name: rule.name.clone(),
                    rule_type: "sync_drift".to_string(),
                    severity: rule.severity,
                    message: format!(
                        "Target file '{}' not found for sync rule '{}'",
                        target_rel, rule.name
                    ),
                    file_a: target_rel.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    file_b: Some(rule.source_file.clone()),
                    start_line_b: Some(source_line),
                    end_line_b: Some(source_line),
                    snippet: None,
                    remediation: rule.remediation.clone(),
                    ..Default::default()
                });
                continue;
            }

            let target_content = match fs::read_to_string(&target_path) {
                Ok(c) => c,
                Err(e) => {
                    violations.push(PolicyViolation {
                        rule_name: rule.name.clone(),
                        rule_type: "sync_drift".to_string(),
                        severity: rule.severity,
                        message: format!(
                            "Failed to read target file '{}' for sync rule '{}': {e}",
                            target_rel, rule.name
                        ),
                        file_a: target_rel.clone(),
                        start_line_a: 1,
                        end_line_a: 1,
                        file_b: Some(rule.source_file.clone()),
                        start_line_b: Some(source_line),
                        end_line_b: Some(source_line),
                        snippet: None,
                        remediation: None,
                        ..Default::default()
                    });
                    continue;
                }
            };

            let key_opt = rule.target_key.as_deref().or_else(|| {
                if rule.target_regex.is_none()
                    && (target_rel.ends_with(".json") || target_rel.ends_with(".toml"))
                {
                    rule.source_key.as_deref()
                } else {
                    None
                }
            });

            let target_extract = extract_value(
                &target_rel,
                &target_content,
                key_opt,
                rule.target_regex.as_deref(),
            );

            match target_extract {
                Ok((target_val, target_line)) => {
                    if target_val != source_val {
                        violations.push(PolicyViolation {
                            rule_name: rule.name.clone(),
                            rule_type: "sync_drift".to_string(),
                            severity: rule.severity,
                            message: format!(
                                "Sync drift detected in '{}': expected '{}' (from '{}'), found '{}'",
                                target_rel, source_val, rule.source_file, target_val
                            ),
                            file_a: target_rel.clone(),
                            start_line_a: target_line,
                            end_line_a: target_line,
                            file_b: Some(rule.source_file.clone()),
                            start_line_b: Some(source_line),
                            end_line_b: Some(source_line),
                            cluster_id: None,
                            token_count: 0,
                            snippet: Some(format!(
                                "expected: \"{}\", found: \"{}\"",
                                source_val, target_val
                            )),
                            remediation: rule.remediation.clone().or_else(|| {
                                Some(format!(
                                    "Update target in '{}' to match authoritative value '{}' from '{}'",
                                    target_rel, source_val, rule.source_file
                                ))
                            }),
                        });
                    }
                }
                Err(e) => {
                    violations.push(PolicyViolation {
                        rule_name: rule.name.clone(),
                        rule_type: "sync_drift".to_string(),
                        severity: rule.severity,
                        message: format!(
                            "Failed to extract target value in '{}' for sync rule '{}': {e}",
                            target_rel, rule.name
                        ),
                        file_a: target_rel.clone(),
                        start_line_a: 1,
                        end_line_a: 1,
                        file_b: Some(rule.source_file.clone()),
                        start_line_b: Some(source_line),
                        end_line_b: Some(source_line),
                        snippet: None,
                        remediation: rule.remediation.clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }

    violations
}

fn extract_value(
    file_path: &str,
    content: &str,
    key_opt: Option<&str>,
    regex_opt: Option<&str>,
) -> Result<(String, usize), String> {
    if let Some(re_pattern) = regex_opt {
        let re =
            Regex::new(re_pattern).map_err(|e| format!("Invalid regex '{re_pattern}': {e}"))?;
        for (idx, line) in content.lines().enumerate() {
            if let Some(caps) = re.captures(line) {
                if let Some(m) = caps.get(1) {
                    return Ok((m.as_str().to_string(), idx + 1));
                } else if let Some(m) = caps.get(0) {
                    return Ok((m.as_str().to_string(), idx + 1));
                }
            }
        }
        return Err(format!(
            "Regex '{re_pattern}' did not match any content in '{file_path}'"
        ));
    }

    if let Some(key) = key_opt {
        if file_path.ends_with(".json") {
            let json_val: serde_json::Value = serde_json::from_str(content)
                .map_err(|e| format!("Failed to parse JSON '{file_path}': {e}"))?;
            let mut curr = &json_val;
            for part in key.split('.') {
                curr = curr.get(part).ok_or_else(|| {
                    format!("Key '{part}' not found in JSON path '{key}' in '{file_path}'")
                })?;
            }
            let val_str = match curr {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                other => other.to_string(),
            };
            let last_part = key.split('.').next_back().unwrap_or(key);
            let line_num = content
                .lines()
                .position(|l| l.contains(&format!("\"{last_part}\"")))
                .map(|p| p + 1)
                .unwrap_or(1);
            return Ok((val_str, line_num));
        }

        if file_path.ends_with(".toml") {
            let toml_val: toml::Value = toml::from_str(content)
                .map_err(|e| format!("Failed to parse TOML '{file_path}': {e}"))?;
            let mut curr = &toml_val;
            for part in key.split('.') {
                curr = curr.get(part).ok_or_else(|| {
                    format!("Key '{part}' not found in TOML path '{key}' in '{file_path}'")
                })?;
            }
            let val_str = match curr {
                toml::Value::String(s) => s.clone(),
                toml::Value::Integer(i) => i.to_string(),
                toml::Value::Float(f) => f.to_string(),
                toml::Value::Boolean(b) => b.to_string(),
                other => other.to_string(),
            };
            let last_part = key.split('.').next_back().unwrap_or(key);
            let line_num = content
                .lines()
                .position(|l| l.contains(last_part))
                .map(|p| p + 1)
                .unwrap_or(1);
            return Ok((val_str, line_num));
        }

        return Err(format!(
            "Cannot extract key '{key}' from non-JSON/TOML file '{file_path}'. Use regex instead."
        ));
    }

    Err("Neither key nor regex was specified for value extraction".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicySeverity;
    use tempfile::tempdir;

    #[test]
    fn test_sync_rule_json_to_code_constant() {
        let dir = tempdir().expect("create tempdir");
        let root = dir.path();

        let pkg_json = root.join("package.json");
        std::fs::write(&pkg_json, r#"{"name": "test-pkg", "version": "1.2.3"}"#)
            .expect("write pkg json");

        let const_ts = root.join("constants.ts");
        std::fs::write(
            &const_ts,
            r#"export const APP_VERSION = "1.2.0"; // Drifting!"#,
        )
        .expect("write const ts");

        let rule = SyncRule {
            name: "version-sync".to_string(),
            description: Some("Ensure package.json version matches APP_VERSION".to_string()),
            source_file: "package.json".to_string(),
            source_key: Some("version".to_string()),
            source_regex: None,
            target_files: vec!["constants.ts".to_string()],
            target_file: None,
            target_key: None,
            target_regex: Some(r#"APP_VERSION\s*=\s*"([^"]+)""#.to_string()),
            severity: PolicySeverity::Error,
            remediation: Some("Bump APP_VERSION in constants.ts to match package.json".to_string()),
        };

        let violations = evaluate_sync_rules(root, &[rule]);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_name, "version-sync");
        assert_eq!(violations[0].rule_type, "sync_drift");
        assert!(violations[0].message.contains("expected '1.2.3'"));
        assert!(violations[0].message.contains("found '1.2.0'"));
        assert_eq!(violations[0].start_line_a, 1);
        assert_eq!(violations[0].file_b.as_deref(), Some("package.json"));
    }

    #[test]
    fn test_sync_rule_in_sync() {
        let dir = tempdir().expect("create tempdir");
        let root = dir.path();

        let pkg_json = root.join("package.json");
        std::fs::write(&pkg_json, r#"{"version": "2.0.0"}"#).expect("write pkg json");

        let const_ts = root.join("constants.ts");
        std::fs::write(&const_ts, r#"export const APP_VERSION = "2.0.0";"#)
            .expect("write const ts");

        let rule = SyncRule {
            name: "version-sync".to_string(),
            description: None,
            source_file: "package.json".to_string(),
            source_key: Some("version".to_string()),
            source_regex: None,
            target_files: vec!["constants.ts".to_string()],
            target_file: None,
            target_key: None,
            target_regex: Some(r#"APP_VERSION\s*=\s*"([^"]+)""#.to_string()),
            severity: PolicySeverity::Error,
            remediation: None,
        };

        let violations = evaluate_sync_rules(root, &[rule]);
        assert!(violations.is_empty());
    }
}
