# CDDM — Exhaustive Feature Matrix & Test Verification Record

> Every feature variant maps to a real test with actual file paths and empirically verified results.
> Last verified: 2026-08-23 | Rust: 63/63 PASS | WebUI: 24/24 PASS | Repository Scripts: 27/27 PASS | CI Workflows: PASS

---

## 1. Rust Backend — `cddm-core`, `cddm-cli`, `cddm-mcp` (63 unit tests)

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

| ID     | Feature Variant                                       | Test Function                                          | Result |
| :----- | :---------------------------------------------------- | :----------------------------------------------------- | :----- |
| F-03.1 | Empty/nonexistent directory scan returns zero results | `detector::tests::test_empty_scan`                     | PASS   |
| F-03.2 | Real duplicate files detected as clone pairs          | `detector::tests::test_scan_with_real_duplicate_files` | PASS   |
| F-03.3 | Scan cancellation via `AtomicBool` flag               | `detector::tests::test_scan_cancellation`              | PASS   |
| F-03.4 | Language filter restricts to specified languages      | `detector::tests::test_scan_language_filter`           | PASS   |
| F-03.5 | Ignore patterns filter out matching paths             | `detector::tests::test_scan_ignore_patterns`           | PASS   |
| F-03.6 | DRY health score always in [0.0, 100.0] range         | `detector::tests::test_dry_health_score_range`         | PASS   |
| F-03.7 | Intra-file clone prevention when `scan_self` disabled | `detector::tests::test_no_self_overlapping_clones`     | PASS   |
| F-03.8 | Persistent disk caching populates & accelerates scan  | `detector::tests::test_scan_with_disk_caching`         | PASS   |

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

| ID     | Feature Variant                                         | Test Function                                   | Result |
| :----- | :------------------------------------------------------ | :---------------------------------------------- | :----- |
| F-06.1 | Rust AST parsing produces `source_file` root            | `ast::parser::tests::test_parse_rust_ast`       | PASS   |
| F-06.2 | TypeScript AST parsing produces `program` root          | `ast::parser::tests::test_parse_typescript_ast` | PASS   |
| F-06.3 | Blake3 Merkle subtree hashing extracts depth >= 2 nodes | `ast::hasher::tests::test_ast_subtree_hashing`  | PASS   |

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

### Clone Refactoring Engine (`crates/cddm-core/src/refactor.rs`)

| ID     | Feature Variant                                        | Test Function                                         | Result |
| :----- | :----------------------------------------------------- | :---------------------------------------------------- | :----- |
| F-11.1 | Identical clone snippet refactoring & patch synthesis  | `refactor::tests::test_identical_snippet_refactoring` | PASS   |
| F-11.2 | Parameter difference detection for renamed identifiers | `refactor::tests::test_renamed_parameter_refactoring` | PASS   |
| F-11.3 | Real filesystem file clone refactoring & patch         | `refactor::tests::test_real_file_clone_refactoring`   | PASS   |
| F-11.4 | Out-of-bounds line range error handling                | `refactor::tests::test_invalid_line_range`            | PASS   |

### Git Differential Scanning Engine (`crates/cddm-core/src/diff.rs`)

| ID     | Feature Variant                                       | Test Function                             | Result |
| :----- | :---------------------------------------------------- | :---------------------------------------- | :----- |
| F-12.1 | Non-git directory differential scan error propagation | `diff::tests::test_diff_scan_non_git_dir` | PASS   |
| F-12.2 | Cross-platform file path normalization for diff match | `diff::tests::test_normalize_path_str`    | PASS   |

### CLI Reporter & Flags (`crates/cddm-cli/src/main.rs`)

| ID     | Feature Variant                                      | Test Function                                          | Result |
| :----- | :--------------------------------------------------- | :----------------------------------------------------- | :----- |
| F-13.1 | `OutputFormat` enum equality & variant parsing       | `main::tests::test_output_format_variants`             | PASS   |
| F-13.2 | CLI SARIF output printing execution                  | `main::tests::test_print_sarif_report_succeeds`        | PASS   |
| F-13.3 | CLI Console and Markdown formatting output execution | `main::tests::test_print_console_and_markdown_reports` | PASS   |
| F-13.4 | CLI Differential scan console & markdown tables      | `main::tests::test_print_diff_reports`                 | PASS   |

### Advanced MCP Server Protocol (`crates/cddm-mcp/src/main.rs`)

| ID     | Feature Variant                                                  | Test Function                                    | Result |
| :----- | :--------------------------------------------------------------- | :----------------------------------------------- | :----- |
| F-14.1 | MCP protocol initialize, version negotiation & serverInfo        | `main::tests::test_mcp_initialize`               | PASS   |
| F-14.2 | MCP ping healthcheck method                                      | `main::tests::test_mcp_ping`                     | PASS   |
| F-14.3 | Tools discovery (`scan_codebase`, `cddm_diff_scan`, refactor)    | `main::tests::test_mcp_tools_list`               | PASS   |
| F-14.4 | Differential scan tool invocation parameter validation           | `main::tests::test_mcp_diff_scan_missing_params` | PASS   |
| F-14.5 | Resources discovery (`cddm://workspace/health` & `clones`)       | `main::tests::test_mcp_resources_list`           | PASS   |
| F-14.6 | Prompts discovery and retrieval (`audit_dry_health`, `refactor`) | `main::tests::test_mcp_prompts_list_and_get`     | PASS   |
| F-14.7 | Standard JSON-RPC 2.0 error handling for invalid/unknown method  | `main::tests::test_mcp_unknown_method`           | PASS   |

---

## 2. WebUI Frontend — React 19 + TypeScript + Vitest (24 unit tests)

| Module          | Test Suite File                                           | Test Cases | Status |
| :-------------- | :-------------------------------------------------------- | :--------- | :----- |
| Store           | `webui/src/store/__tests__/cddm-store.test.ts`            | 7 tests    | PASS   |
| App Shell       | `webui/src/components/__tests__/App.test.tsx`             | 3 tests    | PASS   |
| Config Panel    | `webui/src/components/__tests__/ScanConfigPanel.test.tsx` | 4 tests    | PASS   |
| Progress Bar    | `webui/src/components/__tests__/ScanProgressBar.test.tsx` | 3 tests    | PASS   |
| Results View    | `webui/src/components/__tests__/ScanResults.test.tsx`     | 3 tests    | PASS   |
| Clone Pair Card | `webui/src/components/__tests__/ClonePairCard.test.tsx`   | 2 tests    | PASS   |
| Type System     | `webui/src/types/__tests__/cddm-types.test.ts`            | 2 tests    | PASS   |

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
