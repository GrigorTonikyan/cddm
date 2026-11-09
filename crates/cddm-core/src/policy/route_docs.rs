#![forbid(unsafe_code)]

use crate::types::{PolicyViolation, RouteDocsPolicy};
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Evaluates route documentation completeness policies.
pub fn evaluate_route_docs_policies(
    root_dir: &Path,
    policies: &[RouteDocsPolicy],
) -> Vec<PolicyViolation> {
    if policies.is_empty() {
        return Vec::new();
    }

    let mut violations = Vec::new();

    for policy in policies {
        let routes_dir_path = root_dir.join(&policy.routes_dir);
        if !routes_dir_path.exists() {
            continue;
        }

        let mut doc_paths = policy.doc_files.clone();
        if let Some(ref f) = policy.doc_file
            && !doc_paths.contains(f)
        {
            doc_paths.push(f.clone());
        }
        if doc_paths.is_empty() {
            doc_paths.push("README.md".to_string());
        }

        let mut combined_docs = String::new();
        let mut primary_doc = doc_paths[0].clone();
        let mut found_any_doc = false;

        for doc_rel in &doc_paths {
            let doc_path = root_dir.join(doc_rel);
            if doc_path.exists()
                && let Ok(c) = fs::read_to_string(&doc_path)
            {
                combined_docs.push_str(&c);
                combined_docs.push('\n');
                found_any_doc = true;
                primary_doc = doc_rel.clone();
            }
        }

        if !found_any_doc {
            violations.push(PolicyViolation {
                rule_name: policy
                    .name
                    .clone()
                    .unwrap_or_else(|| "route-docs-sync".to_string()),
                rule_type: "route_docs".to_string(),
                severity: policy.severity,
                message: format!(
                    "Documentation file '{}' specified in route docs policy does not exist",
                    primary_doc
                ),
                file_a: primary_doc.clone(),
                start_line_a: 1,
                end_line_a: 1,
                file_b: None,
                start_line_b: None,
                end_line_b: None,
                cluster_id: None,
                token_count: 0,
                snippet: None,
                remediation: Some(format!(
                    "Create documentation file '{}' to document routes",
                    primary_doc
                )),
            });
            continue;
        }

        let discovered_routes = discover_routes(&routes_dir_path, policy);

        for (route_str, file_rel) in discovered_routes {
            if !is_route_documented(&route_str, &combined_docs) {
                let rule_name = policy
                    .name
                    .clone()
                    .unwrap_or_else(|| "route-docs-sync".to_string());
                violations.push(PolicyViolation {
                    rule_name,
                    rule_type: "route_docs".to_string(),
                    severity: policy.severity,
                    message: format!(
                        "Undocumented framework route '{}' discovered at '{}' is missing from '{}'",
                        route_str, file_rel, primary_doc
                    ),
                    file_a: file_rel.clone(),
                    start_line_a: 1,
                    end_line_a: 1,
                    file_b: Some(primary_doc.clone()),
                    start_line_b: Some(1),
                    end_line_b: Some(1),
                    cluster_id: None,
                    token_count: 0,
                    snippet: Some(format!("Route: {route_str}")),
                    remediation: policy.remediation.clone().or_else(|| {
                        Some(format!(
                            "Document route '{}' in '{}'",
                            route_str, primary_doc
                        ))
                    }),
                });
            }
        }
    }

    violations
}

fn discover_routes(routes_dir_path: &Path, policy: &RouteDocsPolicy) -> Vec<(String, String)> {
    let mut routes = Vec::new();
    let mut seen_routes = HashSet::new();

    let walker = WalkBuilder::new(routes_dir_path)
        .hidden(false)
        .git_ignore(false)
        .filter_entry(|e| {
            if e.file_type().is_some_and(|ft| ft.is_dir())
                && let Some(name) = e.file_name().to_str()
            {
                return !matches!(
                    name,
                    ".git" | "node_modules" | "target" | "dist" | ".next" | ".cache"
                );
            }
            true
        })
        .build();

    for entry_res in walker {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let path = entry.path();
        let rel = match path.strip_prefix(routes_dir_path) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if let Some(route_str) = extract_route_from_path(rel) {
            let norm_rel = rel.to_string_lossy().replace('\\', "/");
            let is_ignored = policy.ignore_routes.iter().any(|pattern| {
                let clean_p = pattern.trim_matches('/');
                route_str
                    .trim_matches('/')
                    .starts_with(clean_p.trim_end_matches('*').trim_end_matches('/'))
                    || norm_rel.contains(clean_p.trim_end_matches('*'))
            });

            if !is_ignored && seen_routes.insert(route_str.clone()) {
                let file_rel = path.to_string_lossy().replace('\\', "/");
                routes.push((route_str, file_rel));
            }
        }
    }

    routes
}

pub fn extract_route_from_path(rel_path: &Path) -> Option<String> {
    let norm = rel_path.to_string_lossy().replace('\\', "/");
    let parts: Vec<&str> = norm.split('/').collect();
    if parts.is_empty() {
        return None;
    }

    let filename = parts.last().unwrap();
    let stem = filename.split('.').next().unwrap_or("");
    let ext = filename.split('.').next_back().unwrap_or("");

    let is_page_or_route = matches!(stem, "page" | "route" | "index");
    let is_astro_or_supported_ext = matches!(ext, "astro" | "tsx" | "jsx" | "ts" | "js");

    if !is_astro_or_supported_ext {
        return None;
    }

    let mut segments = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if !is_page_or_route && !stem.is_empty() {
                segments.push(stem.to_string());
            }
        } else {
            let clean = if part.starts_with('(') && part.ends_with(')') {
                &part[1..part.len() - 1]
            } else {
                part
            };
            if !clean.is_empty() {
                segments.push(clean.to_string());
            }
        }
    }

    if segments.is_empty() {
        Some("/".to_string())
    } else {
        Some(format!("/{}", segments.join("/")))
    }
}

fn is_route_documented(route: &str, docs: &str) -> bool {
    if route == "/" {
        return docs.contains("`/`")
            || docs.contains("\"/\"")
            || docs.contains("'/'")
            || docs.contains(" / ")
            || docs.contains("Root route")
            || docs.contains("root route")
            || docs.contains("Home");
    }

    if docs.contains(route) {
        return true;
    }

    let trimmed = route.trim_start_matches('/');
    if !trimmed.is_empty() {
        let backticked = format!("`{trimmed}`");
        let quoted_double = format!("\"{trimmed}\"");
        let quoted_single = format!("'{trimmed}'");
        if docs.contains(&backticked)
            || docs.contains(&quoted_double)
            || docs.contains(&quoted_single)
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicySeverity;
    use tempfile::tempdir;

    #[test]
    fn test_route_docs_policy_missing_route() {
        let dir = tempdir().expect("create tempdir");
        let root = dir.path();

        let app_dir = root.join("src").join("app");
        std::fs::create_dir_all(app_dir.join("dashboard")).expect("create dashboard dir");
        std::fs::write(
            app_dir.join("dashboard").join("page.tsx"),
            "export default function Dashboard() { return null; }",
        )
        .expect("write page");

        let readme = root.join("README.md");
        std::fs::write(
            &readme,
            "# My App\nNo routes listed yet in documentation.\n",
        )
        .expect("write readme");

        let policy = RouteDocsPolicy {
            name: Some("nextjs-route-docs".to_string()),
            framework: Some("nextjs".to_string()),
            routes_dir: "src/app".to_string(),
            doc_file: Some("README.md".to_string()),
            doc_files: vec![],
            ignore_routes: vec![],
            severity: PolicySeverity::Error,
            remediation: Some("Document routes in README.md".to_string()),
        };

        let violations = evaluate_route_docs_policies(root, &[policy]);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule_type, "route_docs");
        assert!(violations[0].message.contains("/dashboard"));
        assert!(violations[0].message.contains("README.md"));
    }

    #[test]
    fn test_route_docs_policy_documented_route() {
        let dir = tempdir().expect("create tempdir");
        let root = dir.path();

        let app_dir = root.join("src").join("app");
        std::fs::create_dir_all(app_dir.join("dashboard")).expect("create dashboard dir");
        std::fs::write(
            app_dir.join("dashboard").join("page.tsx"),
            "export default function Dashboard() { return null; }",
        )
        .expect("write page");

        let readme = root.join("README.md");
        std::fs::write(
            &readme,
            "# My App\n- `/dashboard`: Main analytics dashboard\n",
        )
        .expect("write readme");

        let policy = RouteDocsPolicy {
            name: Some("nextjs-route-docs".to_string()),
            framework: Some("nextjs".to_string()),
            routes_dir: "src/app".to_string(),
            doc_file: Some("README.md".to_string()),
            doc_files: vec![],
            ignore_routes: vec![],
            severity: PolicySeverity::Error,
            remediation: None,
        };

        let violations = evaluate_route_docs_policies(root, &[policy]);
        assert!(violations.is_empty());
    }
}
