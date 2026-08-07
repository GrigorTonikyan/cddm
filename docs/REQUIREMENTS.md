# CDDM (Code De-Duplication Meister) — System Requirements Document v2.0

> This document defines the exhaustive functional and non-functional requirements for CDDM, aligned with actual implementation status.

---

## 1. Product Overview

CDDM (*Code De-Duplication Meister*) is a standalone, high-performance, multi-threaded polyglot code clone detection engine built in pure Rust (2024 edition). It provides:
- CLI-based code duplication analysis
- An embedded interactive React WebUI
- A Model Context Protocol (MCP) server for AI agent integration
- Dual distribution via Cargo and npm

---

## 2. Functional Requirements

### FR-1: Polyglot Tokenization Engine

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-1.1 | Support 12 programming languages | Grammar registry contains Rust, TypeScript, JavaScript, Python, Go, Java, C, C++, C#, CSS/SCSS, HTML, JSON | ✅ Implemented |
| FR-1.2 | Single-line comment stripping | `//` (C-family), `#` (Python) comments produce zero tokens | ✅ Implemented |
| FR-1.3 | Block comment stripping | `/* */`, `<!-- -->` block comments produce zero tokens | ✅ Implemented |
| FR-1.4 | String literal normalization | `"..."`, `'...'`, `` `...` `` all produce `StringLiteral` token | ✅ Implemented |
| FR-1.5 | Numeric literal normalization | Integers, floats, hex values all produce `NumericLiteral` token | ✅ Implemented |
| FR-1.6 | Keyword recognition | Language keywords map to `Keyword(id)` tokens | ✅ Implemented |
| FR-1.7 | Identifier normalization | All identifiers normalize to `Identifier` token | ✅ Implemented |
| FR-1.8 | Configurable min token threshold | `min_tokens` parameter controls minimum clone size ($N \in [1, \infty)$) | ✅ Implemented |

### FR-2: Winnowing Fingerprint Engine

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-2.1 | Mersenne prime $M_{61}$ rolling hash | `fast_mod_m61()` correctly reduces values modulo $2^{61}-1$ | ✅ Implemented |
| FR-2.2 | Dual-base collision resistance | Two independent hash bases $b_1=313$, $b_2=1{,}000{,}003$ | ✅ Implemented |
| FR-2.3 | Winnowing window selection | Minimum hash selected from each sliding window of size $w$ | ✅ Implemented |
| FR-2.4 | Boundary handling | Inputs with fewer than $k$ tokens return empty fingerprint set | ✅ Implemented |
| FR-2.5 | Deterministic output | Same input always produces identical fingerprints | ✅ Implemented |

### FR-3: Clone Detection Pipeline

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-3.1 | Type-1 (exact) clone detection | Identical token sequences after normalization are matched | ✅ Implemented |
| FR-3.2 | Type-2 (renamed) identifier support | `detect_type2` flag enables identifier normalization | ✅ Partially (flag exists, tokenizer has `_normalize_type2` param but unused) |
| FR-3.3 | Parallel file processing | Rayon `par_iter()` tokenizes and fingerprints files concurrently | ✅ Implemented |
| FR-3.4 | Intra-file clone toggle | `scan_self` flag controls whether same-file pairs are emitted | ✅ Implemented |
| FR-3.5 | Scan cancellation | `AtomicBool` cancel flag aborts scan at each phase boundary | ✅ Implemented |
| FR-3.6 | Progress event channel | `Sender<ScanProgress>` emits Discovery, Tokenization, Indexing, Merging, Complete phases | ✅ Implemented |
| FR-3.7 | Language filter | `languages` field restricts scan to specified language names | ✅ Implemented |
| FR-3.8 | Ignore pattern filtering | `ignore_patterns` field excludes matching file paths | ✅ Implemented |

### FR-4: DRY Health Score

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-4.1 | Score computation | $S = \max(0, \min(100, (100 - 1.5 \cdot D_\%) \cdot (1 - 0.25 \cdot R_{\text{cross}})))$ | ✅ Implemented |
| FR-4.2 | Score range clamping | Score always in $[0.0, 100.0]$ | ✅ Implemented |
| FR-4.3 | Cross-module ratio | Clones spanning different top-level directories penalize score | ✅ Implemented |

### FR-5: Git Blame Annotation

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-5.1 | In-process git blame | Uses `gix` (`gitoxide`) without external `git` binary | ✅ Implemented |
| FR-5.2 | Author + date format | Returns `"Author (line N, YYYY-MM-DD)"` string | ✅ Implemented |
| FR-5.3 | Non-git fallback | Returns `None` for non-git directories | ✅ Implemented |

### FR-6: Tree-sitter AST Module

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-6.1 | CST parsing | `parse_ast_tree()` returns tree-sitter `Tree` for Rust, TypeScript, JavaScript, Python | ✅ Implemented |
| FR-6.2 | Language detection | `get_tree_sitter_language()` maps file extensions to tree-sitter `Language` | ✅ Implemented |
| FR-6.3 | Merkle subtree hashing | `compute_ast_subtree_hashes()` recursively hashes AST nodes with Blake3 | ✅ Implemented |
| FR-6.4 | Minimum depth filter | Only subtrees with depth ≥ `min_depth` are returned | ✅ Implemented |
| FR-6.5 | AST-detector integration | AST hashes used in `detector.rs` for Type-3/4 matching | ⬜ Not yet wired |

### FR-7: Incremental Cache & Watcher

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-7.1 | SHA-256 file hashing | `compute_file_hash()` returns deterministic hex digest | ✅ Implemented |
| FR-7.2 | Modification detection | `is_file_modified()` compares current vs cached hash | ✅ Implemented |
| FR-7.3 | File system watcher | `CddmWatcher::watch_directory()` uses `notify` crate for recursive OS events | ✅ Implemented |

### FR-8: CLI Reporters

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-8.1 | Console ANSI table | `--format console` outputs colored `comfy-table` | ✅ Implemented |
| FR-8.2 | JSON reporter | `--format json` outputs `serde_json::to_string_pretty` | ✅ Implemented |
| FR-8.3 | Markdown reporter | `--format markdown` outputs GFM table | ✅ Implemented |
| FR-8.4 | Failure threshold | `--fail-threshold <PCT>` exits with code 1 if exceeded | ✅ Implemented |

### FR-9: Studio WebUI (`cddm serve`)

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-9.1 | Embedded asset serving | `rust-embed` bundles `webui/dist/` into binary | ✅ Implemented |
| FR-9.2 | SPA routing fallback | Unknown paths serve `index.html` | ✅ Implemented |
| FR-9.3 | REST scan API | `POST /api/scan` accepts `ScanConfig` JSON | ✅ Implemented |
| FR-9.4 | Health check | `GET /api/health` returns status JSON | ✅ Implemented |
| FR-9.5 | Browser auto-open | `--open` flag launches default browser via `opener` | ✅ Implemented |

### FR-10: MCP Server (`cddm-mcp`)

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| FR-10.1 | JSON-RPC 2.0 over stdio | Line-delimited JSON request/response | ✅ Implemented |
| FR-10.2 | `initialize` handler | Returns protocol version and capabilities | ✅ Implemented |
| FR-10.3 | `tools/list` handler | Returns `scan_codebase` tool with JSON Schema | ✅ Implemented |
| FR-10.4 | `tools/call` handler | Executes scan and returns MCP content response | ✅ Implemented |
| FR-10.5 | Error handling | Unknown methods return `-32601`, parse errors return `-32700` | ✅ Implemented |

---

## 3. Non-Functional Requirements

| ID | Requirement | Acceptance Criteria | Status |
|:---|:------------|:--------------------|:-------|
| NFR-1.1 | Performance | Process >10,000 tokens/second on single thread | ✅ Verified (22-54ms for small codebases) |
| NFR-1.2 | Parallelism | Rayon scales across all available CPU cores | ✅ Implemented |
| NFR-2.1 | Type safety (Rust) | All public APIs use `Result<T, E>`, no panics | ✅ Implemented |
| NFR-2.2 | Type safety (TS) | `strict: true`, zero `any` policy | ✅ Verified |
| NFR-3.1 | Dual license | MIT OR Apache-2.0 | ✅ License files present |

---

## 4. Distribution Channels

| Channel | Command | Status |
|:--------|:--------|:-------|
| Cargo | `cargo install cddm` | ✅ Ready |
| npm | `npm install -g cddm` | ✅ Shim configured |
