#![forbid(unsafe_code)]

use crate::detector::run_scan;
use crate::types::{ScanConfig, ScanResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

fn scan_subdirectories_for_manifest(
    root: &Path,
    sub_dir_name: &str,
    manifest_name: &str,
    package_type: &str,
    workspaces: &mut Vec<MonorepoWorkspace>,
) {
    let sub_dir = root.join(sub_dir_name);
    if sub_dir.is_dir()
        && let Ok(entries) = fs::read_dir(sub_dir)
    {
        for entry in entries.flatten() {
            if entry.path().is_dir() && entry.path().join(manifest_name).exists() {
                let name = entry.file_name().to_string_lossy().to_string();
                let rel = entry
                    .path()
                    .strip_prefix(root)
                    .unwrap_or(&entry.path())
                    .to_string_lossy()
                    .replace('\\', "/");
                workspaces.push(MonorepoWorkspace {
                    name,
                    path: rel,
                    manifest_file: manifest_name.to_string(),
                    package_type: package_type.to_string(),
                });
            }
        }
    }
}

/// Automatically discovers workspace packages within a root repository.
pub fn discover_workspaces(root: &Path) -> Vec<MonorepoWorkspace> {
    let mut workspaces = Vec::new();

    // Check Cargo workspace
    let root_cargo = root.join("Cargo.toml");
    if root_cargo.exists()
        && let Ok(content) = fs::read_to_string(&root_cargo)
        && content.contains("[workspace]")
    {
        scan_subdirectories_for_manifest(
            root,
            "crates",
            "Cargo.toml",
            "Rust (Cargo)",
            &mut workspaces,
        );
    }

    // Check packages/ or apps/ directory (JS/TS monorepos)
    for sub in &["packages", "apps", "services"] {
        scan_subdirectories_for_manifest(
            root,
            sub,
            "package.json",
            "JavaScript/TypeScript (npm)",
            &mut workspaces,
        );
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
            package_type: "Root Project".to_string(),
        });
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
        if ws.path != "." && norm.starts_with(&ws.path) {
            return Some(&ws.name);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_discover_workspaces_empty() {
        let dir = tempdir().unwrap();
        let ws = discover_workspaces(dir.path());
        assert_eq!(ws.len(), 1);
        assert_eq!(ws[0].path, ".");
    }

    #[test]
    fn test_discover_workspaces_cargo() {
        let dir = tempdir().unwrap();
        let root_cargo = dir.path().join("Cargo.toml");
        fs::write(root_cargo, "[workspace]\nmembers = [\"crates/*\"]\n").unwrap();

        let crates_dir = dir.path().join("crates");
        fs::create_dir_all(&crates_dir).unwrap();

        let sub_a = crates_dir.join("sub-a");
        fs::create_dir_all(&sub_a).unwrap();
        fs::write(sub_a.join("Cargo.toml"), "[package]\nname = \"sub-a\"\n").unwrap();

        let sub_b = crates_dir.join("sub-b");
        fs::create_dir_all(&sub_b).unwrap();
        fs::write(sub_b.join("Cargo.toml"), "[package]\nname = \"sub-b\"\n").unwrap();

        let ws = discover_workspaces(dir.path());
        assert_eq!(ws.len(), 2);
    }
}
