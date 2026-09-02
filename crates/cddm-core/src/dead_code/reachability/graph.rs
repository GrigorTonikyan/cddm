#![forbid(unsafe_code)]

use crate::monorepo::MonorepoWorkspace;
use std::collections::{HashMap, HashSet, VecDeque};

/// Discovered function declaration with package context.
#[derive(Debug, Clone)]
pub struct PackageFunctionMeta {
    pub name: String,
    pub file_path: String,
    pub package_name: String,
    pub line_start: usize,
    pub line_end: usize,
    pub token_count: usize,
    pub is_exported: bool,
}

/// Discovered reference/call-site occurrence.
#[derive(Debug, Clone)]
pub struct SymbolCallSite {
    pub symbol: String,
    pub file_path: String,
    pub package_name: String,
}

/// Compute transitive callers for a symbol via BFS traversal.
pub fn compute_transitive_callers(
    target_symbol: &str,
    call_graph: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    if let Some(immediate) = call_graph.get(target_symbol) {
        for caller in immediate {
            if visited.insert(caller.clone()) {
                queue.push_back(caller.clone());
                result.push(caller.clone());
            }
        }
    }

    while let Some(current) = queue.pop_front() {
        if let Some(upstream) = call_graph.get(&current) {
            for caller in upstream {
                if visited.insert(caller.clone()) {
                    queue.push_back(caller.clone());
                    result.push(caller.clone());
                }
            }
        }
    }

    result.sort();
    result
}

/// Resolves a file path to its workspace package name.
pub fn resolve_file_package(rel_path: &str, workspaces: &[MonorepoWorkspace]) -> String {
    let norm = rel_path.replace('\\', "/");
    for ws in workspaces {
        let ws_path = ws.path.replace('\\', "/");
        if ws_path != "." && norm.starts_with(&ws_path) {
            return ws.name.clone();
        }
    }
    if norm.starts_with("crates/") {
        let parts: Vec<&str> = norm.split('/').collect();
        if parts.len() > 1 {
            return parts[1].to_string();
        }
    } else if norm.starts_with("webui/") {
        return "webui".to_string();
    } else if norm.starts_with("scripts/") {
        return "scripts".to_string();
    }
    "root".to_string()
}
