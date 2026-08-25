#![forbid(unsafe_code)]

use super::types::ControlFlowGraph;
use std::collections::HashMap;

/// Computes a Weisfeiler-Lehman (WL) graph kernel hash for invariant graph matching.
pub fn compute_weisfeiler_lehman_hash(cfg: &ControlFlowGraph, iterations: usize) -> u64 {
    if cfg.nodes.is_empty() {
        return 0;
    }

    let mut node_colors: HashMap<usize, u64> = cfg
        .nodes
        .iter()
        .map(|n| {
            let color = match n.node_type {
                super::types::CfgNodeType::Entry => 1,
                super::types::CfgNodeType::Exit => 2,
                super::types::CfgNodeType::BasicBlock => 3,
                super::types::CfgNodeType::Branch => 4,
                super::types::CfgNodeType::LoopHeader => 5,
                super::types::CfgNodeType::LoopBody => 6,
                super::types::CfgNodeType::Return => 7,
            };
            (n.id, color)
        })
        .collect();

    // Adjacency map
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in &cfg.edges {
        adj.entry(edge.from).or_default().push(edge.to);
        adj.entry(edge.to).or_default().push(edge.from);
    }

    for _ in 0..iterations {
        let mut new_colors = HashMap::new();
        for node in &cfg.nodes {
            let mut neighbor_colors = Vec::new();
            if let Some(neighbors) = adj.get(&node.id) {
                for n in neighbors {
                    if let Some(c) = node_colors.get(n) {
                        neighbor_colors.push(*c);
                    }
                }
            }
            neighbor_colors.sort_unstable();

            let mut hash = *node_colors.get(&node.id).unwrap_or(&0);
            for nc in neighbor_colors {
                hash = hash.wrapping_mul(31).wrapping_add(nc);
            }
            new_colors.insert(node.id, hash);
        }
        node_colors = new_colors;
    }

    let mut final_hash: u64 = 0;
    let mut sorted_colors: Vec<u64> = node_colors.values().copied().collect();
    sorted_colors.sort_unstable();
    for c in sorted_colors {
        final_hash = final_hash.wrapping_mul(37).wrapping_add(c);
    }

    final_hash
}

/// Calculates structural similarity between two Control Flow Graphs (0.0 to 1.0).
pub fn calculate_graph_similarity(g1: &ControlFlowGraph, g2: &ControlFlowGraph) -> f64 {
    if g1.nodes.is_empty() && g2.nodes.is_empty() {
        return 1.0;
    }
    if g1.nodes.is_empty() || g2.nodes.is_empty() {
        return 0.0;
    }

    let node_sim = 1.0
        - ((g1.nodes.len() as f64 - g2.nodes.len() as f64).abs()
            / (g1.nodes.len().max(g2.nodes.len()) as f64));

    let edge_sim = if g1.edges.is_empty() && g2.edges.is_empty() {
        1.0
    } else {
        1.0 - ((g1.edges.len() as f64 - g2.edges.len() as f64).abs()
            / (g1.edges.len().max(g2.edges.len()) as f64))
    };

    let wl_bonus = if g1.wl_hash == g2.wl_hash && g1.wl_hash != 0 {
        0.2
    } else {
        0.0
    };

    let base_score = 0.5 * node_sim + 0.5 * edge_sim;
    (base_score + wl_bonus).clamp(0.0, 1.0)
}
