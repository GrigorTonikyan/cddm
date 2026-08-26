#![forbid(unsafe_code)]

pub mod diff;
pub mod policy;
pub mod refactor;
pub mod scan;
pub mod semantic;
pub mod trend;

pub use diff::{print_diff_console_report, print_diff_markdown_report};
pub use refactor::{
    print_ast_refactor_recommendation, print_cluster_refactor_recommendation,
    print_refactor_recommendation,
};
pub use scan::{print_console_report, print_markdown_report, print_sarif_report};
pub use semantic::format_semantic_report;
pub use trend::{print_trend_console_report, print_trend_markdown_report};
