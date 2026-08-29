#![forbid(unsafe_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::tokenizer::SubwordTokenizer;
use super::types::{CodeEmbeddingVector, NeuralEmbeddingConfig};

/// In-process Neural Code Embedder synthesizing dense semantic vectors.
#[derive(Debug)]
pub struct NeuralCodeEmbedder;

impl NeuralCodeEmbedder {
    /// Computes dense embedding vector for a given code fragment.
    pub fn embed_code_block(
        code: &str,
        file_path: &str,
        start_line: usize,
        end_line: usize,
        language: &str,
        config: &NeuralEmbeddingConfig,
    ) -> CodeEmbeddingVector {
        let subwords = SubwordTokenizer::tokenize(code);
        let dim = config.dimension.max(32);
        let mut vector = vec![0.0f32; dim];

        let total_subwords = subwords.len().min(config.max_subwords);
        for (pos, subword) in subwords.iter().take(total_subwords).enumerate() {
            let mut hasher = DefaultHasher::new();
            subword.hash(&mut hasher);
            let h = hasher.finish();

            let idx = (h as usize) % dim;
            let pos_weight = 1.0f32 / (1.0f32 + (pos as f32 * 0.01f32));
            vector[idx] += pos_weight;

            // Secondary feature mapping for dense dimensionality diffusion
            let idx2 = ((h >> 32) as usize) % dim;
            vector[idx2] += 0.5f32 * pos_weight;
        }

        // Compute L2 Euclidean norm
        let norm = Self::compute_l2_norm(&vector);
        if norm > 1e-6 {
            for val in &mut vector {
                *val /= norm;
            }
        }

        CodeEmbeddingVector {
            file_path: file_path.to_string(),
            start_line,
            end_line,
            language: language.to_string(),
            vector,
            norm: 1.0,
        }
    }

    /// Computes Cosine Similarity between two normalized dense code vectors.
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0f32;
        for (x, y) in a.iter().zip(b.iter()) {
            dot += x * y;
        }

        dot.clamp(0.0, 1.0)
    }

    fn compute_l2_norm(vec: &[f32]) -> f32 {
        let mut sum_sq = 0.0f32;
        for val in vec {
            sum_sq += val * val;
        }
        sum_sq.sqrt()
    }
}
