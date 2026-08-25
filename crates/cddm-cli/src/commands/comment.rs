#![forbid(unsafe_code)]

use crate::types::PlatformChoice;
use cddm_core::{ScanConfig, generate_pr_markdown_comment, run_scan};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::mpsc;

pub async fn run_comment_command(
    directory: PathBuf,
    min_tokens: usize,
    fail_threshold: f64,
    platform: PlatformChoice,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages: vec![],
        ignore_patterns: ScanConfig::default().ignore_patterns,
        detect_type2: true,
        scan_self: true,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: true,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let result = run_scan(config, tx, cancel_flag).await?;

    let comment_text = generate_pr_markdown_comment(&result, fail_threshold, platform.into());

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out_path, &comment_text)?;
        println!(
            "[PASS] Pull Request markdown comment written to '{}'",
            out_path.display()
        );
    } else {
        println!("{}", comment_text);
    }

    if result.duplication_percentage > fail_threshold {
        std::process::exit(1);
    }

    Ok(())
}
