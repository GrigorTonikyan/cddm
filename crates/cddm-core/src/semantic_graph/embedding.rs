#![forbid(unsafe_code)]

use super::isomorphism::calculate_graph_similarity;
use super::types::{ControlFlowGraph, HybridSimilarity};
use std::collections::HashMap;

/// Normalizes code source by stripping syntactic keywords, delimiters, and extracting semantic operation tokens.
pub fn extract_semantic_tokens(code: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        // Skip comment lines
        if trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
        {
            continue;
        }

        let mut current_word = String::new();
        let chars: Vec<char> = trimmed.chars().collect();

        for &ch in &chars {
            if ch.is_alphanumeric() || ch == '_' {
                // Check for camelCase split: lowercase followed by uppercase
                if ch.is_uppercase()
                    && !current_word.is_empty()
                    && current_word
                        .chars()
                        .last()
                        .map(|c| c.is_lowercase())
                        .unwrap_or(false)
                {
                    process_word_token(&current_word, &mut tokens);
                    current_word.clear();
                }

                if ch == '_' {
                    if !current_word.is_empty() {
                        process_word_token(&current_word, &mut tokens);
                        current_word.clear();
                    }
                } else {
                    current_word.push(ch.to_ascii_lowercase());
                }
            } else {
                if !current_word.is_empty() {
                    process_word_token(&current_word, &mut tokens);
                    current_word.clear();
                }
                match ch {
                    '+' => tokens.push("op_add".to_string()),
                    '-' => tokens.push("op_sub".to_string()),
                    '*' => tokens.push("op_mul".to_string()),
                    '/' => tokens.push("op_div".to_string()),
                    '%' => tokens.push("op_mod".to_string()),
                    '>' => tokens.push("op_gt".to_string()),
                    '<' => tokens.push("op_lt".to_string()),
                    '=' => tokens.push("op_assign".to_string()),
                    '!' => tokens.push("op_not".to_string()),
                    '&' => tokens.push("op_and".to_string()),
                    '|' => tokens.push("op_or".to_string()),
                    _ => {}
                }
            }
        }
        if !current_word.is_empty() {
            process_word_token(&current_word, &mut tokens);
        }
    }
    tokens
}

fn process_word_token(word: &str, tokens: &mut Vec<String>) {
    if word.is_empty() {
        return;
    }
    if word.chars().all(|c| c.is_ascii_digit()) {
        tokens.push("lit_num".to_string());
    } else {
        let canonical = canonicalize_token(word);
        if !canonical.is_empty() {
            tokens.push(canonical.to_string());
        }
    }
}

fn canonicalize_token(word: &str) -> &str {
    match word {
        // Function declarations
        "fn" | "def" | "defp" | "function" | "func" | "fun" | "lambda" => "def_fn",
        // Variable declarations
        "let" | "var" | "const" | "val" | "auto" | "mut" => "decl_var",
        // Control flow
        "if" => "ctrl_if",
        "else" | "elif" | "elsif" => "ctrl_else",
        "for" | "while" | "loop" | "do" | "each" | "repeat" => "ctrl_loop",
        "return" | "yield" => "ctrl_return",
        "break" => "ctrl_break",
        "continue" => "ctrl_continue",
        "match" | "switch" | "case" | "when" | "select" => "ctrl_branch",
        "try" | "catch" | "except" | "finally" | "rescue" | "throw" | "raise" => "ctrl_except",
        // IO
        "println" | "print" | "console" | "log" | "fmt" | "printf" | "puts" | "echo" => "io_print",
        // Literals
        "true" | "false" => "lit_bool",
        "null" | "none" | "nil" | "undefined" | "nullptr" => "lit_null",
        "self" | "this" => "ref_self",
        // Primitive types normalization for cross-language parity
        "int" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64"
        | "u128" | "usize" | "uint" | "long" | "short" | "byte" | "bigint" => "type_int",
        "float" | "double" | "f32" | "f64" | "number" | "decimal" => "type_num",
        "bool" | "boolean" => "type_bool",
        "str" | "string" | "char" | "text" => "type_str",
        "void" | "unit" | "never" => "type_void",
        "vec" | "vector" | "list" | "array" | "slice" => "type_list",
        "map" | "dict" | "dictionary" | "hashmap" | "table" => "type_map",
        // Common syntax noise to skip
        "in" | "of" | "to" | "as" | "is" | "use" | "import" | "from" | "package" | "crate"
        | "namespace" | "module" | "require" | "include" | "public" | "private" | "protected"
        | "static" | "async" | "await" | "export" | "override" | "virtual" | "final" => "",
        // Preserve other domain identifiers
        other => other,
    }
}

fn for_each_subword_3gram(tok: &str, mut cb: impl FnMut(&str)) {
    if tok.len() >= 3
        && !tok.starts_with("op_")
        && !tok.starts_with("ctrl_")
        && !tok.starts_with("decl_")
        && !tok.starts_with("def_")
    {
        let chars: Vec<char> = tok.chars().collect();
        for w in chars.windows(3) {
            let gram: String = w.iter().collect();
            cb(&gram);
        }
    }
}

/// Generates a normalized term-frequency sparse vector from a token list.
pub fn compute_tf_vector(tokens: &[String]) -> HashMap<String, f64> {
    let mut counts: HashMap<String, f64> = HashMap::new();
    if tokens.is_empty() {
        return counts;
    }

    for tok in tokens {
        *counts.entry(tok.clone()).or_insert(0.0) += 1.0;
    }

    // Also add subword character n-grams (3-grams) for identifier similarity with moderate weighting
    for tok in tokens {
        for_each_subword_3gram(tok, |gram| {
            *counts.entry(format!("ng_{}", gram)).or_insert(0.0) += 0.2;
        });
    }

    // Normalize L2 norm
    let sum_sq: f64 = counts.values().map(|v| v * v).sum();
    let norm = sum_sq.sqrt();
    if norm > 0.0 {
        for v in counts.values_mut() {
            *v /= norm;
        }
    }

    counts
}

/// Computes cosine similarity between two sparse vector representations (0.0 to 1.0).
pub fn cosine_similarity(v1: &HashMap<String, f64>, v2: &HashMap<String, f64>) -> f64 {
    if v1.is_empty() || v2.is_empty() {
        return 0.0;
    }

    let (smaller, larger) = if v1.len() < v2.len() {
        (v1, v2)
    } else {
        (v2, v1)
    };

    let dot_product: f64 = smaller
        .iter()
        .filter_map(|(k, val1)| larger.get(k).map(|val2| val1 * val2))
        .sum();

    dot_product.clamp(0.0, 1.0)
}

/// Calculates token embedding cosine similarity between two code snippets.
pub fn calculate_embedding_similarity(code_a: &str, code_b: &str) -> f64 {
    let tokens_a = extract_semantic_tokens(code_a);
    let tokens_b = extract_semantic_tokens(code_b);

    let v1 = compute_tf_vector(&tokens_a);
    let v2 = compute_tf_vector(&tokens_b);

    cosine_similarity(&v1, &v2)
}

fn build_hybrid_similarity(
    graph_similarity: f64,
    token_similarity: f64,
    is_cross_language: bool,
) -> HybridSimilarity {
    let (graph_weight, token_weight) = if is_cross_language {
        (0.60, 0.40)
    } else {
        (0.50, 0.50)
    };

    let hybrid_score =
        (graph_weight * graph_similarity + token_weight * token_similarity).clamp(0.0, 1.0);

    HybridSimilarity {
        graph_similarity: (graph_similarity * 1000.0).round() / 1000.0,
        token_similarity: (token_similarity * 1000.0).round() / 1000.0,
        hybrid_score: (hybrid_score * 1000.0).round() / 1000.0,
        is_cross_language,
    }
}

/// Computes unified hybrid similarity combining graph structural isomorphism and semantic token vector similarity.
pub fn compute_hybrid_similarity(
    cfg_a: &ControlFlowGraph,
    code_a: &str,
    cfg_b: &ControlFlowGraph,
    code_b: &str,
    is_cross_language: bool,
) -> HybridSimilarity {
    let graph_similarity = calculate_graph_similarity(cfg_a, cfg_b);
    let token_similarity = calculate_embedding_similarity(code_a, code_b);
    build_hybrid_similarity(graph_similarity, token_similarity, is_cross_language)
}

/// Compact, sorted sparse vector representation for fast linear two-pointer dot products.
#[derive(Clone, Debug, PartialEq)]
pub struct SparseTfVector {
    /// Sorted list of `(term_hash, weight)` tuples.
    pub entries: Vec<(u64, f64)>,
}

#[inline]
pub fn hash_token_str_fnv1a(s: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in s.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3u64);
    }
    hash
}

impl SparseTfVector {
    pub fn from_tokens(tokens: &[String]) -> Self {
        let mut map: HashMap<u64, f64> = HashMap::new();
        if tokens.is_empty() {
            return Self {
                entries: Vec::new(),
            };
        }

        for tok in tokens {
            let h = hash_token_str_fnv1a(tok);
            *map.entry(h).or_insert(0.0) += 1.0;
        }

        for tok in tokens {
            for_each_subword_3gram(tok, |gram| {
                let h = hash_token_str_fnv1a(&format!("ng_{}", gram));
                *map.entry(h).or_insert(0.0) += 0.2;
            });
        }

        let sum_sq: f64 = map.values().map(|v| v * v).sum();
        let norm = sum_sq.sqrt();
        let mut entries: Vec<(u64, f64)> = if norm > 0.0 {
            map.into_iter().map(|(k, v)| (k, v / norm)).collect()
        } else {
            map.into_iter().collect()
        };

        entries.sort_unstable_by_key(|e| e.0);
        Self { entries }
    }

    #[inline]
    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        if self.entries.is_empty() || other.entries.is_empty() {
            return 0.0;
        }

        let mut i = 0;
        let mut j = 0;
        let mut dot = 0.0f64;

        let a = &self.entries;
        let b = &other.entries;

        while i < a.len() && j < b.len() {
            let (ha, va) = a[i];
            let (hb, vb) = b[j];

            if ha == hb {
                dot += va * vb;
                i += 1;
                j += 1;
            } else if ha < hb {
                i += 1;
            } else {
                j += 1;
            }
        }

        dot.clamp(0.0, 1.0)
    }
}

/// Computes unified hybrid similarity combining graph structural isomorphism and pre-computed sparse TF vectors.
pub fn compute_hybrid_similarity_with_tf(
    cfg_a: &ControlFlowGraph,
    tf_a: &SparseTfVector,
    cfg_b: &ControlFlowGraph,
    tf_b: &SparseTfVector,
    is_cross_language: bool,
) -> HybridSimilarity {
    let graph_similarity = calculate_graph_similarity(cfg_a, cfg_b);
    let token_similarity = tf_a.cosine_similarity(tf_b);
    build_hybrid_similarity(graph_similarity, token_similarity, is_cross_language)
}
