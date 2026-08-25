#![forbid(unsafe_code)]

use cddm_core::ScanResult;
use comfy_table::{Cell, Color, Table};

pub fn scan_metrics_summary(result: &ScanResult) -> [(&'static str, String); 8] {
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
        ("Scan Duration", format!("{} ms", result.duration_ms)),
    ]
}

pub fn print_console_report(result: &ScanResult) {
    println!("\n=== CDDM — Code De-Duplication Meister Report ===");
    for (k, v) in scan_metrics_summary(result) {
        println!("{:<22} {}", format!("{}:", k), v);
    }
    println!();

    if !result.clone_clusters.is_empty() {
        println!("--- Clone Clusters (N-way Equivalence Classes) ---");
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
            let locs_str = cluster
                .occurrences
                .iter()
                .map(|loc| format!("{}:{}-{}", loc.file, loc.start_line, loc.end_line))
                .collect::<Vec<_>>()
                .join(", ");
            let locs_truncated = if locs_str.len() > 55 {
                format!("{}...", &locs_str[..52])
            } else {
                locs_str
            };

            cluster_table.add_row(vec![
                Cell::new(format!("#{}", cluster.id)),
                Cell::new(format!("{:?}", cluster.clone_type)),
                Cell::new(cluster.occurrences.len()),
                Cell::new(cluster.token_count),
                Cell::new(format!("{:.1}%", cluster.similarity * 100.0)).fg(Color::Yellow),
                Cell::new(locs_truncated),
            ]);
        }
        println!("{}", cluster_table);
        if result.clone_clusters.len() > 20 {
            println!(
                "... and {} more clone clusters.",
                result.clone_clusters.len() - 20
            );
        }
        println!();
    }

    if !result.clone_pairs.is_empty() {
        println!("--- Pairwise Clones ---");
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

        println!("{}", table);
        if result.clone_pairs.len() > 25 {
            println!(
                "... and {} more clone pairs.",
                result.clone_pairs.len() - 25
            );
        }
    } else {
        println!("Zero code duplication detected!");
    }

    if !result.policy_violations.is_empty() {
        super::policy::print_policy_violations_console(&result.policy_violations);
    }
}

pub fn print_markdown_report(result: &ScanResult) {
    println!("# CDDM Duplicate Code Scan Report\n");
    for (k, v) in scan_metrics_summary(result) {
        println!("- **{}**: `{}`", k, v);
    }
    println!();

    if !result.clone_clusters.is_empty() {
        println!("### N-way Clone Clusters\n");
        println!("| Cluster | Type | Occurrences | Tokens | Similarity | Locations |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for cluster in &result.clone_clusters {
            let locs_str = cluster
                .occurrences
                .iter()
                .map(|loc| format!("`{}`:{}-{}", loc.file, loc.start_line, loc.end_line))
                .collect::<Vec<_>>()
                .join("<br>");
            println!(
                "| `#{}` | `{:?}` | {} | {} | {:.1}% | {} |",
                cluster.id,
                cluster.clone_type,
                cluster.occurrences.len(),
                cluster.token_count,
                cluster.similarity * 100.0,
                locs_str
            );
        }
        println!();
    }

    if !result.clone_pairs.is_empty() {
        println!("### Pairwise Clones\n");
        println!("| File A | Lines A | File B | Lines B | Tokens | Similarity |");
        println!("| :--- | :--- | :--- | :--- | :--- | :--- |");
        for pair in &result.clone_pairs {
            println!(
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
        println!("Zero code duplication detected!");
    }

    if !result.policy_violations.is_empty() {
        super::policy::print_policy_violations_markdown(&result.policy_violations);
    }
}

pub fn print_sarif_report(result: &ScanResult) -> Result<(), Box<dyn std::error::Error>> {
    let sarif_json = cddm_core::generate_sarif_json(result);
    println!("{}", serde_json::to_string_pretty(&sarif_json)?);
    Ok(())
}
