#![forbid(unsafe_code)]

use super::types::{HubConfig, HubRepoConfig};
use std::fs;
use std::path::Path;

pub const DEFAULT_HUB_CONFIG_FILE: &str = ".cddmhub.toml";

/// Generates a starter `.cddmhub.toml` configuration template.
pub fn generate_default_hub_config(name: Option<&str>) -> String {
    let hub_name = name.unwrap_or("enterprise-organization");
    format!(
        r#"# CDDM Organization Federation Hub Configuration
name = "{hub_name}"
min_tokens = 50
fail_threshold = 5.0
ignore_patterns = ["**/target/**", "**/node_modules/**", "**/dist/**", "**/.git/**"]

[[repositories]]
name = "core-backend"
path = "./services/core-backend"
tags = ["backend", "rust"]
branch = "main"

[[repositories]]
name = "web-frontend"
path = "./apps/web-frontend"
tags = ["frontend", "typescript"]
branch = "main"

[[repositories]]
name = "data-pipeline"
path = "./services/data-pipeline"
tags = ["pipeline", "python"]
branch = "main"
"#
    )
}

/// Loads and parses a HubConfig from a file path.
pub fn load_hub_config<P: AsRef<Path>>(path: P) -> Result<HubConfig, String> {
    let p = path.as_ref();
    if !p.exists() {
        return Err(format!("Hub configuration file not found: {}", p.display()));
    }
    let content = fs::read_to_string(p)
        .map_err(|e| format!("Failed to read hub config {}: {e}", p.display()))?;
    toml::from_str::<HubConfig>(&content)
        .map_err(|e| format!("Invalid hub config TOML format in {}: {e}", p.display()))
}

/// Creates an ad-hoc HubConfig from an explicit list of repository directories.
pub fn build_adhoc_hub_config<P: AsRef<Path>>(
    hub_name: &str,
    repo_paths: &[P],
    min_tokens: usize,
) -> HubConfig {
    let mut repositories = Vec::new();
    for (i, p) in repo_paths.iter().enumerate() {
        let path_ref = p.as_ref();
        let name = path_ref
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .filter(|n| !n.is_empty() && n != ".")
            .unwrap_or_else(|| format!("repo-{}", i + 1));

        repositories.push(HubRepoConfig {
            name,
            path: path_ref.to_string_lossy().replace('\\', "/"),
            tags: Vec::new(),
            branch: None,
        });
    }

    HubConfig {
        name: hub_name.to_string(),
        repositories,
        min_tokens,
        fail_threshold: 15.0,
        ignore_patterns: vec![
            "**/target/**".to_string(),
            "**/node_modules/**".to_string(),
            "**/.git/**".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_and_parse_default_config() {
        let toml_str = generate_default_hub_config(Some("my-org"));
        let parsed: HubConfig = toml::from_str(&toml_str).expect("Valid TOML");
        assert_eq!(parsed.name, "my-org");
        assert_eq!(parsed.repositories.len(), 3);
        assert_eq!(parsed.min_tokens, 50);
    }

    #[test]
    fn test_load_config_file() {
        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join(".cddmhub.toml");
        let toml_str = generate_default_hub_config(Some("acme-corp"));
        fs::write(&cfg_path, toml_str).unwrap();

        let cfg = load_hub_config(&cfg_path).unwrap();
        assert_eq!(cfg.name, "acme-corp");
        assert_eq!(cfg.repositories[0].name, "core-backend");
    }

    #[test]
    fn test_build_adhoc_hub_config() {
        let cfg = build_adhoc_hub_config("test-hub", &["services/a", "services/b"], 60);
        assert_eq!(cfg.name, "test-hub");
        assert_eq!(cfg.repositories.len(), 2);
        assert_eq!(cfg.min_tokens, 60);
        assert_eq!(cfg.repositories[0].name, "a");
        assert_eq!(cfg.repositories[1].name, "b");
    }
}
