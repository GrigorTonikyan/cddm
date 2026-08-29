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
    };

    let res = semantic_neural_handler(axum::Json(req)).await;
    assert!(res.is_ok());
    let axum::Json(result) = res.unwrap();
    assert!(result.total_blocks_embedded > 0);
}
