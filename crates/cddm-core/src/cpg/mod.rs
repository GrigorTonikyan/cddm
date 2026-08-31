#![forbid(unsafe_code)]

pub mod builder;
pub mod interner;
pub mod types;

pub use builder::{build_all_cpgs_from_source, build_cpg_from_function};
pub use interner::{SymbolId, SymbolInterner};
pub use types::{CodePropertyGraph, CpgEdge, CpgEdgeKind, CpgNode, CpgNodeKind};
