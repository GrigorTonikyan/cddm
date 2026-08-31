use crate::types::HookStatus;
use std::fs;
use std::path::{Path, PathBuf};

/// Generates a turnkey Gitea Actions workflow configuration.
pub fn generate_gitea_workflow(fail_threshold: f64, min_tokens: usize) -> String {
    format!(
        r#"name: CDDM Code Duplication & DRY Health

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

jobs:
  cddm-scan:
    name: Code Clone Detection & Modularity Health
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Install CDDM Engine
        run: cargo install cddm

      - name: Execute CDDM Duplicate Code Scan
        run: |
          cddm scan . \
            --min-tokens {min_tokens} \
            --fail-threshold {fail_threshold:.1} \
            --format sarif \
            --output cddm-results.sarif

      - name: Generate PR Markdown Summary Report
        if: gitea.event_name == 'pull_request'
        run: |
          cddm scan . \
            --min-tokens {min_tokens} \
            --format markdown > cddm-summary.md
"#
    )
}

/// Generates a turnkey GitHub Actions workflow configuration.
pub fn generate_github_workflow(fail_threshold: f64, min_tokens: usize) -> String {
    format!(
        r#"name: CDDM Code Duplication & DRY Health

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

permissions:
  contents: read
  security-events: write
  pull-requests: write

jobs:
  cddm-scan:
    name: Code Clone Detection & Modularity Health
    runs-on: ubuntu-latest

    steps:
      - name: Checkout Repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Install CDDM Engine
        run: cargo install cddm

      - name: Execute CDDM Duplicate Code Scan
        run: |
          cddm scan . \
            --min-tokens {min_tokens} \
            --fail-threshold {fail_threshold:.1} \
            --format sarif \
            --output cddm-results.sarif

      - name: Upload SARIF to GitHub Code Scanning
        uses: github/codeql-action/upload-sarif@v3
        if: always()
        with:
          sarif_file: cddm-results.sarif
          category: cddm-clone-detection

      - name: Generate PR Markdown Summary Report
        if: github.event_name == 'pull_request'
        run: |
          cddm scan . \
            --min-tokens {min_tokens} \
            --format markdown > cddm-summary.md
"#
    )
}

/// Generates a turnkey GitLab CI configuration snippet.
pub fn generate_gitlab_ci(fail_threshold: f64, min_tokens: usize) -> String {
    format!(
        r#"cddm_duplication_scan:
  stage: test
  image: rust:latest
  before_script:
    - cargo install cddm
  script:
    - cddm scan . --min-tokens {min_tokens} --fail-threshold {fail_threshold:.1} --format console
  artifacts:
    when: always
    paths:
      - cddm-results.sarif
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
"#
    )
}

/// Generates a turnkey Azure DevOps Pipelines YAML configuration snippet.
pub fn generate_azure_pipelines(fail_threshold: f64, min_tokens: usize) -> String {
    format!(
        r#"trigger:
  - main
  - master

pool:
  vmImage: 'ubuntu-latest'

steps:
  - checkout: self
    fetchDepth: 0

  - script: |
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      cargo install cddm
    displayName: 'Install CDDM Engine'

  - script: |
      source "$HOME/.cargo/env"
      cddm scan . --min-tokens {min_tokens} --fail-threshold {fail_threshold:.1} --format console
    displayName: 'Execute CDDM Duplicate Code Scan'
"#
    )
}

/// Locates the `.git/hooks` directory for a repository root.
fn get_git_hooks_dir(repo_root: &Path) -> Result<PathBuf, String> {
    let git_dir = repo_root.join(".git");
    if !git_dir.exists() {
        return Err(format!(
            "Directory '{}' is not a Git repository (missing .git directory)",
            repo_root.display()
        ));
    }
    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)
            .map_err(|e| format!("Failed to create git hooks directory: {e}"))?;
    }
    Ok(hooks_dir)
}

/// Installs a native Git hook (pre-commit or pre-push) that enforces CDDM scan thresholds.
pub fn install_git_hook(
    repo_root: &Path,
    hook_type: &str,
    fail_threshold: f64,
    min_tokens: usize,
) -> Result<String, String> {
    if hook_type != "pre-commit" && hook_type != "pre-push" {
        return Err(format!(
            "Unsupported hook type '{hook_type}'. Must be 'pre-commit' or 'pre-push'."
        ));
    }

    let hooks_dir = get_git_hooks_dir(repo_root)?;
    let hook_path = hooks_dir.join(hook_type);

    let script_content = format!(
        r#"#!/usr/bin/env sh
# CDDM Automated Git Hook ({hook_type})
# Enforces codebase duplication threshold before allowing commit/push.

echo "[CDDM] Running automated code duplication quality check..."

if command -v cddm >/dev/null 2>&1; then
    cddm scan . --min-tokens {min_tokens} --fail-threshold {fail_threshold:.1}
    EXIT_CODE=$?
    if [ $EXIT_CODE -ne 0 ]; then
        echo "[CDDM] Quality gate failed: Code duplication exceeded threshold of {fail_threshold:.1}%"
        exit $EXIT_CODE
    fi
    echo "[CDDM] Quality gate passed! Proceeding."
    exit 0
else
    echo "[CDDM] Notice: 'cddm' executable not found on PATH. Skipping duplication check."
    exit 0
fi
"#
    );

    fs::write(&hook_path, script_content)
        .map_err(|e| format!("Failed to write hook file '{}': {e}", hook_path.display()))?;

    // On Unix platforms, make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&hook_path) {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            let _ = fs::set_permissions(&hook_path, permissions);
        }
    }

    Ok(format!(
        "Successfully installed CDDM {hook_type} hook at '{}'",
        hook_path.display()
    ))
}

/// Uninstalls a native Git hook (pre-commit or pre-push).
pub fn uninstall_git_hook(repo_root: &Path, hook_type: &str) -> Result<String, String> {
    let hooks_dir = get_git_hooks_dir(repo_root)?;
    let hook_path = hooks_dir.join(hook_type);

    if hook_path.exists() {
        fs::remove_file(&hook_path)
            .map_err(|e| format!("Failed to remove hook file '{}': {e}", hook_path.display()))?;
        Ok(format!(
            "Successfully removed CDDM {hook_type} hook from '{}'",
            hook_path.display()
        ))
    } else {
        Ok(format!(
            "Hook '{hook_type}' was not installed in '{}'",
            hooks_dir.display()
        ))
    }
}

/// Queries the installation status of Git hooks in the repository.
pub fn get_hook_status(repo_root: &Path) -> HookStatus {
    let git_dir = repo_root.join(".git");
    let hooks_dir = git_dir.join("hooks");

    let pre_commit_installed = hooks_dir.join("pre-commit").exists();
    let pre_push_installed = hooks_dir.join("pre-push").exists();

    HookStatus {
        pre_commit_installed,
        pre_push_installed,
        hooks_dir: hooks_dir.to_string_lossy().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_github_workflow() {
        let wf = generate_github_workflow(15.0, 50);
        assert!(wf.contains("name: CDDM Code Duplication & DRY Health"));
        assert!(wf.contains("--min-tokens 50"));
        assert!(wf.contains("--fail-threshold 15.0"));
        assert!(wf.contains("github/codeql-action/upload-sarif@v3"));
    }

    #[test]
    fn test_generate_gitlab_ci() {
        let ci = generate_gitlab_ci(12.5, 40);
        assert!(ci.contains("cddm_duplication_scan:"));
        assert!(ci.contains("--min-tokens 40"));
        assert!(ci.contains("--fail-threshold 12.5"));
    }

    #[test]
    fn test_generate_azure_pipelines() {
        let az = generate_azure_pipelines(10.0, 60);
        assert!(az.contains("vmImage: 'ubuntu-latest'"));
        assert!(az.contains("--min-tokens 60"));
        assert!(az.contains("--fail-threshold 10.0"));
    }

    #[test]
    fn test_git_hook_lifecycle() {
        let temp = tempdir().expect("tempdir");
        let git_dir = temp.path().join(".git");
        fs::create_dir_all(&git_dir).expect("create .git");

        let status_initial = get_hook_status(temp.path());
        assert!(!status_initial.pre_commit_installed);
        assert!(!status_initial.pre_push_installed);

        // Install pre-commit
        let install_res = install_git_hook(temp.path(), "pre-commit", 15.0, 50);
        assert!(install_res.is_ok());

        let status_after_commit = get_hook_status(temp.path());
        assert!(status_after_commit.pre_commit_installed);
        assert!(!status_after_commit.pre_push_installed);

        // Install pre-push
        let install_push_res = install_git_hook(temp.path(), "pre-push", 12.0, 45);
        assert!(install_push_res.is_ok());

        let status_both = get_hook_status(temp.path());
        assert!(status_both.pre_commit_installed);
        assert!(status_both.pre_push_installed);

        // Uninstall pre-commit
        let uninst_res = uninstall_git_hook(temp.path(), "pre-commit");
        assert!(uninst_res.is_ok());

        let status_after_uninst = get_hook_status(temp.path());
        assert!(!status_after_uninst.pre_commit_installed);
        assert!(status_after_uninst.pre_push_installed);
    }

    #[test]
    fn test_install_invalid_hook_type() {
        let temp = tempdir().expect("tempdir");
        let git_dir = temp.path().join(".git");
        fs::create_dir_all(&git_dir).expect("create .git");

        let res = install_git_hook(temp.path(), "invalid-hook", 15.0, 50);
        assert!(res.is_err());
    }
}
