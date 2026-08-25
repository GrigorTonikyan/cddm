#![forbid(unsafe_code)]

use cddm_core::{ScanConfig, discover_workspaces, run_monorepo_scan};
use std::path::PathBuf;

/// Executes the CLI `cddm monorepo` command.
pub async fn run_monorepo_command(
    directory: PathBuf,
    min_tokens: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\x1b[36m--> Discovering monorepo workspaces in '{}'...\x1b[0m",
        directory.display()
    );

    let workspaces = discover_workspaces(&directory);
    println!(
        "\x1b[35mDiscovered {} workspace package(s):\x1b[0m",
        workspaces.len()
    );
    for ws in &workspaces {
        println!(
            "  - \x1b[32m{}\x1b[0m ({}) at '{}'",
            ws.name, ws.package_type, ws.path
        );
    }

    let scan_config = ScanConfig {
        directory: directory.to_string_lossy().to_string(),
        min_tokens,
        languages: vec![],
        ignore_patterns: vec![],
        detect_type2: true,
        scan_self: false,
        enable_git_blame: false,
        cache_dir: None,
        enable_cache: false,
        cddmignore_path: None,
        ignore_tests: false,
        ignore_mocks: false,
        ignore_generated: true,
        rules_path: None,
        enforce_policies: false,
    };

    let summary = run_monorepo_scan(&directory, &scan_config).await?;

    println!("\n\x1b[36m=== Monorepo Scan Metrics ===\x1b[0m");
    println!("  Total Workspaces:         {}", summary.total_workspaces);
    println!("  Total Scanned Files:      {}", summary.total_files);
    println!("  Total Clones:             {}", summary.total_clones);
    println!(
        "  Cross-Workspace Clones:   {}",
        summary.cross_workspace_clones
    );
    println!(
        "  Average DRY Health Score: {:.1} / 100.0",
        summary.average_dry_score
    );

    Ok(())
}
