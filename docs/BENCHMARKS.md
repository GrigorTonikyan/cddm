# CDDM Reproducible Ecosystem Benchmarks

All figures below were measured directly by [`scripts/benchmark-ecosystem.ts`](../scripts/benchmark-ecosystem.ts) on real local executions — no synthetic or estimated timings are included.

> **Captured**: `2026-11-16T18:48:20.622Z`

## 1. Benchmark Environment & Tool Versions

- **Host OS / Arch**: `win32` (`x64`)
- **CPU**: 11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz (16 logical cores)
- **System RAM**: 23.7 GB
- **Runtime**: Bun `1.4.4`
- **CDDM Binary**: `cddm 4.11.0`
- **jscpd**: `jscpd 5.4.3`
- **PMD CPD**: _not installed in environment_

## 2. Controlled Polyglot Corpus (240 files, 11,238 LOC)

Generated deterministically via [`scripts/lib/benchmark-corpus.ts`](../scripts/lib/benchmark-corpus.ts) with 10-way keyword/operator combinatorial AST statements plus planted per-family Type-1/Type-2 clones across 4 languages:

- **Rust**: 60 files (2,700 lines)
- **TypeScript**: 60 files (2,700 lines)
- **Python**: 60 files (3,138 lines)
- **Go**: 60 files (2,700 lines)

| Tool    | Version     | Mode                                  | Median Latency |   Throughput | Clones Found | Peak RSS | Exact CLI Flags                                            | Notes                                            |
| :------ | :---------- | :------------------------------------ | -------------: | -----------: | -----------: | -------: | :--------------------------------------------------------- | :----------------------------------------------- |
| CDDM    | cddm 4.11.0 | Cold scan (Type-1/2/3)                |       131.7 ms | 85,317 LOC/s |          699 |  30.7 MB | `cddm scan <corpus> -m 50 --no-cache -f json`              | Full parse + SIMD M61 winnowing + Type-3 merge   |
| CDDM    | cddm 4.11.0 | Warm redb cache (Type-1/2/3)          |       132.6 ms | 84,758 LOC/s |          699 |  30.2 MB | `cddm scan <corpus> -m 50 --cache-dir <dir> -f json`       | Incremental SHA-256 + redb mmap cache hit        |
| jscpd   | jscpd 5.4.3 | Corpus exact clones                   |       126.3 ms | 88,972 LOC/s |            1 |  21.0 MB | `jscpd <corpus>/src -k 50 -r json -s`                      | Rabin-Karp exact token matching                  |
| jscpd   | jscpd 5.4.3 | Corpus renamed (--ignore-identifiers) |       133.4 ms | 84,262 LOC/s |           82 |  21.0 MB | `jscpd <corpus>/src -k 50 --ignore-identifiers -r json -s` | Type-1 + consistent identifier-normalized Type-2 |
| PMD CPD | N/A         | CPD                                   |            N/A |          N/A |          N/A |      N/A | `pmd cpd --minimum-tokens 50`                              | Not installed in benchmark environment           |

## 3. Repository Self-Scan Benchmark (`gt-dev/cddm`, 691 files / 107,415 lines)

| Tool  | Version     | Mode                                      | Median Latency |    Throughput | Clones Found | Peak RSS | Exact CLI Flags                                              |
| :---- | :---------- | :---------------------------------------- | -------------: | ------------: | -----------: | -------: | :----------------------------------------------------------- |
| CDDM  | cddm 4.11.0 | Self-scan cold (--no-cache)               |       232.7 ms | 461,662 LOC/s |           57 |  54.8 MB | `cddm scan . -m 50 --no-cache -f json`                       |
| CDDM  | cddm 4.11.0 | Self-scan warm (redb cache)               |       249.7 ms | 430,159 LOC/s |           57 |  56.1 MB | `cddm scan . -m 50 --cache-dir <dir> -f json`                |
| jscpd | jscpd 5.4.3 | Repository exact clones                   |       229.1 ms | 468,959 LOC/s |          518 |  21.5 MB | `jscpd . -i <ignored> -k 50 -r json -s`                      |
| jscpd | jscpd 5.4.3 | Repository renamed (--ignore-identifiers) |       251.3 ms | 427,437 LOC/s |         1020 |  21.4 MB | `jscpd . -i <ignored> -k 50 --ignore-identifiers -r json -s` |

## 4. Reproduce Locally

```bash
# Build release binary with embedded WebUI
vp run build:release

# Regenerate benchmarks against the deterministic corpus, jscpd, and gt-dev/cddm self-scan
bun scripts/benchmark-ecosystem.ts --with-jscpd --with-pmd --self-scan --write
```
