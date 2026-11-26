#![forbid(unsafe_code)]

use ignore::{Walk, WalkBuilder};
use std::path::Path;

/// Directory names never traversed by policy evaluators.
pub const SYSTEM_IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "out",
    "coverage",
    ".vscode",
    ".cddm",
    "packaging",
    ".cache",
    ".vite",
    ".vite-hooks",
];

/// Returns true if `name` is a system directory skipped by policy traversal.
pub fn is_system_ignored_dir(name: &str) -> bool {
    SYSTEM_IGNORED_DIRS.contains(&name)
}

/// Builds a policy walker that traverses hidden paths (`.gitea/`, `.agents/`,
/// `.env.example`) while pruning system directories and `extra_ignored` names.
pub fn policy_walker(
    root_dir: &Path,
    respect_gitignore: bool,
    extra_ignored: &'static [&'static str],
) -> Walk {
    WalkBuilder::new(root_dir)
        .hidden(false)
        .git_ignore(respect_gitignore)
        .filter_entry(move |entry| {
            if entry.depth() == 0 || !entry.file_type().is_some_and(|ft| ft.is_dir()) {
                return true;
            }
            entry
                .file_name()
                .to_str()
                .is_none_or(|name| !is_system_ignored_dir(name) && !extra_ignored.contains(&name))
        })
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_ignored_dirs() {
        for name in SYSTEM_IGNORED_DIRS {
            assert!(is_system_ignored_dir(name));
        }
        assert!(!is_system_ignored_dir(".gitea"));
        assert!(!is_system_ignored_dir(".github"));
        assert!(!is_system_ignored_dir(".agents"));
    }
}
