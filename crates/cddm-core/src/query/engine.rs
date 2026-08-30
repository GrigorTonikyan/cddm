#![forbid(unsafe_code)]

use super::memo::QueryMemoCache;
use super::types::{
    CachedAstSummary, CachedTokenization, ContentHash, IncrementalDeltaReport, QueryCacheStats,
    QueryKey,
};
use crate::fingerprint::{Fingerprint, MIN_K_GRAM, WINDOW_OFFSET, winnow};
use crate::grammar::get_grammar_for_path;
use crate::tokenizer::tokenize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Computes the 32-byte Blake3 hash for a source file's string content.
pub fn compute_blake3_hash(content: &str) -> ContentHash {
    *blake3::hash(content.as_bytes()).as_bytes()
}

/// Query-based incremental computation engine for tokenization, AST and fingerprint memoization.
#[derive(Debug, Clone)]
pub struct IncrementalQueryEngine {
    memo: Arc<QueryMemoCache>,
}

impl Default for IncrementalQueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalQueryEngine {
    /// Creates a new IncrementalQueryEngine.
    pub fn new() -> Self {
        Self {
            memo: Arc::new(QueryMemoCache::new()),
        }
    }

    /// Retrieves or computes normalized tokens for a file with automatic memoization.
    pub async fn get_or_compute_tokens(
        &self,
        file_path: &str,
        content: &str,
    ) -> Option<CachedTokenization> {
        let grammar = get_grammar_for_path(Path::new(file_path))?;
        let content_hash = compute_blake3_hash(content);
        let key = QueryKey {
            file_path: file_path.to_string(),
            content_hash,
        };

        if let Some(cached) = self.memo.get_tokens(&key).await {
            return Some(cached);
        }

        let raw_tokens = tokenize(content, grammar, true);
        let mut tokens = Vec::with_capacity(raw_tokens.len());
        let mut spans = Vec::with_capacity(raw_tokens.len());

        for (tok, span) in raw_tokens {
            tokens.push(tok);
            spans.push(span);
        }

        let tokenization = CachedTokenization {
            language: grammar.name.to_string(),
            tokens,
            spans,
            content_hash,
        };

        self.memo.insert_tokens(key, tokenization.clone()).await;
        Some(tokenization)
    }

    /// Retrieves or computes winnowing fingerprints for a file with automatic memoization.
    pub async fn get_or_compute_fingerprints(
        &self,
        file_path: &str,
        content: &str,
        min_tokens: usize,
    ) -> Vec<Fingerprint> {
        let content_hash = compute_blake3_hash(content);
        let key = QueryKey {
            file_path: file_path.to_string(),
            content_hash,
        };

        if let Some(cached) = self.memo.get_fingerprints(&key).await {
            return cached;
        }

        let tokenization = match self.get_or_compute_tokens(file_path, content).await {
            Some(t) => t,
            None => return Vec::new(),
        };

        let paired_tokens: Vec<_> = tokenization
            .tokens
            .into_iter()
            .zip(tokenization.spans)
            .collect();

        let k = (min_tokens / 2).max(MIN_K_GRAM);
        let w = k + WINDOW_OFFSET;

        let fingerprints = winnow(&paired_tokens, k, w);
        self.memo
            .insert_fingerprints(key, fingerprints.clone())
            .await;
        fingerprints
    }

    /// Retrieves or computes an AST summary for a file with automatic memoization.
    pub async fn get_or_compute_ast_summary(
        &self,
        file_path: &str,
        content: &str,
        extension: &str,
    ) -> Option<CachedAstSummary> {
        let content_hash = compute_blake3_hash(content);
        let key = QueryKey {
            file_path: file_path.to_string(),
            content_hash,
        };

        if let Some(cached) = self.memo.get_ast(&key).await {
            return Some(cached);
        }

        let tree = crate::ast::parser::parse_ast_tree(content, extension)?;
        let root = tree.root_node();
        let summary = CachedAstSummary {
            extension: extension.to_string(),
            root_kind: root.kind().to_string(),
            child_count: root.child_count(),
            content_hash,
        };

        self.memo.insert_ast(key, summary.clone()).await;
        Some(summary)
    }

    /// Retrieves or computes a unified Code Property Graph (CPG) with memoization.
    pub async fn get_or_compute_cpg(
        &self,
        file_path: &str,
        content: &str,
        language: &str,
        interner: &crate::cpg::SymbolInterner,
    ) -> Option<Arc<crate::cpg::CodePropertyGraph>> {
        let content_hash = compute_blake3_hash(content);
        let key = QueryKey {
            file_path: file_path.to_string(),
            content_hash,
        };

        if let Some(cached) = self.memo.get_cpg(&key).await {
            return Some(cached);
        }

        let cpg = crate::cpg::build_cpg_from_function(file_path, content, language, interner)?;
        let arc_cpg = Arc::new(cpg);
        self.memo.insert_cpg(key, Arc::clone(&arc_cpg)).await;
        Some(arc_cpg)
    }

    /// Checks if a file's semantic token stream changed, supporting early cutoff for comment/whitespace edits.
    pub async fn is_token_stream_unchanged(&self, file_path: &str, content: &str) -> bool {
        let tokenization = match self.get_or_compute_tokens(file_path, content).await {
            Some(t) => t,
            None => return false,
        };

        let mut hasher = blake3::Hasher::new();
        for tok in &tokenization.tokens {
            let val = crate::fingerprint::token_to_u64(tok);
            hasher.update(&val.to_le_bytes());
        }
        let stream_hash = *hasher.finalize().as_bytes();

        if let Some(prev_hash) = self.memo.get_normalized_token_hash(file_path).await
            && prev_hash == stream_hash
        {
            return true;
        }

        self.memo
            .insert_normalized_token_hash(file_path.to_string(), stream_hash)
            .await;
        false
    }

    /// Compares two snapshots of repository file hashes and computes an incremental delta report.
    pub fn compute_incremental_delta(
        &self,
        old_manifest: &HashMap<String, ContentHash>,
        new_manifest: &HashMap<String, ContentHash>,
    ) -> IncrementalDeltaReport {
        let mut report = IncrementalDeltaReport::default();

        for (path, new_hash) in new_manifest {
            match old_manifest.get(path) {
                Some(old_hash) => {
                    if old_hash == new_hash {
                        report.unmodified_files += 1;
                        report.short_circuited_count += 1;
                    } else {
                        report.modified_files += 1;
                    }
                }
                None => {
                    report.added_files += 1;
                }
            }
        }

        for path in old_manifest.keys() {
            if !new_manifest.contains_key(path) {
                report.removed_files += 1;
            }
        }

        report
    }

    /// Returns cache statistics.
    pub async fn stats(&self) -> QueryCacheStats {
        self.memo.stats().await
    }

    /// Clears the memoization cache.
    pub async fn clear(&self) {
        self.memo.clear().await;
    }
}
