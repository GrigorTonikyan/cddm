#![forbid(unsafe_code)]

use super::types::{CachedAstSummary, CachedTokenization, QueryCacheStats, QueryKey};
use crate::fingerprint::Fingerprint;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

/// In-memory query cache storing memoized tokenizations, ASTs, and fingerprints.
#[derive(Debug)]
pub struct QueryMemoCache {
    tokens_cache: RwLock<HashMap<QueryKey, CachedTokenization>>,
    ast_cache: RwLock<HashMap<QueryKey, CachedAstSummary>>,
    fingerprints_cache: RwLock<HashMap<QueryKey, Vec<Fingerprint>>>,
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
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// Attempts to retrieve memoized tokenization for the specified query key.
    pub async fn get_tokens(&self, key: &QueryKey) -> Option<CachedTokenization> {
        let guard = self.tokens_cache.read().await;
        if let Some(val) = guard.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(val.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Stores a computed tokenization into the memoization cache.
    pub async fn insert_tokens(&self, key: QueryKey, tokenization: CachedTokenization) {
        let mut guard = self.tokens_cache.write().await;
        guard.insert(key, tokenization);
    }

    /// Attempts to retrieve memoized fingerprints for the specified query key.
    pub async fn get_fingerprints(&self, key: &QueryKey) -> Option<Vec<Fingerprint>> {
        let guard = self.fingerprints_cache.read().await;
        if let Some(val) = guard.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(val.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Stores computed fingerprints into the memoization cache.
    pub async fn insert_fingerprints(&self, key: QueryKey, fingerprints: Vec<Fingerprint>) {
        let mut guard = self.fingerprints_cache.write().await;
        guard.insert(key, fingerprints);
    }

    /// Attempts to retrieve memoized AST summary for the specified query key.
    pub async fn get_ast(&self, key: &QueryKey) -> Option<CachedAstSummary> {
        let guard = self.ast_cache.read().await;
        if let Some(val) = guard.get(key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(val.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Stores a computed AST summary into the memoization cache.
    pub async fn insert_ast(&self, key: QueryKey, ast_summary: CachedAstSummary) {
        let mut guard = self.ast_cache.write().await;
        guard.insert(key, ast_summary);
    }

    /// Clears all memoized cache entries.
    pub async fn clear(&self) {
        self.tokens_cache.write().await.clear();
        self.ast_cache.write().await.clear();
        self.fingerprints_cache.write().await.clear();
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
    }

    /// Returns the current cache hit/miss statistics.
    pub async fn stats(&self) -> QueryCacheStats {
        let tokens_len = self.tokens_cache.read().await.len();
        let ast_len = self.ast_cache.read().await.len();
        let fingerprints_len = self.fingerprints_cache.read().await.len();
        QueryCacheStats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            entries: tokens_len + ast_len + fingerprints_len,
        }
    }
}
