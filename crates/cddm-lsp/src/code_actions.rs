#![forbid(unsafe_code)]

use crate::utils::{
    line_range_to_lsp_range, normalize_path_for_compare, path_to_url, to_0_based_line, url_to_path,
};
use cddm_core::{ClonePair, refactor::analyze_clone_refactoring};
use std::collections::HashMap;
use std::path::Path;
use tower_lsp::lsp_types::Url;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Command, Range, TextEdit, WorkspaceEdit,
};

/// Checks if an LSP range overlaps with 1-based start and end lines.
#[must_use]
pub fn range_overlaps_lines(range: &Range, start_line: usize, end_line: usize) -> bool {
    let req_start = range.start.line as usize;
    let req_end = range.end.line as usize;

    req_start <= to_0_based_line(end_line) && req_end >= to_0_based_line(start_line)
}

/// Generates LSP `CodeActionOrCommand` options for a given document and range.
#[must_use]
pub fn generate_code_actions(
    url: &Url,
    range: &Range,
    clones: &[ClonePair],
    workspace_root: &Path,
) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();

    let target_norm = if let Some(path) = url_to_path(url) {
        normalize_path_for_compare(&path.to_string_lossy())
    } else {
        normalize_path_for_compare(url.as_str())
    };

    for clone in clones {
        let Some((my_start, my_end, other_file, other_start, other_end)) =
            crate::utils::match_clone_occurrence(clone, &target_norm)
        else {
            continue;
        };

        if !range_overlaps_lines(range, my_start, my_end) {
            continue;
        }

        let path_a_str = if Path::new(&clone.file_a).is_absolute() {
            clone.file_a.clone()
        } else {
            workspace_root
                .join(&clone.file_a)
                .to_string_lossy()
                .to_string()
        };

        let path_b_str = if Path::new(&clone.file_b).is_absolute() {
            clone.file_b.clone()
        } else {
            workspace_root
                .join(&clone.file_b)
                .to_string_lossy()
                .to_string()
        };

        // Action 1: Extract helper function / synthesize refactor
        if let Ok(suggestion) = analyze_clone_refactoring(
            &path_a_str,
            (clone.start_line_a, clone.end_line_a),
            &path_b_str,
            (clone.start_line_b, clone.end_line_b),
        ) {
            let my_range = line_range_to_lsp_range(my_start, my_end);
            let helper_name = &suggestion.suggested_function_name;
            let call_site =
                format!("// CDDM: Replaced with call to {helper_name}\n{helper_name}();\n");

            let mut changes = HashMap::new();
            changes.insert(
                url.clone(),
                vec![TextEdit {
                    range: my_range,
                    new_text: call_site,
                }],
            );

            let edit = WorkspaceEdit {
                changes: Some(changes),
                document_changes: None,
                change_annotations: None,
            };

            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: format!(
                    "CDDM: Deduplicate using `{helper_name}` ({})",
                    suggestion.strategy
                ),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: None,
                edit: Some(edit),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            }));
        }

        // Action 2: Navigate to counterpart clone occurrence
        let counterpart_path = workspace_root.join(other_file);
        if let Some(counterpart_url) = path_to_url(&counterpart_path) {
            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: format!(
                    "CDDM: Jump to counterpart in {other_file}:{other_start}-{other_end}"
                ),
                kind: Some(CodeActionKind::EMPTY),
                diagnostics: None,
                edit: None,
                command: Some(Command {
                    title: "Open Counterpart Location".to_string(),
                    command: "cddm.openLocation".to_string(),
                    arguments: Some(vec![
                        serde_json::to_value(counterpart_url.as_str()).unwrap_or_default(),
                        serde_json::to_value(other_start).unwrap_or_default(),
                        serde_json::to_value(other_end).unwrap_or_default(),
                    ]),
                }),
                is_preferred: Some(false),
                disabled: None,
                data: None,
            }));
        }
    }

    // Action 3: General Rescan Action
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "CDDM: Rescan workspace duplication".to_string(),
        kind: Some(CodeActionKind::EMPTY),
        diagnostics: None,
        edit: None,
        command: Some(Command {
            title: "Rescan Workspace".to_string(),
            command: "cddm.rescanWorkspace".to_string(),
            arguments: None,
        }),
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }));

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::CloneType;
    use tower_lsp::lsp_types::Position;

    #[test]
    fn test_range_overlaps_lines() {
        let range = Range {
            start: Position {
                line: 5,
                character: 0,
            },
            end: Position {
                line: 12,
                character: 10,
            },
        };

        assert!(range_overlaps_lines(&range, 6, 10));
        assert!(range_overlaps_lines(&range, 1, 6));
        assert!(range_overlaps_lines(&range, 13, 20));
        assert!(!range_overlaps_lines(&range, 15, 20));
        assert!(!range_overlaps_lines(&range, 1, 4));
    }

    #[test]
    fn test_generate_code_actions() {
        let clone = ClonePair {
            file_a: "src/a.rs".to_string(),
            start_line_a: 1,
            end_line_a: 10,
            file_b: "src/b.rs".to_string(),
            start_line_b: 1,
            end_line_b: 10,
            token_count: 55,
            similarity: 1.0,
            fragment_hash: "hash_exact".to_string(),
            clone_type: CloneType::Exact,
            author_a: None,
            author_b: None,
        };

        let url = Url::parse("file:///workspace/src/a.rs").expect("valid url");
        let range = Range {
            start: Position {
                line: 2,
                character: 0,
            },
            end: Position {
                line: 4,
                character: 0,
            },
        };

        let ws_root = Path::new("/workspace");
        let actions = generate_code_actions(&url, &range, &[clone], ws_root);

        assert!(!actions.is_empty());
        let rescan_action = actions.iter().any(|a| match a {
            CodeActionOrCommand::CodeAction(ca) => ca.title.contains("Rescan workspace"),
            _ => false,
        });
        assert!(rescan_action);
    }
}
