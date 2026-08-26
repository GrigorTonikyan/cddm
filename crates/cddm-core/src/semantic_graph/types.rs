#![forbid(unsafe_code)]

use crate::types::CloneType;
use serde::{Deserialize, Serialize};

/// Type of node in a Control Flow Graph (CFG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgNodeType {
    Entry,
    Exit,
    BasicBlock,
    Branch,
    LoopHeader,
    LoopBody,
    Return,
}

/// A node within a Control Flow Graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfgNode {
    pub id: usize,
    pub node_type: CfgNodeType,
    pub label: String,
    pub statement_count: usize,
    pub line_start: usize,
    pub line_end: usize,
}

/// Type of directional edge connecting CFG nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgEdgeType {
    Sequential,
    TrueBranch,
    FalseBranch,
    LoopBack,
    LoopExit,
}

/// An edge between two nodes in a CFG.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CfgEdge {
    pub from: usize,
    pub to: usize,
    pub edge_type: CfgEdgeType,
}

/// Control Flow Graph (CFG) representing a function or procedure body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub file_path: String,
    pub function_name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub nodes: Vec<CfgNode>,
    pub edges: Vec<CfgEdge>,
    pub wl_hash: u64,
}

/// Kind of dependence in a Program Dependence Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PdgEdgeKind {
    DataDependency,
    ControlDependency,
}

/// An edge representing data or control flow dependence in a PDG.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PdgEdge {
    pub from: usize,
    pub to: usize,
    pub variable: String,
    pub kind: PdgEdgeKind,
}

/// Program Dependence Graph (PDG) combining control flow and data dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramDependenceGraph {
    pub cfg: ControlFlowGraph,
    pub data_edges: Vec<PdgEdge>,
}

/// Result of deep semantic graph matching between two functions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticCloneMatch {
    pub file_a: String,
    pub function_a: String,
    pub lines_a: (usize, usize),
    pub file_b: String,
    pub function_b: String,
    pub lines_b: (usize, usize),
    pub similarity: f64,
    pub clone_type: CloneType,
}

/// Hybrid similarity scoring breakdown between two code implementations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HybridSimilarity {
    /// Graph structural isomorphism score from Weisfeiler-Lehman kernel (0.0 to 1.0)
    pub graph_similarity: f64,
    /// Token & operation vector cosine similarity score (0.0 to 1.0)
    pub token_similarity: f64,
    /// Weighted hybrid similarity score (0.0 to 1.0)
    pub hybrid_score: f64,
    /// Whether the match crosses programming language boundaries
    pub is_cross_language: bool,
}

/// A cross-language clone pair discovered across different source languages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossLanguageClonePair {
    pub file_a: String,
    pub language_a: String,
    pub function_a: String,
    pub lines_a: (usize, usize),
    pub file_b: String,
    pub language_b: String,
    pub function_b: String,
    pub lines_b: (usize, usize),
    pub graph_similarity: f64,
    pub token_similarity: f64,
    pub hybrid_score: f64,
    pub clone_type: CloneType,
}

/// Full comparison response between two code fragments or functions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticComparisonResponse {
    pub similarity: f64,
    pub graph_similarity: f64,
    pub token_similarity: f64,
    pub hybrid_score: f64,
    pub is_semantic_clone: bool,
    pub is_cross_language: bool,
    pub wl_hash_a: u64,
    pub wl_hash_b: u64,
    pub function_a: Option<String>,
    pub function_b: Option<String>,
    pub language_a: Option<String>,
    pub language_b: Option<String>,
}
