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

| Milestone  | Target Horizon | Strategic Focus                             | Key Deliverables                                                                                               |
| :--------- | :------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------- |
| **v0.2.0** | Short-term     | CI/CD Integration & AI Agent Tooling        | SARIF `--format sarif`, expanded MCP tools (`get_clone_context`, `suggest_refactor`), official GitHub Action.  |
| **v0.3.0** | Mid-term       | Caching, Differential Scans & Refactoring   | Embedded `redb` disk cache, `cddm diff <branch>`, automated patch synthesis (`cddm refactor`).                 |
| **v0.4.0** | Mid-term       | WebUI Studio & Visual Analytics             | Side-by-side Monaco diff visualizer, D3 hierarchical duplication treemap, historical Git trend graph.          |
| **v0.5.0** | Long-term      | AST Pipeline & Extended Polyglot            | Integrated AST Merkle subtree matching, Type-3 near-miss detection, Go, C/C++, Java Tree-sitter parsers.       |
| **v1.0.0** | Stable Release | High-Throughput Enterprise Engine           | AVX2/NEON SIMD vectorization, memory-mapped zero-copy I/O, semantic AST graph clones (Type-4).                 |
| **v1.1.0** | Stable Release | N-Way Clustering & Multi-Site Deduplication | Disjoint-Set Union-Find clustering, multi-site patch synthesis, N-way cluster cards, Axum cluster endpoint.    |
| **v1.2.0** | Stable Release | Language Server Protocol & IDE Extensions   | Full LSP 3.17 daemon (`crates/cddm-lsp`), official VS Code extension, inline diagnostics & code actions.       |
| **v1.3.0** | Stable Release | Historical Trends & Turnkey Workflows       | Git timeline duplication trajectories (`cddm trend`), turnkey CI workflow generator (`cddm init`, `hook`).     |
| **v1.4.0** | Stable Release | AST Suppressions & Refactor Sandbox         | Intelligent `.cddmignore` engine, inline comment directives, interactive WebUI refactoring sandbox studio.     |
| **v1.5.0** | Stable Release | Polyglot Expansion & AI Prompt Synthesizer  | 16 Tree-sitter AST grammars (Ruby, PHP, Swift, Bash, Lua, JSON, HTML), AI refactor prompt engine, PR comments. |

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
   - `cddm hook install --type pre-commit|pre-push --fail-threshold 15.0`.
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
- [x] Add "Copy AI Prompt" action button to `RefactorSandboxModal.tsx` in WebUI Studio [EP-18]
- [x] Implement turnkey PR/MR markdown quality gate comment generator (`cddm comment`) in `cddm-core::pr_comment` [EP-19]
```

---

## 4. Submitting New Feature Requests & RFCs

To propose an addition or enhancement:

1. Review existing proposals in this document to avoid duplicates.
2. Submit a GitHub Issue using the [Feature Request Template](../.github/ISSUE_TEMPLATE/feature_request.md).
3. Open a discussion or Pull Request adhering to the [CDDM Standards](../AGENTS.md).
