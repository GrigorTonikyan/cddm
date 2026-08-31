/**
 * Gitea Wiki Content Definitions for CDDM
 */

export interface WikiPageDef {
  title: string;
  content: string;
}

export const WIKI_PAGES: WikiPageDef[] = [
  {
    title: "Home",
    content: `# CDDM -- Code De-Duplication Meister Wiki

Welcome to the official documentation and technical wiki for **CDDM** (*Code De-Duplication Meister*).

CDDM is a next-generation, high-performance polyglot code clone detection, AST refactoring, and architectural governance engine written in Rust and TypeScript.

---

## The 4 Interaction Pillars

CDDM enforces strict **100% Feature Parity** across all four primary interaction surfaces:

\`\`\`
+-----------------------------------------------------------------------------+
|                          CDDM Core AST & SIMD Engine                         |
+------┬----------------------┬----------------------┬-----------------┬------+
       |                      |                      |                 |
       v                      v                      v                 v
+---------------+     +---------------+      +---------------+ +---------------+
| 1. CLI Engine |     | 2. WebUI      |      | 3. MCP Server | | 4. TUI Studio |
| Terminal Sub- |     | React 19 FSD  |      | JSON-RPC 2.0  | | Ratatui Term  |
| commands, CI  |     | Monaco Diffs  |      | AI Coding     | | Interactive   |
| & Dogfooding  |     | SSE Live Sync |      | Agent Tools   | | Dashboard     |
+---------------+     +---------------+      +---------------+ +---------------+
\`\`\`

---

## Wiki Table of Contents

- [[Getting-Started|Getting Started]]: Installation, binary downloads, and first scan.
- [[CLI-Reference|CLI Command Reference]]: Comprehensive manual for all CLI subcommands.
- [[WebUI-Studio|WebUI Studio]]: Feature-Sliced React 19 Studio, Monaco diffs, and Treemap.
- [[MCP-Server-Protocol|MCP Server Protocol]]: 27 Model Context Protocol tools for AI agents.
- [[TUI-Studio|TUI Terminal Studio]]: Keyboard-driven Ratatui terminal dashboard.
- [[AST-Engine-and-Deduplication|AST Engine & Deduplication]]: Winnowing, Clone Types 1-4, and SIMD.
- [[4-Pillar-Feature-Parity|Cross-Interface Feature Parity]]: Governance standard & matrices.
- [[CI-CD-and-Releases|CI/CD & Releases]]: Gitea Actions runner matrix & packaging.
`,
  },
  {
    title: "Getting-Started",
    content: `# Getting Started with CDDM

This guide covers installing and running CDDM across all supported environments.

---

## 1. Binary Installation

Pre-compiled standalone binaries are available from the [Releases](https://git.gt-web-dev.com/gt-dev/cddm/releases) page for Linux and Windows:

### Linux (AMD64)
\`\`\`bash
curl -LO https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v1.9.0/cddm-v1.9.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf cddm-v1.9.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv cddm /usr/local/bin/
\`\`\`

### Windows (x64)
Download \`cddm-v1.9.0-x86_64-pc-windows-gnu.zip\`, extract \`cddm.exe\`, and add to your PATH.

---

## 2. Package Managers

### Cargo (from source)
\`\`\`bash
cargo install --git https://git.gt-web-dev.com/gt-dev/cddm.git --bin cddm
\`\`\`

### Bun / npm
\`\`\`bash
npm install -g cddm
\`\`\`

---

## 3. Running Your First Scan

Analyze any repository for code duplication:

\`\`\`bash
cddm scan . --min-tokens 50 --format terminal
\`\`\`

Launch the interactive visual Studio:
\`\`\`bash
cddm serve --port 5173 --open
\`\`\`

Launch the terminal TUI dashboard:
\`\`\`bash
cddm tui .
\`\`\`
`,
  },
  {
    title: "CLI-Reference",
    content: `# CLI Command Reference

CDDM provides a comprehensive suite of subcommands for scanning, refactoring, and reporting.

---

## Command Overview

| Subcommand | Description |
| :--- | :--- |
| \`cddm scan <path>\` | Detect Type-1, Type-2, and Type-3 clones across the codebase |
| \`cddm diff <path>\` | Compare clone drift against a git base branch or commit |
| \`cddm semantic <path>\`| Neural embedding and cross-language semantic clone search |
| \`cddm refactor <path>\`| Apply automated AST deduplication and cluster extraction |
| \`cddm extract <path>\` | Extract clone clusters into standalone shared modules |
| \`cddm heal <path>\`    | Automatically verify and heal refactored AST nodes |
| \`cddm serve\`         | Start the Axum REST & SSE backend for WebUI Studio |
| \`cddm tui <path>\`    | Launch the Ratatui interactive terminal dashboard |
| \`cddm rules\`         | Validate codebase against architectural boundary rules |

---

## Scan Flags & Quality Gates

\`\`\`bash
cddm scan . \\
  --min-tokens 50 \\
  --min-lines 5 \\
  --fail-threshold 5.0 \\
  --format json \\
  --output report.json
\`\`\`
`,
  },
  {
    title: "WebUI-Studio",
    content: `# WebUI Studio Guide

The CDDM WebUI Studio is a full-fidelity visual workspace built with **React 19**, **Vite Plus**, and **Feature-Sliced Design (FSD)**.

---

## Key Features

1. **Monaco Side-by-Side Diffing**: Real-time syntax-highlighted clone comparisons.
2. **Treemap & Sunburst Visualizers**: Hierarchical code duplication density maps.
3. **Live SSE Watch Daemon**: Instant UI updates upon filesystem modifications.
4. **Interactive AST Refactor Wizard**: Preview and apply cluster extractions with 1-click rollback.
5. **Dark / Light Studio Themes**: Accessible contrast with high-density layout.

---

## Starting the WebUI Studio

\`\`\`bash
cddm serve --port 5173 --watch
\`\`\`
Navigate to \`http://localhost:5173\` in any modern browser.
`,
  },
  {
    title: "MCP-Server-Protocol",
    content: `# Model Context Protocol (MCP) Server Reference

CDDM exposes a dedicated Model Context Protocol (MCP) server for integration with AI Coding Agents (such as Antigravity, Claude Code, and Cursor).

---

## Registered MCP Tools (27 Tools)

Every tool has an isolated, verified test suite under \`tests/mcp/tools/\`:

1. \`scan_codebase\`: Execute full codebase duplicate detection scan.
2. \`cddm_get_clone_pair\`: Retrieve specific clone pair details and code snippets.
3. \`cddm_suggest_refactor\`: Synthesize refactoring recommendations for clone pairs.
4. \`cddm_get_clone_cluster\`: Inspect multi-file clone clusters.
5. \`cddm_suggest_cluster_refactor\`: Synthesize cluster-wide shared module extractions.
6. \`cddm_export_sarif\`: Export scan results in OASIS SARIF v2.1.0 format.
7. \`cddm_diff_scan\`: Perform incremental scan against git reference.
8. \`cddm_get_timeline\`: Query historical clone drift timeline.
9. \`cddm_check_suppression\`: Validate \`@cddm-ignore\` suppression annotations.
10. \`cddm_apply_cluster_refactor\`: Apply AST refactoring to codebase.
11. \`cddm_generate_ai_prompt\`: Generate structured context prompts for LLMs.
12. \`cddm_ast_refactor\`: Perform atomic Tree-sitter AST node transformation.
13. \`cddm_verify_refactor\`: Execute test suites and linters to verify refactor.
14. \`cddm_check_policies\`: Enforce architectural boundaries.
15. \`cddm_heal_refactor\`: Rollback or repair broken refactorings.
16. \`cddm_export_cache_pack\`: Export AST hashes into portable cache archive.
17. \`cddm_import_cache_pack\`: Import pre-computed cache packs for instant CI scans.
18. \`cddm_scan_monorepo\`: Scan multi-package workspaces with workspace isolation.
19. \`cddm_get_semantic_graph\`: Compute semantic dependency and clone graph.
20. \`cddm_compare_semantic_graphs\`: Compare semantic graph topologies.
21. \`cddm_scan_cross_language\`: Detect algorithmic clones across Rust, TS, Python, Go.
22. \`cddm_extract_shared_module\`: Extract duplicate logic into dedicated shared library.
23. \`cddm_detect_overlap\`: Detect near-miss functional overlap.
24. \`cddm_scan_hub\`: Query central monorepo federation registry.
25. \`cddm_extract_hub_package\`: Synthesize internal workspace packages.
26. \`cddm_correlate_coverage\`: Correlate code coverage with duplication density.
27. \`cddm_detect_dead_clones\`: Identify duplicate code paths with zero execution coverage.
`,
  },
  {
    title: "TUI-Studio",
    content: `# TUI Terminal Studio Guide

The CDDM TUI (Terminal User Interface) Studio provides a high-speed, keyboard-driven dashboard powered by **Ratatui** and **Crossterm**.

---

## Keybindings & Navigation

| Key | Action |
| :--- | :--- |
| \`Tab\` / \`Shift+Tab\` | Switch between Overview, Clusters, Pairs, and Timeline tabs |
| \`j\` / \`k\` or \`down\` / \`up\` | Navigate list items |
| \`Enter\` | Open detail modal / inspect side-by-side clone diff |
| \`r\` | Trigger instant live rescan |
| \`e\` | Open refactoring wizard |
| \`/\` | Filter clones by path or language |
| \`q\` / \`Esc\` | Exit TUI Studio |

---

## Launching the TUI

\`\`\`bash
cddm tui .
\`\`\`
`,
  },
  {
    title: "AST-Engine-and-Deduplication",
    content: `# AST Engine & Clone Detection Architecture

CDDM leverages polyglot Tree-sitter parsers, SIMD-accelerated rolling hash winnowing, and AST visitor normalization.

---

## Clone Classification

1. **Type-1 (Exact Clones)**: Identical code fragments ignoring whitespace and comments.
2. **Type-2 (Tokenized Clones)**: Structurally identical fragments with renamed variables, types, and literals.
3. **Type-3 (Near-Miss Clones)**: Modified fragments with inserted, deleted, or reordered statements.
4. **Type-4 (Semantic Clones)**: Functionally identical logic implemented with distinct syntax.

---

## SIMD Vector Acceleration

CDDM uses AVX2 (x86_64) and NEON (AArch64) vector instructions to compute rolling polynomial hashes and cosine similarities across AST vector embeddings in sub-millisecond execution time.
`,
  },
  {
    title: "4-Pillar-Feature-Parity",
    content: `# Cross-Interface Feature Parity Mandate

Every core engine capability in CDDM is delivered simultaneously across all four interaction surfaces:
1. **CLI Engine**
2. **WebUI Studio**
3. **MCP Server**
4. **TUI Studio**

---

## Verification Standard

- **Zero Interface Orphans**: No single-surface features allowed.
- **Dynamic AST Matrix Sync**: Automated via \`bun scripts/sync-feature-matrix.ts\`.
- **CI Verification Gate**: Verified in CI via \`bun scripts/check-feature-parity.ts\`.
`,
  },
  {
    title: "CI-CD-and-Releases",
    content: `# CI/CD Pipelines & Release Distribution

CDDM utilizes **Gitea Actions** and self-hosted runners for automated testing, linting, cross-compilation, and release distribution.

---

## Pipeline Architecture

1. **Rust Quality Gate**: \`cargo fmt\`, \`cargo clippy -D warnings\`, \`cargo test --workspace\`.
2. **WebUI Quality Gate**: \`vp check\`, \`vp -C webui run test\`, production asset bundle build.
3. **MCP Protocol & Scripts Gate**: 1:1 tool contract tests (\`bun test tests/mcp\`), living docs check.
4. **Cross-Compilation Matrix**:
   - Native Linux AMD64 (\`x86_64-unknown-linux-gnu\`)
   - Windows x64 (\`x86_64-pc-windows-gnu\` via MinGW GCC)
5. **VS Code VSIX Packager**: Extension packaging and automated artifact distribution.
`,
  },
];
