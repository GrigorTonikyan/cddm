#![forbid(unsafe_code)]

use crate::types::commands::OverlapArgs;
use cddm_core::{OverlapScanResult, scan_workspace_overlap};
use comfy_table::{Cell, Color, Row, Table};

/// Executes the CLI `cddm overlap` command.
pub fn run_overlap_command(args: OverlapArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\x1b[36m--> Scanning '{}' for reimplemented ecosystem library algorithms...\x1b[0m",
        args.directory.display()
    );

    let result = scan_workspace_overlap(&args.directory, args.threshold)?;

    match args.format.to_lowercase().as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        "markdown" | "md" => {
            print_overlap_markdown(&result);
        }
        _ => {
            print_overlap_console(&result);
        }
    }

    Ok(())
}

fn print_overlap_console(result: &OverlapScanResult) {
    if result.matches.is_empty() {
        println!(
            "\n\x1b[32m[PASS] No ecosystem library overlap detected ({} files inspected, {} \
             functions analyzed)!\x1b[0m\n",
            result.total_files_scanned, result.scanned_functions
        );
        return;
    }

    println!(
        "\n\x1b[33m=== Ecosystem Library Overlap Matches ({}) ===\x1b[0m\n",
        result.matches.len()
    );

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Algorithm").fg(Color::Cyan),
        Cell::new("Category").fg(Color::DarkCyan),
        Cell::new("Location").fg(Color::Yellow),
        Cell::new("Confidence").fg(Color::Magenta),
        Cell::new("Recommended Replacement").fg(Color::Green),
        Cell::new("Install Command").fg(Color::DarkGreen),
    ]);

    for m in &result.matches {
        let loc = format!("{}:{}-{}", m.file_path, m.line_span.0, m.line_span.1);
        let conf_pct = format!("{:.0}%", m.confidence * 100.0);
        let conf_cell = if m.confidence >= 0.7 {
            Cell::new(&conf_pct).fg(Color::Green)
        } else {
            Cell::new(&conf_pct).fg(Color::Yellow)
        };

        table.add_row(Row::from(vec![
            Cell::new(&m.algorithm_name),
            Cell::new(&m.category),
            Cell::new(&loc),
            conf_cell,
            Cell::new(&m.recommended_library.package_name).fg(Color::Green),
            Cell::new(&m.recommended_library.install_command).fg(Color::DarkGreen),
        ]));
    }

    println!("{table}");
    println!("\n\x1b[36m{}\x1b[0m\n", result.summary);
}

fn print_overlap_markdown(result: &OverlapScanResult) {
    println!("# Ecosystem Library Overlap Report\n");
    println!("{}\n", result.summary);

    if result.matches.is_empty() {
        println!("No reimplemented library utilities discovered.");
        return;
    }

    println!(
        "| Algorithm | Category | Location | Confidence | Recommended Package | Install Command |"
    );
    println!("| :--- | :--- | :--- | :--- | :--- | :--- |");

    for m in &result.matches {
        println!(
            "| **{}** | {} | `{}:{}-{}` | {:.0}% | `{}` | `{}` |",
            m.algorithm_name,
            m.category,
            m.file_path,
            m.line_span.0,
            m.line_span.1,
            m.confidence * 100.0,
            m.recommended_library.package_name,
            m.recommended_library.install_command
        );
    }
    println!();
}
