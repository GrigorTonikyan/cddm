#![forbid(unsafe_code)]

use super::isomorphism::compute_weisfeiler_lehman_hash;
use super::types::{CfgEdge, CfgEdgeType, CfgNode, CfgNodeType, ControlFlowGraph};

/// Extracts Control Flow Graphs for all functions identified in the source text.
pub fn extract_cfgs_from_source(
    file_path: &str,
    content: &str,
    language: &str,
) -> Vec<ControlFlowGraph> {
    let mut cfgs = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let is_python = language.eq_ignore_ascii_case("python")
        || file_path.ends_with(".py")
        || file_path.ends_with(".pyi");

    let mut current_fn: Option<(String, usize, usize)> = None; // (name, start_line, indent)
    let mut fn_lines = Vec::new();
    let mut brace_depth: isize = 0;
    let mut opened_brace = false;

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        let current_indent = line.len() - line.trim_start().len();

        if is_python {
            let should_finish = current_fn
                .as_ref()
                .map(|(_, _, base_indent)| {
                    !trimmed.is_empty()
                        && !trimmed.starts_with('#')
                        && current_indent <= *base_indent
                })
                .unwrap_or(false);

            if should_finish {
                let (fn_name, start_line, _) = current_fn.take().expect("current_fn present");
                let end_line = fn_lines
                    .last()
                    .map(|(l, _): &(usize, &str)| *l)
                    .unwrap_or(start_line);
                let cfg =
                    build_cfg_from_lines(file_path, &fn_name, start_line, end_line, &fn_lines);
                cfgs.push(cfg);
                fn_lines.clear();
            }

            if current_fn.is_none() && is_function_header(trimmed) {
                let name = extract_fn_name(trimmed);
                current_fn = Some((name, line_num, current_indent));
            }

            if current_fn.is_some() {
                fn_lines.push((line_num, trimmed));
            }
        } else {
            // Brace-delimited / C-style languages
            if current_fn.is_none() && is_function_header(trimmed) {
                let name = extract_fn_name(trimmed);
                current_fn = Some((name, line_num, current_indent));
                brace_depth = 0;
                opened_brace = false;
            }

            if current_fn.is_some() {
                fn_lines.push((line_num, trimmed));

                // Track braces on this line (ignoring comments)
                let code_part = if let Some(pos) = trimmed.find("//") {
                    &trimmed[..pos]
                } else {
                    trimmed
                };

                let open_count = code_part.chars().filter(|&c| c == '{').count() as isize;
                let close_count = code_part.chars().filter(|&c| c == '}').count() as isize;

                if open_count > 0 {
                    opened_brace = true;
                }
                brace_depth += open_count - close_count;

                if opened_brace && brace_depth <= 0 {
                    // Function finished
                    if let Some((fn_name, start_line, _)) = current_fn.take() {
                        let cfg = build_cfg_from_lines(
                            file_path, &fn_name, start_line, line_num, &fn_lines,
                        );
                        cfgs.push(cfg);
                        fn_lines.clear();
                        opened_brace = false;
                        brace_depth = 0;
                    }
                }
            }
        }
    }

    if let Some((fn_name, start_line, _)) = current_fn {
        let end_line = fn_lines
            .last()
            .map(|(l, _): &(usize, &str)| *l)
            .unwrap_or(lines.len());
        let cfg = build_cfg_from_lines(file_path, &fn_name, start_line, end_line, &fn_lines);
        cfgs.push(cfg);
    }

    cfgs
}

fn is_function_header(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
        return false;
    }

    // Direct keyword starts
    if trimmed.starts_with("fn ")
        || trimmed.starts_with("pub fn ")
        || trimmed.starts_with("pub(crate) fn ")
        || trimmed.starts_with("pub async fn ")
        || trimmed.starts_with("pub(crate) async fn ")
        || trimmed.starts_with("async fn ")
        || trimmed.starts_with("unsafe fn ")
        || trimmed.starts_with("pub unsafe fn ")
        || trimmed.starts_with("def ")
        || trimmed.starts_with("async def ")
        || trimmed.starts_with("defp ")
        || trimmed.starts_with("function ")
        || trimmed.starts_with("export function ")
        || trimmed.starts_with("export default function ")
        || trimmed.starts_with("export async function ")
        || trimmed.starts_with("export default async function ")
        || trimmed.starts_with("async function ")
        || trimmed.starts_with("func ")
        || trimmed.starts_with("fun ")
        || trimmed.starts_with("suspend fun ")
    {
        return true;
    }

    // Arrow functions and assignments
    if (trimmed.starts_with("const ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("var ")
        || trimmed.starts_with("export const ")
        || trimmed.starts_with("export let ")
        || trimmed.starts_with("export var "))
        && (trimmed.contains(" = (")
            || trimmed.contains(" = async (")
            || trimmed.contains(" = function")
            || trimmed.contains("=>"))
    {
        return true;
    }

    // Class / Object methods and OOP function headers (Java, C#, C++, Go receivers, TS)
    if (trimmed.starts_with("public ")
        || trimmed.starts_with("private ")
        || trimmed.starts_with("protected ")
        || trimmed.starts_with("static ")
        || trimmed.starts_with("override ")
        || trimmed.starts_with("virtual ")
        || trimmed.starts_with("inline ")
        || trimmed.starts_with("func (")
        || trimmed.starts_with("void ")
        || trimmed.starts_with("int ")
        || trimmed.starts_with("bool ")
        || trimmed.starts_with("boolean ")
        || trimmed.starts_with("double ")
        || trimmed.starts_with("float ")
        || trimmed.starts_with("auto ")
        || trimmed.starts_with("String ")
        || trimmed.starts_with("string "))
        && trimmed.contains('(')
        && (trimmed.ends_with('{')
            || trimmed.ends_with(')')
            || trimmed.ends_with(':')
            || trimmed.contains(") {"))
    {
        return true;
    }

    false
}

fn extract_fn_name(line: &str) -> String {
    let mut text = line.trim();

    // Strip common leading modifiers
    let modifiers = [
        "export default async function ",
        "export default function ",
        "export async function ",
        "export function ",
        "async function ",
        "function ",
        "pub(crate) async fn ",
        "pub async fn ",
        "pub(crate) fn ",
        "pub unsafe fn ",
        "pub fn ",
        "async fn ",
        "unsafe fn ",
        "fn ",
        "async def ",
        "def ",
        "defp ",
        "suspend fun ",
        "fun ",
        "func ",
        "export const ",
        "export let ",
        "export var ",
        "const ",
        "let ",
        "var ",
        "public static ",
        "private static ",
        "protected static ",
        "public override ",
        "protected override ",
        "private override ",
        "public virtual ",
        "public ",
        "private ",
        "protected ",
        "static ",
        "override ",
        "virtual ",
        "inline ",
        "void ",
        "int ",
        "bool ",
        "boolean ",
        "double ",
        "float ",
        "auto ",
        "String ",
        "string ",
    ];

    for m in &modifiers {
        if text.starts_with(m) {
            text = text[m.len()..].trim();
        }
    }

    // Go method receiver handling e.g. `(r *Receiver) MethodName(...)`
    if let Some(close_paren) = text.strip_prefix('(').and_then(|r| r.find(')')) {
        text = text[(close_paren + 2)..].trim();
    }

    // Arrow assignment `foo = (...) =>`
    if let Some(eq_pos) = text.find('=') {
        let candidate = text[..eq_pos].trim();
        if !candidate.is_empty() && !candidate.contains(' ') {
            return candidate.to_string();
        }
    }

    // Standard `foo(...)`
    if let Some(open_paren) = text.find('(') {
        let candidate = text[..open_paren].trim();
        // If candidate contains spaces (e.g. return type + name), take the last word
        let last_word = candidate.split_whitespace().last().unwrap_or("");
        if !last_word.is_empty() {
            return last_word.to_string();
        }
    }

    "anonymous_fn".to_string()
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

        let (node_type, edge_type) = if line_text.starts_with("if ")
            || line_text.starts_with("if(")
            || line_text.starts_with("elif ")
            || line_text.starts_with("else if")
            || line_text.starts_with("case ")
            || line_text.starts_with("when ")
        {
            (CfgNodeType::Branch, CfgEdgeType::TrueBranch)
        } else if line_text.starts_with("for ")
            || line_text.starts_with("for(")
            || line_text.starts_with("while ")
            || line_text.starts_with("while(")
            || line_text.starts_with("loop")
            || line_text.starts_with("do ")
        {
            (CfgNodeType::LoopHeader, CfgEdgeType::LoopBack)
        } else if line_text.starts_with("return") || line_text.starts_with("yield") {
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
