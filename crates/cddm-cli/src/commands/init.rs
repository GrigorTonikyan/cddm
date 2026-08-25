#![forbid(unsafe_code)]

use crate::types::PlatformChoice;
use cddm_core::{generate_azure_pipelines, generate_github_workflow, generate_gitlab_ci};
use std::fs;
use std::path::PathBuf;

pub fn run_init_command(
    platform: PlatformChoice,
    fail_threshold: f64,
    min_tokens: usize,
    output: Option<PathBuf>,
    write: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = match platform {
        PlatformChoice::Github => generate_github_workflow(fail_threshold, min_tokens),
        PlatformChoice::Gitlab => generate_gitlab_ci(fail_threshold, min_tokens),
        PlatformChoice::Azure => generate_azure_pipelines(fail_threshold, min_tokens),
    };

    let default_out_path = match platform {
        PlatformChoice::Github => PathBuf::from(".github/workflows/cddm.yml"),
        PlatformChoice::Gitlab => PathBuf::from(".gitlab-ci.yml"),
        PlatformChoice::Azure => PathBuf::from("azure-pipelines.yml"),
    };

    let target_path = output.unwrap_or(default_out_path);

    if write {
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target_path, &content)?;
        println!(
            "[PASS] Successfully generated {} configuration at '{}'",
            platform,
            target_path.display()
        );
    } else {
        println!("{}", content);
    }

    Ok(())
}
