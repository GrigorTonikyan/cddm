#![forbid(unsafe_code)]

use super::index::HnswVectorIndex;
use super::sq8_index::HnswSq8VectorIndex;
use crate::neural::quantization::SQ8Vector;
use crate::neural::types::CodeEmbeddingVector;

#[test]
fn test_hnsw_empty_and_single_insert() {
    let mut index = HnswVectorIndex::default();
    assert!(index.is_empty());
    assert_eq!(index.len(), 0);

    let vec1 = vec![1.0, 0.0, 0.0];
    let id1 = index.insert(vec1.clone());
    assert_eq!(id1, 0);
    assert_eq!(index.len(), 1);

    let results = index.search_top_k(&vec1, 1, 0.9);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, 0);
    assert!(results[0].1 >= 0.99);
}

#[test]
fn test_hnsw_top_k_search() {
    let mut index = HnswVectorIndex::default();
    let v1 = vec![1.0, 0.0, 0.0];
    let v2 = vec![0.9, 0.1, 0.0];
    let v3 = vec![0.0, 1.0, 0.0];

    index.insert(v1.clone());
    index.insert(v2);
    index.insert(v3);

    let res = index.search_top_k(&v1, 2, 0.5);
    assert!(res.len() >= 2);
    assert_eq!(res[0].0, 0);
    assert_eq!(res[1].0, 1);
}

#[test]
fn test_hnsw_find_all_pairs() {
    let v1 = CodeEmbeddingVector {
        file_path: "src/a.rs".to_string(),
        start_line: 1,
        end_line: 10,
        language: "Rust".to_string(),
        vector: vec![0.95, 0.05, 0.0],
        norm: 1.0,
    };
    let v2 = CodeEmbeddingVector {
        file_path: "src/b.rs".to_string(),
        start_line: 20,
        end_line: 30,
        language: "Rust".to_string(),
        vector: vec![0.93, 0.07, 0.0],
        norm: 1.0,
    };
    let items = vec![v1, v2];
    let pairs = HnswVectorIndex::find_all_pairs(&items, None, 0.85);

    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].file_a, "src/a.rs");
    assert_eq!(pairs[0].file_b, "src/b.rs");
}

#[test]
fn test_hnsw_sq8_quantized_index() {
    let mut index = HnswSq8VectorIndex::default();
    assert!(index.is_empty());

    let v1 = vec![1.0, 0.0, 0.0, 0.0];
    let v2 = vec![0.92, 0.08, 0.0, 0.0];
    let v3 = vec![0.0, 1.0, 0.0, 0.0];

    index.insert_f32(&v1);
    index.insert_f32(&v2);
    index.insert_f32(&v3);

    assert_eq!(index.len(), 3);
    assert!(!index.is_empty());

    // Search with unquantized query
    let results = index.search_top_k(&v1, 2, 0.7);
    assert!(results.len() >= 2);
    assert_eq!(results[0].0, 0);
    assert!(results[0].1 > 0.95);
    assert_eq!(results[1].0, 1);
    assert!(results[1].1 > 0.85);

    // 4x Memory reduction ratio
    let ratio = index.memory_reduction_ratio();
    assert!(
        (ratio - 4.0).abs() < 0.1,
        "Expected ~4.0x memory reduction, got {}",
        ratio
    );
    assert!(index.memory_bytes() > 0);
}

#[test]
fn test_hnsw_sq8_find_all_pairs() {
    let v1 = CodeEmbeddingVector {
        file_path: "crates/core/src/lib.rs".to_string(),
        start_line: 10,
        end_line: 25,
        language: "Rust".to_string(),
        vector: vec![0.85, 0.50, -0.15, 0.05],
        norm: 1.0,
    };
    let v2 = CodeEmbeddingVector {
        file_path: "crates/cli/src/lib.rs".to_string(),
        start_line: 50,
        end_line: 65,
        language: "Rust".to_string(),
        vector: vec![0.84, 0.51, -0.14, 0.06],
        norm: 1.0,
    };
    let items = vec![v1, v2];

    let pairs = HnswSq8VectorIndex::find_all_pairs(&items, None, 0.80);
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].file_a, "crates/core/src/lib.rs");
    assert_eq!(pairs[0].file_b, "crates/cli/src/lib.rs");
    assert!(pairs[0].similarity > 0.95);
    assert!(pairs[0].semantic_rationale.contains("HNSW SQ8"));
}

#[test]
fn test_hnsw_sq8_pre_quantized_insert() {
    let mut index = HnswSq8VectorIndex::default();
    let original = vec![0.5, 0.5, 0.5, 0.5];
    let sq8 = SQ8Vector::quantize(&original);

    let id = index.insert(sq8);
    assert_eq!(id, 0);
    assert_eq!(index.len(), 1);

    let res = index.search_top_k(&original, 1, 0.8);
    assert_eq!(res.len(), 1);
    assert!(res[0].1 > 0.95);
}
