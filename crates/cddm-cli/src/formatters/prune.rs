#![forbid(unsafe_code)]

use cddm_core::dead_code::DeadClonePruneResult;
use comfy_table::{Cell, Color, Row, Table};

/// Format dead clone pruning results according to the requested output format.
pub fn format_prune_report(
    result: &DeadClonePruneResult,
    format: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match format.to_lowercase().trim() {
        "json" => Ok(serde_json::to_string_pretty(result)?),
        "markdown" | "md" => Ok(render_markdown_report(result)),
        "sarif" => Ok(render_sarif_report(result)?),
        _ => Ok(render_console_report(result)),
    }
}

fn render_console_report(result: &DeadClonePruneResult) -> String {
    let mut out = String::new();

    let mode_str = if result.dry_run {
        " [DRY RUN - No Files Modified]"
    } else {
        " [MUTATIONS APPLIED]"
    };

    out.push_str(&format!(
        "\n=== CDDM Dead Clone Cluster Pruning Report{mode_str} ===\n\n"
    ));
    out.push_str(&format!(
        "Total Candidates Detected: {}\n",
        result.total_candidates
    ));
    out.push_str(&format!(
        "Items Pruned / Synthesized: {}\n",
        result.pruned_items
    ));
    out.push_str(&format!(
        "Items Skipped (Unsafe):    {}\n",
        result.skipped_items
    ));
    out.push_str(&format!(
        "Total Lines Removed:       {}\n",
        result.total_lines_removed
    ));
    out.push_str(&format!(
        "Files Affected:            {}\n\n",
        result.files_affected.len()
    ));

    if result.items.is_empty() {
        out.push_str("No dead clone items were targeted or eligible for pruning.\n");
        return out;
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").fg(Color::Cyan),
        Cell::new("File").fg(Color::Cyan),
        Cell::new("Lines").fg(Color::Cyan),
        Cell::new("Status").fg(Color::Cyan),
        Cell::new("Symbol / Reason").fg(Color::Cyan),
        Cell::new("LOC Removed").fg(Color::Green),
    ]);

    for item in result.items.iter().take(50) {
        let span_str = format!("{}:{}", item.line_start, item.line_end);
        let status_color = match item.status {
            cddm_core::dead_code::PruneActionStatus::Pruned
            | cddm_core::dead_code::PruneActionStatus::DryRunPruned => Color::Green,
            cddm_core::dead_code::PruneActionStatus::SkippedUnsafe => Color::Yellow,
            cddm_core::dead_code::PruneActionStatus::Failed => Color::Red,
        };

        let desc = format!("{}\n({})", item.symbol_name, item.reason);

        table.add_row(Row::from(vec![
            Cell::new(item.id.to_string()),
            Cell::new(&item.file_path),
            Cell::new(span_str),
            Cell::new(item.status.as_str()).fg(status_color),
            Cell::new(desc),
            Cell::new(item.lines_removed.to_string()).fg(Color::Green),
        ]));
    }

    out.push_str(&table.to_string());
    out.push('\n');

    if result.items.len() > 50 {
        out.push_str(&format!(
            "\n... and {} more items. Use --format json for full details.\n",
            result.items.len() - 50
        ));
    }

    out
}

fn render_markdown_report(result: &DeadClonePruneResult) -> String {
    let mut md = String::new();
    let mode_str = if result.dry_run { " (Dry Run)" } else { "" };
    md.push_str(&format!(
        "# CDDM Dead Clone Cluster Pruning Report{mode_str}\n\n"
    ));
    md.push_str("## Summary\n\n");
    md.push_str("| Metric | Value |\n");
    md.push_str("| :--- | :--- |\n");
    md.push_str(&format!(
        "| Total Candidates | {} |\n",
        result.total_candidates
    ));
    md.push_str(&format!("| Pruned Items | {} |\n", result.pruned_items));
    md.push_str(&format!("| Skipped Items | {} |\n", result.skipped_items));
    md.push_str(&format!(
        "| Total Lines Removed | {} |\n",
        result.total_lines_removed
    ));
    md.push_str(&format!(
        "| Files Affected | {} |\n\n",
        result.files_affected.len()
    ));

    if !result.items.is_empty() {
        md.push_str("## Pruned Items\n\n");
        md.push_str("| ID | File | Lines | Status | Symbol | Removed LOC |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
        for item in &result.items {
            md.push_str(&format!(
                "| {} | `{}` | {}-{} | `{}` | `{}` | {} |\n",
                item.id,
                item.file_path,
                item.line_start,
                item.line_end,
                item.status.as_str(),
                item.symbol_name,
                item.lines_removed
            ));
        }
        md.push('\n');
    }

    md
}

fn render_sarif_report(result: &DeadClonePruneResult) -> Result<String, serde_json::Error> {
    let results: Vec<serde_json::Value> = result
        .items
        .iter()
        .map(|item| {
            serde_json::json!({
                "ruleId": "CDDM-PRUNE-001",
                "level": "note",
                "message": {
                    "text": format!("Pruned dead clone item '{}': {}", item.symbol_name, item.reason)
                },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {
                            "uri": item.file_path
                        },
                        "region": {
                            "startLine": item.line_start,
                            "endLine": item.line_end
                        }
                    }
                }],
                "properties": {
                    "status": item.status.as_str(),
                    "linesRemoved": item.lines_removed,
                    "confidence": item.confidence
                }
            })
        })
        .collect();

    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "CDDM Prune Engine",
                    "version": "3.3.0",
                    "informationUri": "https://git.gt-web-dev.com/gt-dev/cddm"
                }
            },
            "results": results
        }]
    });

    serde_json::to_string_pretty(&sarif)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::dead_code::{DeadClonePruneResult, PruneActionStatus, PrunedItem};

    #[test]
    fn test_format_prune_report_console_and_json() {
        let res = DeadClonePruneResult {
            total_candidates: 2,
            pruned_items: 2,
            skipped_items: 0,
            total_lines_removed: 40,
            dry_run: true,
            files_affected: vec!["src/dead.rs".to_string()],
            items: vec![PrunedItem {
                id: 1,
                file_path: "src/dead.rs".to_string(),
                symbol_name: "dead_fn".to_string(),
                line_start: 10,
                line_end: 30,
                lines_removed: 21,
                status: PruneActionStatus::DryRunPruned,
                confidence: 0.95,
                reason: "0 callers".to_string(),
                diff_preview: Some("--- a/src/dead.rs".to_string()),
            }],
        };

        let console_out = format_prune_report(&res, "console").unwrap();
        assert!(console_out.contains("CDDM Dead Clone Cluster Pruning Report"));
        assert!(console_out.contains("DRY RUN"));
        assert!(console_out.contains("dead_fn"));

        let json_out = format_prune_report(&res, "json").unwrap();
        assert!(json_out.contains("\"dry_run\": true"));

        let md_out = format_prune_report(&res, "markdown").unwrap();
        assert!(md_out.contains("# CDDM Dead Clone Cluster Pruning Report"));

        let sarif_out = format_prune_report(&res, "sarif").unwrap();
        assert!(sarif_out.contains("CDDM-PRUNE-001"));
    }
}
