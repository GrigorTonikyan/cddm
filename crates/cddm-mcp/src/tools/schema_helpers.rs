#![forbid(unsafe_code)]

use crate::protocol::mcp_tools;
use cddm_core::DEFAULT_MIN_TOKENS;
use serde_json::json;

pub fn tool_def(
    name: &str,
    description: &str,
    input_schema: serde_json::Value,
) -> serde_json::Value {
    let category = if name.contains("scan")
        || name.contains("diff")
        || name.contains("timeline")
        || name.contains("overlap")
        || name.contains("coverage")
        || name.contains("dead_clones")
        || name.contains("graph")
    {
        "detection"
    } else if name.contains("refactor") || name.contains("extract") || name.contains("heal") {
        "refactoring"
    } else if name.contains("polic") || name.contains("suppression") || name.contains("sarif") {
        "governance"
    } else {
        "synchronization"
    };

    let is_destructive = name.contains("apply")
        || name.contains("heal")
        || name.contains("import")
        || (name.contains("extract") && !name.contains("suggest"));

    let is_read_only = !is_destructive
        && (name.contains("scan")
            || name.contains("get_")
            || name.contains("check_")
            || name.contains("detect_")
            || name.contains("export_")
            || name.contains("correlate_")
            || name.contains("compare_"));

    let is_idempotent = is_read_only
        || name.contains("suggest")
        || name.contains("generate")
        || name.contains("verify");

    let is_open_world = name.contains("heal") || name.contains("prompt");

    let annotations = json!({
        "readOnly": is_read_only,
        "consequential": is_destructive,
        "idempotent": is_idempotent,
        "readOnlyHint": is_read_only,
        "destructiveHint": is_destructive,
        "idempotentHint": is_idempotent,
        "openWorldHint": is_open_world
    });

    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "x-cddm-category": category,
        "annotations": annotations
    })
}

pub fn obj_schema(props: serde_json::Value, required: &[&str]) -> serde_json::Value {
    if required.is_empty() {
        json!({ "type": "object", "properties": props })
    } else {
        json!({ "type": "object", "properties": props, "required": required })
    }
}

pub fn dir_and_tokens_schema() -> serde_json::Value {
    obj_schema(
        json!({
            mcp_tools::PARAM_DIRECTORY: { "type": "string", "description": "Target directory path (default: current directory)" },
            mcp_tools::PARAM_MIN_TOKENS: { "type": "number", "description": format!("Minimum token threshold (default: {})", DEFAULT_MIN_TOKENS) }
        }),
        &[],
    )
}

pub fn occurrences_item_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "file": { "type": "string" },
            "start_line": { "type": "integer" },
            "end_line": { "type": "integer" }
        },
        "required": ["file", "start_line", "end_line"]
    })
}

pub fn coverage_schema(include_extra: bool) -> serde_json::Value {
    let mut props = serde_json::Map::new();
    props.insert(
        "report_path".to_string(),
        json!({ "type": "string", "description": "Path to coverage tracefile (e.g. lcov.info, coverage.xml)" }),
    );
    props.insert(
        "report_content".to_string(),
        json!({ "type": "string", "description": "Raw coverage report file content" }),
    );
    props.insert(
        "directory".to_string(),
        json!({ "type": "string", "description": "Target workspace directory path (default: .)" }),
    );
    props.insert(
        "min_tokens".to_string(),
        json!({ "type": "number", "description": "Minimum token threshold (default: 50)" }),
    );
    if include_extra {
        props.insert(
            "format".to_string(),
            json!({ "type": "string", "description": "Coverage format: lcov, cobertura, istanbul, auto (default: auto)" }),
        );
        props.insert(
            "dead_code_only".to_string(),
            json!({ "type": "boolean", "description": "Filter for dead code duplicates with 0 runtime executions" }),
        );
        props.insert(
            "min_hits".to_string(),
            json!({ "type": "number", "description": "Minimum combined runtime execution hits" }),
        );
    }
    json!({
        "type": "object",
        "properties": props
    })
}
