#![forbid(unsafe_code)]

pub mod catalog;
pub mod detector;
pub mod types;

pub use catalog::get_canonical_algorithms;
pub use detector::scan_workspace_overlap;
pub use types::*;
