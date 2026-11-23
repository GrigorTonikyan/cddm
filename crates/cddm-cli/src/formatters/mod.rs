#![forbid(unsafe_code)]

pub mod dead_code;
pub mod diff;
pub mod policy;
pub mod prune;
pub mod refactor;
pub mod scan;
pub mod semantic;
pub mod trend;

pub use dead_code::format_dead_code_report;
pub use prune::format_prune_report;

pub use diff::{
    format_branch_matrix_console_report, format_diff_console_report, format_diff_markdown_report,
};
pub use policy::{print_policy_violations_console, print_policy_violations_markdown};
pub use refactor::{
    print_ast_refactor_recommendation, print_cluster_refactor_recommendation,
    print_refactor_recommendation,
};
pub use scan::{
    format_compact_console_report, format_console_report, format_markdown_report,
    format_sarif_report, print_console_report, print_sarif_report,
};

pub use semantic::format_semantic_report;
pub use trend::{print_trend_console_report, print_trend_markdown_report};

pub fn format_structured_json_output<T: serde::Serialize>(
    payload: &T,
    is_ndjson: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let serialized = if is_ndjson {
        serde_json::to_string(payload)?
    } else {
        serde_json::to_string_pretty(payload)?
    };
    Ok(format!("{serialized}\n"))
}

pub fn print_structured_json_output<T: serde::Serialize>(
    payload: &T,
    is_ndjson: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = format_structured_json_output(payload, is_ndjson)?;
    print!("{rendered}");
    Ok(())
}

pub fn emit_report_output(
    content: &str,
    output: Option<&std::path::Path>,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(out_path, content)?;
        println!("[PASS] {label} written to '{}'", out_path.display());
    } else {
        print!("{content}");
    }
    Ok(())
}
