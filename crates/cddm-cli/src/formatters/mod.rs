#![forbid(unsafe_code)]

pub mod dead_code;
pub mod diff;
pub mod policy;
pub mod refactor;
pub mod scan;
pub mod semantic;
pub mod trend;

pub use dead_code::format_dead_code_report;

pub use diff::{
    print_branch_matrix_console_report, print_diff_console_report, print_diff_markdown_report,
};
pub use policy::{print_policy_violations_console, print_policy_violations_markdown};
pub use refactor::{
    print_ast_refactor_recommendation, print_cluster_refactor_recommendation,
    print_refactor_recommendation,
};
pub use scan::{print_console_report, print_markdown_report, print_sarif_report};
pub use semantic::format_semantic_report;
pub use trend::{print_trend_console_report, print_trend_markdown_report};

pub fn print_structured_json_output<T: serde::Serialize>(
    payload: &T,
    is_ndjson: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if is_ndjson {
        println!("{}", serde_json::to_string(payload)?);
    } else {
        println!("{}", serde_json::to_string_pretty(payload)?);
    }
    Ok(())
}
