# CDDM Task Tracking & Active Implementation TODO

> **Reference Document**: [docs/ROADMAP.md](ROADMAP.md)  
> **Last Updated**: 2026-08-23

---

## Active Development Tasks

### Milestone v0.2.0: CI/CD & AI Agent Tooling (High Priority)

- [ ] **[EP-01] SARIF 2.1.0 Reporter for GitHub Code Scanning**
  - [ ] Add `OutputFormat::Sarif` in `crates/cddm-cli/src/main.rs`
  - [ ] Implement `sarif_reporter.rs` emitting OASIS-compliant SARIF JSON
  - [ ] Map `ClonePair` to SARIF `result` objects with secondary counterpart locations
  - [ ] Add CLI unit and integration tests for SARIF validation
  - [ ] Configure GitHub Code Scanning action workflow sample

- [ ] **[EP-02] Advanced MCP Agentic Toolset & Resource Protocol**
  - [ ] Add `cddm_get_clone_pair` tool to `crates/cddm-mcp/src/main.rs`
  - [ ] Add `cddm_suggest_refactor` tool with parameter identification
  - [ ] Implement `cddm_compare_revisions` tool for diff checking
  - [ ] Expose `cddm://workspace/health` and `cddm://workspace/clones` MCP resources
  - [ ] Add unit tests for JSON-RPC 2.0 requests in `cddm-mcp`

- [ ] **Official GitHub Action (`GrigorTonikyan/cddm-action`)**
  - [ ] Create `.github/actions/cddm-scan/action.yml`
  - [ ] Implement PR summary comment with DRY Health Score delta badge
  - [ ] Support `--fail-threshold` failure enforcement

---

### Milestone v0.3.0: Caching, Differential Scans & Refactoring

- [ ] **[EP-03] Persistent Disk-Backed Fingerprint Cache**
  - [ ] Add `redb` dependency to `crates/cddm-core/Cargo.toml`
  - [ ] Implement persistent cache in `crates/cddm-core/src/cache.rs`
  - [ ] Add CLI flag `--cache-dir <DIR>` (default: `.cddm/cache.db`) and `--no-cache`
  - [ ] Invalidate modified/deleted files during Discovery phase
  - [ ] Benchmark repeat scan latency on large repositories

- [ ] **[EP-08] Differential Scanning & Branch Comparison (`cddm diff`)**
  - [ ] Add `cddm diff <BASE_REF> [TARGET_REF]` subcommand in `cddm-cli`
  - [ ] Integrate in-process `gix` revision range file diffing
  - [ ] Calculate net DRY delta score between revisions
  - [ ] Add integration tests for git branch comparisons

- [ ] **[EP-07] Automated Refactoring & Patch Synthesis (`cddm refactor`)**
  - [ ] Add `cddm refactor --pair <ID>` subcommand
  - [ ] Implement invariant token extraction algorithm
  - [ ] Generate unified `.patch` format output

---

### Milestone v0.4.0: WebUI Studio & Visual Analytics

- [ ] **[EP-04] Interactive Side-by-Side Code Diff Visualizer**
  - [ ] Add backend endpoint `GET /api/snippet` in `crates/cddm-cli/src/serve.rs`
  - [ ] Integrate syntax-highlighted split diff in `ClonePairCard.tsx`
  - [ ] Implement synchronized vertical scrolling and line highlighting
  - [ ] Add unit tests in `ClonePairCard.test.tsx`

- [ ] **[EP-06] Duplication Treemap & Hierarchical Analytics**
  - [ ] Add D3.js or ECharts hierarchical Treemap in `ScanResults.tsx`
  - [ ] Support zoom-in navigation by directory and crate module
  - [ ] Add color-coded duplication density mapping

- [ ] **Historical DRY Health Score Trend Graph**
  - [ ] Add Git historical timeline analyzer via `gix`
  - [ ] Plot DRY score progression over recent commits in WebUI

---

### Milestone v0.5.0: AST Pipeline & Polyglot Expansion

- [ ] **[EP-05] Tree-sitter AST Merkle Pipeline Integration**
  - [ ] Elevate `cddm-core::ast::hasher` to a primary scan phase (`ScanPhase::AstAnalysis`)
  - [ ] Implement Zhang-Shasha AST tree edit distance calculation
  - [ ] Classify `CloneType::NearMiss` and `CloneType::Semantic` clones
  - [ ] Add test cases for modified statement near-miss detection

- [ ] **[EP-09] Polyglot Tree-sitter Grammar Expansion**
  - [ ] Add `tree-sitter-go` support in `cddm-core`
  - [ ] Add `tree-sitter-c` and `tree-sitter-cpp` support
  - [ ] Add `tree-sitter-java` and `tree-sitter-c-sharp` support
  - [ ] Add unit tests verifying parsing and tokenization for each language

---

### Milestone v1.0.0: High-Throughput Enterprise Engine

- [ ] **[EP-10] Memory-Mapped I/O & SIMD Vectorization**
  - [ ] Integrate `memmap2` zero-copy memory mapping for large files
  - [ ] Implement AVX2 / ARM NEON SIMD vector lanes for Mersenne 61 rolling hash
  - [ ] Perform comparative throughput benchmarks on 1M+ LOC codebases

---

## Completed Milestones (Verified)

- [x] **v0.1.0**: Initial Rust core engine, Winnowing rolling hash, CLI scanner.
- [x] **v0.1.1**: Type-2 identifier normalization, Axum HTTP server, React WebUI.
- [x] **v0.1.2**: Tree-sitter AST hashing module, `cddm-mcp` stdio JSON-RPC 2.0 server, in-process `gix` git blame, Vite Plus toolchain integration, Conventional Commits & automated semver pipeline, cross-platform workspace clean (`vp run clean`) & reset (`vp run reset`) runners with 27 unit tests.
