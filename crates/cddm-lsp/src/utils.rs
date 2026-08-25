#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::{Position, Range, Url};

/// Converts a 1-based start and end line into a 0-based LSP `Range`.
#[must_use]
pub fn line_range_to_lsp_range(start_line: usize, end_line: usize) -> Range {
    let start_0 = if start_line > 0 { start_line - 1 } else { 0 };
    let end_0 = if end_line > 0 { end_line - 1 } else { 0 };

    Range {
        start: Position {
            line: u32::try_from(start_0).unwrap_or(u32::MAX),
            character: 0,
        },
        end: Position {
            line: u32::try_from(end_0).unwrap_or(u32::MAX),
            character: u32::MAX,
        },
    }
}

/// Converts a file path string or `Path` to an LSP `Url`.
#[must_use]
pub fn path_to_url(path: &Path) -> Option<Url> {
    let canonical = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok().map(|cwd| cwd.join(path))?
    };

    Url::from_file_path(canonical).ok()
}

/// Converts an LSP `Url` to a normalized `PathBuf`.
#[must_use]
pub fn url_to_path(url: &Url) -> Option<PathBuf> {
    if let Ok(path) = url.to_file_path() {
        Some(path)
    } else {
        // Fallback for URLs with non-standard file schemes on Windows
        let path_str = url.path().trim_start_matches('/');
        Some(PathBuf::from(path_str))
    }
}

/// Normalizes a path string to forward slashes for consistent comparison.
#[must_use]
pub fn normalize_path_for_compare(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches('/')
        .to_string()
}

/// Checks if a file path matches an LSP document URL.
#[must_use]
pub fn path_matches_url(file_path: &str, url: &Url) -> bool {
    let norm_file = normalize_path_for_compare(file_path);
    let norm_url_path = normalize_path_for_compare(url.path());

    if norm_url_path == norm_file
        || norm_url_path.ends_with(&norm_file)
        || norm_file.ends_with(&norm_url_path)
    {
        return true;
    }

    if let Some(doc_path) = url_to_path(url) {
        let doc_str = doc_path.to_string_lossy().to_string();
        let norm_doc = normalize_path_for_compare(&doc_str);

        if norm_doc == norm_file || norm_doc.ends_with(&norm_file) || norm_file.ends_with(&norm_doc)
        {
            return true;
        }
    }

    false
}

/// Matches a clone pair against a target normalized path.
/// Returns (my_start, my_end, other_file, other_start, other_end) if matched.
#[must_use]
pub fn match_clone_occurrence<'a>(
    clone: &'a cddm_core::ClonePair,
    target_norm: &str,
) -> Option<(usize, usize, &'a str, usize, usize)> {
    let norm_a = normalize_path_for_compare(&clone.file_a);
    let norm_b = normalize_path_for_compare(&clone.file_b);

    let is_a =
        norm_a == target_norm || target_norm.ends_with(&norm_a) || norm_a.ends_with(target_norm);
    let is_b =
        norm_b == target_norm || target_norm.ends_with(&norm_b) || norm_b.ends_with(target_norm);

    if is_a {
        Some((
            clone.start_line_a,
            clone.end_line_a,
            &clone.file_b,
            clone.start_line_b,
            clone.end_line_b,
        ))
    } else if is_b {
        Some((
            clone.start_line_b,
            clone.end_line_b,
            &clone.file_a,
            clone.start_line_a,
            clone.end_line_a,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_range_to_lsp_range() {
        let range = line_range_to_lsp_range(1, 10);
        assert_eq!(range.start.line, 0);
        assert_eq!(range.start.character, 0);
        assert_eq!(range.end.line, 9);
        assert_eq!(range.end.character, u32::MAX);

        let zero_range = line_range_to_lsp_range(0, 0);
        assert_eq!(zero_range.start.line, 0);
        assert_eq!(zero_range.end.line, 0);
    }

    #[test]
    fn test_normalize_path_for_compare() {
        assert_eq!(
            normalize_path_for_compare("src\\components\\App.tsx"),
            "src/components/App.tsx"
        );
        assert_eq!(normalize_path_for_compare("./src/main.rs"), "src/main.rs");
        assert_eq!(
            normalize_path_for_compare("/projects/cddm/src/main.rs"),
            "projects/cddm/src/main.rs"
        );
    }

    #[test]
    fn test_path_matches_url() {
        let url = Url::parse("file:///projects/cddm/src/main.rs").expect("valid url");
        assert!(path_matches_url("/projects/cddm/src/main.rs", &url));
        assert!(path_matches_url("src/main.rs", &url));
        assert!(!path_matches_url("src/other.rs", &url));
    }
}
