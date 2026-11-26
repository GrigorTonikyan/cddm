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

pub fn path_matches_glob(matcher: &Gitignore, path: &Path) -> bool {
    let norm = path.to_string_lossy().replace('\\', "/");
    let p = Path::new(&norm);
    if matcher.matched(p, false).is_ignore() {
        return true;
    }
    let components: Vec<&str> = norm.split('/').collect();
    for i in 0..components.len() {
        let sub = components[i..].join("/");
        if matcher.matched(Path::new(&sub), false).is_ignore() {
            return true;
        }
    }
    false
}
