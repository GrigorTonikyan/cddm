#![forbid(unsafe_code)]

use super::interner::SymbolId;
use serde::{Deserialize, Serialize};

/// Type of node within a Code Property Graph (CPG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpgNodeKind {
    FunctionDeclaration,
    Parameter,
    Statement,
    Expression,
    BranchCondition,
    LoopHeader,
    LoopBody,
    ReturnStatement,
    BasicBlock,
}

/// A node in the unified Code Property Graph combining AST, CFG, and PDG information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpgNode {
    pub id: usize,
    pub symbol: SymbolId,
    pub kind: CpgNodeKind,
    pub line_start: usize,
    pub line_end: usize,
    pub statement_count: usize,
}

/// Kind of multi-edge in a Code Property Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpgEdgeKind {
    /// AST hierarchy edge (parent to child node)
    AstChild,
    /// Control Flow Graph sequential step
    CfgSequential,
    /// Control Flow Graph true condition branch
    CfgTrueBranch,
    /// Control Flow Graph false condition branch
    CfgFalseBranch,
    /// Control Flow Graph loop back edge
    CfgLoopBack,
    /// Control Flow Graph loop exit edge
    CfgLoopExit,
    /// Program Dependence Graph data definition-use dependency
    PdgDataDefUse,
    /// Program Dependence Graph control dependency
    PdgControlDependency,
}

/// An edge between two nodes in the Code Property Graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpgEdge {
    pub from: usize,
    pub to: usize,
    pub kind: CpgEdgeKind,
    pub variable_symbol: Option<SymbolId>,
}

/// Unified Code Property Graph (CPG) merging AST, CFG, and PDG layers into a single queryable graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodePropertyGraph {
    pub file_path_symbol: SymbolId,
    pub function_name_symbol: SymbolId,
    pub line_start: usize,
    pub line_end: usize,
    pub nodes: Vec<CpgNode>,
    pub edges: Vec<CpgEdge>,
    pub wl_hash: u64,
}

impl CodePropertyGraph {
    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the total number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns all outgoing edges from a specific node.
    pub fn outgoing_edges(&self, node_id: usize) -> impl Iterator<Item = &CpgEdge> {
        self.edges.iter().filter(move |e| e.from == node_id)
    }

    /// Returns all incoming edges to a specific node.
    pub fn incoming_edges(&self, node_id: usize) -> impl Iterator<Item = &CpgEdge> {
        self.edges.iter().filter(move |e| e.to == node_id)
    }
}
