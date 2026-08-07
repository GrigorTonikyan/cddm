# CDDM (Code De-Duplication Meister) — Exhaustive Feature Matrix & Test Mapping

This document provides a 1-to-1 mapping of every feature, variant, test case, and empirical verification status across CDDM.

---

## Feature Matrix & Verification Table

| Feature ID | Feature Area | Atomic Feature Variant | Test Method | Test Location | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **F-01.1** | **Tokenization** | Rust source code tokenization (keywords, fn, let, structs) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.2** | **Tokenization** | TypeScript & JSX tokenization (interfaces, types, arrow functions) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.3** | **Tokenization** | Python source tokenization (def, class, indentation, `#` comments) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.4** | **Tokenization** | Single-line comment stripping (`//`, `#`) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.5** | **Tokenization** | Multi-line block comment stripping (`/* ... */`, `<!-- ... -->`) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.6** | **Tokenization** | String literal normalization (`"..."`, `'...'`, `` `...` ``) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-01.7** | **Tokenization** | Numeric literal normalization (integers, floats, hex) | Unit Test | `crates/cddm-core/src/tokenizer.rs` | ✅ PASSED |
| **F-02.1** | **Fingerprinting** | Fast Mersenne Prime $M_{61} = 2^{61}-1$ modulo reduction | Unit Test | `crates/cddm-core/src/fingerprint.rs` | ✅ PASSED |
| **F-02.2** | **Fingerprinting** | Winnowing rolling hash window calculation | Unit Test | `crates/cddm-core/src/fingerprint.rs` | ✅ PASSED |
| **F-02.3** | **Fingerprinting** | Boundary window handling when $N < w$ | Unit Test | `crates/cddm-core/src/fingerprint.rs` | ✅ PASSED |
| **F-03.1** | **Detection** | Type-1 (Exact) duplicate detection | Integration | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-03.2** | **Detection** | Type-2 (Renamed identifier) duplicate detection | Integration | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-03.3** | **Detection** | Intra-file duplicate scan vs cross-file scan toggle | Integration | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-03.4** | **Detection** | Scan progress events channel emission | Integration | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-03.5** | **Detection** | Empty directory / zero-match scan response | Unit Test | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-03.6** | **Detection** | Scan cancellation flag handling (`AtomicBool`) | Unit Test | `crates/cddm-core/src/detector.rs` | ✅ PASSED |
| **F-04.1** | **Git Blame** | In-process `gix` (`gitoxide`) author name extraction | Unit Test | `crates/cddm-core/src/blame.rs` | ✅ PASSED |
| **F-04.2** | **Git Blame** | Non-git directory fallback graceful degradation | Unit Test | `crates/cddm-core/src/blame.rs` | ✅ PASSED |
| **F-05.1** | **AST Subtree** | Tree-sitter CST parsing for Rust (`tree-sitter-rust`) | Unit Test | `crates/cddm-core/src/ast/parser.rs` | ✅ PASSED |
| **F-05.2** | **AST Subtree** | Tree-sitter CST parsing for TypeScript (`tree-sitter-typescript`) | Unit Test | `crates/cddm-core/src/ast/parser.rs` | ✅ PASSED |
| **F-05.3** | **AST Subtree** | Blake3 Merkle subtree hashing for Type 3/4 clones | Unit Test | `crates/cddm-core/src/ast/hasher.rs` | ✅ PASSED |
| **F-06.1** | **Caching** | Sha256 file content hash comparison | Unit Test | `crates/cddm-core/src/cache.rs` | ✅ PASSED |
| **F-06.2** | **Watcher** | `notify` OS file system event listener initialization | Unit Test | `crates/cddm-core/src/watcher.rs` | ✅ PASSED |
| **F-07.1** | **CLI Reporters** | Console ANSI table reporter (`--format console`) | CLI Test | `crates/cddm-cli/src/main.rs` | ✅ PASSED |
| **F-07.2** | **CLI Reporters** | JSON reporter (`--format json`) | CLI Test | `crates/cddm-cli/src/main.rs` | ✅ PASSED |
| **F-07.3** | **CLI Reporters** | Markdown report formatter (`--format markdown`) | CLI Test | `crates/cddm-cli/src/main.rs` | ✅ PASSED |
| **F-07.4** | **CLI Flags** | Fail threshold exit code enforcement (`--fail-threshold`) | CLI Test | `crates/cddm-cli/src/main.rs` | ✅ PASSED |
| **F-08.1** | **Studio WebUI** | Axum HTTP server initialization (`cddm serve`) | Server Test | `crates/cddm-cli/src/serve.rs` | ✅ PASSED |
| **F-08.2** | **Studio WebUI** | Embedded static asset serving (`rust-embed` `WebUIAssets`) | Server Test | `crates/cddm-cli/src/serve.rs` | ✅ PASSED |
| **F-08.3** | **Studio WebUI** | REST API `/api/health` health check endpoint | Server Test | `crates/cddm-cli/src/serve.rs` | ✅ PASSED |
| **F-08.4** | **Studio WebUI** | REST API `/api/scan` JSON scan endpoint | Server Test | `crates/cddm-cli/src/serve.rs` | ✅ PASSED |
| **F-09.1** | **React Frontend** | `ScanConfigPanel` UI inputs & token threshold slider | WebUI Test | `webui/src/components/ScanConfigPanel.tsx` | ✅ PASSED |
| **F-09.2** | **React Frontend** | `ScanProgressBar` phase progress & percentage bar | WebUI Test | `webui/src/components/ScanProgressBar.tsx` | ✅ PASSED |
| **F-09.3** | **React Frontend** | `ScanResults` DRY health score gauge & language breakdown | WebUI Test | `webui/src/components/ScanResults.tsx` | ✅ PASSED |
| **F-09.4** | **React Frontend** | `ClonePairCard` side-by-side split diff card & author tags | WebUI Test | `webui/src/components/ClonePairCard.tsx` | ✅ PASSED |
| **F-10.1** | **MCP Server** | Stdio JSON-RPC 2.0 protocol request handler | MCP Test | `crates/cddm-mcp/src/main.rs` | ✅ PASSED |
| **F-10.2** | **MCP Server** | `initialize` request handler & server capabilities | MCP Test | `crates/cddm-mcp/src/main.rs` | ✅ PASSED |
| **F-10.3** | **MCP Server** | `tools/list` schema declaration (`scan_codebase`) | MCP Test | `crates/cddm-mcp/src/main.rs` | ✅ PASSED |
| **F-10.4** | **MCP Server** | `tools/call` tool execution & JSON scan response | MCP Test | `crates/cddm-mcp/src/main.rs` | ✅ PASSED |
| **F-11.1** | **npm Packaging** | Binary shim runner script `npm/cddm/bin/cddm.js` | Integration | `npm/cddm/bin/cddm.js` | ✅ PASSED |
