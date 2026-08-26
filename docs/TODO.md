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

### Milestone v1.2.0: Language Server Protocol & IDE Extensions

- [x] **[EP-12] Real-Time Language Server Protocol (LSP) Engine & VS Code Extension**
  - [x] Implement `crates/cddm-lsp` powered by `tower-lsp` (v0.20.0) over Stdio
  - [x] Implement real-time clone diagnostics (`textDocument/publishDiagnostics`) with counterpart `relatedLocations`
  - [x] Implement quick-fix Code Actions (`textDocument/codeAction`) for instant function extraction
  - [x] Implement rich Markdown hover tooltip cards (`textDocument/hover`)
  - [x] Implement jump navigation between counterpart clone sites (`textDocument/definition`, `references`)
  - [x] Add `cddm lsp` subcommand in `crates/cddm-cli`
  - [x] Implement official VS Code / Cursor extension client in `editors/vscode` using `vscode-languageclient` (v10.1.0)
  - [x] Provide multi-editor drop-in configuration guides in `docs/LSP_SETUP.md` (VS Code, Neovim, Zed, Helix, Sublime)

---

### Milestone v1.3.0: Historical Duplication Trends & Turnkey CI/CD Workflow Generator

- [x] **[EP-13] In-Process Git History Revision Walking & Timeline Duplication Trends**
  - [x] Implement `cddm_core::timeline::collect_git_timeline` sampling historical commits via `gix::rev_walk`
  - [x] In-memory winnowing tokenization per historical commit tree with directory ignore filtering
  - [x] Compute `TimelineSnapshot` history and `TimelineTrend` with score delta and file churn hotspots
  - [x] Implement `cddm trend [DIR] [--max-samples <N>] [--format console|json|markdown]` CLI subcommand
  - [x] Expose Axum REST endpoint `GET /api/timeline` in `cddm-cli::serve`
  - [x] Expose MCP tool `cddm_get_timeline` and resource `cddm://workspace/timeline` in `cddm-mcp`
  - [x] Implement `TimelineExplorerModal.tsx` in WebUI with interactive SVG trajectory chart and commit snapshots table
- [x] **[EP-14] Turnkey CI/CD Workflow & Git Hook Generator**
  - [x] Implement `cddm_core::workflow::generate_github_workflow`, `generate_gitlab_ci`, `generate_azure_pipelines`
  - [x] Implement `cddm_core::workflow::install_git_hook`, `uninstall_git_hook`, `get_hook_status`
  - [x] Implement `cddm init <github|gitlab|azure> [--write]` CLI subcommand
  - [x] Implement `cddm hook <install|uninstall|status>` CLI subcommand
  - [x] Expose Axum REST endpoints `GET /api/workflow/hooks` and `POST /api/workflow/hooks/install`

### Milestone v1.4.0: Intelligent AST Suppression Engine & Interactive Auto-Refactor Sandbox

- [x] **[EP-15] Intelligent AST Suppression & `.cddmignore` Engine**
  - [x] Implement `.cddmignore` glob rule parsing with per-path `[threshold]` and `[type-filter]` overrides in `crates/cddm-core/src/suppression.rs`
  - [x] Implement inline AST comment directives (`// cddm:ignore`, `/* cddm:ignore-start */ ... /* cddm:ignore-end */`, `#[cddm(allow_duplication)]`, `@cddm_ignore`)
  - [x] Implement automatic test, mock, and auto-generated content header detection (`@generated`, `DO NOT EDIT`)
  - [x] Add CLI flags `--cddmignore`, `--ignore-tests`, `--ignore-mocks`, `--ignore-generated` to `scan` and `diff`
  - [x] Add `cddm ignore init` and `cddm ignore check` CLI subcommands
  - [x] Expose Axum REST endpoints `GET /api/suppression/rules` and `POST /api/suppression/rules`
  - [x] Expose MCP tool `cddm_check_suppression` and resource `cddm://workspace/suppressions`
  - [x] Implement WebUI `SuppressionRulesModal.tsx` with category filters, raw editor, and inline directives guide
- [x] **[EP-16] Interactive Auto-Refactor Sandbox & Transactional Git Branching Studio**
  - [x] Implement `cddm_core::refactor::preview_cluster_refactor` with custom function names, target module paths, and parameter variance extraction
  - [x] Implement `cddm_core::refactor::apply_cluster_refactor_branch` with transactional `gix` Git branch creation
  - [x] Expose Axum REST endpoints `POST /api/refactor/sandbox` and `POST /api/refactor/apply-branch`
  - [x] Expose MCP tool `cddm_apply_cluster_refactor`
  - [x] Implement WebUI `RefactorSandboxModal.tsx` with live syntax-colored diff preview, lines saved badges, and "Apply to Git Branch" button

### Milestone v1.5.0: Polyglot AST Expansion & AI Refactoring Prompt Synthesizer

- [x] **[EP-17] Polyglot Tree-sitter Grammar Expansion (Ruby, PHP, Swift, Bash, Lua, JSON, HTML)**
  - [x] Integrate `tree-sitter-ruby`, `tree-sitter-php`, `tree-sitter-swift`, `tree-sitter-bash`, `tree-sitter-lua`, `tree-sitter-json`, and `tree-sitter-html` dependencies
  - [x] Register language grammar keywords, extensions, and comment delimiters in `crates/cddm-core/src/grammar.rs`
  - [x] Dispatch Tree-sitter parsers in `crates/cddm-core/src/ast/parser.rs` and verify AST parsing
- [x] **[EP-18] AI-Augmented Refactoring Prompt Synthesizer & Context Exporter**
  - [x] Implement `cddm_core::ai_prompt::generate_ai_refactor_prompt` generating structured prompt specifications for AI coding assistants
  - [x] Add `--prompt` CLI flag to `cddm refactor` command
  - [x] Expose Axum REST endpoint `POST /api/refactor/ai-prompt` in `crates/cddm-cli/src/serve.rs`
  - [x] Expose MCP tool `cddm_generate_ai_prompt` in `crates/cddm-mcp/src/main.rs`
  - [x] Add "Copy AI Prompt" action button in `RefactorSandboxModal.tsx` in CDDM Studio WebUI
- [x] **[EP-19] Turnkey PR/MR Markdown Quality Gate Comment Generator**
  - [x] Implement `cddm_core::pr_comment::generate_pr_markdown_comment` with DRY Health Score, threshold evaluation, and clone summary table
  - [x] Add `cddm comment [DIR] [--fail-threshold <N>] [--platform github|gitlab|azure] [--output <PATH>]` CLI subcommand

### Milestone v1.6.0: AST-Native Rewrite Engine & Type-Aware Automated Refactoring

- [x] **[EP-20] AST-Native Rewrite Engine with Inferred Typing & CST Node Substitutions**
  - [x] Implement parameter type inference and language-specific signature formatting in `crates/cddm-core/src/ast/type_infer.rs`
  - [x] Implement module import statement synthesizer & deduplication in `crates/cddm-core/src/ast/import_resolver.rs`
  - [x] Implement AST CST node replacement & syntax validator in `crates/cddm-core/src/ast/rewriter.rs`
  - [x] Implement multi-file AST cluster refactoring engine in `crates/cddm-core/src/refactor.rs`
  - [x] Add CLI flags `--ast`, `--fn-name`, `--target-module` to `cddm refactor` command in `crates/cddm-cli/src/main.rs`
  - [x] Expose Axum REST endpoint `POST /api/refactor/ast` in `crates/cddm-cli/src/serve.rs`
  - [x] Expose MCP tool `cddm_ast_refactor` in `crates/cddm-mcp/src/main.rs`
  - [x] Implement AST-Native Rewrite tab and inferred parameter badges in `RefactorSandboxModal.tsx`
- [x] **[EP-21] Closed-Loop Test Suite Verification Runner**
  - [x] Implement automated test suite execution and result capture in `crates/cddm-core/src/refactor.rs`
  - [x] Add CLI flags `--verify` and `--test-cmd` to `cddm refactor` command in `crates/cddm-cli/src/main.rs`
  - [x] Expose Axum REST endpoint `POST /api/refactor/verify` in `crates/cddm-cli/src/serve.rs`
  - [x] Expose MCP tool `cddm_verify_refactor` in `crates/cddm-mcp/src/main.rs`
  - [x] Implement "Run Test Verification" button and interactive status output in `RefactorSandboxModal.tsx`

---

### Milestone v1.7.0: Architectural Boundary Policy Engine & Polyglot Expansion

- [x] **[EP-22] Architecture Boundary & Anti-Duplication Policy Engine (`.cddmrules.toml`)**
  - [x] Implement `PolicyEngine`, `BoundaryRule`, `ZeroDuplicationRule`, `LimitRule` in `crates/cddm-core/src/policy.rs`
  - [x] Integrate policy evaluation into scan execution pipeline in `crates/cddm-core/src/detector.rs`
  - [x] Map policy violations to SARIF 2.1.0 rules (`CDDM_BOUNDARY`, `CDDM_ZERO_DUP`, `CDDM_LIMIT`) in `sarif.rs`
  - [x] Add CLI subcommand `cddm rules init` and `cddm rules check` plus `--rules` and `--enforce-policies` flags in `crates/cddm-cli/src/main.rs`
  - [x] Expose Axum REST endpoints `GET/POST /api/policy/rules` and `POST /api/policy/evaluate` in `crates/cddm-cli/src/serve.rs`
  - [x] Surface LSP policy diagnostics in `crates/cddm-lsp/src/diagnostics.rs`
  - [x] Expose MCP tool `cddm_check_policies` and MCP resource `cddm://workspace/policies` in `crates/cddm-mcp/src/main.rs`
  - [x] Implement `PolicyRulesModal.tsx` Studio visualizer with active policy inspector and live TOML editor in `webui/`
- [x] **[EP-23] Polyglot Language Expansion (Kotlin, Zig, Scala, Elixir, SQL, Dockerfile)**
  - [x] Integrate Tree-sitter parsers (`tree-sitter-kotlin-ng`, `tree-sitter-zig`, `tree-sitter-scala`, `tree-sitter-elixir`, `tree-sitter-sequel`, `tree-sitter-containerfile`)
  - [x] Implement keyword lexers, line/block comment strippers, and grammar definitions in `crates/cddm-core/src/grammar.rs`
  - [x] Add AST parser dispatch branches in `crates/cddm-core/src/ast/parser.rs`
  - [x] Add unit tests verifying parsing and clone detection across all 6 new languages in `crates/cddm-core/`

---

### Milestone v1.8.0: AI Code Surgeon & Autonomous Self-Healing Refactoring Engine

- [x] **[EP-24] Autonomous AI Code Surgeon & Closed-Loop Healing Engine**
  - [x] Implement `AiProvider` async trait with Gemini, Claude, OpenAI, Ollama, and Mock providers in `crates/cddm-core/src/ai/provider.rs`
  - [x] Implement closed-loop error feedback prompting and transactional patch repair in `crates/cddm-core/src/ai/heal.rs`
  - [x] Add CLI subcommand `cddm heal` in `crates/cddm-cli/src/commands/heal.rs`
  - [x] Expose Axum REST endpoint `POST /api/refactor/heal` in `crates/cddm-cli/src/serve/refactor_handlers.rs`
  - [x] Expose MCP tool `cddm_heal_refactor` in `crates/cddm-mcp/src/tools/refactor_tools.rs`
  - [x] Implement WebUI Studio Auto-Heal tab in `webui/src/components/sandbox/AutoHealTab.tsx`

---

### Milestone v1.9.0: Deep Semantic Graph Matching (PDG/CFG) & Monorepo Distributed Cache

- [x] **[EP-25] Deep Semantic Graph Matching (CFG/PDG & Weisfeiler-Lehman Graph Isomorphism)**
  - [x] Implement CFG extraction from AST in `crates/cddm-core/src/semantic_graph/cfg.rs`
  - [x] Implement PDG variable def-use data dependency graph in `crates/cddm-core/src/semantic_graph/pdg.rs`
  - [x] Implement Weisfeiler-Lehman graph kernel hashing and structural clone similarity in `crates/cddm-core/src/semantic_graph/isomorphism.rs`
  - [x] Expose MCP resource `cddm://workspace/semantic_graph` in `crates/cddm-mcp/src/resources/`
- [x] **[EP-26] Monorepo Multi-Workspace Scanner & Distributed Cache Archive (`.cddmpack`)**
  - [x] Implement portable `.cddmpack` export and import with SHA-256 integrity validation in `crates/cddm-core/src/cache/pack.rs`
  - [x] Implement monorepo multi-workspace discovery (Cargo, npm, pnpm, yarn, bun, Go, Gradle, Lerna, Turborepo, Nx) in `crates/cddm-core/src/monorepo.rs`
  - [x] Add CLI commands `cddm cache export`, `cddm cache import`, and `cddm monorepo` in `crates/cddm-cli/src/commands/`
  - [x] Expose Axum REST endpoints `/api/cache/export`, `/api/cache/import`, and `/api/monorepo` in `crates/cddm-cli/src/serve/`
  - [x] Expose MCP tools `cddm_export_cache_pack`, `cddm_import_cache_pack`, and `cddm_scan_monorepo` in `crates/cddm-mcp/src/tools/`

---

### Milestone v2.0.0: Ecosystem Packaging, Distribution & JetBrains Integration

- [x] **[EP-27] Cross-Platform Ecosystem Distribution & Standalone Installers**
  - [x] Create Homebrew Formula in `packaging/homebrew/cddm.rb`
  - [x] Create Scoop Windows manifest in `packaging/scoop/cddm.json`
  - [x] Create Windows Package Manager (Winget) manifest in `packaging/winget/GrigorTonikyan.cddm.yaml`
  - [x] Create cross-platform curl-to-sh standalone installer in `packaging/install.sh`
  - [x] Create Windows PowerShell standalone installer in `packaging/install.ps1`
  - [x] Implement ecosystem packaging validation script in `scripts/package-distribution.ts`
- [x] **[EP-28] JetBrains IDE Integration (IntelliJ, PyCharm, WebStorm, RustRover, GoLand)**
  - [x] Create comprehensive setup guide and configuration walkthrough in `docs/JETBRAINS_SETUP.md`

---

### Milestone v2.1.0: First-Class IDE & Editor Ecosystem (VS Code Embedded Webview & VSIX Pipeline)

- [x] **[EP-29] VS Code Embedded Webview Studio & Turnkey VSIX Packaging Engine**
  - [x] Implement embedded full-screen Webview panel provider in `editors/vscode/src/webview/studio-panel.ts` (`cddm.openStudioView`)
  - [x] Implement Activity Bar DRY health & duplication sidebar dashboard in `editors/vscode/src/webview/sidebar-provider.ts` (`cddm.sidebarView`)
  - [x] Expand LSP document selectors and activation events to all 24 polyglot languages in `editors/vscode/src/extension.ts` and `constants.ts`
  - [x] Add command palette suite (`cddm.showHealth`, `cddm.checkPolicies`, `cddm.exportSarif`, `cddm.openLocation`) in `commands/actions.ts`
  - [x] Implement zero-dependency cross-platform VSIX packaging and validation engine in `scripts/package-vscode.ts`
  - [x] Implement standard Open Packaging Conventions ZIP archive builder in `scripts/lib/zip-builder.ts`
  - [x] Integrate VS Code packaging into `package-distribution.ts`, `sync-version.ts`, and full verification suite `scripts/verify.ts`

---

### Milestone v2.3.0: Cross-Language Semantic Matching & Hybrid Embeddings (High Priority)

- [x] **[EP-30] Cross-Language Semantic Matching & Hybrid Embeddings (Type-4 Polyglot Duplication)**
  - [x] Implement subword 3-gram vector embedding engine and sparse cosine similarity in `crates/cddm-core/src/semantic_graph/embedding.rs`
  - [x] Implement polyglot CFG function extraction and canonical slot-normalized PDG variable def-use tracking in `crates/cddm-core/src/semantic_graph/`
  - [x] Implement unified hybrid similarity calculator ($S_{\text{hybrid}} = \alpha \cdot S_{\text{graph}} + (1 - \alpha) \cdot S_{\text{token}}$)
  - [x] Implement workspace cross-language clone scanner (`scan_cross_language_workspace`) in `crates/cddm-core/src/semantic_graph/cross_language.rs`
  - [x] Add CLI subcommand `cddm semantic [DIR]` and `--cross-language` scanning flags in `crates/cddm-cli/`
  - [x] Expose Axum REST endpoint `POST /api/semantic/scan` and dual-language graph comparison in `crates/cddm-cli/src/serve/`
  - [x] Expose MCP tool `cddm_scan_cross_language`, prompt `cross_language_audit`, and resource `cddm://workspace/cross_language_clones` in `crates/cddm-mcp/`
  - [x] Implement WebUI Studio Cross-Language Explorer tab, dual-language Polyglot Sandbox selectors, and `[Polyglot]` clone badges in `webui/`

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
- [x] **v1.2.0**: Real-Time Language Server Protocol (LSP 3.17) Engine (`crates/cddm-lsp`), `cddm lsp` CLI daemon, official VS Code / Cursor Extension (`editors/vscode`), and multi-editor configuration guide (`docs/LSP_SETUP.md`).
- [x] **v1.3.0**: Historical Duplication Trends & Turnkey CI/CD Workflow Generator with `cddm trend`, `cddm hook`, `cddm init`, `cddm_get_timeline` MCP tool, `cddm://workspace/timeline` resource, and WebUI Studio `TimelineExplorerModal`.
- [x] **v1.4.0**: Intelligent AST Suppression Engine & Interactive Auto-Refactor Sandbox Studio with `.cddmignore` glob rules, per-path threshold overrides, inline comment directives (`// cddm:ignore`, `/* cddm:ignore-start */`), test/mock/generated auto-filtering, `cddm ignore` CLI, parameterized refactor sandbox, transactional Git branch application (`gix`), MCP suppression tools/resources, and WebUI `SuppressionRulesModal` & `RefactorSandboxModal`.
- [x] **v1.5.0**: Polyglot AST Expansion (16 Tree-sitter languages: Ruby, PHP, Swift, Bash, Lua, JSON, HTML) & AI Refactoring Prompt Synthesizer with `cddm refactor --prompt`, `POST /api/refactor/ai-prompt`, MCP `cddm_generate_ai_prompt`, WebUI Studio "Copy AI Prompt" button, and turnkey PR/MR markdown quality gate comment generator (`cddm comment`).
- [x] **v1.6.0**: AST-Native Rewrite Engine & Type-Aware Automated Refactoring with Tree-sitter parameter typing, import synthesis, CST node substitution, closed-loop test verification (`cddm refactor --ast --verify`), Axum endpoints (`/api/refactor/ast`, `/api/refactor/verify`), MCP tools (`cddm_ast_refactor`, `cddm_verify_refactor`), and WebUI Studio AST-Native tabs & test runner panel.
- [x] **v1.7.0**: Architectural Boundary & Anti-Duplication Policy Engine (`.cddmrules.toml`) and Polyglot Language Expansion (Kotlin, Zig, Scala, Elixir, SQL, Dockerfile) across core engine, CLI (`rules init`, `rules check`, `--rules`, `--enforce-policies`), REST API (`/api/policy/*`), LSP diagnostics, MCP server (`cddm_check_policies`, `cddm://workspace/policies`), SARIF reporting (`CDDM_BOUNDARY`, `CDDM_ZERO_DUP`, `CDDM_LIMIT`), and WebUI Policy Studio modal.
- [x] **v1.8.0**: AI Code Surgeon & Autonomous Self-Healing Refactoring Engine (`crates/cddm-core/src/ai`, CLI `cddm heal`, Axum `POST /api/refactor/heal`, MCP `cddm_heal_refactor`, WebUI Auto-Heal tab).
- [x] **v1.9.0**: Deep Semantic Graph Matching (PDG/CFG) & Monorepo Distributed Cache Archive (`.cddmpack`, `crates/cddm-core/src/semantic_graph`, `crates/cddm-core/src/cache/pack.rs`, `crates/cddm-core/src/monorepo.rs`, CLI `cddm cache export/import`, `cddm monorepo`, Axum endpoints, MCP tools).
- [x] **v2.0.0**: Ecosystem Packaging, Distribution & JetBrains Integration (Homebrew Formula, Scoop manifest, Winget manifest, standalone `install.sh`/`install.ps1`, `docs/JETBRAINS_SETUP.md`, `scripts/package-distribution.ts`).
- [x] **v2.1.0**: First-Class IDE & Editor Ecosystem (VS Code Embedded Webview Studio, Activity Bar Dashboard, 24-language polyglot selector, command suite, and turnkey VSIX packaging pipeline).
- [x] **v2.3.0**: Cross-Language Semantic Matching & Hybrid Embeddings (Subword vector embeddings, Weisfeiler-Lehman graph kernels, `cddm semantic`, MCP `cddm_scan_cross_language`, and WebUI Studio Cross-Language Explorer).
