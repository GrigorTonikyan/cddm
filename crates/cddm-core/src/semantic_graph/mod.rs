#![forbid(unsafe_code)]

pub mod cfg;
pub mod cross_language;
pub mod embedding;
pub mod isomorphism;
pub mod pdg;
pub mod slicing;
pub mod types;

pub use cfg::extract_cfgs_from_source;
pub use cross_language::{extract_workspace_cfgs, scan_cross_language_workspace};
pub use embedding::{
    calculate_embedding_similarity, compute_hybrid_similarity, compute_tf_vector,
    cosine_similarity, extract_semantic_tokens,
};
pub use isomorphism::{calculate_graph_similarity, compute_weisfeiler_lehman_hash};
pub use pdg::build_pdg_from_cfg;
pub use slicing::{compute_backward_slice, compute_forward_slice, extract_context_slice};
pub use types::{
    CfgEdge, CfgEdgeType, CfgNode, CfgNodeType, ContextSlice, ControlFlowGraph,
    CrossLanguageClonePair, HybridSimilarity, PdgEdge, PdgEdgeKind, ProgramDependenceGraph,
    ProgramSlice, SemanticCloneMatch, SemanticComparisonResponse,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_cfg_and_wl_hash() {
        let code = r#"
        pub fn compute_total(items: &[i32]) -> i32 {
            let mut sum = 0;
            for x in items {
                if *x > 0 {
                    sum += *x;
                }
            }
            return sum;
        }
        "#;

        let cfgs = extract_cfgs_from_source("src/calc.rs", code, "Rust");
        assert_eq!(cfgs.len(), 1);
        let cfg = &cfgs[0];
        assert_eq!(cfg.function_name, "compute_total");
        assert!(cfg.nodes.len() >= 4);
        assert!(cfg.wl_hash != 0);
    }

    #[test]
    fn test_graph_similarity() {
        let code_a = r#"
        pub fn foo() {
            let a = 1;
            if a > 0 {
                println!("pos");
            }
            return;
        }
        "#;

        let code_b = r#"
        pub fn bar() {
            let b = 2;
            if b > 0 {
                println!("pos");
            }
            return;
        }
        "#;

        let cfgs_a = extract_cfgs_from_source("a.rs", code_a, "Rust");
        let cfgs_b = extract_cfgs_from_source("b.rs", code_b, "Rust");
        assert_eq!(cfgs_a.len(), 1);
        assert_eq!(cfgs_b.len(), 1);

        let sim = calculate_graph_similarity(&cfgs_a[0], &cfgs_b[0]);
        assert!(sim >= 0.8);
    }

    #[test]
    fn test_build_pdg() {
        let code = r#"
        pub fn process() {
            let x = 10;
            let y = x + 5;
            return;
        }
        "#;

        let cfgs = extract_cfgs_from_source("src/lib.rs", code, "Rust");
        let pdg = build_pdg_from_cfg(cfgs[0].clone());
        assert!(!pdg.data_edges.is_empty());
        assert_eq!(pdg.data_edges[0].variable, "x");
    }

    #[test]
    fn test_cross_language_hybrid_matching() {
        // Rust implementation
        let rust_code = r#"
        pub fn calculate_discount(price: f64, is_member: bool) -> f64 {
            let mut rate = 0.05;
            if is_member {
                rate = 0.20;
            }
            let discount = price * rate;
            return discount;
        }
        "#;

        // TypeScript implementation
        let ts_code = r#"
        export function calculateDiscount(price: number, isMember: boolean): number {
            let rate = 0.05;
            if (isMember) {
                rate = 0.20;
            }
            const discount = price * rate;
            return discount;
        }
        "#;

        let cfgs_rust = extract_cfgs_from_source("calc.rs", rust_code, "Rust");
        let cfgs_ts = extract_cfgs_from_source("calc.ts", ts_code, "TypeScript");

        assert_eq!(cfgs_rust.len(), 1);
        assert_eq!(cfgs_ts.len(), 1);

        let hybrid =
            compute_hybrid_similarity(&cfgs_rust[0], rust_code, &cfgs_ts[0], ts_code, true);

        assert!(
            hybrid.hybrid_score >= 0.70,
            "Expected high hybrid similarity between TS and Rust logic, got: {:?}",
            hybrid
        );
        assert!(hybrid.is_cross_language);
    }

    #[test]
    fn test_token_vector_cosine_similarity() {
        let code_a = "let total = 0; for x in items { total += x; } return total;";
        let code_b = "let sum = 0; for (const val of nums) { sum += val; } return sum;";
        let sim = calculate_embedding_similarity(code_a, code_b);
        assert!(
            sim >= 0.35,
            "Embedding similarity should be significant for isomorphic loops, got: {}",
            sim
        );
    }
}
