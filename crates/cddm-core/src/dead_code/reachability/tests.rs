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
