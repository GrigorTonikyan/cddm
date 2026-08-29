#![forbid(unsafe_code)]

/// Default dimension of generated dense neural embedding vectors.
pub const DEFAULT_NEURAL_DIMENSION: usize = 256;

/// Minimum cosine similarity threshold for reporting neural equivalence.
pub const DEFAULT_NEURAL_SIMILARITY_THRESHOLD: f32 = 0.85;

/// Maximum number of subword tokens to evaluate per code block.
pub const DEFAULT_NEURAL_MAX_SUBWORDS: usize = 512;

/// Cosine similarity threshold for High equivalence confidence classification.
pub const HIGH_CONFIDENCE_THRESHOLD: f32 = 0.95;

/// Cosine similarity threshold for Medium equivalence confidence classification.
pub const MEDIUM_CONFIDENCE_THRESHOLD: f32 = 0.88;

/// Minimum lower bound for embedding vector dimension.
pub const MIN_EMBEDDING_DIMENSION: usize = 32;

/// Minimum character length for code block candidates in workspace scanning.
pub const MIN_CODE_BLOCK_LENGTH: usize = 30;

/// Default line chunk size for sliding window code block extraction.
pub const DEFAULT_CHUNK_SIZE: usize = 20;

/// Epsilon cutoff for L2 Euclidean normalization norm.
pub const NORM_EPSILON: f32 = 1e-6;

/// Positional decay weight factor for subword tokens.
pub const POS_WEIGHT_FACTOR: f32 = 0.01;

/// Secondary hash feature weighting for dense dimensional diffusion.
pub const DIFFUSION_WEIGHT: f32 = 0.5;
