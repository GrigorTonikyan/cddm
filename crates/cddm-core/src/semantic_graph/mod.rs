#![forbid(unsafe_code)]

pub mod cfg;
pub mod isomorphism;
pub mod pdg;
pub mod types;

pub use cfg::extract_cfgs_from_source;
pub use isomorphism::{calculate_graph_similarity, compute_weisfeiler_lehman_hash};
pub use pdg::build_pdg_from_cfg;
pub use types::{
    CfgEdge, CfgEdgeType, CfgNode, CfgNodeType, ControlFlowGraph, PdgEdge, PdgEdgeKind,
    ProgramDependenceGraph, SemanticCloneMatch,
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
}
