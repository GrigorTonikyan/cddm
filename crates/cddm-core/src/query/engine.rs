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

    /// Incrementally updates or computes an AST summary for a modified file given an old Tree.
    pub async fn get_or_compute_ast_summary_incremental(
        &self,
        file_path: &str,
        content: &str,
        extension: &str,
        old_tree: &tree_sitter::Tree,
    ) -> Option<CachedAstSummary> {
        let content_hash = compute_blake3_hash(content);
        let key = QueryKey {
            file_path: file_path.to_string(),
            content_hash,
        };

        if let Some(cached) = self.memo.get_ast(&key).await {
            return Some(cached);
        }

        let tree = crate::ast::parser::parse_ast_tree_incremental(content, extension, old_tree)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_engine_tokens_and_fingerprints() {
        let engine = IncrementalQueryEngine::new();
        let code = "fn add(a: i32, b: i32) -> i32 { a + b }";

        let tokens = engine.get_or_compute_tokens("src/calc.rs", code).await;
        assert!(tokens.is_some());
        assert_eq!(tokens.unwrap().language, "Rust");

        let stats1 = engine.stats().await;
        assert_eq!(stats1.misses, 1);

        // Second query should hit cache
        let tokens2 = engine.get_or_compute_tokens("src/calc.rs", code).await;
        assert!(tokens2.is_some());
        let stats2 = engine.stats().await;
        assert_eq!(stats2.hits, 1);
    }

    #[tokio::test]
    async fn test_query_engine_incremental_ast_summary() {
        let engine = IncrementalQueryEngine::new();
        let code1 = "fn multiply(x: i32) -> i32 { x * 2 }";
        let tree1 = crate::ast::parser::parse_ast_tree(code1, "rs").unwrap();

        let ast1 = engine
            .get_or_compute_ast_summary("src/math.rs", code1, "rs")
            .await;
        assert!(ast1.is_some());
        assert_eq!(ast1.unwrap().root_kind, "source_file");

        let code2 = "fn multiply(x: i32, y: i32) -> i32 { x * y }";
        let ast2 = engine
            .get_or_compute_ast_summary_incremental("src/math.rs", code2, "rs", &tree1)
            .await;
        assert!(ast2.is_some());
        assert_eq!(ast2.unwrap().root_kind, "source_file");
    }

    #[test]
    fn test_compute_incremental_delta() {
        let engine = IncrementalQueryEngine::new();
        let hash_a = compute_blake3_hash("content a");
        let hash_b = compute_blake3_hash("content b");
        let hash_c = compute_blake3_hash("content c");

        let mut old_manifest = HashMap::new();
        old_manifest.insert("file1.rs".to_string(), hash_a);
        old_manifest.insert("file2.rs".to_string(), hash_b);

        let mut new_manifest = HashMap::new();
        new_manifest.insert("file1.rs".to_string(), hash_a); // unmodified
        new_manifest.insert("file2.rs".to_string(), hash_c); // modified
        new_manifest.insert("file3.rs".to_string(), hash_b); // added

        let delta = engine.compute_incremental_delta(&old_manifest, &new_manifest);
        assert_eq!(delta.unmodified_files, 1);
        assert_eq!(delta.modified_files, 1);
        assert_eq!(delta.added_files, 1);
        assert_eq!(delta.removed_files, 0);
    }
}
