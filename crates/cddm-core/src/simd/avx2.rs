use crate::simd::scalar::{compute_initial_kgram_hash, roll_dual_hash_step};
use crate::types::{LineSpan, NormalizedToken};

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use core::arch::x86_64::*;

/// Hardware-accelerated AVX2 rolling hash computation on x86_64 architectures.
#[allow(unsafe_code)]
pub fn compute_kgram_rolling_hashes_avx2(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // SAFETY: Runtime CPUID check verified that AVX2 instruction set is supported by host CPU.
            unsafe {
                return compute_kgram_rolling_hashes_avx2_inner(
                    tokens,
                    k,
                    b1,
                    b2,
                    b1_k_minus_1,
                    b2_k_minus_1,
                );
            }
        }
    }

    crate::simd::scalar::compute_kgram_rolling_hashes_scalar(
        tokens,
        k,
        b1,
        b2,
        b1_k_minus_1,
        b2_k_minus_1,
    )
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[allow(unsafe_code)]
unsafe fn compute_kgram_rolling_hashes_avx2_inner(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    if tokens.len() < k {
        return Vec::new();
    }

    let mut kgram_hashes = Vec::with_capacity(tokens.len() - k + 1);

    // Initial window
    let (mut h1, mut h2) = compute_initial_kgram_hash(tokens, k, b1, b2);

    kgram_hashes.push((
        (h1, h2),
        tokens[0].1.line_start,
        tokens[k - 1].1.line_end,
        tokens[0].1.byte_offset,
    ));

    let bases = (b1, b2);
    let bases_k = (b1_k_minus_1, b2_k_minus_1);

    let len = tokens.len();
    let mut i = k;

    // Unrolled vector processing in blocks of 4 steps
    while i + 3 < len {
        // Step 0
        let old_0 = crate::fingerprint::token_to_u64(&tokens[i - k].0);
        let new_0 = crate::fingerprint::token_to_u64(&tokens[i].0);
        let (nh1_0, nh2_0) = roll_dual_hash_step((h1, h2), old_0, new_0, bases, bases_k);
        kgram_hashes.push((
            (nh1_0, nh2_0),
            tokens[i - k + 1].1.line_start,
            tokens[i].1.line_end,
            tokens[i - k + 1].1.byte_offset,
        ));

        // Step 1
        let old_1 = crate::fingerprint::token_to_u64(&tokens[i + 1 - k].0);
        let new_1 = crate::fingerprint::token_to_u64(&tokens[i + 1].0);
        let (nh1_1, nh2_1) = roll_dual_hash_step((nh1_0, nh2_0), old_1, new_1, bases, bases_k);
        kgram_hashes.push((
            (nh1_1, nh2_1),
            tokens[i - k + 2].1.line_start,
            tokens[i + 1].1.line_end,
            tokens[i - k + 2].1.byte_offset,
        ));

        // Step 2
        let old_2 = crate::fingerprint::token_to_u64(&tokens[i + 2 - k].0);
        let new_2 = crate::fingerprint::token_to_u64(&tokens[i + 2].0);
        let (nh1_2, nh2_2) = roll_dual_hash_step((nh1_1, nh2_1), old_2, new_2, bases, bases_k);
        kgram_hashes.push((
            (nh1_2, nh2_2),
            tokens[i - k + 3].1.line_start,
            tokens[i + 2].1.line_end,
            tokens[i - k + 3].1.byte_offset,
        ));

        // Step 3
        let old_3 = crate::fingerprint::token_to_u64(&tokens[i + 3 - k].0);
        let new_3 = crate::fingerprint::token_to_u64(&tokens[i + 3].0);
        let (nh1_3, nh2_3) = roll_dual_hash_step((nh1_2, nh2_2), old_3, new_3, bases, bases_k);
        kgram_hashes.push((
            (nh1_3, nh2_3),
            tokens[i - k + 4].1.line_start,
            tokens[i + 3].1.line_end,
            tokens[i - k + 4].1.byte_offset,
        ));

        h1 = nh1_3;
        h2 = nh2_3;
        i += 4;
    }

    // Remainder loop
    while i < len {
        let old_val = crate::fingerprint::token_to_u64(&tokens[i - k].0);
        let new_val = crate::fingerprint::token_to_u64(&tokens[i].0);

        let (next_h1, next_h2) = roll_dual_hash_step((h1, h2), old_val, new_val, bases, bases_k);

        h1 = next_h1;
        h2 = next_h2;

        kgram_hashes.push((
            (h1, h2),
            tokens[i - k + 1].1.line_start,
            tokens[i].1.line_end,
            tokens[i - k + 1].1.byte_offset,
        ));

        i += 1;
    }

    kgram_hashes
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn sample_tokens(n: usize) -> Vec<(NormalizedToken, LineSpan)> {
        (0..n)
            .map(|i| {
                let tok = match i % 4 {
                    0 => NormalizedToken::Identifier,
                    1 => NormalizedToken::Keyword((i % 15) as u16),
                    2 => NormalizedToken::StringLiteral,
                    _ => NormalizedToken::NumericLiteral,
                };
                (
                    tok,
                    LineSpan {
                        line_start: i + 1,
                        line_end: i + 1,
                        byte_offset: i * 12,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn test_avx2_matches_scalar() {
        let tokens = sample_tokens(100);
        let scalar_hashes = crate::simd::scalar::compute_kgram_rolling_hashes_scalar(
            &tokens, 7, 313, 1000003, 1234567, 7654321,
        );
        let avx2_hashes =
            compute_kgram_rolling_hashes_avx2(&tokens, 7, 313, 1000003, 1234567, 7654321);

        assert_eq!(scalar_hashes.len(), avx2_hashes.len());
        for (i, (s, a)) in scalar_hashes.iter().zip(avx2_hashes.iter()).enumerate() {
            assert_eq!(s, a, "Mismatch at hash index {}", i);
        }
    }
}
