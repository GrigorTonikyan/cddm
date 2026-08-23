# CDDM Task Tracking & Active Implementation TODO

> **Reference Document**: [docs/ROADMAP.md](ROADMAP.md)  
> **Last Updated**: 2026-08-23

---

## Active Development Tasks

### Milestone v0.2.0: CI/CD & AI Agent Tooling (High Priority)

- [x] **[EP-01] SARIF 2.1.0 Reporter for GitHub Code Scanning**
  - [x] Add `OutputFormat::Sarif` in `crates/cddm-cli/src/main.rs`
  - [x] Implement `sarif.rs` emitting OASIS-compliant SARIF 2.1.0 JSON in `cddm-core`
  - [x] Map `ClonePair` to SARIF `result` objects with primary and counterpart `relatedLocations`
  - [x] Add CLI and core unit tests for SARIF validation and rule catalog
  - [x] Configure GitHub Code Scanning action workflow sample

- [x] **[EP-02] Advanced MCP Agentic Toolset & Resource Protocol**
  - [x] Add `cddm_get_clone_pair` tool to `crates/cddm-mcp/src/main.rs`
  - [x] Add `cddm_suggest_refactor` tool with invariant LCS parameter extraction and unified `.patch` format
  - [x] Add `cddm_export_sarif` tool for on-demand SARIF 2.1.0 JSON generation
  - [x] Expose `cddm://workspace/health` and `cddm://workspace/clones` MCP resources (`resources/list`, `resources/read`)
  - [x] Expose `audit_dry_health` and `refactor_clone_pair` MCP prompts (`prompts/list`, `prompts/get`)
  - [x] Add unit tests for JSON-RPC 2.0 requests in `cddm-mcp`

- [x] **Official GitHub Action (`GrigorTonikyan/cddm-action`)**
  - [x] Create `.github/actions/cddm-scan/action.yml`
  - [x] Implement PR summary comment with DRY Health Score metrics and `$GITHUB_STEP_SUMMARY`
  - [x] Support `--fail-threshold` failure enforcement

---

### Milestone v0.3.0: Caching, Differential Scans & Refactoring

- [x] **[EP-03] Persistent Disk-Backed Fingerprint Cache**
  - [x] Add `redb` dependency to `crates/cddm-core/Cargo.toml`
  - [x] Implement persistent cache in `crates/cddm-core/src/cache.rs`
  - [x] Add CLI flag `--cache-dir <DIR>` (default: `.cddm/cache.db`) and `--no-cache`
  - [x] Invalidate modified/deleted files during Discovery phase
  - [x] Benchmark repeat scan latency on large repositories

- [x] **[EP-08] Differential Scanning & Branch Comparison (`cddm diff`)**
  - [x] Add `cddm diff <BASE_REF> [TARGET_REF]` subcommand in `cddm-cli`
  - [x] Integrate in-process `gix` revision range file diffing
  - [x] Calculate net DRY delta score between revisions
  - [x] Add integration tests for git branch comparisons

- [x] **[EP-07] Automated Refactoring & Patch Synthesis (`cddm refactor`)**
  - [x] Add `cddm refactor --pair <ID>` subcommand
  - [x] Implement invariant token extraction algorithm
  - [x] Generate unified `.patch` format output

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
- [x] **v0.2.0**: Zero-emoji strict enforcement policy across codebase, dependency upgrade to latest versions with precision retention, missing_debug_implementations workspace denial, automated multi-manifest semver synchronization with README badge & lockfile updates.
- [x] **v0.3.0**: Persistent ACID disk cache powered by `redb` v4 (`.cddm/cache.db`), Git differential scan engine (`cddm diff`), automated patch refactoring CLI (`cddm refactor`), MCP `cddm_diff_scan` tool, with 63 passing unit tests and sub-30ms repeat scans.
