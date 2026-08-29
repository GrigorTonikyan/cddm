#![forbid(unsafe_code)]

use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use cddm_core::{
    CoverageCorrelationSummary, CoverageFormat, ScanConfig, correlate_coverage,
    load_coverage_report, run_scan,
};
use comfy_table::{Cell, Color, Row, Table};
use tokio::sync::mpsc;

use crate::types::commands::CoverageArgs;

/// Execute the `cddm coverage` CLI command.
pub async fn handle_coverage_command(args: CoverageArgs) -> Result<(), Box<dyn Error>> {
    if !args.report.exists() {
        return Err(format!("Coverage report file not found: {}", args.report.display()).into());
    }

    let coverage_report = load_coverage_report(&args.report, CoverageFormat::Auto)
        .map_err(|e| format!("Failed to load coverage report: {e}"))?;

    let scan_config = ScanConfig {
        directory: args.directory.to_string_lossy().to_string(),
        min_tokens: args.min_tokens,
        ..Default::default()
    };

    let (tx, _rx) = mpsc::channel(100);
    let cancel = Arc::new(AtomicBool::new(false));
    let scan_result = run_scan(scan_config, tx, cancel)
        .await
        .map_err(|e| format!("Codebase scan failed: {e}"))?;

    let mut correlation = correlate_coverage(&scan_result, &coverage_report);

    // Apply filtering
    if args.dead_code_only {
        correlation.metrics.retain(|m| m.is_dead_code);
    }
    if args.min_hits > 0 {
        correlation
            .metrics
            .retain(|m| m.total_combined_hits >= args.min_hits);
    }
    if let Some(threshold) = args.risk_threshold {
        correlation.metrics.retain(|m| m.risk_score >= threshold);
    }

    match args.format.to_lowercase().as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&correlation)?);
        }
        "markdown" | "md" => {
            print_markdown_summary(&correlation);
        }
        _ => {
            print_console_summary(&correlation);
        }
    }

    Ok(())
}

fn print_console_summary(summary: &CoverageCorrelationSummary) {
    println!("\n\x1b[36m=== CDDM Runtime Execution & Coverage-Aware De-duplication ===\x1b[0m\n");

    let mut overview = Table::new();
    overview.set_header(vec![
        Cell::new("Metric").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Green),
    ]);

    super::add_kv_overview_rows(
        &mut overview,
        &[
            (
                "Total Duplicate Clone Pairs",
                summary.total_clone_pairs.to_string(),
            ),
            (
                "Covered Clones Percentage",
                format!("{:.1}%", summary.overall_covered_clones_pct),
            ),
            (
                "Dead Code Clones (0 Executions)",
                summary.dead_code_clones.to_string(),
            ),
            (
                "Test Gap Clones (Asymmetric Coverage)",
                summary.test_gap_clones.to_string(),
            ),
            (
                "Hot Path Clones (>1,000 Executions)",
                summary.hot_path_clones.to_string(),
            ),
            (
                "Total Monitored Runtime Hits",
                summary.total_runtime_hits.to_string(),
            ),
        ],
    );
    println!("{overview}");

    if !summary.metrics.is_empty() {
        println!("\n\x1b[33m=== Clones Correlated with Runtime Execution ===\x1b[0m\n");
        let mut table = Table::new();
        table.set_header(super::make_colored_header(&[
            ("Pair ID", Color::Cyan),
            ("Clone Location A", Color::Cyan),
            ("Hits A", Color::Yellow),
            ("Clone Location B", Color::Cyan),
            ("Hits B", Color::Yellow),
            ("Tier", Color::Magenta),
            ("Risk Score", Color::Red),
        ]));

        for m in summary.metrics.iter().take(25) {
            let tier_str = format!("{:?}", m.execution_tier);
            table.add_row(Row::from(vec![
                Cell::new(format!("#{}", m.clone_pair_id)),
                Cell::new(format!("{}:{}-{}", m.file_a, m.start_line_a, m.end_line_a)),
                Cell::new(m.hits_a.to_string()),
                Cell::new(format!("{}:{}-{}", m.file_b, m.start_line_b, m.end_line_b)),
                Cell::new(m.hits_b.to_string()),
                Cell::new(tier_str),
                Cell::new(format!("{:.1}", m.risk_score)),
            ]));
        }
        println!("{table}");
    }
    println!();
}

fn print_markdown_summary(summary: &CoverageCorrelationSummary) {
    println!("# CDDM Runtime Execution & Coverage Correlation Report\n");
    println!("- **Total Clone Pairs**: {}", summary.total_clone_pairs);
    println!(
        "- **Covered Clones Rate**: {:.1}%",
        summary.overall_covered_clones_pct
    );
    println!("- **Dead Code Clones**: {}", summary.dead_code_clones);
    println!("- **Test Gap Clones**: {}", summary.test_gap_clones);
    println!("- **Hot Path Clones**: {}", summary.hot_path_clones);
    println!("- **Total Runtime Hits**: {}\n", summary.total_runtime_hits);

    println!("| Pair ID | Location A | Hits A | Location B | Hits B | Tier | Risk Score |");
    println!("| :--- | :--- | :--- | :--- | :--- | :--- | :--- |");
    for m in &summary.metrics {
        println!(
            "| #{} | `{}:{}-{}` | {} | `{}:{}-{}` | {} | {:?} | {:.1} |",
            m.clone_pair_id,
            m.file_a,
            m.start_line_a,
            m.end_line_a,
            m.hits_a,
            m.file_b,
            m.start_line_b,
            m.end_line_b,
            m.hits_b,
            m.execution_tier,
            m.risk_score
        );
    }
}
