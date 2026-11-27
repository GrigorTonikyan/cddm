#![forbid(unsafe_code)]

pub use super::budgets::CompiledTokenBudget;
pub use super::patterns::CompiledProhibitedPattern;
use crate::types::{BoundaryRule, LimitRule, ZeroDuplicationRule};
use ignore::gitignore::Gitignore;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct CompiledBoundary {
    pub rule: BoundaryRule,
    pub source_matcher: Gitignore,
    pub target_matchers: Vec<Gitignore>,
}

#[derive(Clone, Debug)]
pub struct CompiledZeroDuplication {
    pub rule: ZeroDuplicationRule,
    pub matcher: Gitignore,
}

#[derive(Clone, Debug)]
pub struct CompiledLimit {
    pub rule: LimitRule,
    pub matcher: Gitignore,
}

pub fn expand_brace_pattern(pattern: &str) -> Vec<String> {
    let Some(open_idx) = pattern.find('{') else {
        return vec![pattern.to_string()];
    };
    let rest = &pattern[open_idx + 1..];
    let Some(close_rel) = rest.find('}') else {
        return vec![pattern.to_string()];
    };
    let prefix = &pattern[..open_idx];
    let inner = &rest[..close_rel];
    let suffix = &rest[close_rel + 1..];

    let mut out = Vec::new();
    for part in inner.split(',') {
        let candidate = format!("{prefix}{}{suffix}", part.trim());
        out.extend(expand_brace_pattern(&candidate));
    }
    out
}

pub fn add_glob_pattern_lines(builder: &mut ignore::gitignore::GitignoreBuilder, pattern: &str) {
    for expanded in expand_brace_pattern(pattern) {
        let _ = builder.add_line(None, &expanded);
    }
}

fn normalize_rel_path(path: &Path) -> String {
    let mut norm = path.to_string_lossy().replace('\\', "/");
    while let Some(rest) = norm.strip_prefix("./") {
        norm = rest.to_string();
    }
    norm.trim_start_matches('/').to_string()
}

pub fn path_matches_glob(matcher: &Gitignore, path: &Path) -> bool {
    let norm = normalize_rel_path(path);
    if norm.is_empty() {
        return false;
    }
    if matcher.matched(Path::new(&norm), false).is_ignore() {
        return true;
    }
    let components: Vec<&str> = norm.split('/').collect();
    for i in 1..components.len() {
        let sub = components[i..].join("/");
        if let ignore::Match::Ignore(glob) = matcher.matched(Path::new(&sub), false)
            && !glob.original().starts_with('/')
        {
            return true;
        }
    }
    false
}

/// Matches `path` against `matcher` after stripping an optional scan `root` prefix so leading-`/`
/// patterns anchor relative to `root`.
pub fn path_matches_glob_rooted(matcher: &Gitignore, path: &Path, root: Option<&str>) -> bool {
    let norm = normalize_rel_path(path);
    let stripped = match root {
        Some(r) => {
            let root_norm = normalize_rel_path(Path::new(r));
            if root_norm.is_empty() || root_norm == "." {
                norm.as_str()
            } else {
                norm.strip_prefix(&root_norm)
                    .and_then(|rest| rest.strip_prefix('/'))
                    .unwrap_or(norm.as_str())
            }
        }
        None => norm.as_str(),
    };
    path_matches_glob(matcher, Path::new(stripped))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ignore::gitignore::GitignoreBuilder;

    fn build_matcher(pattern: &str) -> Gitignore {
        let mut b = GitignoreBuilder::new("");
        add_glob_pattern_lines(&mut b, pattern);
        b.build().expect("valid glob")
    }

    #[test]
    fn test_anchored_vs_unanchored_globs() {
        let anchored_root = build_matcher("/*.js");
        assert!(path_matches_glob(&anchored_root, Path::new("foo.js")));
        assert!(!path_matches_glob(&anchored_root, Path::new("src/foo.js")));

        let unanchored = build_matcher("*.js");
        assert!(path_matches_glob(&unanchored, Path::new("foo.js")));
        assert!(path_matches_glob(&unanchored, Path::new("src/foo.js")));

        let anchored_dir = build_matcher("/src/components/*.tsx");
        assert!(path_matches_glob(
            &anchored_dir,
            Path::new("src/components/A.tsx")
        ));
        assert!(!path_matches_glob(
            &anchored_dir,
            Path::new("src/components/A/A.tsx")
        ));
        assert!(!path_matches_glob(
            &anchored_dir,
            Path::new("pkg/src/components/A.tsx")
        ));

        let unanchored_dir = build_matcher("src/components/*.tsx");
        assert!(path_matches_glob(
            &unanchored_dir,
            Path::new("pkg/src/components/A.tsx")
        ));

        let braced = build_matcher("/{a,b}/*.rs");
        assert!(path_matches_glob(&braced, Path::new("a/mod.rs")));
        assert!(!path_matches_glob(&braced, Path::new("sub/a/mod.rs")));
    }

    #[test]
    fn test_path_matches_glob_rooted_strips_prefix() {
        let anchored = build_matcher("/src/*.rs");
        assert!(path_matches_glob_rooted(
            &anchored,
            Path::new("workspace/src/main.rs"),
            Some("workspace")
        ));
        assert!(!path_matches_glob_rooted(
            &anchored,
            Path::new("workspace/pkg/src/main.rs"),
            Some("workspace")
        ));
    }
}
