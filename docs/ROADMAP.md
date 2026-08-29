# CDDM Engineering Roadmap & Strategic Enhancement Proposals

> **Document Status**: Active / Living RFC Reference  
> **Repository**: [GrigorTonikyan/cddm](https://github.com/GrigorTonikyan/cddm)  
> **Last Updated**: 2026-08-23

---

## 1. Strategic Release Milestones

```text
+----------------------------------------------------------------------------------------------------+
|  v0.1.2 (Current)     v0.2.0 (CI & MCP)       v0.3.0 (Cache & Diff)   v0.4.0 (Studio & Visuals)    v1.0.0 (Production Engine)  |
|  - Rust M61 Engine    - SARIF standard output - Persistent disk cache - Split Monaco diff view     - Type-4 Semantic AST clones|
|  - Winnowing Types 1/2- MCP Agent Context tool- Git branch diffing    - Duplication Treemaps       - SIMD M61 Vectorization    |
|  - Axum Studio WebUI  - GitHub Action runner  - Auto patch synthesis  - Trend evolution graph      - Enterprise Polyglot Engine|
+----------------------------------------------------------------------------------------------------+
```

| Milestone  | Target Horizon | Strategic Focus                              | Key Deliverables                                                                                                           |
| :--------- | :------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| **v0.2.0** | Short-term     | CI/CD Integration & AI Agent Tooling         | SARIF `--format sarif`, expanded MCP tools (`get_clone_context`, `suggest_refactor`), official GitHub Action.              |
| **v0.3.0** | Mid-term       | Caching, Differential Scans & Refactoring    | Embedded `redb` disk cache, `cddm diff <branch>`, automated patch synthesis (`cddm refactor`).                             |
| **v0.4.0** | Mid-term       | WebUI Studio & Visual Analytics              | Side-by-side Monaco diff visualizer, D3 hierarchical duplication treemap, historical Git trend graph.                      |
| **v0.5.0** | Long-term      | AST Pipeline & Extended Polyglot             | Integrated AST Merkle subtree matching, Type-3 near-miss detection, Go, C/C++, Java Tree-sitter parsers.                   |
| **v1.0.0** | Stable Release | High-Throughput Enterprise Engine            | AVX2/NEON SIMD vectorization, memory-mapped zero-copy I/O, semantic AST graph clones (Type-4).                             |
| **v1.1.0** | Stable Release | N-Way Clustering & Multi-Site Deduplication  | Disjoint-Set Union-Find clustering, multi-site patch synthesis, N-way cluster cards, Axum cluster endpoint.                |
| **v1.2.0** | Stable Release | Language Server Protocol & IDE Extensions    | Full LSP 3.17 daemon (`crates/cddm-lsp`), official VS Code extension, inline diagnostics & code actions.                   |
| **v1.3.0** | Stable Release | Historical Trends & Turnkey Workflows        | Git timeline duplication trajectories (`cddm trend`), turnkey CI workflow generator (`cddm init`, `hook`).                 |
| **v1.4.0** | Stable Release | AST Suppressions & Refactor Sandbox          | Intelligent `.cddmignore` engine, inline comment directives, interactive WebUI refactoring sandbox studio.                 |
| **v1.5.0** | Stable Release | Polyglot Expansion & AI Prompt Synthesizer   | 16 Tree-sitter AST grammars (Ruby, PHP, Swift, Bash, Lua, JSON, HTML), AI refactor prompt engine, PR comments.             |
| **v1.6.0** | Stable Release | AST-Native Rewrite & Test Verification       | Tree-sitter CST node substitution, type-aware helper synthesis, import generation, closed-loop test runner.                |
| **v1.7.0** | Stable Release | Boundary Policies & Polyglot Expansion       | Architectural `.cddmrules.toml`, boundary isolation, zero-dup zones, limits, Kotlin/Zig/Scala/Elixir/SQL/Docker.           |
| **v1.8.0** | Stable Release | AI Code Surgeon & Self-Healing Refactor      | Closed-loop autonomous test feedback loop, multi-provider engine (Gemini/Claude/OpenAI/Ollama), `cddm heal`.               |
| **v1.9.0** | Stable Release | Deep Semantic Graph & Monorepo Cache         | CFG/PDG extraction, Weisfeiler-Lehman graph kernels, monorepo multi-workspace scanner, portable `.cddmpack`.               |
| **v2.0.0** | Major Release  | Ecosystem Packaging & JetBrains Integration  | Homebrew, Scoop, Winget, standalone installers (`install.sh`/`install.ps1`), JetBrains LSP integration guide.              |
| **v2.1.0** | Major Release  | IDE & Editor Ecosystem & VSIX Pipeline       | VS Code embedded Webview Studio panel, Activity Bar dashboard, 24 polyglot languages, and turnkey VSIX packager.           |
| **v2.3.0** | Stable Release | Cross-Language Semantic & Hybrid Embeddings  | Subword vector embeddings, Weisfeiler-Lehman graph kernels, `cddm semantic`, MCP & WebUI polyglot clone explorer.          |
| **v2.4.0** | Stable Release | Automated Shared Module & Crate Extraction   | Inferred shared crate generator, manifest mutator, callsite rewriter (`cddm extract`).                                     |
| **v2.5.0** | Stable Release | Interactive Terminal UI Studio & Parity Gate | Terminal UI Studio (`cddm tui`) with 8 tab views & split diff, crossterm/ratatui, 4-pillar parity policy & CI gate.        |
| **v2.6.0** | Stable Release | Polyglot AST Rewriters & Shared Extraction   | Manifest mutators (`pyproject.toml`, `go.mod`, `pom.xml`, `.csproj`), typed package gen, polyglot import/return inference. |

---

## 2. Detailed Enhancement Proposals (EP-01 to EP-10)

### EP-01: SARIF Standard Reporter & GitHub Code Scanning Integration

- **Target Milestone**: `v0.2.0`
- **Component**: `crates/cddm-cli`, `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Completed (v0.2.0)`

#### Problem Statement

CI/CD workflows currently support JSON, Markdown, and Console table formats. GitHub Code Scanning and IDE diagnostic viewers require standard SARIF (Static Analysis Results Interchange Format, OASIS Standard) to render inline warnings on pull requests.

#### Specification & Architecture

1. Add `OutputFormat::Sarif` variant to CLI options in `crates/cddm-cli/src/main.rs`.
2. Map each `ClonePair` to a SARIF `result` object:
   - Rule ID: `CDDM001` (Exact Clone), `CDDM002` (Renamed Clone), `CDDM003` (Near-Miss Clone).
   - Level: `warning` (or `error` if duplication exceeds `--fail-threshold`).
   - Locations: Physical file paths and 1-based start/end line numbers with secondary locations pointing to matched counterpart files.
3. Emit compliant SARIF 2.1.0 JSON schema.

#### Acceptance Criteria

- `cddm scan . --format sarif` emits valid SARIF 2.1.0 validated against official OASIS JSON Schema.
- Uploading SARIF file to GitHub Code Scanning via `github/codeql-action/upload-sarif@v3` surfaces inline PR annotations.

---

### EP-02: Advanced MCP Agentic Toolset & Resource Protocol

- **Target Milestone**: `v0.2.0`
- **Component**: `crates/cddm-mcp`
- **Priority**: `High`
- **Status**: `Completed (v0.2.0)`

#### Problem Statement

`cddm-mcp` currently exposes a single `scan_codebase` tool. AI coding assistants (e.g. Antigravity, Claude Desktop, Cursor) need granular tools to inspect clone AST context, evaluate proposed refactorings, and subscribe to background workspace duplication updates.

#### Specification & Architecture

1. **New Tools**:
   - `cddm_get_clone_pair`: Accepts `clone_id` or `(file_a, line_a)` and returns the full syntax-highlighted token streams, line content, and git blame author metadata.
   - `cddm_suggest_refactor`: Generates a structured deduplication recommendation (parameterized signature, extracted function body, target module destination).
   - `cddm_compare_revisions`: Compares duplication between two git commits or working tree vs HEAD.
2. **MCP Resources**:
   - `cddm://workspace/health`: Read-only JSON resource exposing current DRY Health Score, total files, and language stats.
   - `cddm://workspace/clones`: Queryable list of active clone pairs.

#### Acceptance Criteria

- AI assistants can invoke `cddm_get_clone_pair` to retrieve contextual lines without running a full re-scan.
- Server passes MCP protocol compatibility tests for tools and resources.

---

### EP-03: Embedded Persistent Disk-Backed Fingerprint Cache

- **Target Milestone**: `v0.3.0`
- **Component**: `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Completed (v0.3.0)`

#### Problem Statement

Currently, `cddm` parses and tokenizes every file on every scan. In large enterprise repositories (50,000+ files), scanning takes 200–500ms. Re-scanning after modifying a single file should take `< 30ms`.

#### Specification & Architecture

1. Embed `redb` (pure Rust, zero-dependency ACID key-value store) or `sled` into `cddm-core`.
2. Cache file location: `.cddm/cache.db` (customizable via `--cache-dir`).
3. Store schema:
   - Key: `blake3(relative_path + file_content)` or `(mtime_seconds, file_size)`.
   - Value: Serialized token stream, `LineSpan` offsets, and precomputed `Fingerprint` list.
4. During Discovery phase:
   - Check file metadata / content hash against cache.
   - Skip tokenization and winnowing for unmodified files.
5. Invalidate entries for modified or deleted files.

#### Acceptance Criteria

- Incremental scan on 50,000 LOC codebase with 1 modified file completes in `< 30ms`.
- Cache integrity automatically self-heals when `--no-cache` or corrupt database is detected.

---

### EP-04: Interactive Side-by-Side Monaco Diff Visualizer in WebUI

- **Target Milestone**: `v0.4.0`
- **Component**: `webui/`, `crates/cddm-cli`
- **Priority**: `Medium`
- **Status**: `Completed (v0.4.0)`

#### Problem Statement

The WebUI Studio currently displays static line ranges and author annotations for clone pairs. Developers need an interactive split diff with syntax highlighting, token-level matching highlights, and synchronized scrolling.

#### Specification & Architecture

1. Implement `DiffViewer.tsx` into `ClonePairCard.tsx` with split and unified modes.
2. Fetch snippet contents via REST endpoint `GET /api/snippet?file=path&start=N&end=M&context=4`.
3. Render Fragment A on the left and Fragment B on the right with synchronized scrolling and line highlighting.
4. Provide copy buttons for extracting duplicate and invariant code.

#### Acceptance Criteria

- ClonePair cards expand to display interactive split diffs with syntax highlighting.
- Responsive on desktop and tablet viewport widths.

---

### EP-05: Tree-sitter AST Merkle Pipeline Integration for Type-3 & Type-4 Clones

- **Target Milestone**: `v0.5.0`
- **Component**: `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Completed (v0.5.0)`

#### Problem Statement

Lexical winnowing detects identical and renamed token streams (Type-1 and Type-2). Clones with inserted/deleted statements (Type-3 near-miss) or structurally identical control flow with different syntax (Type-4 semantic) require AST-level matching.

#### Specification & Architecture

1. Elevate `cddm-core::ast::hasher` to a first-class scan phase (`ScanPhase::AstAnalysis`).
2. Construct Tree-sitter Concrete Syntax Trees (CST) and compute Blake3 Merkle subtree hashes for nodes with `depth >= min_depth` (functions, match blocks, class definitions).
3. Compute AST Edit Distance (Zhang-Shasha tree distance) on candidate clusters to calculate structural similarity percentages.
4. Classify clone pairs into `CloneType::NearMiss` and `CloneType::Semantic`.

#### Acceptance Criteria

- Detects functions containing up to 20% modified statements as `CloneType::NearMiss`.
- Correctly reports structural similarity percentage and token differences.

---

### EP-06: Duplication Treemap & Hierarchy Analytics in WebUI

- **Target Milestone**: `v0.4.0`
- **Component**: `webui/`
- **Priority**: `Medium`
- **Status**: `Completed (v0.4.0)`

#### Problem Statement

Developers need a high-level visual representation of where duplication is clustered across nested directories, crates, and modules.

#### Specification & Architecture

1. Add Squarified Treemap layout in `DuplicationTreemap.tsx` integrated in `ScanResults.tsx`.
2. Node size represents total token volume; node color represents local duplication rate (emerald for < 5%, amber for 5-15%, rose for > 15%).
3. Clicking a directory node zooms into subdirectories and filters the clone pair list with breadcrumb navigation.

#### Acceptance Criteria

- Treemap renders smoothly on large codebases (> 5,000 files) with interactive drill-down navigation.

---

### EP-07: Automated Refactoring & Patch Synthesis Engine (`cddm refactor`)

- **Target Milestone**: `v0.3.0`
- **Component**: `crates/cddm-cli`, `crates/cddm-core`
- **Priority**: `Medium`
- **Status**: `Completed (v0.3.0)`

#### Problem Statement

Identifying clones is the first step; refactoring them requires extracting common functions and parameterizing variable identifiers.

#### Specification & Architecture

1. Add CLI subcommand `cddm refactor [OPTIONS]`.
2. Analyze selected clone pair:
   - Identify invariant token sequence (the extracted function body).
   - Identify variable identifiers (function arguments/parameters).
   - Propose a deduplicated function definition in a shared module.
3. Output standard Git `.patch` file or apply modifications directly with user confirmation.

#### Acceptance Criteria

- `cddm refactor --pair <ID>` generates a valid `.patch` that compiles cleanly.

---

### EP-08: Differential Scanning & Branch Comparison (`cddm diff`)

- **Target Milestone**: `v0.3.0`
- **Component**: `crates/cddm-cli`, `crates/cddm-core`
- **Priority**: `Medium`
- **Status**: `Completed (v0.3.0)`

#### Problem Statement

In CI pipelines, developers want to ensure that a pull request does not introduce _new_ duplication, even if legacy duplication exists in the repository.

#### Specification & Architecture

1. Add `cddm diff <BASE_REF> [TARGET_REF]` subcommand.
2. Use in-process `gix` to scan files changed in the git revision range (`git diff --name-only <BASE>..<TARGET>`).
3. Differentiate between:
   - Existing legacy clones.
   - Newly introduced clones.
   - Eliminated / resolved clones.
4. Exit code 1 only if _new_ duplication is introduced.

#### Acceptance Criteria

- `cddm diff main` reports net DRY delta (`+1.2% DRY` or `-0.5% DRY`) and lists newly added clone pairs.

---

### EP-09: Polyglot Language Registry Expansion

- **Target Milestone**: `v0.5.0`
- **Component**: `crates/cddm-core`
- **Priority**: `Medium`
- **Status**: `Completed (v0.5.0)`

#### Problem Statement

Extend full Tree-sitter AST parsing support beyond Rust, TypeScript, JavaScript, and Python.

#### Languages Added

- **Go**: `tree-sitter-go`
- **C / C++**: `tree-sitter-c`, `tree-sitter-cpp`
- **Java**: `tree-sitter-java`
- **C#**: `tree-sitter-c-sharp`

#### Acceptance Criteria

- 100% test coverage for new grammars with verified comment stripping and keyword tokenization.

---

### EP-10: Memory-Mapped I/O & SIMD Mersenne-61 Hash Vectorization

- **Target Milestone**: `v1.0.0`
- **Component**: `crates/cddm-core`
- **Priority**: `Low`
- **Status**: `Completed (v1.0.0)`

#### Problem Statement

Maximizing token throughput on multi-gigabyte codebases to achieve > 20M tokens/second.

#### Specification & Architecture

1. Replace heap file buffer allocation with `memmap2::Mmap` for files > 64 KB.
2. Implement AVX2 and ARM NEON vector lanes for Mersenne 61 rolling polynomial calculation:
   - Compute 4 parallel window hash modulos simultaneously.
3. Provide scalar fallback for architectures without vector extensions.

---

### EP-11: N-Way Clone Graph Clustering & Multi-Site Deduplication Synthesis

- **Target Milestone**: `v1.1.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.1.0)`

#### Problem Statement

Pairwise clone detection scatters $N \ge 3$ duplicate locations across $O(N^2)$ independent pairs, creating redundant clutter and making it tedious to coordinate multi-file refactorings.

#### Specification & Architecture

1. **Connected-Components Graph Clustering (`cddm_core::cluster`)**:
   - Disjoint-Set Union-Find (DSU) with path compression and rank optimization partitions pairwise clone graphs into transitive connected components ($A \sim B \land B \sim C \implies \{A, B, C\}$).
   - Generates canonical `CloneCluster` equivalence classes with unified token metrics, similarity, and occurrence spans.
2. **Multi-Site Consensus Refactoring Synthesizer (`cddm_core::refactor`)**:
   - Computes multi-site consensus invariant lines across all $N$ occurrences.
   - Extracts site-specific parameter diffs and generates unified multi-file `.patch` diffs (`--- a/... +++ b/...`).
3. **Studio API & CLI Support**:
   - Exposes Axum endpoint `POST /api/refactor-cluster`.
   - Adds `--cluster <ID>` option to `cddm refactor` CLI subcommand with console and markdown cluster summaries.
4. **MCP Agentic Integration**:
   - Exposes `cddm_get_clone_cluster` and `cddm_suggest_cluster_refactor` MCP tools.
   - Exposes `cddm://workspace/clusters` real-time queryable MCP resource.
5. **Interactive WebUI Visualizer**:
   - Adds "Pairwise" vs "N-Way Clusters" view mode toggle tabs in `ScanResults.tsx`.
   - Interactive `CloneClusterCard.tsx` with expandable site lists and one-click cluster refactor modal.

#### Acceptance Criteria

- Graph clustering groups transitive multi-way clone chains into single clusters.
- Refactoring engine generates valid multi-file diff patches spanning all occurrences.
- WebUI allows seamless switching between pairwise and clustered duplication views.

---

### EP-12: Real-Time Language Server Protocol (LSP) Engine & VS Code Extension

- **Target Milestone**: `v1.2.0`
- **Component**: `crates/cddm-lsp`, `crates/cddm-cli`, `editors/vscode`
- **Priority**: `High`
- **Status**: `Completed (v1.2.0)`

#### Problem Statement

Developers need inline, real-time code clone feedback inside their code editors without running external CLI commands or opening a separate browser window.

#### Specification & Architecture

1. **Tower-LSP Server Engine (`crates/cddm-lsp`)**:
   - Implements LSP 3.17 protocol standard over JSON-RPC 2.0 Stdio transport.
   - Publishes real-time diagnostics (`textDocument/publishDiagnostics`) with clone severity and counterpart `relatedLocations`.
   - Surfaces quick-fix Code Actions (`textDocument/codeAction`) to extract duplicate code into helper functions.
   - Returns rich Markdown hover cards (`textDocument/hover`) with similarity and token metrics.
   - Supports jump navigation (`textDocument/definition`, `references`) between clone sites.
2. **CLI Subcommand**:
   - Adds `cddm lsp [DIRECTORY]` to launch the LSP daemon directly from terminal.
3. **Official VS Code / Cursor Extension (`editors/vscode`)**:
   - Pure TypeScript extension using `vscode-languageclient` (v10.1.0) with status bar indicators and command bindings.

#### Acceptance Criteria

- LSP daemon starts cleanly over Stdio and responds to standard JSON-RPC 2.0 lifecycle requests.
- Emits accurate diagnostics with counterpart line ranges across all supported languages.
- VS Code extension compiles cleanly and provides commands and status bar health indicators.

---

### EP-13: In-Process Git History Revision Walking & Timeline Duplication Trends

- **Target Milestone**: `v1.3.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.3.0)`

#### Problem Statement

Engineering teams and tech leads need to track whether code duplication is improving or regressing over time across repository commits, identifying high-churn duplicate hotspots.

#### Specification & Architecture

1. **In-Process Git History Traversal (`cddm_core::timeline`)**:
   - Uses `gix` revision walking (`gix::rev_walk`) to sample commits across repository history without external git CLI spawns.
   - Extracts file trees per historical commit and runs in-memory winnowing tokenization with directory ignore filtering.
   - Computes `TimelineSnapshot` records and `TimelineTrend` with score delta and file churn metrics.
2. **CLI Subcommand**:
   - `cddm trend [DIR] [--max-samples <N>] [--format console|json|markdown]` with formatted tables and net score delta.
3. **Axum REST API & MCP Integration**:
   - `GET /api/timeline` endpoint.
   - `cddm_get_timeline` MCP tool and `cddm://workspace/timeline` MCP resource.
4. **WebUI Timeline Visualizer**:
   - Interactive `TimelineExplorerModal.tsx` rendering dual SVG trajectory curves for DRY Health Score and Duplication %, interactive data points, and commit snapshot tables.

#### Acceptance Criteria

- `cddm trend` samples Git history and outputs accurate historical duplication metrics.
- WebUI Studio provides interactive time-series trajectory visualizer.
- MCP tool and resource allow AI assistants to query historical trends.

---

### EP-14: Turnkey CI/CD Workflow & Git Hook Generator

- **Target Milestone**: `v1.3.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.3.0)`

#### Problem Statement

Setting up CI/CD duplication quality gates and local pre-commit hooks manually requires writing custom scripts and workflow files, creating friction for new repositories.

#### Specification & Architecture

1. **Turnkey Workflow Generator (`cddm_core::workflow`)**:
   - Generates GitHub Actions workflow (`.github/workflows/cddm.yml`) with automated OASIS SARIF v2.1.0 upload and PR Markdown summary comments.
   - Generates GitLab CI (`.gitlab-ci.yml`) and Azure DevOps Pipelines (`azure-pipelines.yml`).
2. **Git Hook Lifecycle Manager**:
   - `cddm hook install --type pre-commit|pre-push --fail-threshold 5.0`.
   - `cddm hook uninstall --type pre-commit|pre-push`.
   - `cddm hook status` inspecting `.git/hooks`.
3. **CLI Subcommands**:
   - `cddm init <github|gitlab|azure> [--write]`.
   - `cddm hook <install|uninstall|status>`.
4. **Axum REST API & WebUI**:
   - `GET /api/workflow/hooks` and `POST /api/workflow/hooks/install` endpoints with one-click hook activation in Studio.

#### Acceptance Criteria

- `cddm init github` generates valid GitHub Actions workflow YAML.
- `cddm hook install` creates executable `.git/hooks/pre-commit` script.
- Pre-commit hook enforces `--fail-threshold` before commits.

---

### EP-15: Intelligent AST Suppression & `.cddmignore` Engine

- **Target Milestone**: `v1.4.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.4.0)`

#### Problem Statement

Real-world codebases contain intentional duplication (e.g. test fixtures, mocks, generated protobufs/stubs) that should not inflate duplication percentages or trigger CI gate failures. Fine-grained glob rules and inline AST comment suppression are required.

#### Specification & Architecture

1. **Suppression Engine (`suppression.rs`)**:
   - Parse `.cddmignore` file supporting standard glob exclusions.
   - Per-path threshold overrides (`[threshold] <pattern> min_tokens=N`).
   - Per-path clone type filtering (`[type-filter] <pattern> ignore=Exact,Renamed`).
   - Inline comment directives (`// cddm:ignore`, `/* cddm:ignore-start */ ... /* cddm:ignore-end */`).
   - Rust and Python attributes (`#[cddm(allow_duplication)]`, `# @cddm_ignore`).
   - Auto-generated content header detection (`@generated`, `DO NOT EDIT`).
2. **CLI & REST API**:
   - `cddm ignore init [--force]` and `cddm ignore check <PATH> [--line <N>]`.
   - `GET /api/suppression/rules` and `POST /api/suppression/rules`.
3. **MCP Tool & Resource**:
   - `cddm_check_suppression` tool and `cddm://workspace/suppressions` resource.
4. **WebUI**:
   - `SuppressionRulesModal.tsx` with category filters, raw editor, and inline directives guide.

---

### EP-16: Interactive Auto-Refactor Sandbox & Transactional Git Branching Studio

- **Target Milestone**: `v1.4.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.4.0)`

#### Problem Statement

Consensus refactoring recommendations need a playground where developers can customize extracted function names, choose destination module paths, verify parameter variance across occurrence sites, and apply patches directly to dedicated Git branches.

#### Specification & Architecture

1. **Customized Refactoring Engine (`refactor.rs`)**:
   - `preview_cluster_refactor` accepts custom function names and destination module paths.
   - Live diff hunk calculation and lines saved estimations.
2. **Transactional Git Branch Application**:
   - `apply_cluster_refactor_branch` creates and checks out a new branch (`cddm/refactor-...`) using `gix` before applying hunks.
3. **Axum Endpoints & MCP Tools**:
   - `POST /api/refactor/sandbox` and `POST /api/refactor/apply-branch`.
   - `cddm_apply_cluster_refactor` MCP tool.
4. **WebUI Studio**:
   - `RefactorSandboxModal.tsx` with live parameter inputs, lines saved metrics, colorized diff preview, and one-click "Apply to Git Branch" action.

---

### EP-17: Polyglot Tree-sitter Grammar Expansion (Ruby, PHP, Swift, Bash, Lua, JSON, HTML)

- **Target Milestone**: `v1.5.0`
- **Component**: `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Completed (v1.5.0)`

#### Problem Statement

CDDM supports 9 core languages with native Tree-sitter parsers. Modern polyglot engineering architectures require deep AST structural deduplication across Ruby, PHP, Swift, Bash scripts, Lua game engines, and structural markup (JSON, HTML).

#### Specification & Architecture

1. **Tree-sitter Grammars**:
   - Register `tree-sitter-ruby`, `tree-sitter-php`, `tree-sitter-swift`, `tree-sitter-bash`, `tree-sitter-lua`, `tree-sitter-json`, and `tree-sitter-html` dependencies.
2. **Language Registry**:
   - Map extensions (`.rb`, `.rake`, `.gemspec`, `.php`, `.phtml`, `.swift`, `.sh`, `.bash`, `.lua`, `.json`, `.html`, `.htm`) in `crates/cddm-core/src/grammar.rs`.
3. **Parser Dispatch**:
   - Integrate Tree-sitter language mappings in `crates/cddm-core/src/ast/parser.rs` and verify AST parsing.

---

### EP-18: AI-Augmented Refactoring Prompt Synthesizer & Agent Context Exporter

- **Target Milestone**: `v1.5.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.5.0)`

#### Problem Statement

Developers using AI coding assistants (e.g. Gemini, Antigravity, Claude, ChatGPT) require structured, contextual prompt specifications that detail duplicate clone locations, invariant logic bodies, parameter variance, and target architectures to generate clean refactorings.

#### Specification & Architecture

1. **Core Prompt Engine (`cddm-core::ai_prompt`)**:
   - Synthesize standardized Markdown specifications including Clone Classification, Target Function & Module destination, Code Fragment Occurrences with line ranges, Invariant Body, and Parameter differences.
2. **CLI & API Integration**:
   - Add `cddm refactor --prompt` flag to print the synthesized prompt specification.
   - Expose Axum `POST /api/refactor/ai-prompt` endpoint.
3. **MCP Tool & WebUI Studio**:
   - Expose `cddm_generate_ai_prompt` tool in `cddm-mcp`.
   - Add "Copy AI Prompt" action button in `RefactorSandboxModal.tsx` in CDDM Studio WebUI.

---

### EP-19: Turnkey PR/MR Markdown Quality Gate Comment Generator

- **Target Milestone**: `v1.5.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`
- **Priority**: `Medium`
- **Status**: `Completed (v1.5.0)`

#### Problem Statement

CI/CD pipelines in GitHub Actions, GitLab CI, and Azure Pipelines need a turnkey command to scan a repository, evaluate threshold compliance, and format an executive summary Markdown comment ready for PR/MR comment posting or step summaries.

#### Specification & Architecture

1. **Comment Formatter Engine (`cddm-core::pr_comment`)**:
   - Generate formatted quality gate comments with pass/fail tags, DRY Health Score, Duplication Rate vs threshold, scanned tokens/files, top clone pairs table, and developer action guides.
2. **CLI Subcommand**:
   - Add `cddm comment [DIRECTORY] [--fail-threshold <N>] [--platform github|gitlab|azure] [--output <PATH>]`.
   - Exit with non-zero status code if duplication percentage exceeds `--fail-threshold`.

---

### EP-20: AST-Native Rewrite Engine with Inferred Typing & CST Node Substitutions

- **Target Milestone**: `v1.6.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.6.0)`

#### Problem Statement

Textual diff patch synthesis can produce fragile substitutions when parameter variances require typed signatures, module imports, and syntax validation. An AST-native rewrite engine directly transforms the Tree-sitter Concrete Syntax Tree (CST), inferring parameter types, generating necessary imports, and validating syntactical correctness.

#### Specification & Architecture

1. **Type Inference Engine (`cddm-core::ast::type_infer`)**:
   - Infer language-specific parameter types (e.g. `&str`, `number`, `int`, `string`) across Rust, TypeScript, JavaScript, Python, Go, Java, C#, and C/C++.
   - Format target language function signatures (`pub fn name(...)`, `export function name(...)`, `def name(...):`).
2. **Import Synthesizer (`cddm-core::ast::import_resolver`)**:
   - Generate appropriate module import statements (`use crate::...`, `import { ... }`, `from ... import ...`).
   - Deduplicate existing imports.
3. **AST Rewriter (`cddm-core::ast::rewriter`)**:
   - Substitute clone occurrence CST nodes with synthesized helper call sites.
   - Validate syntax of transformed source files with Tree-sitter parsers.
4. **Tooling & Studio Integration**:
   - Add `cddm refactor --ast` CLI option and `POST /api/refactor/ast` endpoint.
   - Register `cddm_ast_refactor` MCP tool.
   - Add AST-Native Rewrite tab with inferred parameter badges to `RefactorSandboxModal.tsx`.

---

### EP-21: Closed-Loop Test Suite Verification Runner

- **Target Milestone**: `v1.6.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.6.0)`

#### Problem Statement

Automated refactorings should be verifiable in a single click or command invocation to guarantee that no functional regressions have been introduced into the workspace or refactored branch.

#### Specification & Architecture

1. **Test Runner Engine (`cddm-core::refactor`)**:
   - Auto-detect workspace build system (`Cargo.toml` -> `cargo test --workspace`, `package.json` -> `bun test`, `go.mod` -> `go test ./...`) or run custom test commands.
   - Capture execution time, exit codes, stdout, and stderr.
2. **Tooling & Studio Integration**:
   - Add `cddm refactor --verify [--test-cmd <CMD>]` CLI option.
   - Add Axum endpoint `POST /api/refactor/verify`.
   - Register `cddm_verify_refactor` MCP tool.
   - Add "Run Test Verification" button with status output terminal to `RefactorSandboxModal.tsx`.

---

### EP-22: Architecture Boundary & Anti-Duplication Policy Engine (`.cddmrules.toml`)

- **Target Milestone**: `v1.7.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-lsp`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.7.0)`

#### Problem Statement

Large codebases and monorepos require enforceable architectural boundary isolation, zero-duplication zones for security-critical packages, and clone size limits to prevent architectural drift across clean-architecture layers.

#### Specification & Architecture

1. **Policy Configuration (`.cddmrules.toml`)**:
   - Boundary rules (`[[boundaries]]`): Disallow duplication between architectural layers (e.g. domain core copied into presentation or infrastructure).
   - Zero-duplication rules (`[[zero_duplication]]`): Enforce 0% duplication across security, auth, and crypto modules.
   - Limit rules (`[[limits]]`): Enforce maximum token thresholds and multi-site occurrence caps.
2. **Scan & CI/CD Pipeline Integration**:
   - Integrate `PolicyEngine` evaluation into scan execution pipeline and fail CI if violations exceed policy severity.
   - Map violations to SARIF 2.1.0 rules (`CDDM_BOUNDARY`, `CDDM_ZERO_DUP`, `CDDM_LIMIT`).
   - Add `cddm rules init` and `cddm rules check` CLI commands plus `--rules` and `--enforce-policies` options.
   - Expose Axum endpoints `GET/POST /api/policy/rules` and `POST /api/policy/evaluate`.
   - Expose MCP tool `cddm_check_policies` and MCP resource `cddm://workspace/policies`.
   - Surface real-time inline LSP diagnostics in IDEs.
   - Implement `PolicyRulesModal.tsx` in CDDM Studio for visual inspection and live TOML editing.

---

### EP-23: Polyglot Language Expansion (Kotlin, Zig, Scala, Elixir, SQL, Dockerfile)

- **Target Milestone**: `v1.7.0`
- **Component**: `crates/cddm-core`
- **Priority**: `Medium`
- **Status**: `Completed (v1.7.0)`

#### Problem Statement

Extend CDDM's Tree-sitter AST parsing, comment stripping, and keyword lexing capabilities to support modern systems, JVM, and infrastructure languages (Kotlin, Zig, Scala, Elixir, SQL, and Dockerfile).

#### Specification & Architecture

1. **Grammar & Lexer Definitions (`cddm-core::grammar`)**:
   - Register language extensions, line and block comment delimiters, and language keywords.
2. **Tree-sitter AST Dispatch (`cddm-core::ast::parser`)**:
   - Integrate `tree-sitter-kotlin-ng`, `tree-sitter-zig`, `tree-sitter-scala`, `tree-sitter-elixir`, `tree-sitter-sequel`, and `tree-sitter-containerfile`.
   - Add AST parsing and clone detection unit tests across all 6 new languages.

---

### EP-24: Autonomous AI Code Surgeon & Closed-Loop Healing Engine

- **Target Milestone**: `v1.8.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v1.8.0)`

#### Problem Statement

Manual refactoring of duplicate code clusters is error-prone. CDDM provides an autonomous AI Code Surgeon loop that iteratively generates patches, tests them against the project test suite, and feeds compiler/test failure logs back to the LLM until full test passage is achieved.

#### Specification & Architecture

1. **AI Provider Abstraction (`cddm-core::ai::provider`)**:
   - Async `AiProvider` trait supporting Google Gemini, Anthropic Claude, OpenAI GPT-4o, Ollama (local), and Mock provider for testing.
2. **Closed-Loop Healing Loop (`cddm-core::ai::heal`)**:
   - Iterative prompting with error feedback when tests fail or patches do not apply.
   - Transactional Git branch application (`gix`).
3. **Surfaces**:
   - CLI: `cddm heal`
   - REST API: `POST /api/refactor/heal`
   - MCP Tool: `cddm_heal_refactor`
   - WebUI: Interactive Auto-Heal tab in `RefactorSandboxModal.tsx`.

---

### EP-25: Deep Semantic Graph Matching (CFG/PDG & Weisfeiler-Lehman Graph Isomorphism)

- **Target Milestone**: `v1.9.0`
- **Component**: `crates/cddm-core`, `crates/cddm-mcp`
- **Priority**: `Medium`
- **Status**: `Completed (v1.9.0)`

#### Problem Statement

Type-4 semantic clones share identical logical data dependencies and control flows despite having completely different syntactic structures and identifiers.

#### Specification & Architecture

1. **Control Flow Graph Extraction (`cddm-core::semantic_graph::cfg`)**:
   - Function AST to basic blocks, branches, loops, and return edges.
2. **Program Dependence Graph Builder (`cddm-core::semantic_graph::pdg`)**:
   - Variable def-use chains and data dependency edges.
3. **Weisfeiler-Lehman Graph Kernel (`cddm-core::semantic_graph::isomorphism`)**:
   - Multi-iteration neighborhood hashing and graph similarity metrics.
4. **MCP Resource**:
   - `cddm://workspace/semantic_graph`.

---

### EP-26: Monorepo Multi-Workspace Scanner & Distributed Cache Archive (.cddmpack)

- **Target Milestone**: `v1.9.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`
- **Priority**: `High`
- **Status**: `Completed (v1.9.0)`

#### Problem Statement

Enterprise monorepos contain multiple independent packages and workspaces. Teams need cross-workspace duplication detection and portable binary cache export/import for CI pipelines.

#### Specification & Architecture

1. **Portable Cache Pack Archive (`cddm-core::cache::pack`)**:
   - Export and import `.cddmpack` binary archives with SHA-256 integrity checksums.
2. **Monorepo Workspace Discovery (`cddm-core::monorepo`)**:
   - Automatic detection of Cargo, npm/pnpm/yarn/bun workspaces, Go modules, Gradle, Lerna, Turborepo, and Nx.
3. **Surfaces**:
   - CLI: `cddm cache export`, `cddm cache import`, `cddm monorepo`.
   - REST: `/api/cache/export`, `/api/cache/import`, `/api/monorepo`.
   - MCP: `cddm_export_cache_pack`, `cddm_import_cache_pack`, `cddm_scan_monorepo`.

---

### EP-27: Cross-Platform Ecosystem Distribution & Standalone Installers

- **Target Milestone**: `v2.0.0`
- **Component**: `packaging/`, `scripts/`
- **Priority**: `High`
- **Status**: `Completed (v2.0.0)`

#### Problem Statement

Users require seamless one-command installation across macOS, Linux, and Windows via native package managers and standalone shell scripts.

#### Specification & Architecture

1. **Package Managers**:
   - Homebrew Formula: `packaging/homebrew/cddm.rb`
   - Scoop Manifest: `packaging/scoop/cddm.json`
   - Winget Manifest: `packaging/winget/GrigorTonikyan.cddm.yaml`
2. **Standalone Installers**:
   - POSIX Shell: `packaging/install.sh` (curl-to-sh with platform/arch detection)
   - PowerShell: `packaging/install.ps1` (Windows installer with PATH registration)
3. **Validation**:
   - Automated packaging validator: `scripts/package-distribution.ts`.

---

### EP-28: JetBrains IDE Integration (IntelliJ, PyCharm, WebStorm, RustRover, GoLand)

- **Target Milestone**: `v2.0.0`
- **Component**: `docs/`
- **Priority**: `Medium`
- **Status**: `Completed (v2.0.0)`

#### Problem Statement

Engineers using JetBrains IDEs require complete setup instructions for integrating CDDM Language Server Protocol (`cddm lsp`), External Tools, and Git pre-commit hooks.

#### Specification & Architecture

1. **Setup Guide (`docs/JETBRAINS_SETUP.md`)**:
   - Native LSP server configuration.
   - External Tool shortcuts and keybindings.
   - Git quality gate hook setup.

---

### EP-29: VS Code Embedded Webview Studio & Turnkey VSIX Packaging Engine

- **Target Milestone**: `v2.1.0`
- **Component**: `editors/vscode`, `scripts/`
- **Priority**: `High`
- **Status**: `Completed (v2.1.0)`

#### Problem Statement

Developers need an embedded interactive WebUI Studio and Activity Bar sidebar dashboard inside Visual Studio Code and Cursor, with direct jump links to duplicate code sites and a turnkey cross-platform `.vsix` packaging workflow.

#### Specification & Architecture

1. **Embedded Webview Studio (`editors/vscode/src/webview/studio-panel.ts`)**:
   - Full-tab interactive WebUI Studio panel (`cddm.openStudioView`) with bi-directional message bridge to VS Code editor.
2. **Activity Bar Sidebar Dashboard (`editors/vscode/src/webview/sidebar-provider.ts`)**:
   - Native `cddm.sidebarView` providing DRY Health Score gauge, cluster stats, and quick actions.
3. **Polyglot LSP Registration & Commands**:
   - Expand document selector to all 24 supported languages in `extension.ts` and `constants.ts`.
   - Command palette integration (`showHealth`, `checkPolicies`, `exportSarif`, `openLocation`).
4. **VSIX Packaging Pipeline (`scripts/package-vscode.ts`, `scripts/lib/zip-builder.ts`)**:
   - Open Packaging Conventions (OPC) compliant `.vsix` packager generating standalone marketplace installers.

---

### EP-30: Cross-Language Semantic Matching & Hybrid Embeddings (Type-4 Polyglot Duplication)

- **Target Milestone**: `v2.3.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v2.3.0)`

#### Problem Statement

Polyglot enterprise codebases frequently rewrite or duplicate algorithms, validation routines, and domain business logic across different programming languages (e.g. TypeScript frontend matching Rust backend, Python data pipeline matching Go microservice). Detecting these cross-language semantic duplicates requires polyglot CFG/PDG graph isomorphism combined with subword vector embeddings.

#### Specification & Architecture

1. **Subword Vector Embeddings & Cosine Similarity (`cddm-core::semantic_graph::embedding`)**:
   - Sparse normalized term-frequency vectors combined with subword 3-grams and canonical operational tokens (`decl_var`, `ctrl_loop`, `op_add`, `lit_num`).
   - Fast cosine dot-product similarity computation ($0.0 \dots 1.0$).
2. **Polyglot Graph Isomorphism & Def-Use Slots (`cddm-core::semantic_graph::cfg`, `pdg`)**:
   - Language-agnostic CFG function boundary detection across 24 supported languages.
   - Slot-normalized variable def-use chain mapping ($v_0, v_1, \dots$).
   - Weisfeiler-Lehman graph coloring kernel.
3. **Unified Hybrid Similarity Model (`compute_hybrid_similarity`)**:
   - Combines structural graph isomorphism ($S_{\text{graph}}$) and subword token embedding ($S_{\text{token}}$):
     $$S_{\text{hybrid}} = \alpha \cdot S_{\text{graph}} + (1 - \alpha) \cdot S_{\text{token}}$$
4. **Surfaces & Integrations**:
   - **CLI**: `cddm semantic [DIR] [--threshold <N>] [--format console|json|markdown]` and `--cross-language` scanning flags for `scan` and `diff`.
   - **Axum REST API**: `POST /api/semantic/scan` and dual-language `POST /api/semantic-graph`.
   - **MCP Server**: `cddm_scan_cross_language` tool, `cross_language_audit` prompt, and `cddm://workspace/cross_language_clones` resource.
   - **WebUI Studio**: Interactive Cross-Language Explorer tab, dual-language Polyglot Sandbox selectors, and `[Polyglot]` clone badges.

---

### EP-31: Automated Shared Module & Crate Extraction

- **Target Milestone**: `v2.4.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v2.4.0)`

#### Problem Statement

Refactoring duplicate code across packages or crates currently stops at generating a patch. Developers need automated packaging that creates a standalone shared crate or module, updates workspace root manifests (`Cargo.toml`, `package.json`), adds inter-package dependencies, and rewrites all caller occurrences with imports.

#### Specification & Architecture

1. **Extraction Engine (`crates/cddm-core/src/extract/`)**:
   - Manifest updaters for Cargo workspaces, npm/pnpm/yarn packages, pyproject.toml, and go.mod.
   - Target crate/module boilerplate generators with public signature synthesis.
   - Caller AST and text rewriters with import injection and callsite substitutions.
   - Dry-run simulation and transactional disk execution.
2. **CLI Subcommand (`cddm extract`)**:
   - Flags: `--pair`, `--cluster`, `--target`, `--fn-name`, `--crate-type`, `--dry-run`, `--apply`.
3. **Axum REST API & MCP Server**:
   - Endpoints: `POST /api/extract/preview`, `POST /api/extract/apply`.
   - MCP tool: `cddm_extract_shared_module`.
4. **WebUI Studio Visualizer**:
   - `ExtractModuleTab.tsx` in `RefactorSandboxModal.tsx` for visual inspection of generated files, manifest diffs, and caller rewrites.

---

### EP-32: TUI Studio Interactive Terminal Dashboard

- **Target Milestone**: `v2.5.0`
- **Component**: `crates/cddm-cli`, `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Completed (v2.5.0)`

#### Problem Statement

Terminal power-users, remote SSH developers, and CI/CD engineers require a lightweight, keyboard-driven dashboard for real-time deduplication inspection without running a web browser or Node.js server.

#### Specification & Architecture

1. **Ratatui & Crossterm TUI Engine (`crates/cddm-cli/src/tui/`)**:
   - 8 interactive tabs: Scan, Pairs, Clusters, Treemap, Timeline, Rules, Semantic, and Health.
   - Side-by-side synchronized split diff viewer and refactor patch preview with scrolling.
2. **CLI Subcommand (`cddm tui`)**:
   - Zero-dependency terminal UI launched directly via `cddm tui [DIR]`.
3. **Cross-Interface Feature Parity**:
   - Governed by `.agents/rules/interface-feature-parity.md` and verified in `scripts/check-feature-parity.ts`.

---

### EP-33: Polyglot AST-Native Rewriters & Multi-Language Shared Module Extraction

- **Target Milestone**: `v2.6.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v2.6.0)`

#### Problem Statement

Extraction of shared modules must support all major programming languages beyond Rust and TypeScript, updating language-specific workspace manifests, generating typed signatures, and rewriting caller call sites with correct import statements.

#### Specification & Architecture

1. **Multi-Language Manifest Updaters (`crates/cddm-core/src/extract/manifest/`)**:
   - Support for Python (`pyproject.toml`), Go (`go.mod`, `go.work`), Java (`pom.xml`, `build.gradle`), and C# (`.csproj`).
2. **Polyglot Boilerplate & Typing**:
   - Typed function signature generation and class encapsulation across supported languages.
   - Return type and multi-return inference engine in `crates/cddm-core/src/ast/type_infer/`.
3. **CST Replacement & Caller Rewriting**:
   - Import statement resolution and CST replacement in `crates/cddm-core/src/ast/import_resolver.rs` and `rewriter.rs`.

---

### EP-34: Automated Polyglot Unit Test Synthesizer & Behavioral Equivalence Verifier

- **Target Milestone**: `v2.7.0`
- **Component**: `crates/cddm-core`, `crates/cddm-cli`, `crates/cddm-mcp`, `webui`
- **Priority**: `High`
- **Status**: `Completed (v2.7.0)`

#### Problem Statement

When extracting duplicate code across codebases into shared helper modules or crates, engineers must ensure behavioral equivalence by generating idiomatic unit tests for the extracted helpers with sample arguments extracted from duplicate occurrences.

#### Specification & Architecture

1. **Polyglot Test Synthesizer (`crates/cddm-core/src/extract/test_generator.rs`)**:
   - Generates idiomatic test files for Rust (`tests/*_test.rs`), TypeScript/JavaScript (`*.test.ts`), Python (`test_*.py`), Go (`*_test.go`), Java (`*Test.java`), and C# (`*Tests.cs`).
   - Extracts sample arguments from duplicate occurrences to instantiate realistic test invocations.
2. **4-Pillar Cross-Interface Integration**:
   - CLI: `--generate-tests` flag for `cddm extract`.
   - MCP: `generate_tests` parameter on `cddm_extract_shared_module`.
   - WebUI: "Synthesize Unit Tests" checkbox and test file preview viewer in `ExtractModuleTab.tsx`.
   - TUI: `[t] Synthesize Unit Tests` badge in extraction operations.

---

## 3. Prioritized Action Checklist

```markdown
### Milestone v0.2.0 (CI/CD & AI Agent Tooling)

- [x] Implement SARIF 2.1.0 reporter in `cddm-cli` (`--format sarif`) [EP-01]
- [x] Add `cddm_get_clone_pair` and `cddm_suggest_refactor` to `cddm-mcp` [EP-02]
- [x] Expose `cddm://workspace/health` MCP read-only resource [EP-02]
- [x] Publish composite GitHub Action `GrigorTonikyan/cddm-action` [EP-01]

### Milestone v0.3.0 (Caching, Differential & Refactoring)

- [x] Embed `redb` disk-backed persistent fingerprint cache in `cddm-core` [EP-03]
- [x] Add `cddm diff <BASE_REF>` for differential pull-request scans [EP-08]
- [x] Implement `cddm refactor` prototype for automated patch synthesis [EP-07]

### Milestone v0.4.0 (Studio Visual Analytics)

- [x] Add interactive split-diff view with synchronized scrolling in WebUI [EP-04]
- [x] Implement Squarified hierarchical codebase duplication treemap [EP-06]
- [x] Add automated refactoring advisor and `.patch` diff modal in WebUI [EP-04]

### Milestone v0.5.0 (AST Pipeline & Polyglot Expansion)

- [x] Integrate AST Merkle subtree hasher into main `run_scan` pipeline [EP-05]
- [x] Implement Tree-sitter parsers for Go, C/C++, Java, and C# [EP-09]
- [x] Support Type-3 near-miss clone detection with tree edit distance and dynamic similarity [EP-05]

### Milestone v1.0.0 (High-Throughput Enterprise Engine)

- [x] Implement `memmap2` zero-copy memory mapping for large files [EP-10]
- [x] Implement AVX2 / NEON SIMD vectorization for Mersenne 61 rolling hash [EP-10]
- [x] Benchmark and validate 1M+ LOC enterprise monorepo scalability [EP-10]

### Milestone v1.1.0 (N-Way Clustering & Multi-Site Deduplication)

- [x] Implement Disjoint-Set Union-Find connected-components graph clustering [EP-11]
- [x] Implement multi-site consensus invariant deduplication synthesizer & unified diff patch generation [EP-11]
- [x] Expose Axum `POST /api/refactor-cluster` endpoint and CLI `--cluster <ID>` option [EP-11]
- [x] Implement MCP tools (`cddm_get_clone_cluster`, `cddm_suggest_cluster_refactor`) & resource `cddm://workspace/clusters` [EP-11]
- [x] Add WebUI Pairwise vs N-Way Clusters view tabs, `CloneClusterCard.tsx`, and multi-file `RefactorPatchModal.tsx` [EP-11]

### Milestone v1.2.0 (Language Server Protocol & IDE Extensions)

- [x] Implement standard Language Server Protocol (LSP 3.17) daemon in `crates/cddm-lsp` [EP-12]
- [x] Implement real-time clone diagnostics (`textDocument/publishDiagnostics`) with counterpart relatedLocations [EP-12]
- [x] Implement quick-fix refactoring and function extraction code actions (`textDocument/codeAction`) [EP-12]
- [x] Implement rich Markdown hover tooltip information (`textDocument/hover`) [EP-12]
- [x] Add `cddm lsp` CLI subcommand in `cddm-cli` [EP-12]
- [x] Publish official CDDM VS Code and Cursor extension in `editors/vscode` [EP-12]
- [x] Publish multi-editor setup guides for Neovim, Zed, Helix, Sublime Text, and Emacs in `docs/LSP_SETUP.md` [EP-12]

### Milestone v1.3.0 (Historical Duplication Trends & Turnkey CI/CD Workflow Generator)

- [x] Implement in-process Git history revision walking & DRY health trajectory in `cddm-core::timeline` [EP-13]
- [x] Implement `cddm trend` CLI command with ANSI sparklines and markdown report formatting [EP-13]
- [x] Implement turnkey CI/CD workflow generator (`cddm init github|gitlab|azure`) in `cddm-core::workflow` [EP-14]
- [x] Implement cross-platform Git pre-commit & pre-push hook manager (`cddm hook install|uninstall|status`) [EP-14]
- [x] Expose Axum REST endpoints `GET /api/timeline`, `GET /api/workflow/hooks`, `POST /api/workflow/hooks/install` [EP-13, EP-14]
- [x] Implement MCP tool `cddm_get_timeline` and resource `cddm://workspace/timeline` [EP-13]
- [x] Implement interactive `TimelineExplorerModal.tsx` in CDDM Studio WebUI with SVG trajectory chart and commit snapshots [EP-13]

### Milestone v1.4.0 (Intelligent AST Suppression Engine & Interactive Auto-Refactor Sandbox)

- [x] Implement `.cddmignore` glob rule parsing with per-path threshold overrides in `cddm-core::suppression` [EP-15]
- [x] Implement inline AST comment directives and test/mock/generated file auto-detection [EP-15]
- [x] Add `cddm ignore init` and `cddm ignore check` CLI subcommands [EP-15]
- [x] Implement interactive refactor sandbox studio with customized function names and destination modules in `cddm-core::refactor` [EP-16]
- [x] Implement transactional Git branch refactor patch application using in-process `gix` [EP-16]
- [x] Expose Axum endpoints for suppression rules and refactor sandbox simulation [EP-15, EP-16]
- [x] Expose MCP tools `cddm_check_suppression`, `cddm_apply_cluster_refactor`, and resource `cddm://workspace/suppressions` [EP-15, EP-16]
- [x] Implement WebUI `SuppressionRulesModal.tsx` and `RefactorSandboxModal.tsx` in CDDM Studio [EP-15, EP-16]

### Milestone v1.5.0 (Polyglot AST Expansion & AI Refactoring Prompt Synthesizer)

- [x] Implement Tree-sitter parsers for Ruby, PHP, Swift, Bash, Lua, JSON, and HTML [EP-17]
- [x] Implement AI-augmented refactoring prompt synthesizer in `cddm-core::ai_prompt` [EP-18]
- [x] Add `cddm refactor --prompt` CLI option and Axum endpoint `POST /api/refactor/ai-prompt` [EP-18]
- [x] Implement MCP tool `cddm_generate_ai_prompt` [EP-18]
- [x] Implement turnkey PR/MR markdown quality gate comment generator (`cddm comment`) in `cddm-core::pr_comment` [EP-19]

### Milestone v1.6.0 (AST-Native Rewrite Engine & Type-Aware Automated Refactoring)

- [x] Implement Tree-sitter parameter type inference & signature formatting in `cddm-core::ast::type_infer` [EP-20]
- [x] Implement cross-module import statement synthesizer in `cddm-core::ast::import_resolver` [EP-20]
- [x] Implement AST CST node replacement & syntax validation in `cddm-core::ast::rewriter` [EP-20]
- [x] Implement multi-file AST cluster refactoring in `cddm-core::refactor` [EP-20]
- [x] Implement closed-loop test suite verification in `cddm-core::refactor` [EP-21]
- [x] Add CLI flags `--ast`, `--fn-name`, `--target-module`, `--verify`, `--test-cmd` to `cddm refactor` [EP-20, EP-21]
- [x] Expose Axum REST endpoints `POST /api/refactor/ast` and `POST /api/refactor/verify` in `cddm-cli::serve` [EP-20, EP-21]
- [x] Expose MCP tools `cddm_ast_refactor` and `cddm_verify_refactor` in `cddm-mcp` [EP-20, EP-21]
- [x] Implement WebUI Studio AST-Native Rewrite tab, inferred parameter badges, and Test Suite Verification panel in `RefactorSandboxModal.tsx` [EP-20, EP-21]

### Milestone v1.7.0 (Architectural Boundary Policy Engine & Polyglot Expansion)

- [x] Implement `.cddmrules.toml` policy parser, cross-layer boundaries, zero-dup zones, and token limits in `cddm-core::policy` [EP-22]
- [x] Integrate `PolicyEngine` evaluation into scan pipeline and emit SARIF violation rules in `cddm-core::sarif` [EP-22]
- [x] Add `cddm rules init` and `cddm rules check` CLI commands plus `--rules` and `--enforce-policies` options in `cddm-cli` [EP-22]
- [x] Expose Axum REST endpoints `GET/POST /api/policy/rules` and `POST /api/policy/evaluate` in `cddm-cli::serve` [EP-22]
- [x] Expose MCP tool `cddm_check_policies` and MCP resource `cddm://workspace/policies` in `cddm-mcp` [EP-22]
- [x] Surface real-time architectural policy diagnostics in `cddm-lsp::diagnostics` [EP-22]
- [x] Implement `PolicyRulesModal.tsx` visual studio and live TOML editor in WebUI Studio [EP-22]
- [x] Implement Tree-sitter parsers for Kotlin, Zig, Scala, Elixir, SQL, and Dockerfile in `cddm-core::ast::parser` [EP-23]

### Milestone v1.8.0 (AI Code Surgeon & Autonomous Self-Healing Refactor Engine)

- [x] Implement async `AiProvider` trait and Gemini, Claude, OpenAI, Ollama, Mock providers in `cddm-core::ai::provider` [EP-24]
- [x] Implement autonomous error-feedback test healing loop and branch committing in `cddm-core::ai::heal` [EP-24]
- [x] Add `cddm heal` CLI command in `cddm-cli` [EP-24]
- [x] Expose Axum REST endpoint `POST /api/refactor/heal` in `cddm-cli::serve` [EP-24]
- [x] Expose MCP tool `cddm_heal_refactor` in `cddm-mcp` [EP-24]
- [x] Implement WebUI Studio Auto-Heal tab in `RefactorSandboxModal.tsx` [EP-24]

### Milestone v1.9.0 (Deep Semantic Graph Matching & Monorepo Distributed Cache)

- [x] Implement CFG extraction and PDG variable def-use data dependency graphs in `cddm-core::semantic_graph` [EP-25]
- [x] Implement Weisfeiler-Lehman graph kernel hashing and structural clone similarity in `cddm-core::semantic_graph` [EP-25]
- [x] Implement `.cddmpack` portable cache archive export and import with SHA-256 validation in `cddm-core::cache::pack` [EP-26]
- [x] Implement monorepo multi-workspace discovery and scanner in `cddm-core::monorepo` [EP-26]
- [x] Add `cddm cache export`, `cddm cache import`, and `cddm monorepo` CLI commands in `cddm-cli` [EP-26]
- [x] Expose Axum REST endpoints `/api/cache/export`, `/api/cache/import`, `/api/monorepo` in `cddm-cli::serve` [EP-26]
- [x] Expose MCP tools `cddm_export_cache_pack`, `cddm_import_cache_pack`, `cddm_scan_monorepo` in `cddm-mcp` [EP-26]
- [x] Expose MCP resource `cddm://workspace/semantic_graph` in `cddm-mcp` [EP-25]

### Milestone v2.0.0 (Ecosystem Packaging, Distribution & JetBrains Integration)

- [x] Create Homebrew Formula in `packaging/homebrew/cddm.rb` [EP-27]
- [x] Create Scoop Windows manifest in `packaging/scoop/cddm.json` [EP-27]
- [x] Create Winget manifest in `packaging/winget/GrigorTonikyan.cddm.yaml` [EP-27]
- [x] Create standalone cross-platform curl-to-sh installer in `packaging/install.sh` [EP-27]
- [x] Create standalone Windows PowerShell installer in `packaging/install.ps1` [EP-27]
- [x] Implement ecosystem packaging validation script in `scripts/package-distribution.ts` [EP-27]
- [x] Create comprehensive JetBrains IDE setup guide in `docs/JETBRAINS_SETUP.md` [EP-28]

### Milestone v2.1.0 (First-Class IDE & Editor Ecosystem)

- [x] Implement embedded full-screen Webview panel provider in `editors/vscode/src/webview/studio-panel.ts` (`cddm.openStudioView`) [EP-29]
- [x] Implement Activity Bar DRY health & duplication sidebar dashboard in `editors/vscode/src/webview/sidebar-provider.ts` (`cddm.sidebarView`) [EP-29]
- [x] Expand LSP document selectors and activation events to all 24 polyglot languages in `editors/vscode/src/extension.ts` [EP-29]
- [x] Add command palette suite (`cddm.showHealth`, `cddm.checkPolicies`, `cddm.exportSarif`, `cddm.openLocation`) in `commands/actions.ts` [EP-29]
- [x] Implement zero-dependency cross-platform VSIX packaging and validation engine in `scripts/package-vscode.ts` [EP-29]
- [x] Implement standard Open Packaging Conventions ZIP archive builder in `scripts/lib/zip-builder.ts` [EP-29]
- [x] Integrate VS Code packaging into `package-distribution.ts`, `sync-version.ts`, and full verification suite `scripts/verify.ts` [EP-29]

### Milestone v2.3.0 (Cross-Language Semantic Matching & Hybrid Embeddings)

- [x] Implement subword vector embedding engine & cosine similarity in `cddm-core::semantic_graph::embedding` [EP-30]
- [x] Implement polyglot CFG function extraction and slot-normalized PDG def-use analysis in `cddm-core::semantic_graph` [EP-30]
- [x] Implement unified hybrid similarity calculator ($S_{\text{hybrid}} = \alpha S_{\text{graph}} + (1-\alpha) S_{\text{token}}$) in `cddm-core` [EP-30]
- [x] Implement cross-language workspace scanner (`scan_cross_language_workspace`) in `cddm-core` [EP-30]
- [x] Add `cddm semantic [DIR]` CLI command and `--cross-language` scanning flags in `cddm-cli` [EP-30]
- [x] Expose Axum REST endpoint `POST /api/semantic/scan` and dual-language graph comparison in `cddm-cli::serve` [EP-30]
- [x] Expose MCP tool `cddm_scan_cross_language`, prompt `cross_language_audit`, and resource `cddm://workspace/cross_language_clones` in `cddm-mcp` [EP-30]
- [x] Implement WebUI Studio Cross-Language Explorer tab, dual-language Polyglot Sandbox selectors, and `[Polyglot]` badges in `webui/` [EP-30]

### Milestone v2.4.0 (Automated Shared Module & Crate Extraction)

- [x] Implement `cddm_core::extract` engine (manifest, generator, rewriter, executor) [EP-31]
- [x] Implement multi-language workspace manifest mutators (`Cargo.toml`, `package.json`) [EP-31]
- [x] Implement shared crate boilerplate generator with public inferred function signatures [EP-31]
- [x] Implement occurrence caller rewriter with injected import statements and callsite substitutions [EP-31]
- [x] Add `cddm extract` CLI command in `cddm-cli` [EP-31]
- [x] Expose Axum REST endpoints `POST /api/extract/preview` and `POST /api/extract/apply` in `cddm-cli::serve` [EP-31]
- [x] Expose MCP tool `cddm_extract_shared_module` in `cddm-mcp` [EP-31]
- [x] Implement WebUI Studio Extract Shared Crate/Module tab in `RefactorSandboxModal.tsx` and `ExtractModuleTab.tsx` [EP-31]

### Milestone v2.5.0 (Interactive Terminal UI Studio & 4-Pillar Feature Parity)

- [x] Implement high-speed Ratatui & Crossterm TUI engine in `crates/cddm-cli/src/tui/` across 8 dedicated tabs [EP-32]
- [x] Implement interactive side-by-side split diff and unified refactor diff panes with scrolling [EP-32]
- [x] Add `cddm tui` CLI subcommand with live watch re-scanning support [EP-32]
- [x] Create 4-Pillar Cross-Interface Feature Parity governance standard in `.agents/rules/interface-feature-parity.md` [EP-32]
- [x] Create SSoT parity documentation matrix in `docs/FEATURE_PARITY.md` across 15 core capabilities [EP-32]
- [x] Implement automated parity verification gate `scripts/check-feature-parity.ts` integrated into `vp run verify` [EP-32]
- [x] Verify complete cross-interface feature parity across CLI, WebUI, MCP, and TUI surfaces [EP-32]

### Milestone v2.6.0 (Polyglot AST-Native Rewriters & Multi-Language Shared Module Extraction)

- [x] Implement multi-language manifest updaters for Python (`pyproject.toml`), Go (`go.mod`, `go.work`), Java (`pom.xml`, `build.gradle`), and C# (`.csproj`) in `cddm-core::extract::manifest` [EP-33]
- [x] Implement polyglot package boilerplate generators with typed function signatures and class encapsulation in `cddm-core::extract::generator` [EP-33]
- [x] Implement return type and multi-return inference engine in `cddm-core::ast::type_infer` [EP-33]
- [x] Implement polyglot import statement resolution and CST replacement in `cddm-core::ast::import_resolver` and `rewriter` [EP-33]
- [x] Verify complete cross-interface feature parity across CLI, WebUI, MCP, and TUI surfaces [EP-33]

### Milestone v2.7.0 (Automated Polyglot Unit Test Synthesizer & Behavioral Equivalence Verifier)

- [x] Implement polyglot unit test generator for extracted helper functions across Rust, TypeScript, Python, Go, Java, and C# in `cddm-core::extract::test_generator` [EP-34]
- [x] Add `--generate-tests` CLI flag to `cddm extract` in `cddm-cli` [EP-34]
- [x] Expose `generate_tests` parameter on `cddm_extract_shared_module` tool in `cddm-mcp` [EP-34]
- [x] Implement WebUI Studio Synthesize Unit Tests checkbox and test preview viewer in `ExtractModuleTab.tsx` [EP-34]
- [x] Add `[t] Synthesize Unit Tests` badge in TUI Studio Extract view in `cddm-cli::tui` [EP-34]
- [x] Verify complete cross-interface feature parity across CLI, WebUI, MCP, and TUI surfaces [EP-34]

### Milestone v2.8.0 (Automated Micro-Benchmark & Performance Regression Synthesizer)

- [x] Implement polyglot micro-benchmark generator across Rust (Criterion), TypeScript (tinybench), Python (timeit), Go (testing.B), Java (JMH), and C# (BenchmarkDotNet) in `cddm-core::extract::bench_generator` [EP-36]
- [x] Add `--generate-benchmarks` / `--bench` CLI flag to `cddm extract` in `cddm-cli` [EP-36]
- [x] Expose `generate_benchmarks` parameter on `cddm_extract_shared_module` tool in `cddm-mcp` [EP-36]
- [x] Implement WebUI Studio Synthesize Micro-Benchmarks checkbox and benchmark preview viewer in `ExtractModuleTab.tsx` [EP-36]
- [x] Add `[b] Synthesize Benchmarks` badge in TUI Studio Extract view in `cddm-cli::tui` [EP-36]

### Milestone v2.9.0 (Context-Aware Program Slicing for AI Refactor Surgeon)

- [x] Implement backward and forward static program slicing over Program Dependence Graphs in `cddm-core::semantic_graph::slicing` [EP-37]
- [x] Implement context boundary slice extraction (`extract_context_slice`) capturing data definitions and dependencies [EP-37]
- [x] Integrate context slices into AI prompt generation (`cddm-core::ai_prompt`) and closed-loop healer (`cddm-core::ai::heal`) [EP-37]

### Milestone v3.0.0 (Ecosystem Library Reimplementation & Overlap Detector)

- [x] Implement built-in canonical open-source algorithm catalog and keyword matching engine in `cddm-core::overlap` [EP-38]
- [x] Implement workspace overlap scanner (`scan_workspace_overlap`) with language package recommendations in `cddm-core` [EP-38]
- [x] Add `cddm overlap [DIR]` CLI command with console, JSON, and Markdown report formatters in `cddm-cli` [EP-38]
- [x] Expose Axum REST endpoints `GET /api/overlap/catalog` and `POST /api/overlap/scan` in `cddm-cli::serve` [EP-38]
- [x] Expose MCP tool `cddm_detect_overlap` and resource `cddm://workspace/overlap` in `cddm-mcp` [EP-38]
- [x] Implement Tab 9 (Ecosystem Overlap) in interactive TUI Studio (`cddm-cli::tui`) [EP-38]
- [x] Implement WebUI Studio `OverlapDetectorModal.tsx` and algorithm catalog viewer in `webui/` [EP-38]
- [x] Verify complete 4-pillar cross-interface feature parity across CLI, WebUI, MCP, and TUI surfaces [EP-38]

### Milestone v3.1.0 (Automated GitHub PR Fix-Bot Action)

- [x] Create turnkey composite GitHub Action in `.github/actions/cddm-fix-bot/action.yml` for automated scanning, PR comment generation, and auto-extraction [EP-39]
```

---

## 4. Submitting New Feature Requests & RFCs

To propose an addition or enhancement:

1. Review existing proposals in this document to avoid duplicates.
2. Submit a GitHub Issue using the [Feature Request Template](../.github/ISSUE_TEMPLATE/feature_request.md).
3. Open a discussion or Pull Request adhering to the [CDDM Standards](../AGENTS.md).
