#![forbid(unsafe_code)]

use cddm_core::TimelineTrend;
use comfy_table::{Cell, Color, Table};

pub fn print_trend_console_report(trend: &TimelineTrend) {
    println!("\n=== CDDM Historical Duplication Trend & Timeline Evolution ===\n");

    let mut table = Table::new();
    table.set_header(vec![
        "Commit",
        "Date",
        "Author",
        "Message",
        "Files",
        "Tokens",
        "Clones",
        "Clusters",
        "Dup %",
        "DRY Score",
    ]);

    for s in &trend.snapshots {
        let tag_suffix = s
            .tag
            .as_ref()
            .map(|t| format!(" ({t})"))
            .unwrap_or_default();
        let commit_cell = format!("{}{tag_suffix}", s.short_hash);
        let score_color = if s.dry_health_score >= 90.0 {
            Color::Green
        } else if s.dry_health_score >= 80.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        table.add_row(vec![
            Cell::new(commit_cell),
            Cell::new(&s.formatted_date),
            Cell::new(&s.author),
            Cell::new(if s.message.len() > 30 {
                format!("{}...", &s.message[..27])
            } else {
                s.message.clone()
            }),
            Cell::new(s.total_files),
            Cell::new(s.total_tokens),
            Cell::new(s.total_clones),
            Cell::new(s.total_clusters),
            Cell::new(format!("{:.1}%", s.duplication_percentage)),
            Cell::new(format!("{:.1}", s.dry_health_score)).fg(score_color),
        ]);
    }
    println!("{table}\n");

    let delta_sign = if trend.score_delta >= 0.0 { "+" } else { "" };
    println!(
        "Summary: Initial Score: {:.1} -> Current Score: {:.1} ({}{:.1} DRY delta) | Duplication \
         Change: {:+.2}%",
        trend.initial_score,
        trend.current_score,
        delta_sign,
        trend.score_delta,
        trend.duplication_delta
    );
}

pub fn print_trend_markdown_report(trend: &TimelineTrend) {
    println!("# CDDM Historical Duplication Trend\n");
    println!(
        "| Commit | Date | Author | Message | Files | Tokens | Clones | Clusters | Duplication | \
         DRY Score |"
    );
    println!("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |");
    for s in &trend.snapshots {
        let tag_str = s
            .tag
            .as_ref()
            .map(|t| format!(" ({t})"))
            .unwrap_or_default();
        let msg_clean = s.message.replace('|', "\\|");
        println!(
            "| `{}`{} | {} | {} | {} | {} | {} | {} | {} | {:.1}% | **{:.1}** |",
            s.short_hash,
            tag_str,
            s.formatted_date,
            s.author,
            msg_clean,
            s.total_files,
            s.total_tokens,
            s.total_clones,
            s.total_clusters,
            s.duplication_percentage,
            s.dry_health_score
        );
    }
    println!(
        "\n**Historical DRY Score Delta**: {:+.1} (Initial: {:.1} -> Current: {:.1})\n",
        trend.score_delta, trend.initial_score, trend.current_score
    );
}
