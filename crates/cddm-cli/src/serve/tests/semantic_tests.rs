#![forbid(unsafe_code)]

use super::super::types::SemanticNeuralRequest;
use super::super::*;

#[tokio::test]
async fn test_semantic_neural_handler() {
    let req = SemanticNeuralRequest {
        directory: Some(".".to_string()),
        threshold: Some(0.85),
        dimension: Some(256),
        max_subwords: Some(256),
        use_hnsw: None,
        use_sq8: None,
    };

    let res = semantic_neural_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(result) = res.unwrap();
    assert!(result.total_blocks_embedded > 0);
}

#[tokio::test]
async fn test_semantic_neural_handler_hnsw_sq8() {
    let req = SemanticNeuralRequest {
        directory: Some(".".to_string()),
        threshold: Some(0.85),
        dimension: Some(256),
        max_subwords: Some(256),
        use_hnsw: Some(true),
        use_sq8: Some(true),
    };

    let res = semantic_neural_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(result) = res.unwrap();
    assert_eq!(result.index_type.as_deref(), Some("hnsw_sq8"));
    assert_eq!(result.memory_reduction_ratio, Some(4.0));
}
