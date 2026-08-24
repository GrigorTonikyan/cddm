use cddm_core::fingerprint::{fast_mod_m61, winnow};
use cddm_core::io::read_file_source;
use cddm_core::simd::scalar::compute_kgram_rolling_hashes_scalar;
use cddm_core::simd::{compute_kgram_rolling_hashes, compute_kgram_rolling_hashes_avx2};
use cddm_core::types::{LineSpan, NormalizedToken};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::io::Write;
use tempfile::NamedTempFile;

fn generate_synthetic_tokens(count: usize) -> Vec<(NormalizedToken, LineSpan)> {
    (0..count)
        .map(|i| {
            let tok = match i % 6 {
                0 => NormalizedToken::Identifier,
                1 => NormalizedToken::Keyword((i % 25) as u16),
                2 => NormalizedToken::StringLiteral,
                3 => NormalizedToken::NumericLiteral,
                4 => NormalizedToken::Punctuation((i % 15) as u8),
                _ => NormalizedToken::Identifier,
            };
            (
                tok,
                LineSpan {
                    line_start: (i / 10) + 1,
                    line_end: (i / 10) + 1,
                    byte_offset: i * 6,
                },
            )
        })
        .collect()
}

fn bench_fast_mod_m61(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_mod_m61");
    let test_vals: Vec<u128> = (0..10_000)
        .map(|i| (i as u128) * 1_000_000_007 + 42)
        .collect();

    group.bench_function("10k_reductions", |b| {
        b.iter(|| {
            for &val in &test_vals {
                black_box(fast_mod_m61(black_box(val)));
            }
        });
    });

    group.finish();
}

fn bench_rolling_hash(c: &mut Criterion) {
    let mut group = c.benchmark_group("rolling_hash_engines");
    let sizes = [1_000, 10_000, 50_000];

    for &size in &sizes {
        let tokens = generate_synthetic_tokens(size);
        let k = 25;
        let b1 = 313;
        let b2 = 1000003;
        let b1_k = 987654321;
        let b2_k = 123456789;

        group.bench_with_input(BenchmarkId::new("scalar", size), &tokens, |b, toks| {
            b.iter(|| {
                compute_kgram_rolling_hashes_scalar(
                    black_box(toks),
                    black_box(k),
                    b1,
                    b2,
                    b1_k,
                    b2_k,
                )
            });
        });

        group.bench_with_input(
            BenchmarkId::new("avx2_or_vector", size),
            &tokens,
            |b, toks| {
                b.iter(|| {
                    compute_kgram_rolling_hashes_avx2(
                        black_box(toks),
                        black_box(k),
                        b1,
                        b2,
                        b1_k,
                        b2_k,
                    )
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("auto_dispatch", size),
            &tokens,
            |b, toks| {
                b.iter(|| {
                    compute_kgram_rolling_hashes(black_box(toks), black_box(k), b1, b2, b1_k, b2_k)
                });
            },
        );
    }

    group.finish();
}

fn bench_winnowing(c: &mut Criterion) {
    let mut group = c.benchmark_group("winnowing_pipeline");
    let tokens = generate_synthetic_tokens(20_000);

    group.bench_function("winnow_20k_tokens", |b| {
        b.iter(|| winnow(black_box(&tokens), black_box(25), black_box(30)));
    });

    group.finish();
}

fn bench_file_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io");

    // Create 100KB temporary source file
    let mut large_file = NamedTempFile::new().unwrap();
    let line = "pub fn test_bench_line_function() -> u64 { 1000003 }\n";
    for _ in 0..2000 {
        large_file.write_all(line.as_bytes()).unwrap();
    }
    large_file.flush().unwrap();

    group.bench_function("std_fs_read_to_string_100kb", |b| {
        b.iter(|| {
            let s = std::fs::read_to_string(large_file.path()).unwrap();
            black_box(s);
        });
    });

    group.bench_function("mmap_read_file_source_100kb", |b| {
        b.iter(|| {
            let s = read_file_source(large_file.path()).unwrap();
            black_box(s.as_str());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_fast_mod_m61,
    bench_rolling_hash,
    bench_winnowing,
    bench_file_io
);
criterion_main!(benches);
