#![forbid(unsafe_code)]

use cddm_core::{CloneStatus, DiffScanResult};
use comfy_table::{Cell, Color, Table};
use std::fmt::Write as _;

pub fn format_diff_console_report(diff_result: &DiffScanResult) -> String {
    let mut out =
        String::from("\n=== CDDM — Code De-Duplication Meister Differential Report ===\n");
    let sum = &diff_result.summary;
    let _ = writeln!(out, "{:<22} {}", "Base Reference:", sum.base_ref);
    let _ = writeln!(out, "{:<22} {}", "Target Reference:", sum.target_ref);
    let _ = writeln!(
        out,
        "{:<22} {:.1} / 100.0",
        "Baseline DRY Score:", sum.base_dry_score
    );
    let _ = writeln!(
        out,
        "{:<22} {:.1} / 100.0",
        "Target DRY Score:", sum.target_dry_score
    );
    let delta_str = if sum.net_dry_delta >= 0.0 {
        format!("+{:.2}% (Improved)", sum.net_dry_delta)
    } else {
        format!("{:.2}% (Regressed)", sum.net_dry_delta)
    };
    let _ = writeln!(out, "{:<22} {}", "Net DRY Delta:", delta_str);
    let _ = writeln!(out, "{:<22} {}", "Changed Files:", sum.total_changed_files);
    let _ = writeln!(out, "{:<22} {}", "New Clones:", sum.new_clones);
    let _ = writeln!(out, "{:<22} {}", "Legacy Clones:", sum.legacy_clones);
    let _ = writeln!(out, "{:<22} {}", "Resolved Clones:", sum.resolved_clones);
    let _ = writeln!(out, "{:<22} {} ms", "Duration:", diff_result.duration_ms);
    out.push('\n');

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

        let _ = writeln!(out, "{table}");
    } else {
        out.push_str("No clone pairs present in compared changeset.\n");
    }
    out
}

pub fn format_diff_markdown_report(diff_result: &DiffScanResult) -> String {
    let mut out = String::from("# CDDM Differential Scan Report\n\n");
    let sum = &diff_result.summary;
    let _ = writeln!(out, "- **Base Reference**: `{}`", sum.base_ref);
    let _ = writeln!(out, "- **Target Reference**: `{}`", sum.target_ref);
    let _ = writeln!(out, "- **Baseline DRY Score**: `{:.1}`", sum.base_dry_score);
    let _ = writeln!(out, "- **Target DRY Score**: `{:.1}`", sum.target_dry_score);
    let _ = writeln!(out, "- **Net DRY Delta**: `{:.2}%`", sum.net_dry_delta);
    let _ = writeln!(out, "- **New Clones**: `{}`", sum.new_clones);
    let _ = writeln!(out, "- **Legacy Clones**: `{}`", sum.legacy_clones);
    out.push('\n');

    if !diff_result.diff_clones.is_empty() {
        out.push_str("| Status | File A | Lines A | File B | Lines B | Tokens |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for item in &diff_result.diff_clones {
            let pair = &item.clone_pair;
            let _ = writeln!(
                out,
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
    out
}

pub fn format_branch_matrix_console_report(matrix: &cddm_core::BranchMatrixReport) -> String {
    let mut out = String::from("\n=== CDDM — Multi-Branch & Worktree Clone Drift Matrix ===\n");
    let _ = writeln!(
        out,
        "{:<22} {}",
        "Branches Evaluated:",
        matrix.branches.join(", ")
    );
    if let Some(cleanest) = &matrix.cleanest_branch {
        let _ = writeln!(out, "{:<22} {}", "Cleanest Branch:", cleanest);
    }
    if let Some(drift) = &matrix.highest_drift_branch {
        let _ = writeln!(out, "{:<22} {}", "Highest Drift:", drift);
    }
    let _ = writeln!(out, "{:<22} {}", "Summary:", matrix.summary);
    out.push('\n');

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Base Branch"),
        Cell::new("Target Branch"),
        Cell::new("Base DRY"),
        Cell::new("Target DRY"),
        Cell::new("Net Delta"),
        Cell::new("Changed Files"),
        Cell::new("New Clones"),
        Cell::new("Divergence %"),
    ]);

    for item in &matrix.matrix {
        let delta_color = if item.net_dry_delta >= 0.0 {
            Color::Green
        } else {
            Color::Red
        };
        table.add_row(vec![
            Cell::new(&item.base_branch),
            Cell::new(&item.target_branch),
            Cell::new(format!("{:.1}", item.base_dry_score)),
            Cell::new(format!("{:.1}", item.target_dry_score)),
            Cell::new(format!("{:+.2}%", item.net_dry_delta)).fg(delta_color),
            Cell::new(item.changed_files_count),
            Cell::new(item.new_clones_count),
            Cell::new(format!("{:.1}%", item.divergence_index)).fg(Color::Yellow),
        ]);
    }
    let _ = writeln!(out, "{table}");
    out
}
