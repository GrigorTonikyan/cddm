#![forbid(unsafe_code)]

use super::isomorphism::compute_weisfeiler_lehman_hash;
use super::types::{CfgEdge, CfgEdgeType, CfgNode, CfgNodeType, ControlFlowGraph};

/// Extracts Control Flow Graphs for all functions identified in the source text.
pub fn extract_cfgs_from_source(
    file_path: &str,
    content: &str,
    _language: &str,
) -> Vec<ControlFlowGraph> {
    let mut cfgs = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut current_fn: Option<(String, usize)> = None;
    let mut fn_lines = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = idx + 1;

        if (trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("def ")
            || trimmed.starts_with("function ")
            || trimmed.starts_with("export function ")
            || trimmed.starts_with("public ")
            || trimmed.starts_with("func "))
            && trimmed.contains('(')
        {
            if let Some((fn_name, start_line)) = current_fn.take() {
                let cfg =
                    build_cfg_from_lines(file_path, &fn_name, start_line, line_num - 1, &fn_lines);
                cfgs.push(cfg);
                fn_lines.clear();
            }

            let name = extract_fn_name(trimmed);
            current_fn = Some((name, line_num));
        }

        if current_fn.is_some() {
            fn_lines.push((line_num, trimmed));
        }
    }

    if let Some((fn_name, start_line)) = current_fn {
        let cfg = build_cfg_from_lines(file_path, &fn_name, start_line, lines.len(), &fn_lines);
        cfgs.push(cfg);
    }

    cfgs
}

fn extract_fn_name(line: &str) -> String {
    let without_modifiers = line
        .trim_start_matches("pub ")
        .trim_start_matches("export ")
        .trim_start_matches("public ")
        .trim_start_matches("private ")
        .trim_start_matches("protected ")
        .trim_start_matches("async ")
        .trim_start_matches("fn ")
        .trim_start_matches("def ")
        .trim_start_matches("function ")
        .trim_start_matches("func ");

    if let Some(open_paren) = without_modifiers.find('(') {
        without_modifiers[..open_paren].trim().to_string()
    } else {
        "anonymous_fn".to_string()
    }
}

fn build_cfg_from_lines(
    file_path: &str,
    fn_name: &str,
    start_line: usize,
    end_line: usize,
    lines: &[(usize, &str)],
) -> ControlFlowGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut next_id = 0;

    let entry_id = next_id;
    next_id += 1;
    nodes.push(CfgNode {
        id: entry_id,
        node_type: CfgNodeType::Entry,
        label: format!("Entry: {}", fn_name),
        statement_count: 1,
        line_start: start_line,
        line_end: start_line,
    });

    let mut last_node_id = entry_id;

    for (line_num, line_text) in lines {
        if line_text.is_empty() || *line_text == "{" || *line_text == "}" {
            continue;
        }

        let (node_type, edge_type) = if line_text.starts_with("if ") || line_text.starts_with("if(")
        {
            (CfgNodeType::Branch, CfgEdgeType::TrueBranch)
        } else if line_text.starts_with("for ")
            || line_text.starts_with("while ")
            || line_text.starts_with("loop")
        {
            (CfgNodeType::LoopHeader, CfgEdgeType::LoopBack)
        } else if line_text.starts_with("return") {
            (CfgNodeType::Return, CfgEdgeType::Sequential)
        } else {
            (CfgNodeType::BasicBlock, CfgEdgeType::Sequential)
        };

        let node_id = next_id;
        next_id += 1;

        nodes.push(CfgNode {
            id: node_id,
            node_type,
            label: line_text.to_string(),
            statement_count: 1,
            line_start: *line_num,
            line_end: *line_num,
        });

        edges.push(CfgEdge {
            from: last_node_id,
            to: node_id,
            edge_type,
        });

        last_node_id = node_id;
    }

    let exit_id = next_id;
    nodes.push(CfgNode {
        id: exit_id,
        node_type: CfgNodeType::Exit,
        label: "Exit".to_string(),
        statement_count: 1,
        line_start: end_line,
        line_end: end_line,
    });

    edges.push(CfgEdge {
        from: last_node_id,
        to: exit_id,
        edge_type: CfgEdgeType::Sequential,
    });

    let mut cfg = ControlFlowGraph {
        file_path: file_path.to_string(),
        function_name: fn_name.to_string(),
        line_start: start_line,
        line_end: end_line,
        nodes,
        edges,
        wl_hash: 0,
    };

    cfg.wl_hash = compute_weisfeiler_lehman_hash(&cfg, 2);
    cfg
}
