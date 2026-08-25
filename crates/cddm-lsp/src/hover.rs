#![forbid(unsafe_code)]

use crate::code_actions::range_overlaps_lines;
use crate::utils::{line_range_to_lsp_range, normalize_path_for_compare, path_to_url, url_to_path};
use cddm_core::{ClonePair, CloneType};
use std::path::Path;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range, Url};

/// Generates hover tooltip information for duplicate code under the cursor.
#[must_use]
pub fn generate_hover(
    url: &Url,
    position: &Position,
    clones: &[ClonePair],
    workspace_root: &Path,
) -> Option<Hover> {
    let target_norm = if let Some(path) = url_to_path(url) {
        normalize_path_for_compare(&path.to_string_lossy())
    } else {
        normalize_path_for_compare(url.as_str())
    };

    let pos_range = Range {
        start: *position,
        end: *position,
    };

    for clone in clones {
        let Some((my_start, my_end, other_file, other_start, other_end)) =
            crate::utils::match_clone_occurrence(clone, &target_norm)
        else {
            continue;
        };

        if !range_overlaps_lines(&pos_range, my_start, my_end) {
            continue;
        }

        let clone_type_desc = match clone.clone_type {
            CloneType::Exact => "Type-1 (Exact Clone)",
            CloneType::Renamed => "Type-2 (Renamed Identifiers / Literals)",
            CloneType::NearMiss => "Type-3 (Near-Miss with Statement Variations)",
            CloneType::Semantic => "Type-4 (AST Semantic Structural Subtree)",
        };

        let sim_pct = (clone.similarity * 100.0).round();
        let counterpart_path = workspace_root.join(other_file);
        let counterpart_link = if let Some(counterpart_url) = path_to_url(&counterpart_path) {
            format!("[`{other_file}:{other_start}-{other_end}`]({counterpart_url})")
        } else {
            format!("`{other_file}:{other_start}-{other_end}`")
        };

        let markdown = format!(
            "### [CDDM] Code Duplication Detected\n\n* **Classification**: {clone_type_desc}\n* \
             **Similarity**: {sim_pct}%\n* **Volume**: {} tokens across {} lines \
             (L{my_start}-L{my_end})\n* **Duplicate Counterpart**: \
             {counterpart_link}\n\n---\n*Tip: Use Code Action (`Alt+Enter` / `Ctrl+.`) to extract \
             a shared function.*",
            clone.token_count,
            my_end.saturating_sub(my_start) + 1,
        );

        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(line_range_to_lsp_range(my_start, my_end)),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hover() {
        let clone = ClonePair {
            file_a: "src/a.rs".to_string(),
            start_line_a: 5,
            end_line_a: 15,
            file_b: "src/b.rs".to_string(),
            start_line_b: 20,
            end_line_b: 30,
            token_count: 75,
            similarity: 0.98,
            fragment_hash: "hash_renamed".to_string(),
            clone_type: CloneType::Renamed,
            author_a: None,
            author_b: None,
        };

        let url = Url::parse("file:///workspace/src/a.rs").expect("valid url");
        let pos = Position {
            line: 7,
            character: 5,
        };
        let ws_root = Path::new("/workspace");

        let hover = generate_hover(&url, &pos, &[clone], ws_root);
        assert!(hover.is_some());

        let h = hover.unwrap();
        if let HoverContents::Markup(m) = h.contents {
            assert!(m.value.contains("Code Duplication Detected"));
            assert!(m.value.contains("Type-2"));
            assert!(m.value.contains("98%"));
            assert!(m.value.contains("75 tokens"));
        } else {
            panic!("expected markup contents");
        }
    }
}
