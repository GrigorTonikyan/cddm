#![forbid(unsafe_code)]

pub mod engine;
pub mod memo;
pub mod types;

pub use engine::{IncrementalQueryEngine, compute_blake3_hash};
pub use memo::QueryMemoCache;
pub use types::{
    CachedTokenization, ContentHash, IncrementalDeltaReport, QueryCacheStats, QueryKey,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_query_tokenization_and_memoization() {
        let engine = IncrementalQueryEngine::new();
        let code = "fn add(a: i32, b: i32) -> i32 { a + b }";

        // First call should be a cache miss
        let res1 = engine
            .get_or_compute_tokens("src/lib.rs", code)
            .await
            .unwrap();
        assert_eq!(res1.language, "Rust");
        assert!(!res1.tokens.is_empty());

        let stats1 = engine.stats().await;
        assert_eq!(stats1.misses, 1);
        assert_eq!(stats1.hits, 0);

        // Second call with same path and content should be a cache hit
        let res2 = engine
            .get_or_compute_tokens("src/lib.rs", code)
            .await
            .unwrap();
        assert_eq!(res1.content_hash, res2.content_hash);
        assert_eq!(res1.tokens.len(), res2.tokens.len());

        let stats2 = engine.stats().await;
        assert_eq!(stats2.hits, 1);
        assert_eq!(stats2.misses, 1);
        assert_eq!(stats2.hit_ratio(), 50.0);
    }

    #[tokio::test]
    async fn test_query_fingerprints_memoization() {
        let engine = IncrementalQueryEngine::new();
        let code = r#"
            pub fn compute_factorial(n: u64) -> u64 {
                let mut result = 1;
                for i in 1..=n {
                    result *= i;
                }
                result
            }
        "#;

        let fps1 = engine
            .get_or_compute_fingerprints("src/math.rs", code, 10)
            .await;
        assert!(!fps1.is_empty());

        let fps2 = engine
            .get_or_compute_fingerprints("src/math.rs", code, 10)
            .await;
        assert_eq!(fps1.len(), fps2.len());
        assert_eq!(fps1, fps2);
    }

    #[tokio::test]
    async fn test_query_incremental_delta_computation() {
        let engine = IncrementalQueryEngine::new();

        let mut old_manifest = HashMap::new();
        let hash_a = compute_blake3_hash("content a");
        let hash_b = compute_blake3_hash("content b");
        let hash_c = compute_blake3_hash("content c");

        old_manifest.insert("a.rs".to_string(), hash_a);
        old_manifest.insert("b.rs".to_string(), hash_b);
        old_manifest.insert("c.rs".to_string(), hash_c);

        let mut new_manifest = HashMap::new();
        let hash_b_modified = compute_blake3_hash("content b modified");
        let hash_d = compute_blake3_hash("content d new");

        new_manifest.insert("a.rs".to_string(), hash_a); // unmodified
        new_manifest.insert("b.rs".to_string(), hash_b_modified); // modified
        new_manifest.insert("d.rs".to_string(), hash_d); // added
        // c.rs was removed

        let delta = engine.compute_incremental_delta(&old_manifest, &new_manifest);
        assert_eq!(delta.unmodified_files, 1);
        assert_eq!(delta.short_circuited_count, 1);
        assert_eq!(delta.modified_files, 1);
        assert_eq!(delta.added_files, 1);
        assert_eq!(delta.removed_files, 1);
    }
}
