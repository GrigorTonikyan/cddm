#![forbid(unsafe_code)]

use super::interner::SymbolInterner;
use super::types::{CodePropertyGraph, CpgEdge, CpgEdgeKind, CpgNode, CpgNodeKind};
use crate::semantic_graph::cfg::extract_cfgs_from_source;
use crate::semantic_graph::pdg::build_pdg_from_cfg;
use crate::semantic_graph::types::{CfgEdgeType, CfgNodeType};

/// Builds a unified Code Property Graph from AST, CFG, and PDG data for a single function.
pub fn build_cpg_from_function(
    file_path: &str,
    code: &str,
    language: &str,
    interner: &SymbolInterner,
) -> Option<CodePropertyGraph> {
    let cfgs = extract_cfgs_from_source(file_path, code, language);
    let cfg = cfgs.into_iter().next()?;
    build_cpg_from_cfg(file_path, cfg, interner)
}

/// Builds unified Code Property Graphs for all functions identified in the source text.
pub fn build_all_cpgs_from_source(
    file_path: &str,
    code: &str,
    language: &str,
    interner: &SymbolInterner,
) -> Vec<CodePropertyGraph> {
    let cfgs = extract_cfgs_from_source(file_path, code, language);
    let mut cpgs = Vec::with_capacity(cfgs.len());
    for cfg in cfgs {
        if let Some(cpg) = build_cpg_from_cfg(file_path, cfg, interner) {
            cpgs.push(cpg);
        }
    }
    cpgs
}

fn build_cpg_from_cfg(
    file_path: &str,
    cfg: crate::semantic_graph::types::ControlFlowGraph,
    interner: &SymbolInterner,
) -> Option<CodePropertyGraph> {
    let pdg = build_pdg_from_cfg(cfg.clone());

    let file_path_sym = interner.intern(file_path);
    let function_name_sym = interner.intern(&cfg.function_name);

    let mut nodes = Vec::with_capacity(cfg.nodes.len());
    for node in &cfg.nodes {
        let kind = match node.node_type {
            CfgNodeType::Entry | CfgNodeType::BasicBlock | CfgNodeType::Exit => {
                CpgNodeKind::BasicBlock
            }
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

    #[test]
    fn test_polyglot_all_cpgs_extraction() {
        let py_code = r#"
def calculate_area(w, h):
    area = w * h
    if area > 100:
        print("large")
    return area

def compute_perimeter(w, h):
    return (w + h) * 2
"#;
        let interner = SymbolInterner::new();
        let cpgs = build_all_cpgs_from_source("calc.py", py_code, "python", &interner);
        assert_eq!(cpgs.len(), 2);
        assert_eq!(
            interner.resolve(cpgs[0].function_name_symbol),
            Some("calculate_area".to_string())
        );
        assert_eq!(
            interner.resolve(cpgs[1].function_name_symbol),
            Some("compute_perimeter".to_string())
        );

        let ts_code = r#"
export function filterActive(users: any[]) {
    return users.filter(u => u.active);
}
export const calculateTotal = (prices: number[]) => {
    let sum = 0;
    for (const p of prices) { sum += p; }
    return sum;
};
"#;
        let ts_cpgs = build_all_cpgs_from_source("users.ts", ts_code, "typescript", &interner);
        assert_eq!(ts_cpgs.len(), 2);
    }
}
