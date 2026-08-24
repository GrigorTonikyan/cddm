# CDDM — Exhaustive Feature Matrix & Test Verification Record

> Every feature variant maps to a real test with actual file paths and empirically verified results.
> Last verified: 2026-08-24 | Rust: 101/101 PASS | WebUI: 112/112 PASS | Repository Scripts: 27/27 PASS | Playwright E2E: 9/9 PASS | CI Workflows: PASS

---

## 1. Rust Backend — `cddm-core`, `cddm-cli`, `cddm-mcp` (101 unit tests)

### Tokenization Engine (`crates/cddm-core/src/tokenizer.rs`)

| ID     | Feature Variant                                           | Test Function                                          | Result |
| :----- | :-------------------------------------------------------- | :----------------------------------------------------- | :----- |
| F-01.1 | Rust source tokenization (`fn`, `let`, `struct` keywords) | `tokenizer::tests::test_tokenize_rust`                 | PASS   |
| F-01.2 | TypeScript tokenization (`function`, `return`, numbers)   | `tokenizer::tests::test_tokenize_ts`                   | PASS   |
| F-01.3 | Python tokenization (`def`, `class`, `#` comments)        | `tokenizer::tests::test_tokenize_python`               | PASS   |
| F-01.4 | Line & block comment stripping (`//` and `/* */`)         | `tokenizer::tests::test_tokenize_comment_stripping`    | PASS   |
| F-01.5 | String literal normalization (`"..."`, `'...'`)           | `tokenizer::tests::test_tokenize_string_normalization` | PASS   |
| F-01.6 | Empty source produces empty token vector                  | `tokenizer::tests::test_tokenize_empty_source`         | PASS   |

### Winnowing Fingerprint Engine (`crates/cddm-core/src/fingerprint.rs`)

| ID     | Feature Variant                                   | Test Function                                        | Result |
| :----- | :------------------------------------------------ | :--------------------------------------------------- | :----- |
| F-02.1 | Fast Mersenne prime M61 modular reduction         | `fingerprint::tests::test_fast_mod_m61`              | PASS   |
| F-02.2 | Large value M61 modular reduction                 | `fingerprint::tests::test_fast_mod_m61_large_values` | PASS   |
| F-02.3 | Winnowing window hash calculation                 | `fingerprint::tests::test_winnowing`                 | PASS   |
| F-02.4 | Winnow with fewer than k tokens returns empty     | `fingerprint::tests::test_winnow_too_few_tokens`     | PASS   |
| F-02.5 | Winnowing determinism (same input -> same output) | `fingerprint::tests::test_winnow_deterministic`      | PASS   |

### Clone Detection Pipeline (`crates/cddm-core/src/detector.rs`)

| ID      | Feature Variant                                           | Test Function                                                  | Result |
| :------ | :-------------------------------------------------------- | :------------------------------------------------------------- | :----- |
| F-03.1  | Empty/nonexistent directory scan returns zero results     | `detector::tests::test_empty_scan`                             | PASS   |
| F-03.2  | Real duplicate files detected as clone pairs              | `detector::tests::test_scan_with_real_duplicate_files`         | PASS   |
| F-03.3  | Scan cancellation via `AtomicBool` flag                   | `detector::tests::test_scan_cancellation`                      | PASS   |
| F-03.4  | Language filter restricts to specified languages          | `detector::tests::test_scan_language_filter`                   | PASS   |
| F-03.5  | Ignore patterns filter out matching paths                 | `detector::tests::test_scan_ignore_patterns`                   | PASS   |
| F-03.6  | DRY health score always in [0.0, 100.0] range             | `detector::tests::test_dry_health_score_range`                 | PASS   |
| F-03.7  | Intra-file clone prevention when `scan_self` disabled     | `detector::tests::test_no_self_overlapping_clones`             | PASS   |
| F-03.8  | Persistent disk caching populates & accelerates scan      | `detector::tests::test_scan_with_disk_caching`                 | PASS   |
| F-03.9  | Exact vs Renamed clone classification in detector scan    | `detector::tests::test_exact_and_renamed_clone_classification` | PASS   |
| F-03.10 | Polyglot multi-language AST scan (Go, Java, Rust, TS, JS) | `detector::tests::test_polyglot_ast_scan`                      | PASS   |

### Language Grammar Registry (`crates/cddm-core/src/grammar.rs`)

| ID     | Feature Variant                                               | Test Function                                   | Result |
| :----- | :------------------------------------------------------------ | :---------------------------------------------- | :----- |
| F-04.1 | Extension lookup for `.rs`, `.ts`, `.py`, `.go`, `.css`, etc. | `grammar::tests::test_get_grammar_for_path`     | PASS   |
| F-04.2 | Grammar properties (keywords, comment delimiters)             | `grammar::tests::test_grammar_properties`       | PASS   |
| F-04.3 | All supported extensions resolve correctly                    | `grammar::tests::test_all_supported_extensions` | PASS   |
| F-04.4 | Polyglot support (>30 programming languages)                  | `grammar::tests::test_supported_language_count` | PASS   |

### Git Blame Annotation (`crates/cddm-core/src/blame.rs`)

| ID     | Feature Variant                         | Test Function                            | Result |
| :----- | :-------------------------------------- | :--------------------------------------- | :----- |
| F-05.1 | Non-git directory returns `None`        | `blame::tests::test_non_git_repo_author` | PASS   |
| F-05.2 | Temp directory (non-git) returns `None` | `blame::tests::test_blame_with_temp_dir` | PASS   |

### Tree-sitter AST Module (`crates/cddm-core/src/ast/`)

| ID      | Feature Variant                                          | Test Function                                                | Result |
| :------ | :------------------------------------------------------- | :----------------------------------------------------------- | :----- |
| F-06.1  | Rust AST parsing produces `source_file` root             | `ast::parser::tests::test_parse_rust_ast`                    | PASS   |
| F-06.2  | TypeScript AST parsing produces `program` root           | `ast::parser::tests::test_parse_typescript_ast`              | PASS   |
| F-06.3  | JavaScript AST parsing produces `program` root           | `ast::parser::tests::test_parse_javascript_ast`              | PASS   |
| F-06.4  | Python AST parsing produces `module` root                | `ast::parser::tests::test_parse_python_ast`                  | PASS   |
| F-06.5  | Go AST parsing produces `source_file` root               | `ast::parser::tests::test_parse_go_ast`                      | PASS   |
| F-06.6  | C AST parsing produces `translation_unit` root           | `ast::parser::tests::test_parse_c_ast`                       | PASS   |
| F-06.7  | C++ AST parsing produces `translation_unit` root         | `ast::parser::tests::test_parse_cpp_ast`                     | PASS   |
| F-06.8  | Java AST parsing produces `program` root                 | `ast::parser::tests::test_parse_java_ast`                    | PASS   |
| F-06.9  | C# AST parsing produces `compilation_unit` root          | `ast::parser::tests::test_parse_c_sharp_ast`                 | PASS   |
| F-06.10 | Blake3 Merkle subtree hashing extracts depth >= 2 nodes  | `ast::hasher::tests::test_ast_subtree_hashing`               | PASS   |
| F-06.11 | Exact identical text clone classification (Type-1)       | `ast::hasher::tests::test_exact_clone_classification`        | PASS   |
| F-06.12 | Renamed identifier clone classification (Type-2)         | `ast::hasher::tests::test_renamed_clone_classification`      | PASS   |
| F-06.13 | Modified statement near-miss clone detection (Type-3)    | `ast::hasher::tests::test_near_miss_clone_classification`    | PASS   |
| F-06.14 | AST Merkle subtree structural semantic matching (Type-4) | `ast::hasher::tests::test_ast_semantic_clone_classification` | PASS   |

### Persistent redb Disk Cache (`crates/cddm-core/src/cache.rs`)

| ID     | Feature Variant                                          | Test Function                                               | Result |
| :----- | :------------------------------------------------------- | :---------------------------------------------------------- | :----- |
| F-07.1 | Cache modification detection (new, same, changed)        | `cache::tests::test_fingerprint_cache`                      | PASS   |
| F-07.2 | Real file Blake3 hashing (valid 64-char hex)             | `cache::tests::test_compute_file_hash_real_file`            | PASS   |
| F-07.3 | Nonexistent file returns `None`                          | `cache::tests::test_compute_file_hash_nonexistent`          | PASS   |
| F-07.4 | Persistent redb lifecycle (batch save, fetch, fast stat) | `cache::tests::test_disk_cache_lifecycle`                   | PASS   |
| F-07.5 | Disabled no-op cache safety                              | `cache::tests::test_disk_cache_disabled`                    | PASS   |
| F-07.6 | Auto-healing recovery on corrupted database header       | `cache::tests::test_disk_cache_auto_healing_corrupted_file` | PASS   |

### File System Watcher (`crates/cddm-core/src/watcher.rs`)

| ID     | Feature Variant                    | Test Function                           | Result |
| :----- | :--------------------------------- | :-------------------------------------- | :----- |
| F-08.1 | Watcher creation on temp directory | `watcher::tests::test_watcher_creation` | PASS   |

### Type System & Serialization (`crates/cddm-core/src/types.rs`)

| ID     | Feature Variant                              | Test Function                                         | Result |
| :----- | :------------------------------------------- | :---------------------------------------------------- | :----- |
| F-09.1 | `ScanConfig` default values with cache flags | `types::tests::test_scan_config_default`              | PASS   |
| F-09.2 | `ScanConfig` JSON serde roundtrip            | `types::tests::test_scan_config_serde_roundtrip`      | PASS   |
| F-09.3 | `CloneType` all 4 variants serde correctly   | `types::tests::test_clone_type_serde_variants`        | PASS   |
| F-09.4 | Full `ScanResult` JSON serde roundtrip       | `types::tests::test_scan_result_serde_roundtrip`      | PASS   |
| F-09.5 | `LineSpan` equality comparison               | `types::tests::test_line_span_equality`               | PASS   |
| F-09.6 | `ScanPhase` enum serde roundtrip             | `types::tests::test_scan_phase_serde`                 | PASS   |
| F-09.7 | `CloneStatus` display and serde roundtrip    | `types::tests::test_clone_status_display_and_serde`   | PASS   |
| F-09.8 | `DiffScanResult` JSON serde roundtrip        | `types::tests::test_diff_scan_result_serde_roundtrip` | PASS   |

### OASIS SARIF 2.1.0 Reporter (`crates/cddm-core/src/sarif.rs`)

| ID     | Feature Variant                                            | Test Function                                   | Result |
| :----- | :--------------------------------------------------------- | :---------------------------------------------- | :----- |
| F-10.1 | OASIS SARIF v2.1.0 structured report generation            | `sarif::tests::test_sarif_report_generation`    | PASS   |
| F-10.2 | SARIF JSON serde serialization and deserialization         | `sarif::tests::test_sarif_json_serde_roundtrip` | PASS   |
| F-10.3 | Mapping all 4 clone types to rule catalog and rule indices | `sarif::tests::test_all_clone_types_mapped`     | PASS   |

### Connected-Components Graph Clustering (`crates/cddm-core/src/cluster.rs`)

| ID     | Feature Variant                                           | Test Function                                          | Result |
| :----- | :-------------------------------------------------------- | :----------------------------------------------------- | :----- |
| F-11.1 | Empty clone pairs list produces zero clusters             | `cluster::tests::test_empty_clone_pairs_clustering`    | PASS   |
| F-11.2 | Single clone pair partitioned into 2-occurrence cluster   | `cluster::tests::test_single_clone_pair_clustering`    | PASS   |
| F-11.3 | Transitive 3-way clone pairing grouped into 1 cluster     | `cluster::tests::test_three_way_transitive_clustering` | PASS   |
| F-11.4 | Disjoint multi-component clone graphs clustered correctly | `cluster::tests::test_multi_cluster_partitioning`      | PASS   |

### Multi-Site Deduplication & Refactoring Engine (`crates/cddm-core/src/refactor.rs`)

| ID     | Feature Variant                                        | Test Function                                             | Result |
| :----- | :----------------------------------------------------- | :-------------------------------------------------------- | :----- |
| F-12.1 | Identical clone snippet refactoring & patch synthesis  | `refactor::tests::test_identical_snippet_refactoring`     | PASS   |
| F-12.2 | Parameter difference detection for renamed identifiers | `refactor::tests::test_renamed_parameter_refactoring`     | PASS   |
| F-12.3 | Real filesystem file clone refactoring & patch         | `refactor::tests::test_real_file_clone_refactoring`       | PASS   |
| F-12.4 | Out-of-bounds line range error handling                | `refactor::tests::test_invalid_line_range`                | PASS   |
| F-12.5 | Multi-site consensus invariant snippet extraction      | `refactor::tests::test_analyze_cluster_snippets_exact`    | PASS   |
| F-12.6 | Multi-site file-based consensus refactoring & patch    | `refactor::tests::test_analyze_cluster_refactoring_files` | PASS   |

### Git Differential Scanning Engine (`crates/cddm-core/src/diff.rs`)

| ID     | Feature Variant                                       | Test Function                             | Result |
| :----- | :---------------------------------------------------- | :---------------------------------------- | :----- |
| F-13.1 | Non-git directory differential scan error propagation | `diff::tests::test_diff_scan_non_git_dir` | PASS   |
| F-13.2 | Cross-platform file path normalization for diff match | `diff::tests::test_normalize_path_str`    | PASS   |

### CLI Reporter & Studio Server (`crates/cddm-cli/src/main.rs`, `serve.rs`)

| ID     | Feature Variant                                      | Test Function                                          | Result |
| :----- | :--------------------------------------------------- | :----------------------------------------------------- | :----- |
| F-14.1 | `OutputFormat` enum equality & variant parsing       | `main::tests::test_output_format_variants`             | PASS   |
| F-14.2 | CLI SARIF output printing execution                  | `main::tests::test_print_sarif_report_succeeds`        | PASS   |
| F-14.3 | CLI Console and Markdown formatting output execution | `main::tests::test_print_console_and_markdown_reports` | PASS   |
| F-14.4 | CLI Differential scan console & markdown tables      | `main::tests::test_print_diff_reports`                 | PASS   |
| F-14.5 | Axum `/api/refactor-cluster` handler synthesis       | `serve::tests::test_refactor_cluster_handler_success`  | PASS   |

### Advanced MCP Server Protocol (`crates/cddm-mcp/src/main.rs`)

| ID     | Feature Variant                                                  | Test Function                                     | Result |
| :----- | :--------------------------------------------------------------- | :------------------------------------------------ | :----- |
| F-15.1 | MCP protocol initialize, version negotiation & serverInfo        | `main::tests::test_mcp_initialize`                | PASS   |
| F-15.2 | MCP ping healthcheck method                                      | `main::tests::test_mcp_ping`                      | PASS   |
| F-15.3 | Tools discovery (scan, diff, cluster refactor, SARIF)            | `main::tests::test_mcp_tools_list`                | PASS   |
| F-15.4 | Differential scan tool invocation parameter validation           | `main::tests::test_mcp_diff_scan_missing_params`  | PASS   |
| F-15.5 | Resources discovery (`health`, `clones`, `clusters`)             | `main::tests::test_mcp_resources_list`            | PASS   |
| F-15.6 | Prompts discovery and retrieval (`audit_dry_health`, `refactor`) | `main::tests::test_mcp_prompts_list_and_get`      | PASS   |
| F-15.7 | Standard JSON-RPC 2.0 error handling for invalid/unknown method  | `main::tests::test_mcp_unknown_method`            | PASS   |
| F-15.8 | MCP `cddm_suggest_cluster_refactor` explicit occurrences         | `main::tests::test_mcp_cluster_refactor_explicit` | PASS   |
| F-15.9 | MCP `cddm://workspace/clusters` resource contents read           | `main::tests::test_mcp_resources_read_clusters`   | PASS   |

### Zero-Copy Memory-Mapped File I/O (`crates/cddm-core/src/io/`)

| ID     | Feature Variant                                             | Test Function                                     | Result |
| :----- | :---------------------------------------------------------- | :------------------------------------------------ | :----- |
| F-15.1 | Small file reads (<= 64KB) utilize heap buffer              | `io::mmap::tests::test_read_small_file_uses_heap` | PASS   |
| F-15.2 | Large file reads (> 64KB) utilize zero-copy `memmap2::Mmap` | `io::mmap::tests::test_read_large_file_uses_mmap` | PASS   |
| F-15.3 | Empty file reads return empty heap buffer                   | `io::mmap::tests::test_read_empty_file`           | PASS   |
| F-15.4 | Nonexistent file error propagation                          | `io::mmap::tests::test_read_nonexistent_file`     | PASS   |
| F-15.5 | Non-UTF-8 invalid byte error handling                       | `io::mmap::tests::test_read_non_utf8_large_file`  | PASS   |
| F-15.6 | `FileSource` debug representation formatting                | `io::mmap::tests::test_debug_formatting`          | PASS   |

### SIMD Mersenne-61 Rolling Hash Engine (`crates/cddm-core/src/simd/`)

| ID     | Feature Variant                                               | Test Function                                             | Result |
| :----- | :------------------------------------------------------------ | :-------------------------------------------------------- | :----- |
| F-16.1 | Scalar dual-base k-gram rolling hashes calculation            | `simd::scalar::tests::test_scalar_kgram_hashes`           | PASS   |
| F-16.2 | AVX2 hardware acceleration output parity with scalar baseline | `simd::avx2::tests::test_avx2_matches_scalar`             | PASS   |
| F-16.3 | ARM NEON vector lanes output parity with scalar baseline      | `simd::neon::tests::test_neon_matches_scalar`             | PASS   |
| F-16.4 | Automatic runtime hardware vectorization dispatcher           | `simd::tests::test_compute_kgram_rolling_hashes_dispatch` | PASS   |

---

## 2. WebUI Frontend — React 19 + TypeScript + Vitest (112 unit tests across 30 suites)

| Module             | Test Suite File                                                                  | Test Cases | Status |
| :----------------- | :------------------------------------------------------------------------------- | :--------- | :----- |
| Store              | `webui/src/store/__tests__/cddm-store.test.ts`                                   | 9 tests    | PASS   |
| App Shell          | `webui/src/components/__tests__/App.test.tsx`                                    | 5 tests    | PASS   |
| Config Panel       | `webui/src/components/__tests__/ScanConfigPanel.test.tsx`                        | 4 tests    | PASS   |
| Progress Bar       | `webui/src/components/__tests__/ScanProgressBar.test.tsx`                        | 3 tests    | PASS   |
| Results View       | `webui/src/components/__tests__/ScanResults.test.tsx`                            | 8 tests    | PASS   |
| Clone Pair Card    | `webui/src/components/__tests__/ClonePairCard.test.tsx`                          | 2 tests    | PASS   |
| Clone Cluster Card | `webui/src/components/__tests__/CloneClusterCard.test.tsx`                       | 2 tests    | PASS   |
| Diff Viewer        | `webui/src/components/__tests__/DiffViewer.test.tsx`                             | 3 tests    | PASS   |
| Refactor Modal     | `webui/src/components/__tests__/RefactorPatchModal.test.tsx`                     | 3 tests    | PASS   |
| Duplication Map    | `webui/src/components/__tests__/DuplicationTreemap.test.tsx`                     | 3 tests    | PASS   |
| Treemap Explorer   | `webui/src/components/__tests__/TreemapExplorerModal.test.tsx`                   | 3 tests    | PASS   |
| Health Audit       | `webui/src/components/__tests__/HealthAuditModal.test.tsx`                       | 3 tests    | PASS   |
| Export Report      | `webui/src/components/__tests__/ExportReportModal.test.tsx`                      | 3 tests    | PASS   |
| Lang Analytics     | `webui/src/components/__tests__/LanguageAnalyticsModal.test.tsx`                 | 2 tests    | PASS   |
| Config Modal       | `webui/src/components/__tests__/ScanConfigModal.test.tsx`                        | 2 tests    | PASS   |
| Clone Diff Modal   | `webui/src/components/__tests__/ClonePairDiffModal.test.tsx`                     | 3 tests    | PASS   |
| Type System        | `webui/src/types/__tests__/cddm-types.test.ts`                                   | 2 tests    | PASS   |
| UI Badge           | `webui/src/components/ui/__tests__/badge.test.tsx`                               | 2 tests    | PASS   |
| UI Icon Button     | `webui/src/components/ui/__tests__/icon-button.test.tsx`                         | 2 tests    | PASS   |
| UI Card            | `webui/src/components/ui/__tests__/collapsible-card.test.tsx`                    | 2 tests    | PASS   |
| UI Code Block      | `webui/src/components/ui/__tests__/code-block.test.tsx`                          | 3 tests    | PASS   |
| Win2x Geometry     | `webui/src/components/ui/win2x-manager/__tests__/geometry-engine.test.ts`        | 7 tests    | PASS   |
| Win2x Driver       | `webui/src/components/ui/win2x-manager/__tests__/pointer-driver.test.ts`         | 2 tests    | PASS   |
| Win2x Storage      | `webui/src/components/ui/win2x-manager/__tests__/storage-adapter.test.ts`        | 5 tests    | PASS   |
| Win2x ScrollLock   | `webui/src/components/ui/win2x-manager/__tests__/use-body-scroll-lock.test.ts`   | 3 tests    | PASS   |
| Win2x Drag Hook    | `webui/src/components/ui/win2x-manager/__tests__/use-pointer-drag.test.ts`       | 2 tests    | PASS   |
| Win2x Resize Hook  | `webui/src/components/ui/win2x-manager/__tests__/use-pointer-resize.test.ts`     | 2 tests    | PASS   |
| Win2x Context      | `webui/src/components/ui/win2x-manager/__tests__/win2x-manager-context.test.tsx` | 5 tests    | PASS   |
| Win2x Window       | `webui/src/components/ui/win2x-manager/__tests__/win2x-window.test.tsx`          | 13 tests   | PASS   |
| Win2x Tab Bar      | `webui/src/components/ui/win2x-manager/__tests__/tab-bar.test.tsx`               | 4 tests    | PASS   |

---

## 3. Repository Scripts & Release Tooling — Bun Test (27 unit tests)

| Module           | Test Suite File                         | Test Cases | Status |
| :--------------- | :-------------------------------------- | :--------- | :----- |
| Commit Validator | `scripts/__tests__/version.test.ts`     | 4 tests    | PASS   |
| Semantic Release | `scripts/__tests__/version.test.ts`     | 5 tests    | PASS   |
| Doc Integrity    | `scripts/__tests__/docs.test.ts`        | 5 tests    | PASS   |
| No-Emoji Policy  | `scripts/__tests__/no-emojis.test.ts`   | 5 tests    | PASS   |
| Workspace Engine | `scripts/__tests__/clean-reset.test.ts` | 8 tests    | PASS   |

---

## 4. GitHub Automation & Governance Validation

| Layer                       | Validation Target                                              | Status     |
| :-------------------------- | :------------------------------------------------------------- | :--------- |
| **Commit Message Linter**   | `@commitlint/cli` + `commitlint.config.ts` in `.vite-hooks`    | Enforced   |
| **Semantic Versioning**     | `bumpp` + `conventional-changelog` via `vp run bump`           | Enforced   |
| **CI Automation**           | `.github/workflows/ci.yml` (Matrix build, clippy, fmt, vitest) | Configured |
| **Release Automation**      | `.github/workflows/release.yml` (Cross-compiling 4 targets)    | Configured |
| **Documentation Integrity** | `bun scripts/check-docs.ts` (Links, tables, roadmap sync)      | Enforced   |
| **Community Templates**     | Issue templates & Pull Request checklist template              | Active     |
| **Registry Metadata**       | `crates.io` keywords/categories + `npm` keywords/links         | Validated  |
