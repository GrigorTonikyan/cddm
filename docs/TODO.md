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

- [x] **[EP-04] Interactive Side-by-Side Code Diff Visualizer**
  - [x] Add backend endpoint `GET /api/snippet` in `crates/cddm-cli/src/serve.rs`
  - [x] Integrate syntax-highlighted split diff in `ClonePairCard.tsx` and `DiffViewer.tsx`
  - [x] Implement synchronized vertical scrolling and line highlighting
  - [x] Add unit tests in `DiffViewer.test.tsx` and `ClonePairCard.test.tsx`

- [x] **[EP-06] Duplication Treemap & Hierarchical Analytics**
  - [x] Add Squarified Treemap layout in `DuplicationTreemap.tsx` integrated in `ScanResults.tsx`
  - [x] Support zoom-in navigation by directory and crate module
  - [x] Add color-coded duplication density mapping

- [x] **One-Click Refactoring Patch Visualizer in WebUI**
  - [x] Add `POST /api/refactor` endpoint in `crates/cddm-cli/src/serve.rs`
  - [x] Implement `RefactorPatchModal.tsx` displaying invariant suggestions and `.patch` diffs
  - [x] Support one-click patch copy and `.patch` file download

- [x] **Universal Atomic UI & Pure `win2x-manager` Windowing System**
  - [x] Modular Atomic UI primitives (`Portal`, `Backdrop`, `Badge`, `IconButton`, `CollapsibleCard`, `CodeBlock`)
  - [x] Pure `win2x-manager` subsystem with 120fps hardware `translate3d` compositor pipeline
  - [x] Hardware pointer capture (`setPointerCapture`) and dynamic blur decoupling
  - [x] Zero hardcoded values, typed constants/enums, and modern nested CSS Modules scoping

---

### Milestone v0.5.0: AST Pipeline & Polyglot Expansion

- [x] **[EP-05] Tree-sitter AST Merkle Pipeline Integration**
  - [x] Elevate `cddm-core::ast::hasher` to a primary scan phase (`ScanPhase::AstAnalysis`)
  - [x] Implement Zhang-Shasha AST tree edit distance and LCS sequence similarity calculation
  - [x] Classify `CloneType::Exact`, `CloneType::Renamed`, `CloneType::NearMiss`, and `CloneType::Semantic` clones
  - [x] Add test cases for modified statement near-miss detection and dynamic similarity scoring

- [x] **[EP-09] Polyglot Tree-sitter Grammar Expansion**
  - [x] Add `tree-sitter-go` support in `cddm-core`
  - [x] Add `tree-sitter-c` and `tree-sitter-cpp` support
  - [x] Add `tree-sitter-java` and `tree-sitter-c-sharp` support
  - [x] Add unit tests verifying parsing and tokenization across all 9 supported AST languages

---

### Milestone v1.0.0: High-Throughput Enterprise Engine

- [x] **[EP-10] Memory-Mapped I/O & SIMD Vectorization**
  - [x] Integrate `memmap2` zero-copy memory mapping for large files
  - [x] Implement AVX2 / ARM NEON SIMD vector lanes for Mersenne 61 rolling hash
  - [x] Perform comparative throughput benchmarks on 1M+ LOC codebases

---

### Milestone v1.1.0: N-Way Clone Graph Clustering & Multi-Site Deduplication

- [x] **[EP-11] N-Way Graph Clustering & Consensus Refactoring**
  - [x] Implement `cddm_core::cluster::cluster_clone_pairs` via Disjoint-Set Union-Find
  - [x] Implement `cddm_core::refactor::analyze_cluster_refactoring` multi-site consensus synthesizer
  - [x] Add Axum endpoint `POST /api/refactor-cluster` in `cddm-cli`
  - [x] Add `--cluster <ID>` option to `cddm refactor` CLI subcommand
  - [x] Add MCP tools (`cddm_get_clone_cluster`, `cddm_suggest_cluster_refactor`) & resource `cddm://workspace/clusters`
  - [x] Add WebUI Pairwise vs N-Way Clusters view tabs, `CloneClusterCard.tsx`, and unified multi-file `RefactorPatchModal.tsx`

---

## Completed Milestones (Verified)

- [x] **v0.1.0**: Initial Rust core engine, Winnowing rolling hash, CLI scanner.
- [x] **v0.1.1**: Type-2 identifier normalization, Axum HTTP server, React WebUI.
- [x] **v0.1.2**: Tree-sitter AST hashing module, `cddm-mcp` stdio JSON-RPC 2.0 server, in-process `gix` git blame, Vite Plus toolchain integration, Conventional Commits & automated semver pipeline, cross-platform workspace clean (`vp run clean`) & reset (`vp run reset`) runners with 27 unit tests.
- [x] **v0.2.0**: Zero-emoji strict enforcement policy across codebase, dependency upgrade to latest versions with precision retention, missing_debug_implementations workspace denial, automated multi-manifest semver synchronization with README badge & lockfile updates.
- [x] **v0.3.0**: Persistent ACID disk cache powered by `redb` v4 (`.cddm/cache.db`), Git differential scan engine (`cddm diff`), automated patch refactoring CLI (`cddm refactor`), MCP `cddm_diff_scan` tool, with 63 passing unit tests and sub-30ms repeat scans.
- [x] **v0.4.0**: Interactive WebUI Studio with side-by-side synchronized diff visualizer (`DiffViewer.tsx`), secure Axum snippet API (`GET /api/snippet`), on-demand refactoring patch synthesis (`POST /api/refactor`), and hierarchical Squarified Duplication Treemap (`DuplicationTreemap.tsx`).
- [x] **v0.5.0**: AST Merkle Pipeline Integration (`ScanPhase::AstAnalysis`), dynamic clone classification (Type-1 Exact, Type-2 Renamed, Type-3 Near-Miss, Type-4 Semantic), dynamic similarity calculation, and polyglot Tree-sitter expansion with native parsers for Go, C, C++, Java, and C#.
- [x] **v1.0.0**: Enterprise High-Throughput Engine with `memmap2` zero-copy I/O for large files, AVX2 and ARM NEON SIMD Mersenne-61 rolling hash vectorization, and Criterion throughput benchmarking suite.
- [x] **v1.1.0**: N-Way Clone Graph Clustering & Multi-Site Deduplication Synthesis Engine with Disjoint-Set Union-Find transitive partitioning, consensus multi-file diff patches, Axum cluster API, CLI `--cluster`, MCP cluster tools & resources, and WebUI N-way cluster cards.
