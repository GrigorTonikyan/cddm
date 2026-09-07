#![forbid(unsafe_code)]

use super::*;

#[test]
fn test_cross_package_reachability_classification() {
    let files = vec![
        (
            "crates/core/src/lib.rs".to_string(),
            "rs".to_string(),
            "pub fn shared_utility() -> bool { true }\nfn internal_helper() { let _ = 1; }"
                .to_string(),
        ),
        (
            "crates/cli/src/main.rs".to_string(),
            "rs".to_string(),
            "fn main() { let _ = shared_utility(); }".to_string(),
        ),
    ];

    let (dead_items, summary) = trace_cross_package_reachability(&files, ".", 1);

    assert!(summary.live_cross_package_symbols >= 1);
    let shared_trace = summary
        .symbol_traces
        .iter()
        .find(|t| t.symbol_name == "shared_utility");
    assert!(shared_trace.is_some());
    let trace = shared_trace.unwrap();
    assert_eq!(trace.status, ReachabilityStatus::LiveCrossPackage);

    let internal_dead = dead_items
        .iter()
        .find(|d| d.symbol_name == "internal_helper");
    assert!(internal_dead.is_some());
}

#[test]
fn test_unused_export_detection() {
    let files = vec![
        (
            "crates/core/src/codec.rs".to_string(),
            "rs".to_string(),
            "pub fn unused_exported_codec() -> usize { 42 }".to_string(),
        ),
        (
            "crates/cli/src/main.rs".to_string(),
            "rs".to_string(),
            "fn main() { let _ = 100; }".to_string(),
        ),
    ];

    let (dead_items, summary) = trace_cross_package_reachability(&files, ".", 1);

    assert_eq!(summary.unused_exported_symbols, 1);
    let codec_dead = dead_items
        .iter()
        .find(|d| d.symbol_name == "unused_exported_codec");
    assert!(codec_dead.is_some());
    let item = codec_dead.unwrap();
    assert!(item.is_exported);
    assert_eq!(item.confidence, 0.90);
}

#[test]
fn test_closing_brace_not_flagged_as_unreachable_statement() {
    // Reproducer from Gitea Issue #144
    let ts_code = r#"
function resolveWorkspacePath(moduleDir: string, moduleName: string): string | null {
  const workspacePaths = [
    resolve(moduleDir, '../../../../packages', moduleName),
    resolve(moduleDir, '../../../../apps', moduleName),
  ];

  for (const wsPath of workspacePaths) {
    if (existsSync(wsPath) && statSync(wsPath).isDirectory()) {
      return wsPath;
    }
  }

  return null;
}
"#;

    let files = vec![(
        "apps/cdn/src/manifest.ts".to_string(),
        "ts".to_string(),
        ts_code.to_string(),
    )];

    let (dead_items, _summary) = trace_cross_package_reachability(&files, ".", 1);

    let unreachable_blocks: Vec<_> = dead_items
        .iter()
        .filter(|d| d.kind == DeadCodeKind::UnreachableBlock)
        .collect();

    assert!(
        unreachable_blocks.is_empty(),
        "Expected zero unreachable blocks for valid TypeScript return statements, but found: {:?}",
        unreachable_blocks
    );
}

#[test]
fn test_actual_unreachable_statement_detected_in_typescript() {
    let ts_code = r#"
function calculate(n: number): number {
    return n * 2;
    const deadValue = 42;
}
"#;

    let files = vec![(
        "src/calc.ts".to_string(),
        "ts".to_string(),
        ts_code.to_string(),
    )];

    let (dead_items, _summary) = trace_cross_package_reachability(&files, ".", 1);

    let unreachable_blocks: Vec<_> = dead_items
        .iter()
        .filter(|d| d.kind == DeadCodeKind::UnreachableBlock)
        .collect();

    assert_eq!(
        unreachable_blocks.len(),
        1,
        "Expected exactly 1 unreachable block for deadValue statement, found: {:?}",
        unreachable_blocks
    );
    let dead_stmt = unreachable_blocks[0];
    assert_eq!(dead_stmt.symbol_name, "<unreachable_statement>");
    assert_eq!(dead_stmt.line_start, 4);
    assert_eq!(dead_stmt.line_end, 4);
}

#[test]
fn test_rust_return_closing_brace_not_flagged() {
    let rs_code = r#"
fn compute_total(a: i32, b: i32) -> i32 {
    return a + b;
}
"#;

    let files = vec![(
        "src/compute.rs".to_string(),
        "rs".to_string(),
        rs_code.to_string(),
    )];

    let (dead_items, _summary) = trace_cross_package_reachability(&files, ".", 1);

    let unreachable_blocks: Vec<_> = dead_items
        .iter()
        .filter(|d| d.kind == DeadCodeKind::UnreachableBlock)
        .collect();

    assert!(
        unreachable_blocks.is_empty(),
        "Expected no unreachable blocks for Rust return statement, found: {:?}",
        unreachable_blocks
    );
}
