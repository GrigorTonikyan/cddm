#![forbid(unsafe_code)]

use super::interner::SymbolInterner;
use super::types::{CodePropertyGraph, CpgEdge, CpgEdgeKind, CpgNode, CpgNodeKind};
use crate::semantic_graph::cfg::extract_cfgs_from_source;
use crate::semantic_graph::pdg::build_pdg_from_cfg;
use crate::semantic_graph::types::{CfgEdgeType, CfgNodeType};

/// Builds a unified Code Property Graph from AST, CFG, and PDG data for a function.
pub fn build_cpg_from_function(
    file_path: &str,
    code: &str,
    language: &str,
    interner: &SymbolInterner,
) -> Option<CodePropertyGraph> {
    let cfgs = extract_cfgs_from_source(file_path, code, language);
    let cfg = cfgs.into_iter().next()?;
    let pdg = build_pdg_from_cfg(cfg.clone());

    let file_path_sym = interner.intern(file_path);
    let function_name_sym = interner.intern(&cfg.function_name);

    let mut nodes = Vec::with_capacity(cfg.nodes.len());
    for node in &cfg.nodes {
        let kind = match node.node_type {
            CfgNodeType::Entry | CfgNodeType::BasicBlock => CpgNodeKind::BasicBlock,
            CfgNodeType::Exit => CpgNodeKind::BasicBlock,
            CfgNodeType::Branch => CpgNodeKind::BranchCondition,
            CfgNodeType::LoopHeader => CpgNodeKind::LoopHeader,
            CfgNodeType::LoopBody => CpgNodeKind::LoopBody,
            CfgNodeType::Return => CpgNodeKind::ReturnStatement,
        };
        let label_sym = interner.intern(&node.label);

        nodes.push(CpgNode {
            id: node.id,
            symbol: label_sym,
            kind,
            line_start: node.line_start,
            line_end: node.line_end,
            statement_count: node.statement_count,
        });
    }

    let mut edges = Vec::new();

    // Map CFG edges
    for e in &cfg.edges {
        let kind = match e.edge_type {
            CfgEdgeType::Sequential => CpgEdgeKind::CfgSequential,
            CfgEdgeType::TrueBranch => CpgEdgeKind::CfgTrueBranch,
            CfgEdgeType::FalseBranch => CpgEdgeKind::CfgFalseBranch,
            CfgEdgeType::LoopBack => CpgEdgeKind::CfgLoopBack,
            CfgEdgeType::LoopExit => CpgEdgeKind::CfgLoopExit,
        };
        edges.push(CpgEdge {
            from: e.from,
            to: e.to,
            kind,
            variable_symbol: None,
        });
    }

    // Map PDG data flow edges
    for e in &pdg.data_edges {
        let var_sym = interner.intern(&e.variable);
        let kind = match e.kind {
            crate::semantic_graph::PdgEdgeKind::DataDependency => CpgEdgeKind::PdgDataDefUse,
            crate::semantic_graph::PdgEdgeKind::ControlDependency => {
                CpgEdgeKind::PdgControlDependency
            }
        };
        edges.push(CpgEdge {
            from: e.from,
            to: e.to,
            kind,
            variable_symbol: Some(var_sym),
        });
    }

    Some(CodePropertyGraph {
        file_path_symbol: file_path_sym,
        function_name_symbol: function_name_sym,
        line_start: cfg.line_start,
        line_end: cfg.line_end,
        nodes,
        edges,
        wl_hash: cfg.wl_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpg_construction() {
        let code = r#"
        pub fn sum_items(items: &[i32]) -> i32 {
            let mut total = 0;
            for x in items {
                if *x > 0 {
                    total += *x;
                }
            }
            total
        }
        "#;

        let interner = SymbolInterner::new();
        let cpg = build_cpg_from_function("src/calc.rs", code, "rust", &interner).unwrap();

        assert!(cpg.node_count() >= 3);
        assert!(cpg.edge_count() >= 2);
        assert_eq!(
            interner.resolve(cpg.file_path_symbol),
            Some("src/calc.rs".to_string())
        );
        assert_eq!(
            interner.resolve(cpg.function_name_symbol),
            Some("sum_items".to_string())
        );
    }
}
