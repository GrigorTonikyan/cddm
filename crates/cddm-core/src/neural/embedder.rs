#![forbid(unsafe_code)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::constants::*;
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
        let dim = config.dimension.max(MIN_EMBEDDING_DIMENSION);
        let mut vector = vec![0.0f32; dim];

        let total_subwords = subwords.len().min(config.max_subwords);
        for (pos, subword) in subwords.iter().take(total_subwords).enumerate() {
            let mut hasher = DefaultHasher::new();
            subword.hash(&mut hasher);
            let h = hasher.finish();

            let idx = (h as usize) % dim;
            let pos_weight = 1.0f32 / (1.0f32 + (pos as f32 * POS_WEIGHT_FACTOR));
            vector[idx] += pos_weight;

            // Secondary feature mapping for dense dimensionality diffusion
            let idx2 = ((h >> 32) as usize) % dim;
            vector[idx2] += DIFFUSION_WEIGHT * pos_weight;
        }

        // Compute L2 Euclidean norm
        let norm = Self::compute_l2_norm(&vector);
        if norm > NORM_EPSILON {
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
    #[inline]
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        crate::simd::compute_dot_product_f32(a, b).clamp(0.0, 1.0)
    }

    #[inline]
    fn compute_l2_norm(vec: &[f32]) -> f32 {
        crate::simd::compute_l2_norm_f32(vec)
    }
}
