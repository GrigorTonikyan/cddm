#![forbid(unsafe_code)]

use super::types::{ContextSlice, ProgramDependenceGraph, ProgramSlice};
use std::collections::{HashSet, VecDeque};

/// Computes a backward static program slice on a Program Dependence Graph (PDG) from a criterion node.
/// Identifies all statements and variables that directly or indirectly affect the criterion node.
pub fn compute_backward_slice(
    pdg: &ProgramDependenceGraph,
    criterion_node_id: usize,
) -> ProgramSlice {
    let mut visited_nodes = HashSet::new();
    let mut sliced_variables = HashSet::new();
    let mut queue = VecDeque::new();

    visited_nodes.insert(criterion_node_id);
    queue.push_back(criterion_node_id);

    while let Some(current_id) = queue.pop_front() {
        // 1. Traverse Data Dependencies backwards (edges where .to == current_id)
        for edge in &pdg.data_edges {
            if edge.to == current_id {
                sliced_variables.insert(edge.variable.clone());
                if visited_nodes.insert(edge.from) {
                    queue.push_back(edge.from);
                }
            }
        }

        // 2. Traverse Control Flow predecessors (CFG edges where .to == current_id)
        for edge in &pdg.cfg.edges {
            if edge.to == current_id && visited_nodes.insert(edge.from) {
                queue.push_back(edge.from);
            }
        }
    }

    let mut sorted_nodes: Vec<usize> = visited_nodes.into_iter().collect();
    sorted_nodes.sort_unstable();

    let mut sorted_vars: Vec<String> = sliced_variables.into_iter().collect();
    sorted_vars.sort();

    ProgramSlice {
        criterion_node_id,
        is_backward: true,
        sliced_node_ids: sorted_nodes,
        sliced_variables: sorted_vars,
    }
}

/// Computes a forward static program slice on a Program Dependence Graph (PDG) from a criterion node.
/// Identifies all statements and variables that are directly or indirectly affected by the criterion node.
pub fn compute_forward_slice(
    pdg: &ProgramDependenceGraph,
    criterion_node_id: usize,
) -> ProgramSlice {
    let mut visited_nodes = HashSet::new();
    let mut sliced_variables = HashSet::new();
    let mut queue = VecDeque::new();

    visited_nodes.insert(criterion_node_id);
    queue.push_back(criterion_node_id);

    while let Some(current_id) = queue.pop_front() {
        // 1. Traverse Data Dependencies forwards (edges where .from == current_id)
        for edge in &pdg.data_edges {
            if edge.from == current_id {
                sliced_variables.insert(edge.variable.clone());
                if visited_nodes.insert(edge.to) {
                    queue.push_back(edge.to);
                }
            }
        }

        // 2. Traverse Control Flow successors (CFG edges where .from == current_id)
        for edge in &pdg.cfg.edges {
            if edge.from == current_id && visited_nodes.insert(edge.to) {
                queue.push_back(edge.to);
            }
        }
    }

    let mut sorted_nodes: Vec<usize> = visited_nodes.into_iter().collect();
    sorted_nodes.sort_unstable();

    let mut sorted_vars: Vec<String> = sliced_variables.into_iter().collect();
    sorted_vars.sort();

    ProgramSlice {
        criterion_node_id,
        is_backward: false,
        sliced_node_ids: sorted_nodes,
        sliced_variables: sorted_vars,
    }
}

/// Extracts a surrounding context slice for a given line range inside a function.
pub fn extract_context_slice(
    pdg: &ProgramDependenceGraph,
    start_line: usize,
    end_line: usize,
) -> ContextSlice {
    let mut inside_node_ids = Vec::new();
    let mut defined_variables = Vec::new();
    let mut required_variables = HashSet::new();
    let mut upstream_statements = Vec::new();
    let mut downstream_statements = Vec::new();

    // 1. Identify nodes inside the range vs outside
    for node in &pdg.cfg.nodes {
        if node.line_start >= start_line && node.line_end <= end_line {
            inside_node_ids.push(node.id);
        }
    }

    // 2. Inspect data dependencies crossing the boundary
    for edge in &pdg.data_edges {
        let from_inside = inside_node_ids.contains(&edge.from);
        let to_inside = inside_node_ids.contains(&edge.to);

        // Required by inside (defined outside, used inside)
        if !from_inside && to_inside {
            required_variables.insert(edge.variable.clone());
            if let Some(from_node) = pdg.cfg.nodes.iter().find(|n| n.id == edge.from)
                && !upstream_statements.contains(&from_node.label)
            {
                upstream_statements.push(from_node.label.clone());
            }
        }

        // Defined inside, used outside
        if from_inside && !to_inside {
            if !defined_variables.contains(&edge.variable) {
                defined_variables.push(edge.variable.clone());
            }
            if let Some(to_node) = pdg.cfg.nodes.iter().find(|n| n.id == edge.to)
                && !downstream_statements.contains(&to_node.label)
            {
                downstream_statements.push(to_node.label.clone());
            }
        }
    }

    let mut req_vars_sorted: Vec<String> = required_variables.into_iter().collect();
    req_vars_sorted.sort();

    ContextSlice {
        enclosing_function: pdg.cfg.function_name.clone(),
        line_span: (start_line, end_line),
        defined_variables,
        required_variables: req_vars_sorted,
        upstream_statements,
        downstream_statements,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_graph::cfg::extract_cfgs_from_source;
    use crate::semantic_graph::pdg::build_pdg_from_cfg;

    #[test]
    fn test_backward_and_forward_slicing() {
        let code = r#"
fn compute_total() {
    let base = 100;
    let factor = 2;
    let mut sum = base * factor;
    sum += 5;
    println!("{}", sum);
}
"#;
        let cfgs = extract_cfgs_from_source("test.rs", code, "rust");
        assert!(!cfgs.is_empty());
        let pdg = build_pdg_from_cfg(cfgs[0].clone());

        if let Some(last_node) = pdg.cfg.nodes.last() {
            let back_slice = compute_backward_slice(&pdg, last_node.id);
            assert!(back_slice.is_backward);
            assert!(!back_slice.sliced_node_ids.is_empty());

            let fwd_slice = compute_forward_slice(&pdg, pdg.cfg.nodes[0].id);
            assert!(!fwd_slice.is_backward);
            assert!(!fwd_slice.sliced_node_ids.is_empty());
        }
    }

    #[test]
    fn test_context_slice_extraction() {
        let code = r#"
fn process_items() {
    let count = 10;
    let multiplier = 5;
    let result = count * multiplier;
    let formatted = format!("Total: {}", result);
    println!("{}", formatted);
}
"#;
        let cfgs = extract_cfgs_from_source("sample.rs", code, "rust");
        assert!(!cfgs.is_empty());
        let pdg = build_pdg_from_cfg(cfgs[0].clone());

        let slice = extract_context_slice(&pdg, 4, 5);
        assert_eq!(slice.enclosing_function, "process_items");
        assert_eq!(slice.line_span, (4, 5));
    }
}
