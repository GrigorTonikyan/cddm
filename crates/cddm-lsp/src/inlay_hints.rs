#![forbid(unsafe_code)]

use crate::utils::{line_range_to_lsp_range, normalize_path_for_compare, url_to_path};
use cddm_core::ClonePair;
use tower_lsp::lsp_types::{InlayHint, InlayHintLabel, InlayHintTooltip, Range, Url};

/// Generates inline Inlay Hints for clone occurrences within a document range.
#[must_use]
pub fn generate_inlay_hints(url: &Url, range: &Range, clones: &[ClonePair]) -> Vec<InlayHint> {
    let mut hints = Vec::new();

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

        if !crate::code_actions::range_overlaps_lines(range, my_start, my_end) {
            continue;
        }

        let lsp_range = line_range_to_lsp_range(my_start, my_end);
        let label = format!(
            "(clone: {:.0}% -> {}:{})",
            clone.similarity * 100.0,
            other_file,
            other_start
        );
        let tooltip = format!(
            "CDDM {} duplicate ({} tokens, {:.1}% similarity)\nCounterpart: {}:{}",
            clone.clone_type,
            clone.token_count,
            clone.similarity * 100.0,
            other_file,
            other_start
        );

        hints.push(InlayHint {
            position: lsp_range.start,
            label: InlayHintLabel::String(label),
            kind: None,
            text_edits: None,
            tooltip: Some(InlayHintTooltip::String(tooltip)),
            padding_left: Some(true),
            padding_right: Some(true),
            data: None,
        });
    }

    hints
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::types::CloneType;
    use tower_lsp::lsp_types::Position;

    #[test]
    fn test_generate_inlay_hints() {
        let url = Url::parse("file:///workspace/src/math.rs").unwrap();
        let query_range = Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 50,
                character: 0,
            },
        };

        let clone = ClonePair {
            file_a: "src/math.rs".to_string(),
            start_line_a: 10,
            end_line_a: 20,
            file_b: "src/calc.rs".to_string(),
            start_line_b: 30,
            end_line_b: 40,
            token_count: 50,
            similarity: 0.92,
            fragment_hash: "hash1".to_string(),
            clone_type: CloneType::Renamed,
            author_a: None,
            author_b: None,
        };

        let hints = generate_inlay_hints(&url, &query_range, &[clone]);
        assert_eq!(hints.len(), 1);
        if let InlayHintLabel::String(text) = &hints[0].label {
            assert!(text.contains("clone: 92%"));
            assert!(text.contains("src/calc.rs:30"));
        } else {
            panic!("Expected String InlayHintLabel");
        }
    }
}
