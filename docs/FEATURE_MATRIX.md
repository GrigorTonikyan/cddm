# CDDM — Exhaustive Feature Matrix & Test Verification Record

> Every feature variant maps to a real test, with actual file paths and empirically verified results.
> Last verified: 2026-08-08 | Rust: 36/36 PASS | WebUI: 24/24 PASS

---

## Rust Backend — cddm-core (36 tests)

### Tokenization Engine (`crates/cddm-core/src/tokenizer.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-01.1 | Rust source tokenization (fn, let, struct keywords) | `tokenizer::tests::test_tokenize_rust` | ✅ PASS |
| F-01.2 | TypeScript tokenization (function, return, number) | `tokenizer::tests::test_tokenize_ts` | ✅ PASS |
| F-01.3 | Python tokenization (def, class, # comments) | `tokenizer::tests::test_tokenize_python` | ✅ PASS |
| F-01.4 | Line & block comment stripping (// and /* */) | `tokenizer::tests::test_tokenize_comment_stripping` | ✅ PASS |
| F-01.5 | String literal normalization ("...", '...') | `tokenizer::tests::test_tokenize_string_normalization` | ✅ PASS |
| F-01.6 | Empty source produces empty token vec | `tokenizer::tests::test_tokenize_empty_source` | ✅ PASS |

### Winnowing Fingerprint Engine (`crates/cddm-core/src/fingerprint.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-02.1 | Fast Mersenne prime $M_{61}$ modular reduction (zero, small, boundary) | `fingerprint::tests::test_fast_mod_m61` | ✅ PASS |
| F-02.2 | Large value $M_{61}$ modular reduction | `fingerprint::tests::test_fast_mod_m61_large_values` | ✅ PASS |
| F-02.3 | Winnowing window hash calculation | `fingerprint::tests::test_winnowing` | ✅ PASS |
| F-02.4 | Winnow with fewer than k tokens returns empty | `fingerprint::tests::test_winnow_too_few_tokens` | ✅ PASS |
| F-02.5 | Winnowing determinism (same input → same output) | `fingerprint::tests::test_winnow_deterministic` | ✅ PASS |

### Clone Detection Pipeline (`crates/cddm-core/src/detector.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-03.1 | Empty/nonexistent directory scan returns zero results | `detector::tests::test_empty_scan` | ✅ PASS |
| F-03.2 | Real duplicate files detected as clone pairs | `detector::tests::test_scan_with_real_duplicate_files` | ✅ PASS |
| F-03.3 | Scan cancellation via AtomicBool flag | `detector::tests::test_scan_cancellation` | ✅ PASS |
| F-03.4 | Language filter restricts to specified languages | `detector::tests::test_scan_language_filter` | ✅ PASS |
| F-03.5 | Ignore patterns filter out matching paths | `detector::tests::test_scan_ignore_patterns` | ✅ PASS |
| F-03.6 | DRY health score always in [0.0, 100.0] range | `detector::tests::test_dry_health_score_range` | ✅ PASS |

### Language Grammar Registry (`crates/cddm-core/src/grammar.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-04.1 | Extension lookup for .rs, .ts, .py, .go, .css, unknown, no-ext | `grammar::tests::test_get_grammar_for_path` | ✅ PASS |
| F-04.2 | Grammar properties (keywords, comment delimiters) | `grammar::tests::test_grammar_properties` | ✅ PASS |
| F-04.3 | All supported extensions resolve correctly | `grammar::tests::test_all_supported_extensions` | ✅ PASS |
| F-04.4 | At least 12 languages in registry | `grammar::tests::test_supported_language_count` | ✅ PASS |

### Git Blame Annotation (`crates/cddm-core/src/blame.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-05.1 | Non-git directory returns None | `blame::tests::test_non_git_repo_author` | ✅ PASS |
| F-05.2 | Temp directory (non-git) returns None | `blame::tests::test_blame_with_temp_dir` | ✅ PASS |

### Tree-sitter AST Module (`crates/cddm-core/src/ast/`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-06.1 | Rust AST parsing produces source_file root | `ast::parser::tests::test_parse_rust_ast` | ✅ PASS |
| F-06.2 | TypeScript AST parsing produces program root | `ast::parser::tests::test_parse_typescript_ast` | ✅ PASS |
| F-06.3 | Blake3 Merkle subtree hashing extracts depth≥2 nodes | `ast::hasher::tests::test_ast_subtree_hashing` | ✅ PASS |

### Incremental Cache (`crates/cddm-core/src/cache.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-07.1 | Cache modification detection (new, same, changed) | `cache::tests::test_fingerprint_cache` | ✅ PASS |
| F-07.2 | Real file SHA-256 hashing (valid 64-char hex) | `cache::tests::test_compute_file_hash_real_file` | ✅ PASS |
| F-07.3 | Nonexistent file returns None | `cache::tests::test_compute_file_hash_nonexistent` | ✅ PASS |
| F-07.4 | Multiple files cached independently | `cache::tests::test_cache_multiple_files` | ✅ PASS |

### File System Watcher (`crates/cddm-core/src/watcher.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-08.1 | Watcher creation on temp directory | `watcher::tests::test_watcher_creation` | ✅ PASS |

### Type System & Serialization (`crates/cddm-core/src/types.rs`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-09.1 | ScanConfig default values | `types::tests::test_scan_config_default` | ✅ PASS |
| F-09.2 | ScanConfig JSON serde roundtrip | `types::tests::test_scan_config_serde_roundtrip` | ✅ PASS |
| F-09.3 | CloneType all 4 variants serde correctly | `types::tests::test_clone_type_serde_variants` | ✅ PASS |
| F-09.4 | Full ScanResult JSON serde roundtrip | `types::tests::test_scan_result_serde_roundtrip` | ✅ PASS |
| F-09.5 | LineSpan equality comparison | `types::tests::test_line_span_equality` | ✅ PASS |

---

## WebUI Frontend — React 19 + TypeScript 5.8 + Vitest (24 tests)

### Zustand Store (`webui/src/store/__tests__/cddm-store.test.ts`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-10.1 | Default scan config initialization | `should initialize with default scan config` | ✅ PASS |
| F-10.2 | Config partial update | `should update scan config cleanly` | ✅ PASS |
| F-10.3 | Mock scan start with API fallback | `should handle mock scan start fallback` | ✅ PASS |
| F-10.4 | Cancel scan sets error state | `should cancel scan and set error state` | ✅ PASS |
| F-10.5 | Partial config merge preserves other fields | `should merge partial config updates without losing other fields` | ✅ PASS |
| F-10.6 | Concurrent scan rejection | `should not allow concurrent scans` | ✅ PASS |
| F-10.7 | Full state reset | `should reset all state on resetScan` | ✅ PASS |

### App Shell (`webui/src/components/__tests__/App.test.tsx`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-11.1 | Renders CDDM Studio header | `should render CDDM Studio header` | ✅ PASS |
| F-11.2 | Renders ScanConfigPanel | `should render ScanConfigPanel component` | ✅ PASS |
| F-11.3 | Shows error banner on store error | `should show error banner when store has error` | ✅ PASS |

### ScanConfigPanel (`webui/src/components/__tests__/ScanConfigPanel.test.tsx`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-12.1 | Renders inputs and controls | `should render inputs and controls correctly` | ✅ PASS |
| F-12.2 | Directory input updates store | `should update target directory input` | ✅ PASS |
| F-12.3 | Min tokens slider | `should update min tokens slider` | ✅ PASS |
| F-12.4 | Git blame toggle renders | `should render git blame toggle` | ✅ PASS |

### ScanProgressBar (`webui/src/components/__tests__/ScanProgressBar.test.tsx`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-13.1 | Returns null when not scanning | `should return null when not scanning` | ✅ PASS |
| F-13.2 | Renders progress bar when scanning | `should render progress bar when scanning` | ✅ PASS |
| F-13.3 | Displays phase name and percentage | `should display phase name and percentage` | ✅ PASS |

### ScanResults (`webui/src/components/__tests__/ScanResults.test.tsx`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-14.1 | Returns null when no results | `should return null when results is null` | ✅ PASS |
| F-14.2 | Renders DRY health score and metrics | `should render DRY health score and clone details when results exist` | ✅ PASS |
| F-14.3 | Renders clone pair count | `should render clone pair count` | ✅ PASS |

### ClonePairCard (`webui/src/components/__tests__/ClonePairCard.test.tsx`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-15.1 | Renders collapsed summary card | `should render collapsed summary card` | ✅ PASS |
| F-15.2 | Expands to show details on click | `should expand split details on click` | ✅ PASS |

### Type System (`webui/src/types/__tests__/cddm-types.test.ts`)

| ID | Feature Variant | Test Function | Result |
|:---|:----------------|:--------------|:-------|
| F-16.1 | ScanConfig interface shape validation | `should verify ScanConfig interface shape matches expected keys` | ✅ PASS |
| F-16.2 | CloneType union variant coverage | `should verify CloneType union covers all variants` | ✅ PASS |

---

## Summary

| Layer | Test Files | Test Cases | Pass Rate |
|:------|:-----------|:-----------|:----------|
| **Rust Backend** (`cargo test --workspace`) | 9 modules | 36 | **36/36 (100%)** |
| **WebUI Frontend** (`bun run test`) | 7 files | 24 | **24/24 (100%)** |
| **Total** | **16** | **60** | **60/60 (100%)** |
