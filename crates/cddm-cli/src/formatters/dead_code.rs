#![forbid(unsafe_code)]

use cddm_core::dead_code::DeadCodeSummary;
use comfy_table::{Cell, Color, Row, Table};

/// Format dead code detection results according to the requested output format.
pub fn format_dead_code_report(
    summary: &DeadCodeSummary,
    format: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match format.to_lowercase().trim() {
        "json" => Ok(serde_json::to_string_pretty(summary)?),
        "markdown" | "md" => Ok(render_markdown_report(summary)),
        "sarif" => Ok(render_sarif_report(summary)?),
        _ => Ok(render_console_report(summary)),
    }
}

fn render_console_report(summary: &DeadCodeSummary) -> String {
    let mut out = String::new();

    out.push_str("\n=== CDDM Dead Code Analysis Report ===\n\n");
    out.push_str(&format!(
        "Total Dead Code Items:   {}\n",
        summary.total_dead_items
    ));
    out.push_str(&format!(
        "Unreferenced Functions:  {}\n",
        summary.dead_functions
    ));
    out.push_str(&format!(
        "Unreachable Blocks:      {}\n",
        summary.unreachable_blocks
    ));
    out.push_str(&format!(
        "Dead Duplicate Clones:   {}\n",
        summary.dead_clones
    ));
    out.push_str(&format!(
        "Uncovered Test Items:    {}\n",
        summary.uncovered_items
    ));
    out.push_str(&format!(
        "Total Dead Code Lines:   {}\n",
        summary.total_dead_lines
    ));
    out.push_str(&format!(
        "Estimated Line Savings:  {:.2}%\n\n",
        summary.estimated_savings_pct
    ));

    if summary.items.is_empty() {
        out.push_str("No dead code items detected in the analyzed scope.\n");
        return out;
    }

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("ID").fg(Color::Cyan),
        Cell::new("File").fg(Color::Cyan),
        Cell::new("Lines").fg(Color::Cyan),
        Cell::new("Kind").fg(Color::Cyan),
        Cell::new("Symbol / Reason").fg(Color::Cyan),
        Cell::new("Saved LOC").fg(Color::Green),
    ]);

    for item in summary.items.iter().take(50) {
        let span_str = format!("{}:{}", item.line_start, item.line_end);
        let kind_str = item.kind.display_label();
        let desc = format!("{}\n({})", item.symbol_name, item.reason);

        table.add_row(Row::from(vec![
            Cell::new(item.id.to_string()),
            Cell::new(&item.file_path),
            Cell::new(span_str),
            Cell::new(kind_str),
            Cell::new(desc),
            Cell::new(item.estimated_lines_saved.to_string()).fg(Color::Green),
        ]));
    }

    out.push_str(&table.to_string());
    out.push('\n');

    if summary.items.len() > 50 {
        out.push_str(&format!(
            "\n... and {} more items. Use --format json for full listing.\n",
            summary.items.len() - 50
        ));
    }

    out
}

fn render_markdown_report(summary: &DeadCodeSummary) -> String {
    let mut md = String::new();
    md.push_str("# CDDM Dead Code Analysis Report\n\n");
    md.push_str("## Executive Summary\n\n");
    md.push_str("| Metric | Value |\n");
    md.push_str("| :--- | :--- |\n");
    md.push_str(&format!(
        "| **Total Dead Code Items** | `{}` |\n",
        summary.total_dead_items
    ));
    md.push_str(&format!(
        "| **Unreferenced Functions** | `{}` |\n",
        summary.dead_functions
    ));
    md.push_str(&format!(
        "| **Unreachable Blocks** | `{}` |\n",
        summary.unreachable_blocks
    ));
    md.push_str(&format!(
        "| **Dead Duplicate Clones** | `{}` |\n",
        summary.dead_clones
    ));
    md.push_str(&format!(
        "| **Uncovered Items** | `{}` |\n",
        summary.uncovered_items
    ));
    md.push_str(&format!(
        "| **Total Dead Lines** | `{}` |\n",
        summary.total_dead_lines
    ));
    md.push_str(&format!(
        "| **Estimated Savings** | `{:.2}%` |\n\n",
        summary.estimated_savings_pct
    ));

    md.push_str("## Detected Dead Code Items\n\n");
    md.push_str("| ID | File | Lines | Kind | Symbol | Saved Lines | Confidence |\n");
    md.push_str("| :- | :--- | :---- | :--- | :----- | :---------- | :--------- |\n");

    for item in &summary.items {
        md.push_str(&format!(
            "| {} | `{}` | `{}-{}` | {} | `{}` | {} | {:.0}% |\n",
            item.id,
            item.file_path,
            item.line_start,
            item.line_end,
            item.kind.display_label(),
            item.symbol_name,
            item.estimated_lines_saved,
            item.confidence * 100.0
        ));
    }

    md
}

fn render_sarif_report(summary: &DeadCodeSummary) -> Result<String, Box<dyn std::error::Error>> {
    let results: Vec<serde_json::Value> = summary
        .items
        .iter()
        .map(|item| {
            serde_json::json!({
                "ruleId": format!("cddm-dead-code/{}", item.kind.as_str()),
                "level": "warning",
                "message": { "text": format!("{}: {}", item.symbol_name, item.reason) },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": item.file_path },
                        "region": {
                            "startLine": item.line_start,
                            "endLine": item.line_end
                        }
                    }
                }]
            })
        })
        .collect();

    let sarif = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "CDDM",
                    "version": "1.9.0",
                    "informationUri": "https://git.gt-web-dev.com/gt-dev/cddm"
                }
            },
            "results": results
        }]
    });

    Ok(serde_json::to_string_pretty(&sarif)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cddm_core::dead_code::{DeadCodeItem, DeadCodeKind};

    #[test]
    fn test_format_dead_code_report_variants() {
        let item = DeadCodeItem {
            id: 1,
            file_path: "src/utils.rs".to_string(),
            symbol_name: "unused_func".to_string(),
            kind: DeadCodeKind::UnreferencedFunction,
            line_start: 10,
            line_end: 25,
            token_count: 50,
            estimated_lines_saved: 16,
            reason: "Unreferenced function".to_string(),
            confidence: 0.95,
        };

        let summary = DeadCodeSummary {
            total_dead_items: 1,
            dead_functions: 1,
            unreachable_blocks: 0,
            dead_clones: 0,
            uncovered_items: 0,
            total_dead_lines: 16,
            estimated_savings_pct: 5.2,
            items: vec![item],
        };

        let console_res = format_dead_code_report(&summary, "console").unwrap();
        assert!(console_res.contains("unused_func"));

        let json_res = format_dead_code_report(&summary, "json").unwrap();
        assert!(json_res.contains("unused_func"));

        let md_res = format_dead_code_report(&summary, "markdown").unwrap();
        assert!(md_res.contains("# CDDM Dead Code Analysis Report"));

        let sarif_res = format_dead_code_report(&summary, "sarif").unwrap();
        assert!(sarif_res.contains("2.1.0"));
    }
}
