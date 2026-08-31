#![forbid(unsafe_code)]

use rayon::prelude::*;
use std::fs;
use std::path::Path;

use super::constants::*;
use super::embedder::NeuralCodeEmbedder;
use super::hnsw::{HnswConfig, HnswVectorIndex};
use super::types::{
    CodeEmbeddingVector, EquivalenceConfidence, NeuralClonePair, NeuralEmbeddingConfig,
    NeuralScanResult,
};

/// High-level Neural Code Equivalence Detector.
#[derive(Debug)]
pub struct NeuralMatcher;

impl NeuralMatcher {
    /// Scans a workspace directory using the fast HNSW index for O(N log N) vector retrieval.
    pub fn scan_workspace_hnsw(
        workspace_root: &Path,
        config: &NeuralEmbeddingConfig,
        hnsw_config: Option<HnswConfig>,
    ) -> Result<NeuralScanResult, String> {
        let mut vectors: Vec<CodeEmbeddingVector> = Vec::new();
        Self::collect_embeddings_recursive(workspace_root, workspace_root, config, &mut vectors)?;

        let total_blocks = vectors.len();
        let pairs =
            HnswVectorIndex::find_all_pairs(&vectors, hnsw_config, config.similarity_threshold);
        let high_count = pairs
            .iter()
            .filter(|p| p.confidence == EquivalenceConfidence::High)
            .count();
        let total_pairs = pairs.len();

        Ok(NeuralScanResult {
            total_blocks_embedded: total_blocks,
            total_neural_pairs: total_pairs,
            high_confidence_count: high_count,
            pairs,
        })
    }
    /// Scans a workspace directory or compares file vectors for neural algorithmic equivalence.
    pub fn scan_workspace(
        workspace_root: &Path,
        config: &NeuralEmbeddingConfig,
    ) -> Result<NeuralScanResult, String> {
        let mut vectors: Vec<CodeEmbeddingVector> = Vec::new();
        Self::collect_embeddings_recursive(workspace_root, workspace_root, config, &mut vectors)?;

        let total_blocks = vectors.len();
        let mut candidate_pairs = Vec::new();

        for i in 0..vectors.len() {
            for j in (i + 1)..vectors.len() {
                let vec_a = &vectors[i];
                let vec_b = &vectors[j];

                // Skip self-comparison of the exact same span in same file
                if vec_a.file_path == vec_b.file_path && vec_a.start_line == vec_b.start_line {
                    continue;
                }

                candidate_pairs.push((i, j));
            }
        }

        let mut pairs: Vec<NeuralClonePair> = candidate_pairs
            .par_iter()
            .filter_map(|&(i, j)| {
                let vec_a = &vectors[i];
                let vec_b = &vectors[j];

                let similarity =
                    NeuralCodeEmbedder::cosine_similarity(&vec_a.vector, &vec_b.vector);
                if similarity >= config.similarity_threshold {
                    let confidence = if similarity >= HIGH_CONFIDENCE_THRESHOLD {
                        EquivalenceConfidence::High
                    } else if similarity >= MEDIUM_CONFIDENCE_THRESHOLD {
                        EquivalenceConfidence::Medium
                    } else {
                        EquivalenceConfidence::Low
                    };

                    let rationale = format!(
                        "Neural embedding cosine similarity {:.1}% across {} and {}",
                        similarity * 100.0,
                        vec_a.language,
                        vec_b.language
                    );

                    Some(NeuralClonePair {
                        file_a: vec_a.file_path.clone(),
                        start_line_a: vec_a.start_line,
                        end_line_a: vec_a.end_line,
                        language_a: vec_a.language.clone(),
                        file_b: vec_b.file_path.clone(),
                        start_line_b: vec_b.start_line,
                        end_line_b: vec_b.end_line,
                        language_b: vec_b.language.clone(),
                        similarity,
                        confidence,
                        semantic_rationale: rationale,
                    })
                } else {
                    None
                }
            })
            .collect();

        let high_count = pairs
            .iter()
            .filter(|p| p.confidence == EquivalenceConfidence::High)
            .count();

        // Sort descending by similarity
        pairs.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let total_pairs = pairs.len();
        Ok(NeuralScanResult {
            total_blocks_embedded: total_blocks,
            total_neural_pairs: total_pairs,
            high_confidence_count: high_count,
            pairs,
        })
    }

    fn collect_embeddings_recursive(
        root: &Path,
        current: &Path,
        config: &NeuralEmbeddingConfig,
        vectors: &mut Vec<CodeEmbeddingVector>,
    ) -> Result<(), String> {
        let entries = match fs::read_dir(current) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.')
                    || name == "target"
                    || name == "node_modules"
                    || name == "dist"
                {
                    continue;
                }
                Self::collect_embeddings_recursive(root, &path, config, vectors)?;
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                let is_code = matches!(
                    ext.as_str(),
                    "rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "java" | "c" | "cpp" | "cs"
                );
                if is_code {
                    let content = match fs::read_to_string(&path) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };

                    let rel_path = path
                        .strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/");

                    // Extract chunk blocks
                    let lines: Vec<&str> = content.lines().collect();
                    let chunk_size = DEFAULT_CHUNK_SIZE;
                    let mut start = 1;
                    while start <= lines.len() {
                        let end = (start + chunk_size).min(lines.len());
                        let block_code = lines[start - 1..end].join("\n");
                        if block_code.trim().len() > MIN_CODE_BLOCK_LENGTH {
                            let vec = NeuralCodeEmbedder::embed_code_block(
                                &block_code,
                                &rel_path,
                                start,
                                end,
                                &ext,
                                config,
                            );
                            vectors.push(vec);
                        }
                        start += chunk_size / 2; // 50% overlap for sliding window
                    }
                }
            }
        }

        Ok(())
    }
}
