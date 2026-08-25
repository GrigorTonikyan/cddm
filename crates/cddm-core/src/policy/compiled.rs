#![forbid(unsafe_code)]

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
