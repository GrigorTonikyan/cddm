#![forbid(unsafe_code)]

use crate::utils::{line_range_to_lsp_range, normalize_path_for_compare, path_to_url, url_to_path};
use cddm_core::{ClonePair, CloneType, ScanResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Url,
};

/// Formats the diagnostic code string from a `CloneType`.
#[must_use]
pub fn format_diagnostic_code(clone_type: &CloneType) -> String {
    match clone_type {
        CloneType::Exact => "CDDM-Exact".to_string(),
        CloneType::Renamed => "CDDM-Renamed".to_string(),
        CloneType::NearMiss => "CDDM-NearMiss".to_string(),
        CloneType::Semantic => "CDDM-Semantic".to_string(),
    }
}

/// Converts a `ClonePair` into diagnostics for a specific target file URL.
#[must_use]
pub fn clone_pair_to_diagnostics(
    clone: &ClonePair,
    target_url: &Url,
    workspace_root: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let target_norm = if let Some(path) = url_to_path(target_url) {
        normalize_path_for_compare(&path.to_string_lossy())
    } else {
        normalize_path_for_compare(target_url.as_str())
    };

    let norm_a = normalize_path_for_compare(&clone.file_a);
    let norm_b = normalize_path_for_compare(&clone.file_b);

    let is_a =
        norm_a == target_norm || target_norm.ends_with(&norm_a) || norm_a.ends_with(&target_norm);
    let is_b =
        norm_b == target_norm || target_norm.ends_with(&norm_b) || norm_b.ends_with(&target_norm);

    if is_a {
        diagnostics.push(build_clone_diagnostic(
            clone.start_line_a,
            clone.end_line_a,
            &clone.file_b,
            clone.start_line_b,
            clone.end_line_b,
            clone.token_count,
            clone.similarity,
            &clone.clone_type,
            workspace_root,
            target_url,
        ));
    }

    if is_b && norm_a != norm_b {
        diagnostics.push(build_clone_diagnostic(
            clone.start_line_b,
            clone.end_line_b,
            &clone.file_a,
            clone.start_line_a,
            clone.end_line_a,
            clone.token_count,
            clone.similarity,
            &clone.clone_type,
            workspace_root,
            target_url,
        ));
    }

    diagnostics
}

#[allow(clippy::too_many_arguments)]
fn build_clone_diagnostic(
    start_line: usize,
    end_line: usize,
    other_file: &str,
    other_start: usize,
    other_end: usize,
    token_count: usize,
    similarity: f64,
    clone_type: &CloneType,
    workspace_root: &Path,
    target_url: &Url,
) -> Diagnostic {
    let range = line_range_to_lsp_range(start_line, end_line);
    let counterpart_path = workspace_root.join(other_file);
    let counterpart_url = path_to_url(&counterpart_path).unwrap_or_else(|| target_url.clone());
    let counterpart_range = line_range_to_lsp_range(other_start, other_end);

    let code_str = format_diagnostic_code(clone_type);
    let sim_pct = (similarity * 100.0).round();
    let message = format!(
        "Code duplication: {} lines ({} tokens, {}% match) duplicate of {}:{}-{} ({:?})",
        end_line.saturating_sub(start_line) + 1,
        token_count,
        sim_pct,
        other_file,
        other_start,
        other_end,
        clone_type
    );

    let related = vec![DiagnosticRelatedInformation {
        location: Location {
            uri: counterpart_url,
            range: counterpart_range,
        },
        message: format!("Duplicate clone counterpart in {}", other_file),
    }];

    Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::WARNING),
        code: Some(NumberOrString::String(code_str)),
        code_description: None,
        source: Some("CDDM".to_string()),
        message,
        related_information: Some(related),
        tags: None,
        data: None,
    }
}

/// Generates diagnostics across all files for a full `ScanResult`.
#[must_use]
pub fn generate_workspace_diagnostics(
    scan_result: &ScanResult,
    workspace_root: &Path,
) -> HashMap<Url, Vec<Diagnostic>> {
    let mut file_diagnostics: HashMap<Url, Vec<Diagnostic>> = HashMap::new();

    for clone in &scan_result.clone_pairs {
        let path_a = if Path::new(&clone.file_a).is_absolute() {
            PathBuf::from(&clone.file_a)
        } else {
            workspace_root.join(&clone.file_a)
        };

        if let Some(url_a) = path_to_url(&path_a) {
            let diags_a = clone_pair_to_diagnostics(clone, &url_a, workspace_root);
            file_diagnostics.entry(url_a).or_default().extend(diags_a);
        }

        let path_b = if Path::new(&clone.file_b).is_absolute() {
            PathBuf::from(&clone.file_b)
        } else {
            workspace_root.join(&clone.file_b)
        };

        if let Some(url_b) = path_to_url(&path_b) {
            let diags_b = clone_pair_to_diagnostics(clone, &url_b, workspace_root);
            file_diagnostics.entry(url_b).or_default().extend(diags_b);
        }
    }

    for violation in &scan_result.policy_violations {
        let path = if Path::new(&violation.file_a).is_absolute() {
            PathBuf::from(&violation.file_a)
        } else {
            workspace_root.join(&violation.file_a)
        };

        if let Some(url) = path_to_url(&path) {
            let range = line_range_to_lsp_range(violation.start_line_a, violation.end_line_a);
            let sev = match violation.severity {
                cddm_core::PolicySeverity::Error => DiagnosticSeverity::ERROR,
                cddm_core::PolicySeverity::Warning => DiagnosticSeverity::WARNING,
                cddm_core::PolicySeverity::Info => DiagnosticSeverity::INFORMATION,
            };
            let code = format!("CDDM-Policy-{}", violation.rule_name);
            let diag = Diagnostic {
                range,
                severity: Some(sev),
                code: Some(NumberOrString::String(code)),
                code_description: None,
                source: Some("CDDM-Policy".to_string()),
                message: violation.message.clone(),
                related_information: None,
                tags: None,
                data: None,
            };
            file_diagnostics.entry(url).or_default().push(diag);
        }
    }

    file_diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_diagnostic_code() {
        assert_eq!(format_diagnostic_code(&CloneType::Exact), "CDDM-Exact");
        assert_eq!(format_diagnostic_code(&CloneType::Renamed), "CDDM-Renamed");
        assert_eq!(
            format_diagnostic_code(&CloneType::NearMiss),
            "CDDM-NearMiss"
        );
        assert_eq!(
            format_diagnostic_code(&CloneType::Semantic),
            "CDDM-Semantic"
        );
    }

    #[test]
    fn test_clone_pair_to_diagnostics() {
        let clone = ClonePair {
            file_a: "src/a.rs".to_string(),
            start_line_a: 10,
            end_line_a: 20,
            file_b: "src/b.rs".to_string(),
            start_line_b: 30,
            end_line_b: 40,
            token_count: 85,
            similarity: 0.95,
            fragment_hash: "hash123".to_string(),
            clone_type: CloneType::Renamed,
            author_a: None,
            author_b: None,
        };

        let target_url = Url::parse("file:///workspace/src/a.rs").expect("valid url");
        let ws_root = Path::new("/workspace");

        let diags = clone_pair_to_diagnostics(&clone, &target_url, ws_root);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].range.start.line, 9);
        assert_eq!(diags[0].range.end.line, 19);
        assert_eq!(
            diags[0].code,
            Some(NumberOrString::String("CDDM-Renamed".to_string()))
        );
        assert!(diags[0].message.contains("85 tokens"));
        assert!(diags[0].message.contains("95% match"));
    }
}
