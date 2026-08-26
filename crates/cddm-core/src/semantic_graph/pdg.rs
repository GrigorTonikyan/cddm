use super::types::{ControlFlowGraph, PdgEdge, PdgEdgeKind, ProgramDependenceGraph};

/// Constructs a Program Dependence Graph (PDG) from a Control Flow Graph.
pub fn build_pdg_from_cfg(cfg: ControlFlowGraph) -> ProgramDependenceGraph {
    let mut data_edges = Vec::new();
    let mut defined_vars: Vec<(String, usize, String)> = Vec::new(); // (original_name, def_node_id, canonical_slot)

    for node in &cfg.nodes {
        let label = &node.label;

        // 1. Variable declarations: let / const / var / val / auto
        if label.starts_with("let ")
            || label.starts_with("var ")
            || label.starts_with("const ")
            || label.starts_with("val ")
            || label.starts_with("auto ")
        {
            let without_decl = label
                .trim_start_matches("let ")
                .trim_start_matches("var ")
                .trim_start_matches("const ")
                .trim_start_matches("val ")
                .trim_start_matches("auto ");
            if let Some(eq_pos) = without_decl.find('=') {
                let var_name = without_decl[..eq_pos].trim().trim_start_matches("mut ");
                if !var_name.is_empty() {
                    let slot = format!("v{}", defined_vars.len());
                    defined_vars.push((var_name.to_string(), node.id, slot));
                }
            }
        }
        // 2. Loop variables: for x in ... or for (const x of ...)
        else if label.starts_with("for ") || label.starts_with("for(") {
            let loop_header = label.trim_start_matches("for").trim_start_matches('(');
            let var_candidate = if let Some(in_pos) = loop_header.find(" in ") {
                loop_header[..in_pos].trim()
            } else if let Some(of_pos) = loop_header.find(" of ") {
                loop_header[..of_pos].trim()
            } else {
                ""
            };
            let clean_var = var_candidate
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ");
            if !clean_var.is_empty() && !clean_var.contains(' ') {
                let slot = format!("v{}", defined_vars.len());
                defined_vars.push((clean_var.to_string(), node.id, slot));
            }
        }
        // 3. Direct assignment: x = ...
        else if let Some(eq_pos) = label.find('=') {
            let prefix = label[..eq_pos].trim();
            if !prefix.contains(' ')
                && !prefix.is_empty()
                && prefix.chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                let slot = format!("v{}", defined_vars.len());
                defined_vars.push((prefix.to_string(), node.id, slot));
            }
        }

        // Check if current node uses any previously defined variable
        for (var_name, def_node_id, _) in &defined_vars {
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
