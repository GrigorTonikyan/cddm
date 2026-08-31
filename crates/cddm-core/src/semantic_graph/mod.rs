#![forbid(unsafe_code)]

pub mod cfg;
pub mod cross_language;
pub mod embedding;
pub mod isomorphism;
pub mod pdg;
pub mod slicing;
pub mod types;

pub use cfg::extract_cfgs_from_source;
pub use cross_language::{
    ExtractedCfgItem, extract_workspace_cfgs, extract_workspace_cfgs_parallel,
    scan_cross_language_workspace, scan_cross_language_workspace_with_progress,
    scan_semantic_workspace, scan_semantic_workspace_with_progress,
};
pub use embedding::{
    SparseTfVector, calculate_embedding_similarity, compute_hybrid_similarity,
    compute_hybrid_similarity_with_tf, compute_tf_vector, cosine_similarity,
    extract_semantic_tokens,
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

    #[test]
    fn test_subword_camel_snake_case_alignment() {
        let tokens_snake =
            extract_semantic_tokens("let discount_rate = calculate_user_discount(price);");
        let tokens_camel =
            extract_semantic_tokens("const discountRate = calculateUserDiscount(price);");

        let tf_snake = compute_tf_vector(&tokens_snake);
        let tf_camel = compute_tf_vector(&tokens_camel);

        let sim = cosine_similarity(&tf_snake, &tf_camel);
        assert!(
            sim >= 0.90,
            "CamelCase and snake_case tokens should have very high similarity, got: {}",
            sim
        );
    }

    #[test]
    fn test_multi_function_cross_language_isolation() {
        let rust_multi = r#"
        pub fn format_greeting(name: &str) -> String {
            let msg = format!("Hello, {}", name);
            return msg;
        }

        pub fn compute_tax(income: f64, is_resident: bool) -> f64 {
            let mut rate = 0.15;
            if is_resident {
                rate = 0.25;
            }
            let tax = income * rate;
            return tax;
        }

        pub fn log_event(event_type: &str, code: i32) {
            println!("Event: {} code: {}", event_type, code);
        }
        "#;

        let python_multi = r#"
def parse_header(header: str) -> list[str]:
    parts = header.split(":")
    return parts

def compute_tax(income: float, is_resident: bool) -> float:
    rate = 0.15
    if is_resident:
        rate = 0.25
    tax = income * rate
    return tax

def render_badge(role: str) -> bool:
    if role == "admin":
        return True
    return False
        "#;

        let cfgs_rust = extract_cfgs_from_source("calc.rs", rust_multi, "Rust");
        let cfgs_py = extract_cfgs_from_source("calc.py", python_multi, "Python");

        assert_eq!(cfgs_rust.len(), 3);
        assert_eq!(cfgs_py.len(), 3);

        assert_eq!(cfgs_rust[1].function_name, "compute_tax");
        assert_eq!(cfgs_py[1].function_name, "compute_tax");

        let lines_rust: Vec<&str> = rust_multi.lines().collect();
        let lines_py: Vec<&str> = python_multi.lines().collect();

        let snippet_rust =
            lines_rust[(cfgs_rust[1].line_start - 1)..cfgs_rust[1].line_end].join("\n");
        let snippet_py = lines_py[(cfgs_py[1].line_start - 1)..cfgs_py[1].line_end].join("\n");

        let hybrid =
            compute_hybrid_similarity(&cfgs_rust[1], &snippet_rust, &cfgs_py[1], &snippet_py, true);
        assert!(
            hybrid.hybrid_score >= 0.70,
            "Isolated function matching should succeed across Rust and Python, got: {:?}",
            hybrid
        );
    }

    #[test]
    fn test_sparse_tf_vector_two_pointer_cosine() {
        let tokens_a = extract_semantic_tokens("let discount = price * 0.15; return discount;");
        let tokens_b = extract_semantic_tokens("const discount = price * 0.15; return discount;");

        let sparse_a = SparseTfVector::from_tokens(&tokens_a);
        let sparse_b = SparseTfVector::from_tokens(&tokens_b);

        let sim = sparse_a.cosine_similarity(&sparse_b);
        assert!(
            sim >= 0.95,
            "Sparse TF vector cosine similarity should be >= 0.95, got: {}",
            sim
        );
    }

    #[test]
    fn test_compute_hybrid_similarity_with_tf_matches_standard() {
        let code_a = r#"
        pub fn add_tax(val: f64) -> f64 {
            let tax = val * 0.20;
            return val + tax;
        }
        "#;
        let code_b = r#"
        export function addTax(val: number): number {
            const tax = val * 0.20;
            return val + tax;
        }
        "#;

        let cfgs_a = extract_cfgs_from_source("tax.rs", code_a, "Rust");
        let cfgs_b = extract_cfgs_from_source("tax.ts", code_b, "TypeScript");

        let tokens_a = extract_semantic_tokens(code_a);
        let tokens_b = extract_semantic_tokens(code_b);

        let sparse_a = SparseTfVector::from_tokens(&tokens_a);
        let sparse_b = SparseTfVector::from_tokens(&tokens_b);

        let hybrid_standard =
            compute_hybrid_similarity(&cfgs_a[0], code_a, &cfgs_b[0], code_b, true);
        let hybrid_sparse = embedding::compute_hybrid_similarity_with_tf(
            &cfgs_a[0], &sparse_a, &cfgs_b[0], &sparse_b, true,
        );

        assert!(
            (hybrid_standard.hybrid_score - hybrid_sparse.hybrid_score).abs() < 0.05,
            "Standard ({}) and sparse TF ({}) hybrid scores should be closely matched",
            hybrid_standard.hybrid_score,
            hybrid_sparse.hybrid_score
        );
    }
}
