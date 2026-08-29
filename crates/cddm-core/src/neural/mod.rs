#![forbid(unsafe_code)]

pub mod embedder;
pub mod matcher;
pub mod tokenizer;
pub mod types;

pub use embedder::NeuralCodeEmbedder;
pub use matcher::NeuralMatcher;
pub use tokenizer::SubwordTokenizer;
pub use types::*;

use std::path::Path;

/// Scan workspace directory for neural algorithmic equivalence.
pub fn scan_neural_clones(
    workspace_root: &Path,
    config: &NeuralEmbeddingConfig,
) -> Result<NeuralScanResult, String> {
    NeuralMatcher::scan_workspace(workspace_root, config)
}

/// Computes embedding vector for a single code snippet.
pub fn compute_code_embedding(
    code: &str,
    language: &str,
    config: &NeuralEmbeddingConfig,
) -> CodeEmbeddingVector {
    NeuralCodeEmbedder::embed_code_block(code, "snippet", 1, code.lines().count(), language, config)
}

/// Compares two code snippets using neural embeddings and returns similarity (0.0 to 1.0).
pub fn compare_code_embeddings(
    code_a: &str,
    lang_a: &str,
    code_b: &str,
    lang_b: &str,
    config: &NeuralEmbeddingConfig,
) -> f32 {
    let vec_a = compute_code_embedding(code_a, lang_a, config);
    let vec_b = compute_code_embedding(code_b, lang_b, config);
    NeuralCodeEmbedder::cosine_similarity(&vec_a.vector, &vec_b.vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subword_tokenizer() {
        let code = "fn calculateTotalPrice_v2(itemsCount: usize) -> f64";
        let subwords = SubwordTokenizer::tokenize(code);
        assert!(subwords.contains(&"calculate".to_string()));
        assert!(subwords.contains(&"total".to_string()));
        assert!(subwords.contains(&"price".to_string()));
        assert!(subwords.contains(&"items".to_string()));
        assert!(subwords.contains(&"count".to_string()));
    }

    #[test]
    fn test_neural_embedding_similarity() {
        let config = NeuralEmbeddingConfig::default();

        let rust_code = r#"
        pub fn compute_sum(values: &[i32]) -> i32 {
            let mut total = 0;
            for v in values {
                total += v;
            }
            total
        }
        "#;

        let python_code = r#"
        def compute_sum(values: list[int]) -> int:
            total = 0
            for v in values:
                total += v
            return total
        "#;

        let unrelated_code = r#"
        pub fn render_html_header(title: &str) -> String {
            format!("<html><head><title>{}</title></head></html>", title)
        }
        "#;

        let sim_equivalent = compare_code_embeddings(rust_code, "rs", python_code, "py", &config);
        let sim_unrelated = compare_code_embeddings(rust_code, "rs", unrelated_code, "rs", &config);

        assert!(sim_equivalent > 0.60);
        assert!(sim_unrelated < 0.35);
        assert!(sim_equivalent > sim_unrelated);
    }
}
