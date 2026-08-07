use crate::types::{LineSpan, NormalizedToken};

/// A fast modulo operation for the Mersenne prime M_61 (2^61 - 1).
#[inline]
pub fn fast_mod_m61(x: u128) -> u64 {
    const M61: u128 = (1 << 61) - 1;
    let mut t = (x & M61) + (x >> 61);
    if t >= M61 {
        t -= M61;
    }
    t as u64
}

/// A fingerprint representing a winnowed hash and its location span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint {
    /// Composite hash `(hash1, hash2)`
    pub hash: (u64, u64),
    /// Span of lines this fingerprint covers
    pub span: LineSpan,
}

/// Converts a token into a unique integer for hashing.
fn token_to_u64(token: &NormalizedToken) -> u64 {
    match token {
        NormalizedToken::Identifier => 1,
        NormalizedToken::StringLiteral => 2,
        NormalizedToken::NumericLiteral => 3,
        NormalizedToken::Keyword(id) => 1000 + (*id as u64),
        NormalizedToken::Punctuation(id) => 2000 + (*id as u64),
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

    let b1: u64 = 313;
    let b2: u64 = 1000003;
    
    // Precompute b1^(k-1) and b2^(k-1) modulo M_61
    let mut b1_k_minus_1: u64 = 1;
    let mut b2_k_minus_1: u64 = 1;
    for _ in 0..(k - 1) {
        b1_k_minus_1 = fast_mod_m61((b1_k_minus_1 as u128) * (b1 as u128));
        b2_k_minus_1 = fast_mod_m61((b2_k_minus_1 as u128) * (b2 as u128));
    }

    let mut kgram_hashes = Vec::with_capacity(tokens.len() - k + 1);
    let mut h1: u64 = 0;
    let mut h2: u64 = 0;

    // Initial window
    for i in 0..k {
        let val = token_to_u64(&tokens[i].0);
        h1 = fast_mod_m61((h1 as u128) * (b1 as u128) + (val as u128));
        h2 = fast_mod_m61((h2 as u128) * (b2 as u128) + (val as u128));
    }
    
    kgram_hashes.push((
        (h1, h2),
        tokens[0].1.line_start,
        tokens[k - 1].1.line_end,
        tokens[0].1.byte_offset,
    ));

    // Rolling hash for the rest
    for i in k..tokens.len() {
        let old_val = token_to_u64(&tokens[i - k].0);
        let new_val = token_to_u64(&tokens[i].0);

        // Remove old_val * b^(k-1)
        let sub1 = fast_mod_m61((old_val as u128) * (b1_k_minus_1 as u128));
        h1 = if h1 >= sub1 { h1 - sub1 } else { h1 + ((1u64 << 61) - 1) - sub1 };
        
        let sub2 = fast_mod_m61((old_val as u128) * (b2_k_minus_1 as u128));
        h2 = if h2 >= sub2 { h2 - sub2 } else { h2 + ((1u64 << 61) - 1) - sub2 };

        // Multiply by b and add new_val
        h1 = fast_mod_m61((h1 as u128) * (b1 as u128) + (new_val as u128));
        h2 = fast_mod_m61((h2 as u128) * (b2 as u128) + (new_val as u128));

        kgram_hashes.push((
            (h1, h2),
            tokens[i - k + 1].1.line_start,
            tokens[i].1.line_end,
            tokens[i - k + 1].1.byte_offset,
        ));
    }

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

    #[test]
    fn test_winnowing() {
        let mut tokens = Vec::new();
        for i in 0..20 {
            tokens.push((
                NormalizedToken::Identifier,
                LineSpan {
                    line_start: i,
                    line_end: i,
                    byte_offset: i * 10,
                },
            ));
        }
        
        let fp = winnow(&tokens, 5, 4);
        assert!(!fp.is_empty());
        assert!(fp.len() <= 20 - 5 + 1);
    }

    #[test]
    fn test_winnow_too_few_tokens() {
        let tokens = vec![
            (NormalizedToken::Identifier, LineSpan { line_start: 1, line_end: 1, byte_offset: 0 })
        ];
        let fp = winnow(&tokens, 5, 4);
        assert!(fp.is_empty());
    }

    #[test]
    fn test_winnow_deterministic() {
        let mut tokens = Vec::new();
        for i in 0..20 {
            tokens.push((
                NormalizedToken::Identifier,
                LineSpan {
                    line_start: i,
                    line_end: i,
                    byte_offset: i * 10,
                },
            ));
        }
        
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
