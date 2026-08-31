#![forbid(unsafe_code)]

use crate::utils::{line_range_to_lsp_range, normalize_path_for_compare, path_to_url, url_to_path};
use cddm_core::ClonePair;
use std::path::Path;
use tower_lsp::lsp_types::{CodeLens, Command, Range, Url};

/// Generates interactive CodeLens items for code clone occurrences in a document.
#[must_use]
pub fn generate_code_lenses(
    url: &Url,
    clones: &[ClonePair],
    workspace_root: &Path,
) -> Vec<CodeLens> {
    let mut lenses = Vec::new();

    let target_norm = if let Some(path) = url_to_path(url) {
        normalize_path_for_compare(&path.to_string_lossy())
    } else {
        normalize_path_for_compare(url.as_str())
    };

    for clone in clones {
        let Some((my_start, my_end, other_file, other_start, _other_end)) =
            crate::utils::match_clone_occurrence(clone, &target_norm)
        else {
            continue;
        };

        let clone_range = line_range_to_lsp_range(my_start, my_end);
        let header_range = Range {
            start: clone_range.start,
            end: clone_range.start,
        };

        let counterpart_path = if Path::new(other_file).is_absolute() {
            other_file.to_string()
        } else {
            workspace_root
                .join(other_file)
                .to_string_lossy()
                .to_string()
        };

        let counterpart_url = path_to_url(Path::new(&counterpart_path));
        let title = format!(
            "CDDM: {} duplicate (similarity: {:.1}%) -> {}:{}",
            clone.clone_type,
            clone.similarity * 100.0,
            other_file,
            other_start
        );

        let args = if let Some(ref c_url) = counterpart_url {
            vec![
                serde_json::json!(c_url.to_string()),
                serde_json::json!(other_start),
            ]
        } else {
            vec![
                serde_json::json!(counterpart_path),
                serde_json::json!(other_start),
            ]
        };

        lenses.push(CodeLens {
            range: header_range,
            command: Some(Command {
                title,
                command: "cddm.openLocation".to_string(),
                arguments: Some(args),
            }),
            data: None,
        });
    }

    lenses
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::types::CloneType;

    #[test]
    fn test_generate_code_lenses() {
        let root = Path::new("/workspace");
        let url = Url::parse("file:///workspace/src/math.rs").unwrap();

        let clone = ClonePair {
            file_a: "src/math.rs".to_string(),
            start_line_a: 10,
            end_line_a: 20,
            file_b: "src/calc.rs".to_string(),
            start_line_b: 30,
            end_line_b: 40,
            token_count: 50,
            similarity: 0.95,
            fragment_hash: "hash1".to_string(),
            clone_type: CloneType::Exact,
            author_a: None,
            author_b: None,
        };

        let lenses = generate_code_lenses(&url, &[clone], root);
        assert_eq!(lenses.len(), 1);
        let cmd = lenses[0].command.as_ref().unwrap();
        assert!(cmd.title.contains("Exact duplicate"));
        assert!(cmd.title.contains("src/calc.rs:30"));
        assert_eq!(cmd.command, "cddm.openLocation");
    }
}
