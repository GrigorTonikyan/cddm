#![forbid(unsafe_code)]

use crate::detector::run_scan;
use crate::types::{ScanConfig, ScanResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub const PKG_TYPE_RUST_CARGO: &str = "Rust (Cargo)";
pub const PKG_TYPE_GO_WORKSPACE: &str = "Go Workspace";
pub const PKG_TYPE_PYTHON_UV_POETRY: &str = "Python (uv/Poetry)";
pub const PKG_TYPE_JS_BUN: &str = "JavaScript/TypeScript (Bun)";
pub const PKG_TYPE_JS_PNPM: &str = "JavaScript/TypeScript (pnpm)";
pub const PKG_TYPE_JS_YARN: &str = "JavaScript/TypeScript (Yarn)";
pub const PKG_TYPE_JS_NPM: &str = "JavaScript/TypeScript (npm)";
pub const PKG_TYPE_ROOT_PROJECT: &str = "Root Project";

/// A detected submodule, package, or workspace crate within a monorepo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonorepoWorkspace {
    pub name: String,
    pub path: String,
    pub manifest_file: String,
    pub package_type: String,
}

/// Comprehensive aggregated summary across multiple monorepo workspaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonorepoScanSummary {
    pub workspaces: Vec<MonorepoWorkspace>,
    pub total_workspaces: usize,
    pub total_files: usize,
    pub total_tokens: usize,
    pub total_clones: usize,
    pub cross_workspace_clones: usize,
    pub average_dry_score: f64,
    pub scan_result: ScanResult,
}

/// Compact representation of monorepo scan summary to conserve AI agent context tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompactMonorepoSummary {
    pub summary_mode: bool,
    pub total_workspaces: usize,
    pub total_files: usize,
    pub total_tokens: usize,
    pub total_clones: usize,
    pub cross_workspace_clones: usize,
    pub average_dry_score: f64,
    pub duplication_percentage: f64,
    pub workspaces: Vec<MonorepoWorkspace>,
    pub top_clusters: Vec<crate::types::CompactClusterSummary>,
}

impl CompactMonorepoSummary {
    pub fn from_summary(summary: &MonorepoScanSummary, top_n: usize) -> Self {
        let mut sorted_clusters = summary.scan_result.clone_clusters.clone();
        sorted_clusters.sort_by(|a, b| {
            let weight_b = b.token_count * b.occurrences.len();
            let weight_a = a.token_count * a.occurrences.len();
            weight_b.cmp(&weight_a)
        });

        let top_clusters = sorted_clusters
            .into_iter()
            .take(top_n)
            .map(|c| crate::types::CompactClusterSummary::from_cluster(&c))
            .collect();

        Self {
            summary_mode: true,
            total_workspaces: summary.total_workspaces,
            total_files: summary.total_files,
            total_tokens: summary.total_tokens,
            total_clones: summary.total_clones,
            cross_workspace_clones: summary.cross_workspace_clones,
            average_dry_score: summary.average_dry_score,
            duplication_percentage: summary.scan_result.duplication_percentage,
            workspaces: summary.workspaces.clone(),
            top_clusters,
        }
    }
}

/// Detects the manifest file and ecosystem package type for a given package directory.
pub fn detect_package_in_dir(root: &Path, dir: &Path) -> Option<(String, String)> {
    // 1. Rust (Cargo)
    if dir.join("Cargo.toml").exists() {
        return Some(("Cargo.toml".to_string(), PKG_TYPE_RUST_CARGO.to_string()));
    }

    // 2. Go (go.mod or go.work)
    if dir.join("go.mod").exists() {
        return Some(("go.mod".to_string(), PKG_TYPE_GO_WORKSPACE.to_string()));
    }
    if dir.join("go.work").exists() {
        return Some(("go.work".to_string(), PKG_TYPE_GO_WORKSPACE.to_string()));
    }

    // 3. Python (uv, poetry, requirements, pipfile)
    if dir.join("pyproject.toml").exists() {
        return Some((
            "pyproject.toml".to_string(),
            PKG_TYPE_PYTHON_UV_POETRY.to_string(),
        ));
    }
    if dir.join("uv.lock").exists() {
        return Some(("uv.lock".to_string(), PKG_TYPE_PYTHON_UV_POETRY.to_string()));
    }
    if dir.join("requirements.txt").exists() {
        return Some((
            "requirements.txt".to_string(),
            PKG_TYPE_PYTHON_UV_POETRY.to_string(),
        ));
    }
    if dir.join("Pipfile").exists() {
        return Some(("Pipfile".to_string(), PKG_TYPE_PYTHON_UV_POETRY.to_string()));
    }

    // 4. JavaScript / TypeScript (Bun, pnpm, Yarn, npm)
    if dir.join("package.json").exists() {
        let is_bun = dir.join("bunfig.toml").exists()
            || dir.join("bun.lock").exists()
            || dir.join("bun.lockb").exists()
            || root.join("bunfig.toml").exists()
            || root.join("bun.lock").exists()
            || root.join("bun.lockb").exists();

        if is_bun {
            return Some(("package.json".to_string(), PKG_TYPE_JS_BUN.to_string()));
        }

        let is_pnpm = dir.join("pnpm-workspace.yaml").exists()
            || dir.join("pnpm-lock.yaml").exists()
            || root.join("pnpm-workspace.yaml").exists()
            || root.join("pnpm-lock.yaml").exists();

        if is_pnpm {
            return Some(("package.json".to_string(), PKG_TYPE_JS_PNPM.to_string()));
        }

        let is_yarn = dir.join("yarn.lock").exists() || root.join("yarn.lock").exists();
        if is_yarn {
            return Some(("package.json".to_string(), PKG_TYPE_JS_YARN.to_string()));
        }

        return Some(("package.json".to_string(), PKG_TYPE_JS_NPM.to_string()));
    }

    // Standalone Bun configuration
    if dir.join("bunfig.toml").exists() {
        return Some(("bunfig.toml".to_string(), PKG_TYPE_JS_BUN.to_string()));
    }
    if dir.join("bun.lock").exists() || dir.join("bun.lockb").exists() {
        return Some(("bun.lock".to_string(), PKG_TYPE_JS_BUN.to_string()));
    }

    None
}

fn register_workspace_dir(
    root: &Path,
    dir: &Path,
    workspaces: &mut Vec<MonorepoWorkspace>,
    seen_paths: &mut HashSet<String>,
) {
    let rel = dir
        .strip_prefix(root)
        .unwrap_or(dir)
        .to_string_lossy()
        .replace('\\', "/");

    if seen_paths.contains(&rel) {
        return;
    }

    if let Some((manifest_file, package_type)) = detect_package_in_dir(root, dir) {
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| rel.clone());
        workspaces.push(MonorepoWorkspace {
            name,
            path: rel.clone(),
            manifest_file,
            package_type,
        });
        seen_paths.insert(rel);
    }
}

fn scan_sub_container(
    root: &Path,
    sub_dir_name: &str,
    workspaces: &mut Vec<MonorepoWorkspace>,
    seen_paths: &mut HashSet<String>,
) {
    let sub_dir = root.join(sub_dir_name);
    if sub_dir.is_dir()
        && let Ok(entries) = fs::read_dir(&sub_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                register_workspace_dir(root, &path, workspaces, seen_paths);
            }
        }
    }
}

fn scan_cargo_workspace(
    root: &Path,
    workspaces: &mut Vec<MonorepoWorkspace>,
    seen_paths: &mut HashSet<String>,
) {
    let root_cargo = root.join("Cargo.toml");
    if root_cargo.exists()
        && let Ok(content) = fs::read_to_string(&root_cargo)
        && content.contains("[workspace]")
    {
        scan_sub_container(root, "crates", workspaces, seen_paths);
    }
}

fn scan_go_work(
    root: &Path,
    workspaces: &mut Vec<MonorepoWorkspace>,
    seen_paths: &mut HashSet<String>,
) {
    let go_work = root.join("go.work");
    if go_work.exists()
        && let Ok(content) = fs::read_to_string(&go_work)
    {
        let mut in_use_block = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use (") {
                in_use_block = true;
                continue;
            }
            if in_use_block && trimmed.starts_with(')') {
                in_use_block = false;
                continue;
            }
            let candidate = if in_use_block {
                Some(trimmed.trim_matches('"'))
            } else {
                trimmed
                    .strip_prefix("use ")
                    .map(|stripped| stripped.trim().trim_matches('"'))
            };
            if let Some(dir_str) = candidate {
                let clean_dir = dir_str.trim();
                if clean_dir.is_empty() {
                    continue;
                }
                let target_dir = if let Some(stripped) = clean_dir.strip_prefix("./") {
                    root.join(stripped)
                } else {
                    root.join(clean_dir)
                };
                if target_dir.is_dir() {
                    register_workspace_dir(root, &target_dir, workspaces, seen_paths);
                }
            }
        }
    }
}

/// Automatically discovers workspace packages within a root repository.
pub fn discover_workspaces(root: &Path) -> Vec<MonorepoWorkspace> {
    let mut workspaces = Vec::new();
    let mut seen_paths = HashSet::new();

    // 1. Cargo workspaces (crates/)
    scan_cargo_workspace(root, &mut workspaces, &mut seen_paths);

    // 2. Go workspaces (go.work)
    scan_go_work(root, &mut workspaces, &mut seen_paths);

    // 3. Polyglot monorepo package containers
    for sub in &["packages", "apps", "services", "libs", "modules", "tools"] {
        scan_sub_container(root, sub, &mut workspaces, &mut seen_paths);
    }

    if workspaces.is_empty() {
        let root_name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "root".to_string());
        workspaces.push(MonorepoWorkspace {
            name: root_name,
            path: ".".to_string(),
            manifest_file: "root".to_string(),
            package_type: PKG_TYPE_ROOT_PROJECT.to_string(),
        });
    } else {
        workspaces.sort_by(|a, b| a.path.cmp(&b.path));
    }

    workspaces
}

/// Executes a monorepo scan across detected workspaces and aggregates metrics.
pub async fn run_monorepo_scan(
    root: &Path,
    config: &ScanConfig,
) -> Result<MonorepoScanSummary, String> {
    let workspaces = discover_workspaces(root);
    let (progress_tx, _progress_rx) = tokio::sync::mpsc::channel(100);
    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let scan_res = run_scan(config.clone(), progress_tx, cancel_flag).await?;

    let mut cross_workspace_clones = 0;
    for pair in &scan_res.clone_pairs {
        let ws_a = find_matching_workspace(&pair.file_a, &workspaces);
        let ws_b = find_matching_workspace(&pair.file_b, &workspaces);
        if ws_a != ws_b {
            cross_workspace_clones += 1;
        }
    }

    Ok(MonorepoScanSummary {
        total_workspaces: workspaces.len(),
        workspaces,
        total_files: scan_res.total_files,
        total_tokens: scan_res.total_tokens,
        total_clones: scan_res.total_clones,
        cross_workspace_clones,
        average_dry_score: scan_res.dry_health_score,
        scan_result: scan_res,
    })
}

fn find_matching_workspace<'a>(
    file_path: &str,
    workspaces: &'a [MonorepoWorkspace],
) -> Option<&'a str> {
    let norm = file_path.replace('\\', "/");
    for ws in workspaces {
        if ws.path != "." {
            let prefix = format!("{}/", ws.path);
            if norm == ws.path || norm.starts_with(&prefix) {
                return Some(&ws.name);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests;
