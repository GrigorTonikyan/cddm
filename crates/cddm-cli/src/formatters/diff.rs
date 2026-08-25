#![forbid(unsafe_code)]

use cddm_core::{CloneStatus, DiffScanResult};
use comfy_table::{Cell, Color, Table};

pub fn print_diff_console_report(diff_result: &DiffScanResult) {
    let sum = &diff_result.summary;
    println!("\n=== CDDM — Code De-Duplication Meister Differential Report ===");
    println!("{:<22} {}", "Base Reference:", sum.base_ref);
    println!("{:<22} {}", "Target Reference:", sum.target_ref);
    println!(
        "{:<22} {:.1} / 100.0",
        "Baseline DRY Score:", sum.base_dry_score
    );
    println!(
        "{:<22} {:.1} / 100.0",
        "Target DRY Score:", sum.target_dry_score
    );
    let delta_str = if sum.net_dry_delta >= 0.0 {
        format!("+{:.2}% (Improved)", sum.net_dry_delta)
    } else {
        format!("{:.2}% (Regressed)", sum.net_dry_delta)
    };
    println!("{:<22} {}", "Net DRY Delta:", delta_str);
    println!("{:<22} {}", "Changed Files:", sum.total_changed_files);
    println!("{:<22} {}", "New Clones:", sum.new_clones);
    println!("{:<22} {}", "Legacy Clones:", sum.legacy_clones);
    println!("{:<22} {}", "Resolved Clones:", sum.resolved_clones);
    println!("{:<22} {} ms", "Duration:", diff_result.duration_ms);
    println!();

    if !diff_result.diff_clones.is_empty() {
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("Status"),
            Cell::new("File A"),
            Cell::new("Lines A"),
            Cell::new("File B"),
            Cell::new("Lines B"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
        ]);

        for item in diff_result.diff_clones.iter().take(25) {
            let pair = &item.clone_pair;
            let status_cell = match item.status {
                CloneStatus::New => Cell::new("NEW").fg(Color::Red),
                CloneStatus::Legacy => Cell::new("LEGACY").fg(Color::Blue),
                CloneStatus::Resolved => Cell::new("RESOLVED").fg(Color::Green),
            };

            table.add_row(vec![
                status_cell,
                Cell::new(&pair.file_a),
                Cell::new(format!("{}-{}", pair.start_line_a, pair.end_line_a)),
                Cell::new(&pair.file_b),
                Cell::new(format!("{}-{}", pair.start_line_b, pair.end_line_b)),
                Cell::new(pair.token_count),
                Cell::new(format!("{:.1}%", pair.similarity * 100.0)).fg(Color::Yellow),
            ]);
        }

        println!("{}", table);
    } else {
        println!("No clone pairs present in compared changeset.");
    }
}

pub fn print_diff_markdown_report(diff_result: &DiffScanResult) {
    let sum = &diff_result.summary;
    println!("# CDDM Differential Scan Report\n");
    println!("- **Base Reference**: `{}`", sum.base_ref);
    println!("- **Target Reference**: `{}`", sum.target_ref);
    println!("- **Baseline DRY Score**: `{:.1}`", sum.base_dry_score);
    println!("- **Target DRY Score**: `{:.1}`", sum.target_dry_score);
    println!("- **Net DRY Delta**: `{:.2}%`", sum.net_dry_delta);
    println!("- **New Clones**: `{}`", sum.new_clones);
    println!("- **Legacy Clones**: `{}`", sum.legacy_clones);
    println!();

    if !diff_result.diff_clones.is_empty() {
        println!("| Status | File A | Lines A | File B | Lines B | Tokens |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for item in &diff_result.diff_clones {
            let pair = &item.clone_pair;
            println!(
                "| `{}` | `{}` | {}-{} | `{}` | {}-{} | {} |",
                item.status,
                pair.file_a,
                pair.start_line_a,
                pair.end_line_a,
                pair.file_b,
                pair.start_line_b,
                pair.end_line_b,
                pair.token_count
            );
        }
    }
}
