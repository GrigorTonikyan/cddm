#![forbid(unsafe_code)]

pub mod comment;
pub mod diff;
pub mod hook;
pub mod ignore;
pub mod init;
pub mod lsp;
pub mod refactor;
pub mod rules;
pub mod scan;
pub mod trend;
pub mod watch;

pub use comment::run_comment_command;
pub use diff::run_diff_command;
pub use hook::run_hook_command;
pub use ignore::run_ignore_command;
pub use init::run_init_command;
pub use lsp::run_lsp_command;
pub use refactor::run_refactor_command;
pub use rules::run_rules_command;
pub use scan::run_scan_command;
pub use trend::run_trend_command;
pub use watch::run_watch_command;
