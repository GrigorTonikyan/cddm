#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Confidence classification for neural algorithmic equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquivalenceConfidence {
    High,
    Medium,
    Low,
}

/// Configuration options for the in-process neural code embedder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralEmbeddingConfig {
    /// Dimension of the generated dense embedding vectors (e.g. 128, 256, 384).
    pub dimension: usize,
    /// Minimum cosine similarity threshold for reporting equivalence (0.0 to 1.0).
    pub similarity_threshold: f32,
    /// Maximum subword token n-grams to evaluate per code block.
    pub max_subwords: usize,
}

impl Default for NeuralEmbeddingConfig {
    fn default() -> Self {
        Self {
            dimension: 256,
            similarity_threshold: 0.85,
            max_subwords: 512,
        }
    }
}

/// A dense floating-point vector representation of a code block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeEmbeddingVector {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub language: String,
    pub vector: Vec<f32>,
    pub norm: f32,
}

/// An identified algorithmic equivalence clone pair detected via neural embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeuralClonePair {
    pub file_a: String,
    pub start_line_a: usize,
    pub end_line_a: usize,
    pub language_a: String,
    pub file_b: String,
    pub start_line_b: usize,
    pub end_line_b: usize,
    pub language_b: String,
    pub similarity: f32,
    pub confidence: EquivalenceConfidence,
    pub semantic_rationale: String,
}

/// Aggregate summary of neural code embedding scan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NeuralScanResult {
    pub total_blocks_embedded: usize,
    pub total_neural_pairs: usize,
    pub high_confidence_count: usize,
    pub pairs: Vec<NeuralClonePair>,
}
