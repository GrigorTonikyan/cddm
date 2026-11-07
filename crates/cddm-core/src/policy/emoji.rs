#![forbid(unsafe_code)]

use crate::types::{PolicySeverity, PolicyViolation};
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::Path;

/// Returns true if a character matches known emoji / pictographic codepoint ranges.
pub fn is_emoji_char(c: char) -> bool {
    matches!(
        c,
        '\u{1F300}'..='\u{1FAFF}'
            | '\u{1F000}'..='\u{1F02F}'
            | '\u{1F0A0}'..='\u{1F0FF}'
            | '\u{1F100}'..='\u{1F64F}'
            | '\u{2600}'..='\u{27BF}'
            | '\u{2300}'..='\u{23FF}'
            | '\u{2B50}'
            | '\u{2B55}'
    )
}

fn is_ignored_extension(ext: &str) -> bool {
    matches!(
        ext.to_lowercase().as_str(),
        "lock"
            | "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "webp"
            | "ico"
            | "svg"
            | "bmp"
            | "tiff"
            | "wasm"
            | "exe"
            | "dll"
            | "so"
            | "dylib"
            | "bin"
            | "db"
            | "pdf"
            | "vsix"
            | "zip"
            | "tar"
            | "gz"
            | "tgz"
            | "7z"
            | "bz2"
            | "xz"
            | "zst"
            | "woff"
            | "woff2"
            | "ttf"
            | "eot"
            | "otf"
            | "pyc"
            | "pyo"
            | "class"
            | "jar"
            | "mp3"
            | "mp4"
    )
}

fn is_ignored_directory(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "node_modules"
            | "target"
            | "dist"
            | "out"
            | "coverage"
            | ".vscode"
            | ".cddm"
            | "packaging"
            | ".cache"
            | ".vite"
            | ".vite-hooks"
            | "test-results"
            | "playwright-report"
            | "blob-report"
    )
}

/// Evaluates zero-emoji policy across all repository files under `root_dir`.
pub fn evaluate_zero_emoji(root_dir: &Path) -> Vec<PolicyViolation> {
    let mut violations = Vec::new();
    let walker = WalkBuilder::new(root_dir)
        .hidden(true)
        .git_ignore(true)
        .build();

    let mut visited_paths = HashSet::new();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        // Check if inside any ignored directory component
        let is_in_ignored_dir = path.components().any(|c| {
            c.as_os_str()
                .to_str()
                .map(is_ignored_directory)
                .unwrap_or(false)
        });
        if is_in_ignored_dir {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|s| s.to_str())
            && is_ignored_extension(ext)
        {
            continue;
        }

        let canonical_str = path.to_string_lossy().replace('\\', "/");
        if visited_paths.contains(&canonical_str) {
            continue;
        }
        visited_paths.insert(canonical_str.clone());

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue, // Skip binary / unreadable files
        };

        for (line_idx, line) in content.lines().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                if is_emoji_char(ch) {
                    let line_no = line_idx + 1;
                    let col_no = col_idx + 1;
                    let snippet = line.trim().to_string();

                    violations.push(PolicyViolation {
                        rule_name: "zero-emoji".to_string(),
                        rule_type: "zero_emojis".to_string(),
                        severity: PolicySeverity::Error,
                        message: format!(
                            "Disallowed emoji character '{ch}' (U+{:04X}) detected at {canonical_str}:{line_no}:{col_no}",
                            ch as u32
                        ),
                        file_a: canonical_str.clone(),
                        start_line_a: line_no,
                        end_line_a: line_no,
                        file_b: None,
                        start_line_b: None,
                        end_line_b: None,
                        cluster_id: None,
                        token_count: 1,
                        snippet: Some(snippet),
                        remediation: Some(
                            "Remove emoji or pictographic glyph from codebase".to_string(),
                        ),
                    });
                    break; // Report at most one violation per line to avoid noise
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_emoji_char() {
        assert!(is_emoji_char('\u{1F600}')); // Grinning face
        assert!(is_emoji_char('\u{2705}')); // White check mark
        assert!(is_emoji_char('\u{26A0}')); // Warning sign
        assert!(!is_emoji_char('A'));
        assert!(!is_emoji_char('1'));
        assert!(!is_emoji_char(';'));
    }
}
