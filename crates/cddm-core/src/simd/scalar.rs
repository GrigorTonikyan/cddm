use crate::fingerprint::{fast_mod_m61, token_to_u64};
use crate::types::{LineSpan, NormalizedToken};

/// Computes the initial k-gram hash for dual bases `b1` and `b2`.
#[inline]
pub fn compute_initial_kgram_hash(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
) -> (u64, u64) {
    let mut h1: u64 = 0;
    let mut h2: u64 = 0;

    for token in tokens.iter().take(k) {
        let val = token_to_u64(&token.0);
        h1 = fast_mod_m61((h1 as u128) * (b1 as u128) + (val as u128));
        h2 = fast_mod_m61((h2 as u128) * (b2 as u128) + (val as u128));
    }

    (h1, h2)
}

/// Updates a dual-base rolling hash pair with a single token roll (subtract old, multiply base, add new).
#[inline]
pub fn roll_dual_hash_step(
    current_hashes: (u64, u64),
    old_val: u64,
    new_val: u64,
    bases: (u64, u64),
    bases_k_minus_1: (u64, u64),
) -> (u64, u64) {
    const M61: u64 = (1u64 << 61) - 1;
    let (h1, h2) = current_hashes;
    let (b1, b2) = bases;
    let (b1_k_minus_1, b2_k_minus_1) = bases_k_minus_1;

    // Remove old_val * b^(k-1)
    let sub1 = fast_mod_m61((old_val as u128) * (b1_k_minus_1 as u128));
    let h1_sub = if h1 >= sub1 {
        h1 - sub1
    } else {
        h1 + M61 - sub1
    };

    let sub2 = fast_mod_m61((old_val as u128) * (b2_k_minus_1 as u128));
    let h2_sub = if h2 >= sub2 {
        h2 - sub2
    } else {
        h2 + M61 - sub2
    };

    // Multiply by b and add new_val
    let next_h1 = fast_mod_m61((h1_sub as u128) * (b1 as u128) + (new_val as u128));
    let next_h2 = fast_mod_m61((h2_sub as u128) * (b2 as u128) + (new_val as u128));

    (next_h1, next_h2)
}

/// Computes the complete series of rolling k-gram hashes sequentially with branch-minimized scalar math.
pub fn compute_kgram_rolling_hashes_scalar(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    let (mut kgram_hashes, (mut h1, mut h2)) =
        match super::init_kgram_rolling_state(tokens, k, b1, b2) {
            Some(state) => state,
            None => return Vec::new(),
        };

    let bases = (b1, b2);
    let bases_k = (b1_k_minus_1, b2_k_minus_1);

    // Rolling hash loop
    for i in k..tokens.len() {
        let old_val = token_to_u64(&tokens[i - k].0);
        let new_val = token_to_u64(&tokens[i].0);

        let (next_h1, next_h2) = roll_dual_hash_step((h1, h2), old_val, new_val, bases, bases_k);

        h1 = next_h1;
        h2 = next_h2;

        kgram_hashes.push((
            (h1, h2),
            tokens[i - k + 1].1.line_start,
            tokens[i].1.line_end,
            tokens[i - k + 1].1.byte_offset,
        ));
    }

    kgram_hashes
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn sample_tokens(n: usize) -> Vec<(NormalizedToken, LineSpan)> {
        (0..n)
            .map(|i| {
                let tok = match i % 5 {
                    0 => NormalizedToken::Identifier,
                    1 => NormalizedToken::Keyword((i % 20) as u16),
                    2 => NormalizedToken::StringLiteral,
                    3 => NormalizedToken::NumericLiteral,
                    _ => NormalizedToken::Punctuation((i % 10) as u8),
                };
                (
                    tok,
                    LineSpan {
                        line_start: i + 1,
                        line_end: i + 1,
                        byte_offset: i * 8,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn test_scalar_kgram_hashes() {
        let tokens = sample_tokens(25);
        let hashes =
            compute_kgram_rolling_hashes_scalar(&tokens, 5, 313, 1000003, 9600980005, 12345);
        assert_eq!(hashes.len(), 21);
        assert_eq!(hashes[0].1, 1);
        assert_eq!(hashes[0].2, 5);
        assert_eq!(hashes[20].1, 21);
        assert_eq!(hashes[20].2, 25);
    }
}
