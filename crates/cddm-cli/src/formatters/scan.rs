#![forbid(unsafe_code)]

use cddm_core::ScanResult;
use comfy_table::{Cell, Color, Table};
use std::fmt::Write as _;

pub fn scan_metrics_summary(result: &ScanResult) -> [(&'static str, String); 9] {
    [
        ("Scan ID", result.scan_id.clone()),
        ("Total Files", result.total_files.to_string()),
        ("Total Tokens", result.total_tokens.to_string()),
        ("Total Clone Pairs", result.total_clones.to_string()),
        ("Total Clone Clusters", result.total_clusters.to_string()),
        (
            "Duplication Rate",
            format!("{:.2}%", result.duplication_percentage),
        ),
        (
            "DRY Health Score",
            format!("{:.1} / 100.0", result.dry_health_score),
        ),
        (
            "SIMD Engine",
            cddm_core::get_active_simd_engine().to_string(),
        ),
        ("Scan Duration", format!("{} ms", result.duration_ms)),
    ]
}

fn format_cluster_locations(cluster: &cddm_core::CloneCluster, truncate: bool) -> String {
    let locs_str = cluster
        .occurrences
        .iter()
        .map(|loc| format!("{}:{}-{}", loc.file, loc.start_line, loc.end_line))
        .collect::<Vec<_>>()
        .join(", ");
    if truncate && locs_str.len() > 55 {
        format!("{}...", &locs_str[..52])
    } else {
        locs_str
    }
}

pub fn format_console_report(result: &ScanResult) -> String {
    let mut out = String::from("\n=== CDDM — Code De-Duplication Meister Report ===\n");
    for (k, v) in scan_metrics_summary(result) {
        let _ = writeln!(out, "{:<22} {}", format!("{k}:"), v);
    }
    out.push('\n');

    if !result.clone_clusters.is_empty() {
        out.push_str("--- Clone Clusters (N-way Equivalence Classes) ---\n");
        let mut cluster_table = Table::new();
        cluster_table.set_header(vec![
            Cell::new("Cluster"),
            Cell::new("Type"),
            Cell::new("Occurrences"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
            Cell::new("Locations"),
        ]);

        for cluster in result.clone_clusters.iter().take(20) {
            cluster_table.add_row(vec![
                Cell::new(format!("#{}", cluster.id)),
                Cell::new(format!("{:?}", cluster.clone_type)),
                Cell::new(cluster.occurrences.len()),
                Cell::new(cluster.token_count),
                Cell::new(format!("{:.1}%", cluster.similarity * 100.0)).fg(Color::Yellow),
                Cell::new(format_cluster_locations(cluster, true)),
            ]);
        }
        let _ = writeln!(out, "{cluster_table}");
        if result.clone_clusters.len() > 20 {
            let _ = writeln!(
                out,
                "... and {} more clone clusters.",
                result.clone_clusters.len() - 20
            );
        }
        out.push('\n');
    }

    if !result.clone_pairs.is_empty() {
        out.push_str("--- Pairwise Clones ---\n");
        let mut table = Table::new();
        table.set_header(vec![
            Cell::new("File A"),
            Cell::new("Lines A"),
            Cell::new("File B"),
            Cell::new("Lines B"),
            Cell::new("Tokens"),
            Cell::new("Similarity"),
        ]);

        for pair in result.clone_pairs.iter().take(25) {
            table.add_row(vec![
                Cell::new(&pair.file_a),
                Cell::new(format!("{}-{}", pair.start_line_a, pair.end_line_a)),
                Cell::new(&pair.file_b),
                Cell::new(format!("{}-{}", pair.start_line_b, pair.end_line_b)),
                Cell::new(pair.token_count),
                Cell::new(format!("{:.1}%", pair.similarity * 100.0)).fg(Color::Yellow),
            ]);
        }

        let _ = writeln!(out, "{table}");
        if result.clone_pairs.len() > 25 {
            let _ = writeln!(
                out,
                "... and {} more clone pairs.",
                result.clone_pairs.len() - 25
            );
        }
    } else {
        out.push_str("Zero code duplication detected!\n");
    }

    if !result.policy_violations.is_empty() {
        out.push_str(&super::policy::format_policy_violations_console(
            &result.policy_violations,
        ));
    }
    out
}

pub fn print_console_report(result: &ScanResult) {
    print!("{}", format_console_report(result));
}

pub fn format_markdown_report(result: &ScanResult) -> String {
    let mut out = String::from("# CDDM Duplicate Code Scan Report\n\n");
    for (k, v) in scan_metrics_summary(result) {
        let _ = writeln!(out, "- **{k}**: `{v}`");
    }
    out.push('\n');

    if !result.clone_clusters.is_empty() {
        out.push_str("### N-way Clone Clusters\n\n");
        out.push_str("| Cluster | Type | Occurrences | Tokens | Similarity | Locations |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for cluster in &result.clone_clusters {
            let locs_str = cluster
                .occurrences
                .iter()
                .map(|loc| format!("`{}`:{}-{}", loc.file, loc.start_line, loc.end_line))
                .collect::<Vec<_>>()
                .join("<br>");
            let _ = writeln!(
                out,
                "| `#{}` | `{:?}` | {} | {} | {:.1}% | {} |",
                cluster.id,
                cluster.clone_type,
                cluster.occurrences.len(),
                cluster.token_count,
                cluster.similarity * 100.0,
                locs_str
            );
        }
        out.push('\n');
    }

    if !result.clone_pairs.is_empty() {
        out.push_str("### Pairwise Clones\n\n");
        out.push_str("| File A | Lines A | File B | Lines B | Tokens | Similarity |\n");
        out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for pair in &result.clone_pairs {
            let _ = writeln!(
                out,
                "| `{}` | {}-{} | `{}` | {}-{} | {} | {:.1}% |",
                pair.file_a,
                pair.start_line_a,
                pair.end_line_a,
                pair.file_b,
                pair.start_line_b,
                pair.end_line_b,
                pair.token_count,
                pair.similarity * 100.0
            );
        }
    } else {
        out.push_str("Zero code duplication detected!\n");
    }

    if !result.policy_violations.is_empty() {
        out.push_str(&super::policy::format_policy_violations_markdown(
            &result.policy_violations,
        ));
    }
    out
}

pub fn format_sarif_report(result: &ScanResult) -> Result<String, Box<dyn std::error::Error>> {
    let sarif_json = cddm_core::generate_sarif_json(result);
    Ok(format!("{}\n", serde_json::to_string_pretty(&sarif_json)?))
}

pub fn print_sarif_report(result: &ScanResult) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = format_sarif_report(result)?;
    print!("{rendered}");
    Ok(())
}

pub fn format_compact_console_report(result: &ScanResult) -> String {
    let mut out =
        String::from("\n=== CDDM — Code De-Duplication Meister Report (Summary Mode) ===\n");
    for (k, v) in scan_metrics_summary(result) {
        let _ = writeln!(out, "{:<22} {}", format!("{k}:"), v);
    }
    out.push('\n');

    if !result.clone_clusters.is_empty() {
        out.push_str("--- Top Clone Clusters (Showing Top 5) ---\n");
        let mut cluster_table = Table::new();
        cluster_table.set_header(vec![
            Cell::new("Cluster"),
            Cell::new("Type"),
            Cell::new("Occurrences"),
            Cell::new("Tokens"),
            Cell::new("Locations"),
        ]);

        for cluster in result.clone_clusters.iter().take(5) {
            cluster_table.add_row(vec![
                Cell::new(format!("#{}", cluster.id)),
                Cell::new(format!("{:?}", cluster.clone_type)),
                Cell::new(cluster.occurrences.len()),
                Cell::new(cluster.token_count),
                Cell::new(format_cluster_locations(cluster, true)),
            ]);
        }
        let _ = writeln!(out, "{cluster_table}");
        if result.clone_clusters.len() > 5 {
            let _ = writeln!(
                out,
                "... and {} more clone clusters omitted in summary mode.",
                result.clone_clusters.len() - 5
            );
        }
        out.push('\n');
    }
    out
}
