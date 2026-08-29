#![forbid(unsafe_code)]

use crate::types::commands::{HubArgs, HubSubcommand};
use cddm_core::{
    HubExtractRequest, HubExtractResult, HubScanSummary, build_adhoc_hub_config,
    generate_default_hub_config, generate_hub_extraction, load_hub_config, run_hub_scan,
};
use comfy_table::{Cell, Color, Row, Table};
use std::fs;
use std::path::Path;

/// Executes the CLI `cddm hub` command.
pub async fn run_hub_command(args: HubArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.action {
        HubSubcommand::Init { config, name } => {
            let toml_content = generate_default_hub_config(name.as_deref());
            if config.exists() {
                println!(
                    "\x1b[33m[WARN] Hub config '{}' already exists. Overwrite? (skipping to \
                     prevent data loss)\x1b[0m",
                    config.display()
                );
            } else {
                fs::write(&config, toml_content)?;
                println!(
                    "\x1b[32m[SUCCESS] Generated Organization Federation Hub config at '{}'\x1b[0m",
                    config.display()
                );
            }
        }

        HubSubcommand::Scan {
            targets,
            format,
            min_tokens,
        } => {
            let config = if targets.len() == 1 && Path::new(&targets[0]).is_file() {
                load_hub_config(&targets[0])?
            } else {
                let paths: Vec<&Path> = targets.iter().map(Path::new).collect();
                build_adhoc_hub_config("federation-hub", &paths, min_tokens)
            };

            println!(
                "\x1b[36m--> Scanning Organization Federation Hub '{}' ({} repositories)...\x1b[0m",
                config.name,
                config.repositories.len()
            );

            let summary = run_hub_scan(&config).await?;

            match format.to_lowercase().as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&summary)?);
                }
                "markdown" | "md" => {
                    print_hub_markdown(&summary);
                }
                _ => {
                    print_hub_console(&summary);
                }
            }
        }

        HubSubcommand::Extract {
            config,
            cluster,
            pkg_name,
            pkg_type,
            target_dir,
            dry_run,
        } => {
            let hub_cfg = if config.exists() {
                load_hub_config(&config)?
            } else {
                build_adhoc_hub_config("federation-hub", &[Path::new(".")], 50)
            };

            println!(
                "\x1b[36m--> Generating shared package extraction for Hub Cluster #{} \
                 ('{}')...\x1b[0m",
                cluster, pkg_name
            );

            let summary = run_hub_scan(&hub_cfg).await?;
            let request = HubExtractRequest {
                hub_config: Some(hub_cfg),
                cluster_id: cluster,
                target_package_name: pkg_name,
                package_type: pkg_type,
                target_dir,
                dry_run,
            };

            let result = generate_hub_extraction(&summary, &request)?;
            print_extraction_result(&result, dry_run);
        }
    }

    Ok(())
}

fn print_hub_console(summary: &HubScanSummary) {
    println!(
        "\n\x1b[32m=== CDDM Organization Federation Hub: {} ===\x1b[0m\n",
        summary.hub_name
    );

    let mut overview = Table::new();
    overview.set_header(vec![
        Cell::new("Metric").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Green),
    ]);
    overview.add_row(Row::from(vec![
        Cell::new("Total Member Repositories"),
        Cell::new(summary.total_repos.to_string()),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Total Files Analyzed"),
        Cell::new(summary.total_files.to_string()),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Total Scanned Tokens"),
        Cell::new(summary.total_tokens.to_string()),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Cross-Repository Clone Pairs"),
        Cell::new(summary.cross_repo_clones.to_string()),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Cross-Repository Clusters"),
        Cell::new(summary.cross_repo_clusters.to_string()),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Organization DRY Health Score"),
        Cell::new(format!("{:.2} / 100.0", summary.organization_dry_score)),
    ]));
    overview.add_row(Row::from(vec![
        Cell::new("Cross-Repo Duplication Rate"),
        Cell::new(format!("{:.2}%", summary.cross_repo_duplication_pct)),
    ]));
    println!("{overview}");

    if !summary.duplication_matrix.is_empty() {
        println!("\n\x1b[33m=== Inter-Repository Duplication Matrix ===\x1b[0m\n");
        let mut matrix_table = Table::new();
        matrix_table.set_header(vec![
            Cell::new("Repository A").fg(Color::Cyan),
            Cell::new("Repository B").fg(Color::Cyan),
            Cell::new("Shared Clones").fg(Color::Yellow),
            Cell::new("Shared Tokens").fg(Color::Magenta),
        ]);
        for row in &summary.duplication_matrix {
            matrix_table.add_row(Row::from(vec![
                Cell::new(&row.repo_a),
                Cell::new(&row.repo_b),
                Cell::new(row.shared_clones.to_string()),
                Cell::new(row.shared_tokens.to_string()),
            ]));
        }
        println!("{matrix_table}");
    }

    if !summary.clusters.is_empty() {
        println!("\n\x1b[33m=== Cross-Repository Extraction Candidates ===\x1b[0m\n");
        let mut cluster_table = Table::new();
        cluster_table.set_header(vec![
            Cell::new("Cluster ID").fg(Color::Cyan),
            Cell::new("Member Repositories").fg(Color::DarkCyan),
            Cell::new("Occurrences").fg(Color::Yellow),
            Cell::new("Tokens").fg(Color::Magenta),
            Cell::new("Suggested Shared Package").fg(Color::Green),
        ]);
        for c in &summary.clusters {
            cluster_table.add_row(Row::from(vec![
                Cell::new(format!("#{}", c.id)),
                Cell::new(c.repos.join(", ")),
                Cell::new(c.occurrences.len().to_string()),
                Cell::new(c.token_count.to_string()),
                Cell::new(&c.suggested_package),
            ]));
        }
        println!("{cluster_table}");
    }
    println!();
}

fn print_hub_markdown(summary: &HubScanSummary) {
    println!(
        "# CDDM Organization Federation Hub Report: {}\n",
        summary.hub_name
    );
    println!("- **Total Repositories**: {}", summary.total_repos);
    println!("- **Total Files**: {}", summary.total_files);
    println!("- **Total Tokens**: {}", summary.total_tokens);
    println!(
        "- **Cross-Repo Clone Pairs**: {}",
        summary.cross_repo_clones
    );
    println!("- **Cross-Repo Clusters**: {}", summary.cross_repo_clusters);
    println!(
        "- **Organization DRY Health Score**: {:.2} / 100.0",
        summary.organization_dry_score
    );
    println!(
        "- **Cross-Repo Duplication Rate**: {:.2}%\n",
        summary.cross_repo_duplication_pct
    );

    if !summary.duplication_matrix.is_empty() {
        println!("## Inter-Repository Duplication Matrix\n");
        println!("| Repository A | Repository B | Shared Clones | Shared Tokens |");
        println!("| :--- | :--- | :--- | :--- |");
        for row in &summary.duplication_matrix {
            println!(
                "| `{}` | `{}` | {} | {} |",
                row.repo_a, row.repo_b, row.shared_clones, row.shared_tokens
            );
        }
        println!();
    }
}

fn print_extraction_result(result: &HubExtractResult, dry_run: bool) {
    let mode = if dry_run { "[DRY RUN]" } else { "[APPLIED]" };
    println!(
        "\n\x1b[32m=== {} Shared Package Extraction: {} ({}) ===\x1b[0m\n",
        mode, result.package_name, result.package_type
    );
    println!("- **Destination Directory**: `{}`", result.target_dir);
    println!("- **Generated Files**: {}", result.generated_files.len());
    println!(
        "- **Member Repositories Updated**: {}",
        result.repos_updated
    );
    println!("- **Estimated Lines Saved**: {}\n", result.lines_saved);

    println!("\x1b[33m--- Generated Package Files ---\x1b[0m");
    for file in &result.generated_files {
        println!("  • `{}`", file.file_path);
    }

    println!("\n\x1b[33m--- Repository Caller Updates ---\x1b[0m");
    for update in &result.repo_updates {
        println!(
            "  • Repository `{}` ({} caller rewrites, {} manifest updates)",
            update.repo_name,
            update.caller_rewrites.len(),
            update.manifest_updates.len()
        );
    }
    println!();
}
