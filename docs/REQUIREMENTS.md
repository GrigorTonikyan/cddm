# CDDM (Code De-Duplication Meister) — System Requirements Document v3.0

> This document defines the exhaustive functional and non-functional requirements for CDDM, aligned with actual implementation status as of v1.0.0.

---

## 1. Product Overview

CDDM (_Code De-Duplication Meister_) is a standalone, high-performance, multi-threaded polyglot code clone detection engine built in pure Rust (2024 edition). It provides:

- CLI-based code duplication analysis (`cddm scan`)
- An embedded interactive React 19 WebUI (`cddm serve`)
- A Model Context Protocol (MCP) server for AI agent integration (`cddm-mcp`)
- Dual distribution via Cargo (`crates.io`), npm (`npmjs.com`), and GitHub Releases

---

## 2. Functional Requirements

### FR-1: Polyglot Tokenization Engine

| ID     | Requirement                       | Acceptance Criteria                                                                                                                                           | Status      |
| :----- | :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------- |
| FR-1.1 | Support 30+ programming languages | Grammar registry contains Rust, TypeScript, JavaScript, Python, Go, Java, C, C++, C#, CSS/SCSS, HTML, JSON, Ruby, PHP, Swift, Kotlin, Zig, Dart, Elixir, etc. | Implemented |
| FR-1.2 | Single-line comment stripping     | `//` (C-family), `#` (Python) comments produce zero tokens                                                                                                    | Implemented |
| FR-1.3 | Block comment stripping           | `/* */`, `<!-- -->` block comments produce zero tokens                                                                                                        | Implemented |
| FR-1.4 | String literal normalization      | `"..."`, `'...'`, `` `...` `` all produce `StringLiteral` token                                                                                               | Implemented |
| FR-1.5 | Numeric literal normalization     | Integers, floats, hex values all produce `NumericLiteral` token                                                                                               | Implemented |
| FR-1.6 | Keyword recognition               | Language keywords map to `Keyword(id)` tokens                                                                                                                 | Implemented |
| FR-1.7 | Identifier normalization          | All identifiers normalize to `Identifier` token                                                                                                               | Implemented |
| FR-1.8 | Configurable min token threshold  | `min_tokens` parameter controls minimum clone size (N >= 1)                                                                                                   | Implemented |

### FR-2: Winnowing Fingerprint Engine

| ID     | Requirement                     | Acceptance Criteria                                          | Status      |
| :----- | :------------------------------ | :----------------------------------------------------------- | :---------- |
| FR-2.1 | Mersenne prime M61 rolling hash | `fast_mod_m61()` correctly reduces values modulo 2^61 - 1    | Implemented |
| FR-2.2 | Dual-base collision resistance  | Two independent hash bases b1 = 313, b2 = 1000003            | Implemented |
| FR-2.3 | Winnowing window selection      | Minimum hash selected from each sliding window of size w     | Implemented |
| FR-2.4 | Boundary handling               | Inputs with fewer than k tokens return empty fingerprint set | Implemented |
| FR-2.5 | Deterministic output            | Same input always produces identical fingerprints            | Implemented |

### FR-3: Clone Detection Pipeline

| ID     | Requirement                         | Acceptance Criteria                                                                      | Status      |
| :----- | :---------------------------------- | :--------------------------------------------------------------------------------------- | :---------- |
| FR-3.1 | Type-1 (exact) clone detection      | Identical token sequences after normalization are matched                                | Implemented |
| FR-3.2 | Type-2 (renamed) identifier support | `detect_type2` flag enables identifier normalization                                     | Implemented |
| FR-3.3 | Parallel file processing            | Rayon `par_iter()` tokenizes and fingerprints files concurrently                         | Implemented |
| FR-3.4 | Intra-file clone toggle             | `scan_self` flag controls whether same-file pairs are emitted                            | Implemented |
| FR-3.5 | Scan cancellation                   | `AtomicBool` cancel flag aborts scan at each phase boundary                              | Implemented |
| FR-3.6 | Progress event channel              | `Sender<ScanProgress>` emits Discovery, Tokenization, Indexing, Merging, Complete phases | Implemented |
| FR-3.7 | Language filter                     | `languages` field restricts scan to specified language names                             | Implemented |
| FR-3.8 | Ignore pattern filtering            | `ignore_patterns` field excludes matching file paths                                     | Implemented |

### FR-4: DRY Health Score

| ID     | Requirement          | Acceptance Criteria                                                                            | Status      |
| :----- | :------------------- | :--------------------------------------------------------------------------------------------- | :---------- |
| FR-4.1 | Score computation    | Score = max(0, min(100, (100 - 1.5 _Duplication_Percentage)_ (1 - 0.25 * Cross_Module_Ratio))) | Implemented |
| FR-4.2 | Score range clamping | Score always in [0.0, 100.0]                                                                   | Implemented |
| FR-4.3 | Cross-module ratio   | Clones spanning different top-level directories penalize score                                 | Implemented |

### FR-5: Git Blame Annotation

| ID     | Requirement          | Acceptance Criteria                                   | Status      |
| :----- | :------------------- | :---------------------------------------------------- | :---------- |
| FR-5.1 | In-process git blame | Uses `gix` (`gitoxide`) without external `git` binary | Implemented |
| FR-5.2 | Author + date format | Returns `"Author (line N, YYYY-MM-DD)"` string        | Implemented |
| FR-5.3 | Non-git fallback     | Returns `None` for non-git directories                | Implemented |

### FR-6: Tree-sitter AST Module

| ID     | Requirement            | Acceptance Criteria                                                                    | Status      |
| :----- | :--------------------- | :------------------------------------------------------------------------------------- | :---------- |
| FR-6.1 | CST parsing            | `parse_ast_tree()` returns tree-sitter `Tree` for Rust, TypeScript, JavaScript, Python | Implemented |
| FR-6.2 | Language detection     | `get_tree_sitter_language()` maps file extensions to tree-sitter `Language`            | Implemented |
| FR-6.3 | Merkle subtree hashing | `compute_ast_subtree_hashes()` recursively hashes AST nodes with Blake3                | Implemented |
| FR-6.4 | Minimum depth filter   | Only subtrees with depth >= min_depth are returned                                     | Implemented |

### FR-7: CLI & Reporters

| ID     | Requirement        | Acceptance Criteria                                    | Status      |
| :----- | :----------------- | :----------------------------------------------------- | :---------- |
| FR-7.1 | Console ANSI table | `--format console` outputs colored `comfy-table`       | Implemented |
| FR-7.2 | JSON reporter      | `--format json` outputs formatted `serde_json`         | Implemented |
| FR-7.3 | Markdown reporter  | `--format markdown` outputs GFM table                  | Implemented |
| FR-7.4 | Failure threshold  | `--fail-threshold <PCT>` exits with code 1 if exceeded | Implemented |

---

## 3. Distribution & CI/CD Requirements

| Requirement                | Acceptance Criteria                                                                  | Status     |
| :------------------------- | :----------------------------------------------------------------------------------- | :--------- |
| **Cargo Distribution**     | Published workspace on `crates.io` with categories and keywords                      | Configured |
| **npm Distribution**       | Published cross-platform binary package wrapper `cddm` on `npmjs.com`                | Configured |
| **GitHub Releases**        | GitHub Actions workflow `.github/workflows/release.yml` compiles standalone binaries | Configured |
| **Continuous Integration** | GitHub Actions `.github/workflows/ci.yml` builds and tests Rust + WebUI              | Configured |
