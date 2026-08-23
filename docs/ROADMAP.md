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

| Milestone  | Target Horizon | Strategic Focus                           | Key Deliverables                                                                                              |
| :--------- | :------------- | :---------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| **v0.2.0** | Short-term     | CI/CD Integration & AI Agent Tooling      | SARIF `--format sarif`, expanded MCP tools (`get_clone_context`, `suggest_refactor`), official GitHub Action. |
| **v0.3.0** | Mid-term       | Caching, Differential Scans & Refactoring | Embedded `redb` disk cache, `cddm diff <branch>`, automated patch synthesis (`cddm refactor`).                |
| **v0.4.0** | Mid-term       | WebUI Studio & Visual Analytics           | Side-by-side Monaco diff visualizer, D3 hierarchical duplication treemap, historical Git trend graph.         |
| **v0.5.0** | Long-term      | AST Pipeline & Extended Polyglot          | Integrated AST Merkle subtree matching, Type-3 near-miss detection, Go, C/C++, Java Tree-sitter parsers.      |
| **v1.0.0** | Stable Release | High-Throughput Enterprise Engine         | AVX2/NEON SIMD vectorization, memory-mapped zero-copy I/O, semantic AST graph clones (Type-4).                |

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
- **Component**: `webui/`
- **Priority**: `Medium`
- **Status**: `Proposed`

#### Problem Statement

The WebUI Studio currently displays static line ranges and author annotations for clone pairs. Developers need an interactive split diff with syntax highlighting, token-level matching highlights, and synchronized scrolling.

#### Specification & Architecture

1. Integrate `@monaco-editor/react` (or `@codemirror/lang-*`) into `ClonePairCard.tsx`.
2. Fetch snippet contents via REST endpoint `GET /api/snippet?file=path&start=N&end=M`.
3. Render Fragment A on the left and Fragment B on the right with token-level diff annotations.
4. Provide copy buttons for extracting common logic.

#### Acceptance Criteria

- ClonePair cards expand to display interactive split diffs with syntax highlighting.
- Responsive on desktop and tablet viewport widths.

---

### EP-05: Tree-sitter AST Merkle Pipeline Integration for Type-3 & Type-4 Clones

- **Target Milestone**: `v0.5.0`
- **Component**: `crates/cddm-core`
- **Priority**: `High`
- **Status**: `Proposed`

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
- **Status**: `Proposed`

#### Problem Statement

Developers need a high-level visual representation of where duplication is clustered across nested directories, crates, and modules.

#### Specification & Architecture

1. Add D3.js or ECharts hierarchical Treemap and Sunburst charts in `ScanResults.tsx`.
2. Node size represents total token volume; node color represents local duplication rate (emerald for < 5%, amber for 5-15%, rose for > 15%).
3. Clicking a directory node zooms into subdirectories and filters the clone pair list.

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
- **Status**: `Proposed`

#### Problem Statement

Extend full Tree-sitter AST parsing support beyond Rust, TypeScript, JavaScript, and Python.

#### Languages to Add

- **Go**: `tree-sitter-go`
- **C / C++**: `tree-sitter-c`, `tree-sitter-cpp`
- **Java**: `tree-sitter-java`
- **C#**: `tree-sitter-c-sharp`
- **Kotlin**: `tree-sitter-kotlin`
- **SQL / GraphQL / Proto**: Domain-specific grammars

#### Acceptance Criteria

- 100% test coverage for new grammars with verified comment stripping and keyword tokenization.

---

### EP-10: Memory-Mapped I/O & SIMD Mersenne-61 Hash Vectorization

- **Target Milestone**: `v1.0.0`
- **Component**: `crates/cddm-core`
- **Priority**: `Low`
- **Status**: `Proposed`

#### Problem Statement

Maximizing token throughput on multi-gigabyte codebases to achieve > 20M tokens/second.

#### Specification & Architecture

1. Replace heap file buffer allocation with `memmap2::Mmap` for files > 64 KB.
2. Implement AVX2 and ARM NEON vector lanes for Mersenne 61 rolling polynomial calculation:
   - Compute 4 parallel window hash modulos simultaneously.
3. Provide scalar fallback for architectures without vector extensions.

#### Acceptance Criteria

- Benchmarks show 2.5x - 4.0x speedup in rolling hash computation on x86_64 (AVX2) and aarch64 (NEON).

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

- [ ] Add Monaco / CodeMirror interactive split-diff view in WebUI [EP-04]
- [ ] Implement D3.js / ECharts hierarchical codebase duplication treemap [EP-06]
- [ ] Add Git historical DRY Health Score trend graph [EP-06]

### Milestone v0.5.0 (AST Pipeline & Polyglot Expansion)

- [ ] Integrate AST Merkle subtree hasher into main `run_scan` pipeline [EP-05]
- [ ] Implement Tree-sitter parsers for Go, C/C++, Java, and C# [EP-09]
- [ ] Support Type-3 near-miss clone detection with tree edit distance [EP-05]

### Milestone v1.0.0 (High-Throughput Enterprise Engine)

- [ ] Implement `memmap2` zero-copy memory mapping for large files [EP-10]
- [ ] Implement AVX2 / NEON SIMD vectorization for Mersenne 61 rolling hash [EP-10]
- [ ] Benchmark and validate 1M+ LOC enterprise monorepo scalability [EP-10]
```

---

## 4. Submitting New Feature Requests & RFCs

To propose an addition or enhancement:

1. Review existing proposals in this document to avoid duplicates.
2. Submit a GitHub Issue using the [Feature Request Template](../.github/ISSUE_TEMPLATE/feature_request.md).
3. Open a discussion or Pull Request adhering to the [CDDM Standards](../AGENTS.md).
