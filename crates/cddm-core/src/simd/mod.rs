//! SIMD-accelerated rolling hash and modular reduction engines for Mersenne-61 Winnowing.

pub mod avx2;
pub mod neon;
pub mod scalar;

pub use avx2::compute_kgram_rolling_hashes_avx2;
pub use neon::compute_kgram_rolling_hashes_neon;
pub use scalar::compute_kgram_rolling_hashes_scalar;

use crate::types::{LineSpan, NormalizedToken};

/// Automatically dispatches rolling hash computation to the fastest available hardware vector engine.
///
/// Priority order:
/// 1. AVX2 (x86_64 when CPU support is detected at runtime)
/// 2. ARM NEON (AArch64)
/// 3. Optimized branch-minimized scalar engine (portable fallback)
#[inline]
pub fn compute_kgram_rolling_hashes(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    #[cfg(target_arch = "x86_64")]
    {
        compute_kgram_rolling_hashes_avx2(tokens, k, b1, b2, b1_k_minus_1, b2_k_minus_1)
    }

    #[cfg(target_arch = "aarch64")]
    {
        compute_kgram_rolling_hashes_neon(tokens, k, b1, b2, b1_k_minus_1, b2_k_minus_1)
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        compute_kgram_rolling_hashes_scalar(tokens, k, b1, b2, b1_k_minus_1, b2_k_minus_1)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_compute_kgram_rolling_hashes_dispatch() {
        let tokens: Vec<(NormalizedToken, LineSpan)> = (0..30)
            .map(|i| {
                (
                    NormalizedToken::Identifier,
                    LineSpan {
                        line_start: i + 1,
                        line_end: i + 1,
                        byte_offset: i * 6,
                    },
                )
            })
            .collect();

        let res = compute_kgram_rolling_hashes(&tokens, 5, 313, 1000003, 54321, 98765);
        assert_eq!(res.len(), 26);
    }
}
