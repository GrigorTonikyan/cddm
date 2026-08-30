#![forbid(unsafe_code)]

use super::types::{CachedAstSummary, CachedTokenization, QueryCacheStats, QueryKey};
use crate::fingerprint::Fingerprint;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

use crate::cpg::CodePropertyGraph;
use std::sync::Arc;

/// In-memory query cache storing memoized tokenizations, ASTs, fingerprints, and CPGs.
#[derive(Debug)]
pub struct QueryMemoCache {
    tokens_cache: RwLock<HashMap<QueryKey, CachedTokenization>>,
    ast_cache: RwLock<HashMap<QueryKey, CachedAstSummary>>,
    fingerprints_cache: RwLock<HashMap<QueryKey, Vec<Fingerprint>>>,
    cpg_cache: RwLock<HashMap<QueryKey, Arc<CodePropertyGraph>>>,
    normalized_token_hashes: RwLock<HashMap<String, [u8; 32]>>,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl Default for QueryMemoCache {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryMemoCache {
    /// Creates a new empty query memoization cache.
    pub fn new() -> Self {
        Self {
            tokens_cache: RwLock::new(HashMap::new()),
            ast_cache: RwLock::new(HashMap::new()),
            fingerprints_cache: RwLock::new(HashMap::new()),
            cpg_cache: RwLock::new(HashMap::new()),
            normalized_token_hashes: RwLock::new(HashMap::new()),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// Attempts to retrieve memoized tokenization for the specified query key.
    pub async fn get_tokens(&self, key: &QueryKey) -> Option<CachedTokenization> {
        self.get_entry(&self.tokens_cache, key).await
    }

    /// Stores a computed tokenization into the memoization cache.
    pub async fn insert_tokens(&self, key: QueryKey, tokenization: CachedTokenization) {
        self.tokens_cache.write().await.insert(key, tokenization);
    }

    /// Attempts to retrieve memoized fingerprints for the specified query key.
    pub async fn get_fingerprints(&self, key: &QueryKey) -> Option<Vec<Fingerprint>> {
        self.get_entry(&self.fingerprints_cache, key).await
    }

    /// Stores computed fingerprints into the memoization cache.
    pub async fn insert_fingerprints(&self, key: QueryKey, fingerprints: Vec<Fingerprint>) {
        self.fingerprints_cache
            .write()
            .await
            .insert(key, fingerprints);
    }

    /// Attempts to retrieve memoized AST summary for the specified query key.
    pub async fn get_ast(&self, key: &QueryKey) -> Option<CachedAstSummary> {
        self.get_entry(&self.ast_cache, key).await
    }

    /// Stores a computed AST summary into the memoization cache.
    pub async fn insert_ast(&self, key: QueryKey, ast_summary: CachedAstSummary) {
        self.ast_cache.write().await.insert(key, ast_summary);
    }

    /// Attempts to retrieve memoized CPG for the specified query key.
    pub async fn get_cpg(&self, key: &QueryKey) -> Option<Arc<CodePropertyGraph>> {
        self.get_entry(&self.cpg_cache, key).await
    }

    /// Stores a computed CPG into the memoization cache.
    pub async fn insert_cpg(&self, key: QueryKey, cpg: Arc<CodePropertyGraph>) {
        self.cpg_cache.write().await.insert(key, cpg);
    }

    /// Checks if a file's normalized token stream hash is already known.
    pub async fn get_normalized_token_hash(&self, file_path: &str) -> Option<[u8; 32]> {
        self.normalized_token_hashes
            .read()
            .await
            .get(file_path)
            .copied()
    }

    /// Records a file's normalized token stream hash for early-cutoff checking.
    pub async fn insert_normalized_token_hash(&self, file_path: String, hash: [u8; 32]) {
        self.normalized_token_hashes
            .write()
            .await
            .insert(file_path, hash);
    }

    async fn get_entry<T: Clone>(
        &self,
        cache: &RwLock<HashMap<QueryKey, T>>,
        key: &QueryKey,
    ) -> Option<T> {
        let guard = cache.read().await;
        if let Some(val) = guard.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(val.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Clears all memoized cache entries.
    pub async fn clear(&self) {
        self.tokens_cache.write().await.clear();
        self.ast_cache.write().await.clear();
        self.fingerprints_cache.write().await.clear();
        self.cpg_cache.write().await.clear();
        self.normalized_token_hashes.write().await.clear();
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
    }

    /// Returns the current cache hit/miss statistics.
    pub async fn stats(&self) -> QueryCacheStats {
        let tokens_len = self.tokens_cache.read().await.len();
        let ast_len = self.ast_cache.read().await.len();
        let fingerprints_len = self.fingerprints_cache.read().await.len();
        let cpg_len = self.cpg_cache.read().await.len();
        QueryCacheStats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            entries: tokens_len + ast_len + fingerprints_len + cpg_len,
        }
    }
}
