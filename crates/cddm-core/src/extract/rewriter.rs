#![forbid(unsafe_code)]

use super::types::{CallerRewrite, ExtractTargetKind};
use crate::ast::import_resolver::generate_import_statement;
use crate::ast::rewriter::{CloneSiteReplacement, rewrite_source_file, validate_ast_syntax};
use crate::types::CloneLocation;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Rewrites caller occurrence files to replace duplicate code blocks with helper invocations and imports.
pub fn rewrite_caller_files(
    workspace_root: &Path,
    occurrences: &[CloneLocation],
    function_name: &str,
    target_path: &str,
    target_kind: ExtractTargetKind,
    ext: &str,
) -> Result<(Vec<CallerRewrite>, bool), String> {
    let mut files_map: BTreeMap<String, Vec<&CloneLocation>> = BTreeMap::new();
    for occ in occurrences {
        files_map.entry(occ.file.clone()).or_default().push(occ);
    }

    let mut rewrites = Vec::new();
    let mut all_syntax_valid = true;

    for (file_path, occs) in files_map {
        let abs_path = workspace_root.join(&file_path);
        let raw_content = fs::read_to_string(&abs_path)
            .map_err(|e| format!("Failed to read caller file '{}': {}", file_path, e))?;

        let import_stmt = match target_kind {
            ExtractTargetKind::NewCrate => {
                generate_crate_import_statement(&file_path, target_path, function_name, ext)
            }
            ExtractTargetKind::NewModule
            | ExtractTargetKind::ExistingModule
            | ExtractTargetKind::Auto => {
                generate_import_statement(&file_path, target_path, function_name, ext)
            }
        };

        let mut replacements = Vec::new();
        for occ in occs {
            replacements.push(CloneSiteReplacement {
                start_line: occ.start_line,
                end_line: occ.end_line,
                arguments: Vec::new(),
            });
        }

        let rewritten = rewrite_source_file(
            &file_path,
            &raw_content,
            ext,
            function_name,
            Some(target_path),
            replacements,
        );

        if !validate_ast_syntax(&rewritten.rewritten_source, ext) {
            all_syntax_valid = false;
        }

        let diff_patch = generate_file_diff(&file_path, &raw_content, &rewritten.rewritten_source);

        rewrites.push(CallerRewrite {
            file_path,
            injected_import: import_stmt,
            rewritten_content: rewritten.rewritten_source,
            diff_patch,
        });
    }

    Ok((rewrites, all_syntax_valid))
}

fn generate_crate_import_statement(
    _caller_file: &str,
    target_path: &str,
    function_name: &str,
    ext: &str,
) -> Option<String> {
    let target_name = Path::new(target_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("shared_utils");

    match ext.to_lowercase().as_str() {
        "rs" => {
            let crate_ident = target_name.replace('-', "_");
            Some(format!("use {}::{};", crate_ident, function_name))
        }
        "ts" | "tsx" | "js" | "jsx" => Some(format!(
            "import {{ {} }} from \"{}\";",
            function_name, target_name
        )),
        "py" => Some(format!("from {} import {}", target_name, function_name)),
        "go" => Some(format!("import \"{}\"", target_name)),
        _ => None,
    }
}

fn generate_file_diff(file_path: &str, original: &str, modified: &str) -> String {
    let norm_path = file_path.replace('\\', "/");
    let orig_lines: Vec<&str> = original.lines().collect();
    let mod_lines: Vec<&str> = modified.lines().collect();

    let mut diff = format!("--- a/{}\n+++ b/{}\n", norm_path, norm_path);

    // Simple hunk preview
    diff.push_str("@@ -1,");
    diff.push_str(&orig_lines.len().to_string());
    diff.push_str(" +1,");
    diff.push_str(&mod_lines.len().to_string());
    diff.push_str(" @@\n");

    for line in mod_lines.iter().take(10) {
        diff.push_str("+ ");
        diff.push_str(line);
        diff.push('\n');
    }
    if mod_lines.len() > 10 {
        diff.push_str("...\n");
    }

    diff
}
