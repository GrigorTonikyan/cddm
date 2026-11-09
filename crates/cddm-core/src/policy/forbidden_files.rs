#![forbid(unsafe_code)]

use super::compiled::path_matches_glob;
use crate::types::{ForbiddenFileRule, PolicyViolation};
use ignore::WalkBuilder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Compiled forbidden file rule matcher.
#[derive(Clone, Debug)]
pub struct CompiledForbiddenFile {
    pub rule: ForbiddenFileRule,
    pub matcher: Gitignore,
}

impl CompiledForbiddenFile {
    /// Compiles a ForbiddenFileRule into a CompiledForbiddenFile.
    pub fn compile(rule: &ForbiddenFileRule) -> Result<Self, String> {
        let mut b = GitignoreBuilder::new("");
        let mut count = 0;
        if let Some(p) = rule.pattern.as_deref() {
            let _ = b.add_line(None, p);
            count += 1;
        }
        for p in &rule.patterns {
            let _ = b.add_line(None, p);
            count += 1;
        }
        if count == 0 {
            return Err(format!(
                "Forbidden file rule '{}' has no pattern or patterns defined",
                rule.name
            ));
        }
        let matcher = b.build().map_err(|e| {
            format!(
                "Failed to compile forbidden file pattern for '{}': {e}",
                rule.name
            )
        })?;

        Ok(Self {
            rule: rule.clone(),
            matcher,
        })
    }
}

fn is_system_ignored_dir(name: &str) -> bool {
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
    )
}

/// Evaluates forbidden files across all entries under `root_dir`.
pub fn evaluate_forbidden_files(
    root_dir: &Path,
    rules: &[CompiledForbiddenFile],
) -> Vec<PolicyViolation> {
    if rules.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();

    let walker = WalkBuilder::new(root_dir)
        .hidden(false)
        .git_ignore(false)
        .filter_entry(|entry| {
            if entry.file_type().is_some_and(|ft| ft.is_dir())
                && let Some(name) = entry.file_name().to_str()
            {
                return !is_system_ignored_dir(name);
            }
            true
        })
        .build();

    for entry_res in walker {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path == root_dir {
            continue;
        }

        let rel_path = path.strip_prefix(root_dir).unwrap_or(path);
        let rel_str = rel_path.to_string_lossy().replace('\\', "/");

        for rule in rules {
            if path_matches_glob(&rule.matcher, rel_path) {
                violations.push(PolicyViolation {
                    rule_name: rule.rule.name.clone(),
                    rule_type: "forbidden_file".to_string(),
                    severity: rule.rule.severity,
                    message: format!("Forbidden file '{}' discovered on disk", rel_str),
                    file_a: rel_str.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    file_b: None,
                    start_line_b: None,
                    end_line_b: None,
                    cluster_id: None,
                    token_count: 0,
                    snippet: Some(rel_str.clone()),
                    remediation: rule.rule.remediation.clone().or_else(|| {
                        Some("Remove the forbidden file or update policy configuration".to_string())
                    }),
                });
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicySeverity;
    use tempfile::tempdir;

    #[test]
    fn test_forbidden_file_detection() {
        let dir = tempdir().expect("create tempdir");
        let root = dir.path();

        let bad_file = root.join(".eslintrc.json");
        std::fs::write(&bad_file, "{}").expect("write bad file");

        let lock_file = root.join("yarn.lock");
        std::fs::write(&lock_file, "# yarn lockfile").expect("write yarn lock");

        let safe_file = root.join("index.ts");
        std::fs::write(&safe_file, "console.log('hi');").expect("write safe file");

        let rule = ForbiddenFileRule {
            name: "no-legacy-linters-or-yarn".to_string(),
            description: Some("Ban legacy linters and non-bun lockfiles".to_string()),
            pattern: None,
            patterns: vec![".eslintrc*".to_string(), "yarn.lock".to_string()],
            severity: PolicySeverity::Error,
            remediation: Some("Use oxlint and bun.lock".to_string()),
        };

        let compiled = CompiledForbiddenFile::compile(&rule).expect("compile rule");
        let violations = evaluate_forbidden_files(root, &[compiled]);

        assert_eq!(violations.len(), 2);
        assert!(
            violations
                .iter()
                .any(|v| v.file_a.contains(".eslintrc.json"))
        );
        assert!(violations.iter().any(|v| v.file_a.contains("yarn.lock")));
        assert_eq!(violations[0].rule_type, "forbidden_file");
        assert_eq!(violations[0].severity, PolicySeverity::Error);
    }
}
