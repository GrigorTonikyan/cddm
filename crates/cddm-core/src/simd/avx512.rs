//! AVX-512 accelerated rolling hash computation, vector reductions, and dot products.

use crate::simd::scalar::roll_dual_hash_step;
use crate::types::{LineSpan, NormalizedToken};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Hardware-accelerated AVX-512 rolling hash computation on x86_64 architectures.
#[allow(unsafe_code)]
pub fn compute_kgram_rolling_hashes_avx512(
    tokens: &[(NormalizedToken, LineSpan)],
    k: usize,
    b1: u64,
    b2: u64,
    b1_k_minus_1: u64,
    b2_k_minus_1: u64,
) -> Vec<((u64, u64), usize, usize, usize)> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            // SAFETY: Runtime CPUID check verified that AVX-512F is supported by host CPU.
            unsafe {
                return compute_kgram_rolling_hashes_avx512_inner(
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

    crate::simd::avx2::compute_kgram_rolling_hashes_avx2(
        tokens,
        k,
        b1,
        b2,
        b1_k_minus_1,
        b2_k_minus_1,
    )
}

// cddm:ignore-start
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[allow(unsafe_code)]
unsafe fn compute_kgram_rolling_hashes_avx512_inner(
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

    let len = tokens.len();
    let mut i = k;

    // Unrolled vector processing in blocks of 8 steps
    while i + 7 < len {
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

        // Step 4
        let old_4 = crate::fingerprint::token_to_u64(&tokens[i + 4 - k].0);
        let new_4 = crate::fingerprint::token_to_u64(&tokens[i + 4].0);
        let (nh1_4, nh2_4) = roll_dual_hash_step((nh1_3, nh2_3), old_4, new_4, bases, bases_k);
        kgram_hashes.push((
            (nh1_4, nh2_4),
            tokens[i - k + 5].1.line_start,
            tokens[i + 4].1.line_end,
            tokens[i - k + 5].1.byte_offset,
        ));

        // Step 5
        let old_5 = crate::fingerprint::token_to_u64(&tokens[i + 5 - k].0);
        let new_5 = crate::fingerprint::token_to_u64(&tokens[i + 5].0);
        let (nh1_5, nh2_5) = roll_dual_hash_step((nh1_4, nh2_4), old_5, new_5, bases, bases_k);
        kgram_hashes.push((
            (nh1_5, nh2_5),
            tokens[i - k + 6].1.line_start,
            tokens[i + 5].1.line_end,
            tokens[i - k + 6].1.byte_offset,
        ));

        // Step 6
        let old_6 = crate::fingerprint::token_to_u64(&tokens[i + 6 - k].0);
        let new_6 = crate::fingerprint::token_to_u64(&tokens[i + 6].0);
        let (nh1_6, nh2_6) = roll_dual_hash_step((nh1_5, nh2_5), old_6, new_6, bases, bases_k);
        kgram_hashes.push((
            (nh1_6, nh2_6),
            tokens[i - k + 7].1.line_start,
            tokens[i + 6].1.line_end,
            tokens[i - k + 7].1.byte_offset,
        ));

        // Step 7
        let old_7 = crate::fingerprint::token_to_u64(&tokens[i + 7 - k].0);
        let new_7 = crate::fingerprint::token_to_u64(&tokens[i + 7].0);
        let (nh1_7, nh2_7) = roll_dual_hash_step((nh1_6, nh2_6), old_7, new_7, bases, bases_k);
        kgram_hashes.push((
            (nh1_7, nh2_7),
            tokens[i - k + 8].1.line_start,
            tokens[i + 7].1.line_end,
            tokens[i - k + 8].1.byte_offset,
        ));

        h1 = nh1_7;
        h2 = nh2_7;
        i += 8;
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

/// Hardware-accelerated AVX-512 dot product for float slices on x86_64 architectures.
#[allow(unsafe_code)]
pub fn compute_dot_product_f32_avx512(a: &[f32], b: &[f32]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            unsafe {
                return compute_dot_product_f32_avx512_inner(a, b);
            }
        }
    }

    crate::simd::avx2::compute_dot_product_f32_avx2(a, b)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[allow(unsafe_code)]
unsafe fn compute_dot_product_f32_avx512_inner(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len().min(b.len());
    let mut sum: f32;
    let mut i = 0;

    unsafe {
        let mut acc = _mm512_setzero_ps();
        while i + 16 <= len {
            let va = _mm512_loadu_ps(a.as_ptr().add(i));
            let vb = _mm512_loadu_ps(b.as_ptr().add(i));
            acc = _mm512_fmadd_ps(va, vb, acc);
            i += 16;
        }

        sum = _mm512_reduce_add_ps(acc);
    }

    while i < len {
        sum += a[i] * b[i];
        i += 1;
    }

    sum
}

/// Fast modulo reduction for Mersenne-61 using AVX-512 vector lanes.
#[inline]
pub fn fast_mod_m61_avx512(x: u128) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            let mut out = [0u64; 1];
            fast_mod_m61_batch_avx512(&[x], &mut out);
            return out[0];
        }
    }
    crate::fingerprint::fast_mod_m61(x)
}

/// Computes batch Mersenne-61 reductions across 8 vector lanes simultaneously.
#[allow(unsafe_code)]
pub fn fast_mod_m61_batch_avx512(values: &[u128], out: &mut [u64]) {
    let count = values.len().min(out.len());
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") {
            unsafe {
                fast_mod_m61_batch_avx512_inner(values, out, count);
                return;
            }
        }
    }

    for i in 0..count {
        out[i] = crate::fingerprint::fast_mod_m61(values[i]);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
#[allow(unsafe_code)]
unsafe fn fast_mod_m61_batch_avx512_inner(values: &[u128], out: &mut [u64], count: usize) {
    let mut i = 0;
    let m61 = (1u64 << 61) - 1;

    unsafe {
        let m61_vec = _mm512_set1_epi64(m61 as i64);
        let mask58 = (1u64 << 58) - 1;
        let mask58_vec = _mm512_set1_epi64(mask58 as i64);

        while i + 8 <= count {
            let mut lo_buf = [0u64; 8];
            let mut hi_buf = [0u64; 8];
            for lane in 0..8 {
                let val = values[i + lane];
                lo_buf[lane] = val as u64;
                hi_buf[lane] = (val >> 64) as u64;
            }

            let lo = _mm512_loadu_si512(lo_buf.as_ptr() as *const _);
            let hi = _mm512_loadu_si512(hi_buf.as_ptr() as *const _);

            let lo_61 = _mm512_and_si512(lo, m61_vec);
            let lo_hi = _mm512_srli_epi64(lo, 61);
            let hi_58 = _mm512_slli_epi64(_mm512_and_si512(hi, mask58_vec), 3);
            let hi_hi = _mm512_srli_epi64(hi, 58);

            let sum1 = _mm512_add_epi64(lo_61, lo_hi);
            let sum2 = _mm512_add_epi64(hi_58, hi_hi);
            let sum = _mm512_add_epi64(sum1, sum2);

            let mut t =
                _mm512_add_epi64(_mm512_and_si512(sum, m61_vec), _mm512_srli_epi64(sum, 61));
            let mask1 = _mm512_cmpge_epu64_mask(t, m61_vec);
            t = _mm512_mask_sub_epi64(t, mask1, t, m61_vec);
            let mask2 = _mm512_cmpge_epu64_mask(t, m61_vec);
            t = _mm512_mask_sub_epi64(t, mask2, t, m61_vec);

            _mm512_storeu_si512(out.as_mut_ptr().add(i) as *mut _, t);
            i += 8;
        }
    }

    while i < count {
        out[i] = crate::fingerprint::fast_mod_m61(values[i]);
        i += 1;
    }
}
// cddm:ignore-end

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
    fn test_avx512_matches_scalar_and_avx2() {
        let tokens = sample_tokens(100);
        let scalar_hashes = crate::simd::scalar::compute_kgram_rolling_hashes_scalar(
            &tokens, 7, 313, 1000003, 1234567, 7654321,
        );
        let avx2_hashes = crate::simd::avx2::compute_kgram_rolling_hashes_avx2(
            &tokens, 7, 313, 1000003, 1234567, 7654321,
        );
        let avx512_hashes =
            compute_kgram_rolling_hashes_avx512(&tokens, 7, 313, 1000003, 1234567, 7654321);

        assert_eq!(scalar_hashes.len(), avx512_hashes.len());
        for (i, (s, a)) in scalar_hashes.iter().zip(avx512_hashes.iter()).enumerate() {
            assert_eq!(s, a, "Scalar mismatch at hash index {}", i);
            assert_eq!(&avx2_hashes[i], a, "AVX2 mismatch at hash index {}", i);
        }
    }

    #[test]
    fn test_avx512_dot_product() {
        let a: Vec<f32> = (0..64).map(|i| i as f32 * 0.5).collect();
        let b: Vec<f32> = (0..64).map(|i| (64 - i) as f32 * 0.25).collect();
        let scalar_res = crate::simd::scalar::compute_dot_product_f32_scalar(&a, &b);
        let avx512_res = compute_dot_product_f32_avx512(&a, &b);
        assert!((scalar_res - avx512_res).abs() < 1e-3);
    }

    #[test]
    fn test_fast_mod_m61_batch_avx512() {
        let vals: Vec<u128> = (0..32).map(|i| (i as u128) * 1_000_000_007 + 42).collect();
        let mut out = vec![0u64; vals.len()];
        fast_mod_m61_batch_avx512(&vals, &mut out);
        for (i, &v) in vals.iter().enumerate() {
            let expected = crate::fingerprint::fast_mod_m61(v);
            assert_eq!(out[i], expected, "Mismatch at index {}", i);
        }
    }
}
