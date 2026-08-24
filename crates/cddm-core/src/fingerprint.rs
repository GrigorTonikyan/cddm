use crate::types::{LineSpan, NormalizedToken};

/// Mersenne prime M_61 = 2^61 - 1 used for rolling hash modular reduction.
pub const MERSENNE_61: u128 = (1 << 61) - 1;

/// Primary rolling hash base (prime).
pub const HASH_BASE_1: u64 = 313;

/// Secondary rolling hash base for dual-base collision avoidance.
pub const HASH_BASE_2: u64 = 1000003;

/// Absolute minimum size of k-gram rolling window.
pub const MIN_K_GRAM: usize = 10;

/// Default offset added to k-gram size for window size w.
pub const WINDOW_OFFSET: usize = 5;

/// Numeric hash value representing an identifier token.
pub const TOKEN_IDENTIFIER_VAL: u64 = 1;

/// Numeric hash value representing a string literal token.
pub const TOKEN_STRING_VAL: u64 = 2;

/// Numeric hash value representing a numeric literal token.
pub const TOKEN_NUMERIC_VAL: u64 = 3;

/// Base integer offset for language keyword IDs.
pub const TOKEN_KEYWORD_OFFSET: u64 = 1000;

/// Base integer offset for punctuation IDs.
pub const TOKEN_PUNCTUATION_OFFSET: u64 = 2000;

/// A fast modulo operation for the Mersenne prime M_61 (2^61 - 1).
#[inline]
pub fn fast_mod_m61(x: u128) -> u64 {
    let mut t = (x & MERSENNE_61) + (x >> 61);
    if t >= MERSENNE_61 {
        t -= MERSENNE_61;
    }
    t as u64
}

use serde::{Deserialize, Serialize};

/// A fingerprint representing a winnowed hash and its location span.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint {
    /// Composite hash `(hash1, hash2)`
    pub hash: (u64, u64),
    /// Span of lines this fingerprint covers
    pub span: LineSpan,
}

/// Converts a token into a unique integer for hashing.
#[inline]
pub fn token_to_u64(token: &NormalizedToken) -> u64 {
    match token {
        NormalizedToken::Identifier => TOKEN_IDENTIFIER_VAL,
        NormalizedToken::StringLiteral => TOKEN_STRING_VAL,
        NormalizedToken::NumericLiteral => TOKEN_NUMERIC_VAL,
        NormalizedToken::Keyword(id) => TOKEN_KEYWORD_OFFSET + (*id as u64),
        NormalizedToken::Punctuation(id) => TOKEN_PUNCTUATION_OFFSET + (*id as u64),
    }
}

/// Computes the sequence of Winnowing fingerprints for a list of tokens.
///
/// `k`: size of the k-gram (number of tokens in a rolling window)
/// `w`: size of the winnowing window
pub fn winnow(tokens: &[(NormalizedToken, LineSpan)], k: usize, w: usize) -> Vec<Fingerprint> {
    if tokens.len() < k {
        return Vec::new();
    }

    let b1: u64 = HASH_BASE_1;
    let b2: u64 = HASH_BASE_2;

    // Precompute b1^(k-1) and b2^(k-1) modulo M_61
    let mut b1_k_minus_1: u64 = 1;
    let mut b2_k_minus_1: u64 = 1;
    for _ in 0..(k - 1) {
        b1_k_minus_1 = fast_mod_m61((b1_k_minus_1 as u128) * (b1 as u128));
        b2_k_minus_1 = fast_mod_m61((b2_k_minus_1 as u128) * (b2 as u128));
    }

    let kgram_hashes =
        crate::simd::compute_kgram_rolling_hashes(tokens, k, b1, b2, b1_k_minus_1, b2_k_minus_1);

    // Winnowing
    let mut fingerprints = Vec::new();

    if kgram_hashes.is_empty() {
        return fingerprints;
    }

    if kgram_hashes.len() < w {
        let mut min_idx = 0;
        for j in 1..kgram_hashes.len() {
            if kgram_hashes[j].0 <= kgram_hashes[min_idx].0 {
                min_idx = j;
            }
        }
        let item = &kgram_hashes[min_idx];
        fingerprints.push(Fingerprint {
            hash: item.0,
            span: LineSpan {
                line_start: item.1,
                line_end: item.2,
                byte_offset: item.3,
            },
        });
        return fingerprints;
    }

    let mut min_idx = 0;
    for i in 0..=(kgram_hashes.len() - w) {
        let window_end = i + w;
        if min_idx < i {
            min_idx = i;
            for j in (i + 1)..window_end {
                if kgram_hashes[j].0 <= kgram_hashes[min_idx].0 {
                    min_idx = j;
                }
            }
            let item = &kgram_hashes[min_idx];
            fingerprints.push(Fingerprint {
                hash: item.0,
                span: LineSpan {
                    line_start: item.1,
                    line_end: item.2,
                    byte_offset: item.3,
                },
            });
        } else {
            let new_idx = window_end - 1;
            if kgram_hashes[new_idx].0 <= kgram_hashes[min_idx].0 {
                min_idx = new_idx;
                let item = &kgram_hashes[min_idx];
                fingerprints.push(Fingerprint {
                    hash: item.0,
                    span: LineSpan {
                        line_start: item.1,
                        line_end: item.2,
                        byte_offset: item.3,
                    },
                });
            }
        }
    }

    fingerprints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_mod_m61() {
        assert_eq!(fast_mod_m61(0), 0);
        assert_eq!(fast_mod_m61(100), 100);
        let m61: u128 = (1 << 61) - 1;
        assert_eq!(fast_mod_m61(m61), 0);
        assert_eq!(fast_mod_m61(m61 + 5), 5);
    }

    fn make_test_tokens(count: usize) -> Vec<(NormalizedToken, LineSpan)> {
        (0..count)
            .map(|i| {
                (
                    NormalizedToken::Identifier,
                    LineSpan {
                        line_start: i,
                        line_end: i,
                        byte_offset: i * 10,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn test_winnowing() {
        let tokens = make_test_tokens(20);
        let fp = winnow(&tokens, 5, 4);
        assert!(!fp.is_empty());
        assert!(fp.len() <= 20 - 5 + 1);
    }

    #[test]
    fn test_winnow_too_few_tokens() {
        let tokens = make_test_tokens(1);
        let fp = winnow(&tokens, 5, 4);
        assert!(fp.is_empty());
    }

    #[test]
    fn test_winnow_deterministic() {
        let tokens = make_test_tokens(20);
        let fp1 = winnow(&tokens, 5, 4);
        let fp2 = winnow(&tokens, 5, 4);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_fast_mod_m61_large_values() {
        const M61: u64 = (1 << 61) - 1;
        let large_val = (M61 as u128) * 10 + 42;
        let result = fast_mod_m61(large_val);
        assert_eq!(result, 42);
    }
}
