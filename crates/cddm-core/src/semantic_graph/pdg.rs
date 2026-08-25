#![forbid(unsafe_code)]

use super::types::{ControlFlowGraph, PdgEdge, PdgEdgeKind, ProgramDependenceGraph};
use std::collections::HashSet;

/// Constructs a Program Dependence Graph (PDG) from a Control Flow Graph.
pub fn build_pdg_from_cfg(cfg: ControlFlowGraph) -> ProgramDependenceGraph {
    let mut data_edges = Vec::new();
    let mut defined_vars = HashSet::new();

    for node in &cfg.nodes {
        // Extract basic variable assignments (e.g. `let x = ...` or `x = ...`)
        let label = &node.label;
        if label.starts_with("let ") || label.starts_with("var ") || label.starts_with("const ") {
            let without_decl = label
                .trim_start_matches("let ")
                .trim_start_matches("var ")
                .trim_start_matches("const ");
            if let Some(eq_pos) = without_decl.find('=') {
                let var_name = without_decl[..eq_pos].trim().trim_start_matches("mut ");
                if !var_name.is_empty() {
                    defined_vars.insert((var_name.to_string(), node.id));
                }
            }
        }

        // Check if node uses previously defined variables
        for (var_name, def_node_id) in &defined_vars {
            if *def_node_id != node.id && label.contains(var_name) {
                data_edges.push(PdgEdge {
                    from: *def_node_id,
                    to: node.id,
                    variable: var_name.clone(),
                    kind: PdgEdgeKind::DataDependency,
                });
            }
        }
    }

    ProgramDependenceGraph { cfg, data_edges }
}
