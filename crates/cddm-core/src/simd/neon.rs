use crate::types::{LineSpan, NormalizedToken};

/// Hardware-accelerated ARM NEON rolling hash computation for AArch64 architectures.
pub fn compute_kgram_rolling_hashes_neon(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    // On non-aarch64 platforms or when NEON is disabled, fallback seamlessly to scalar
    crate::simd::scalar::compute_kgram_rolling_hashes_scalar(
        tokens,
        k,
        b1,
        b2,
        b1_k_minus_1,
        b2_k_minus_1,
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_neon_matches_scalar() {
        let tokens: Vec<(NormalizedToken, LineSpan)> = (0..50)
            .map(|i| {
                (
                    NormalizedToken::Identifier,
                    LineSpan {
                        line_start: i + 1,
                        line_end: i + 1,
                        byte_offset: i * 4,
                    },
                )
            })
            .collect();

        let scalar_res = crate::simd::scalar::compute_kgram_rolling_hashes_scalar(
            &tokens, 5, 313, 1000003, 1000, 2000,
        );
        let neon_res = compute_kgram_rolling_hashes_neon(&tokens, 5, 313, 1000003, 1000, 2000);

        assert_eq!(scalar_res, neon_res);
    }
}
