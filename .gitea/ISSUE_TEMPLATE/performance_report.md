---
name: Performance & Optimization Proposal
about: Report a performance bottleneck or propose a SIMD / memory optimization
title: "[PERF] "
labels: ["perf: simd-optimization", "priority: medium"]
---

### Performance Bottleneck / Optimization Target

Describe the performance observation or optimization candidate.

### Benchmark Data & Profiling Metrics

- **Scanned Codebase Size**: (e.g., 1,000,000 LOC, 50,000 files)
- **Current Latency / Throughput**: (e.g., 4.2s on 16 cores)
- **Target Latency / Improvement**: (e.g., < 1.0s using AVX2 SIMD hashing)
- **Memory Footprint**: (e.g., peak RSS 2.4 GB)

### Proposed Architectural Optimization

1. Optimization mechanism (e.g., SIMD vectorization, Rayon work-stealing, compact AST bitsets).
2. Affected crate(s) (`cddm-core`, `cddm-cli`, `cddm-mcp`).

### Verification & Criterion Benchmarks

- [ ] Includes micro-benchmark in `crates/cddm-core/benches/`.
- [ ] Confirmed zero regression on standard test suites.
