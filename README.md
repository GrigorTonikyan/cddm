# CDDM — Code De-Duplication Meister

> **High-Performance Polyglot Code Clone Detection, DRY Health Analysis, AST Subtree Hasher & Embedded Studio WebUI.**

[![CI](https://git.gt-web-dev.com/gt-dev/cddm/actions/workflows/ci.yml/badge.svg)](https://git.gt-web-dev.com/gt-dev/cddm/actions/workflows/ci.yml)
[![License: MIT / Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Vite Plus](https://img.shields.io/badge/vite%2B-0.3.0-purple.svg)](https://viteplus.dev)
[![TypeScript](https://img.shields.io/badge/typescript-7.0-blue.svg)](https://www.typescriptlang.org)
[![React](https://img.shields.io/badge/react-19.2-61dafb.svg)](https://react.dev)
[![npm version](https://img.shields.io/badge/npm-1.10.0-red.svg)](https://www.npmjs.com/package/cddm)
[![crates.io](https://img.shields.io/badge/crates.io-1.10.0-brightgreen.svg)](https://crates.io/crates/cddm)

---

## Table of Contents

- [Overview](#overview)
- [The 4 Interface Pillars](#the-4-interface-pillars)
- [Key Features](#key-features)
- [Supported Languages](#supported-languages)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Interface Documentation](#interface-documentation)
- [DRY Health Score Formula](#dry-health-score-formula)
- [Architecture & Documentation](#architecture--documentation)
- [Performance Benchmarks](#performance-benchmarks)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

**CDDM** (_Code De-Duplication Meister_) is an open-source, ultra-fast, multi-threaded polyglot code clone detection, structural AST refactoring, and architectural governance engine built natively in Rust. Designed to scale seamlessly across enterprise monorepos, CDDM detects exact (Type 1), renamed (Type 2), structural near-miss (Type 3), and semantic graph (Type 4) clones in milliseconds.

---

## The 4 Interface Pillars

CDDM strictly enforces **100% Cross-Interface Feature Parity** across all four primary interaction surfaces:

```text
+----------------------------------------------------------------------------------------------------+
|                                    CDDM Unified Core Engine                                        |
+----------------------------------------------------------------------------------------------------+
|  1. CLI Engine        |  2. WebUI Studio       |  3. MCP Server         |  4. TUI Studio           |
|  - 22 Subcommands     |  - React 19 Studio     |  - 30 Agent Tools      |  - 12 Ratatui Tabs       |
|  - Scriptable stdout  |  - Monaco Split Diffs  |  - 17 MCP Resources    |  - Split Diff Viewer     |
|  - Turnkey CI/CD      |  - SSE Live Stream     |  - 3 Agent Prompts     |  - Keyboard-Driven       |
|  [docs/CLI.md]        |  [docs/WEBUI.md]       |  [docs/MCP.md]         |  [docs/TUI.md]           |
+----------------------------------------------------------------------------------------------------+
```

---

## Key Features

| Feature                              | Description                                                                                                              |
| :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| **M61 Rolling Hash Winnowing**       | Sub-linear O(N) token fingerprinting using Mersenne Prime $M_{61} = 2^{61} - 1$ for collision-resistant clone detection. |
| **SIMD Vectorized Reduction**        | Hardware-accelerated AVX2 (x86_64) and ARM NEON (AArch64) vector lanes computing parallel Mersenne-61 modular reduction. |
| **Zero-Copy Memory Mapping**         | High-throughput `memmap2` zero-copy memory mapping for large files (> 64KB) bypassing heap allocation overhead.          |
| **Tree-sitter AST Hasher**           | Parses source trees into ASTs and hashes subtrees with `blake3` Merkle hashing to identify structural near-misses.       |
| **Cross-Language Semantic Matching** | Detects algorithmic equivalents across different languages using Weisfeiler-Lehman graph kernels and neural subwords.    |
| **Polyglot Dead Code Detection**     | Detects unreferenced functions, unreachable blocks, and 0-hit duplicate clones correlated with coverage.                 |
| **Ecosystem Library Overlap**        | Detects reimplemented standard and third-party library algorithms (e.g. lodash, itertools) and suggests native imports.  |
| **Organization Federation Hub**      | Multi-repository duplication scanner and cross-repo shared package extractor (`.cddmhub.toml`).                          |
| **Runtime Coverage Correlation**     | Ingests LCOV, Cobertura, and Istanbul coverage tracefiles to calculate clone execution hit counts and risk scores.       |
| **DRY Health Scoring**               | Mathematical 0.0 - 100.0 codebase health index factoring in duplication percentage and cross-module boundaries.          |
| **In-Process Git Blame**             | Powered by `gix` (`gitoxide`) to annotate duplicate fragments with author names and commit timestamps in-process.        |
| **N-Way Graph Clustering**           | Disjoint-Set Union-Find algorithm partitioning pairwise clone graphs into transitive $N$-way equivalence classes.        |
| **Multi-Site Patch Synthesizer**     | Computes consensus invariant lines and synthesizes unified multi-file `.patch` diffs across all clone sites.             |
| **Autonomous AI Code Surgeon**       | Closed-loop iterative AI refactoring with automated test suite verification and rollback guarantees (`cddm heal`).       |
| **Distributed Cache Packs**          | Exports and imports portable `.cddmpack` binary fingerprint archives for lightning-fast CI/CD runs.                      |
| **Turnkey CI/CD Generators**         | Turnkey workflow generators for Gitea Actions, GitHub Actions, GitLab CI, and Azure Pipelines.                           |

---

## Supported Languages

CDDM includes built-in tokenizers and Tree-sitter AST parsers for **23 programming languages**:

```text
Rust (.rs)           TypeScript (.ts, .tsx)  JavaScript (.js, .jsx, .mjs)
Python (.py)         Go (.go)                Java (.java)
C (.c, .h)           C++ (.cpp, .hpp, .cc)   C# (.cs)
Ruby (.rb)           PHP (.php)              Kotlin (.kt, .kts)
Swift (.swift)       SQL (.sql)              Bash / Shell (.sh, .bash)
Lua (.lua)           CSS / SCSS / LESS       HTML (.html, .htm)
JSON (.json)         Zig (.zig, .zon)        Scala (.scala, .sc)
Elixir (.ex, .exs)   Dockerfile / Containerfile
```

---

## Installation

### Via Homebrew (macOS & Linux)

```bash
brew install GrigorTonikyan/cddm/cddm
```

### Via Scoop (Windows)

```bash
scoop bucket add cddm https://git.gt-web-dev.com/gt-dev/cddm.git
scoop install cddm
```

### Via Windows Package Manager (Winget)

```powershell
winget install GrigorTonikyan.cddm
```

### Via Cargo (Recommended for Rust developers)

```bash
cargo install cddm
```

### Via npm (Cross-platform binary wrapper)

```bash
npm install -g cddm
```

### Via VS Code / Cursor Marketplace

Install the official **CDDM Studio Extension** (`editors/vscode`) directly from the extension marketplace for in-editor squiggles, quick-fix refactorings, and the embedded WebUI sidebar.

---

## Quick Start

```bash
# 1. Fast duplicate scan on current workspace
cddm scan ./src

# 2. Differential scan comparing working tree against main branch in CI
cddm diff main --fail-threshold 3.0

# 3. Launch interactive 12-tab Terminal UI (TUI)
cddm tui

# 4. Launch visual React 19 Studio WebUI in browser
cddm serve --port 3000 --open

# 5. Autonomous AI Code Surgeon refactoring with test loop
cddm heal --cluster 1 --provider gemini --model gemini-2.5-pro --verify --test-cmd "cargo test"

# 6. Generate turnkey Gitea Actions CI/CD workflow
cddm init gitea --write
```

---

## Interface Documentation

For comprehensive technical manuals on each interaction surface, explore our dedicated guides:

- **[CLI Command Reference](docs/CLI.md)** — Exhaustive manual for all 22 CLI subcommands, flags, and recipes.
- **[MCP Server Specification](docs/MCP.md)** — Stdio JSON-RPC 2.0 reference for all 30 AI agent tools, 17 resources, and prompts.
- **[Embedded Studio WebUI](docs/WEBUI.md)** — React 19 Studio guide, 19 interactive modals, REST & SSE API catalog.
- **[Terminal UI (TUI) Studio](docs/TUI.md)** — Keyboard shortcuts and 12-tab navigation guide.
- **[Language Server & IDE Setup](docs/LSP_SETUP.md)** — Real-time in-editor setup for VS Code, Cursor, Neovim, Zed, Helix, Sublime.
- **[4-Pillar Feature Parity Matrix](docs/FEATURE_PARITY.md)** — Authoritative governance matrix across all 21 core capabilities.

---

## DRY Health Score Formula

CDDM evaluates codebase modularity health using a continuous mathematical scoring function:

$$\text{Score} = \max\left(0, \min\left(100, (100 - 1.5 \times \text{Duplication\_Percentage}) \times (1 - 0.25 \times \text{Cross\_Module\_Ratio})\right)\right)$$

Where:

- **Duplication_Percentage**: Ratio of duplicate clone tokens to total workspace tokens ($\frac{\text{Clone Tokens}}{\text{Total Tokens}} \times 100$).
- **Cross_Module_Ratio**: Proportion of clone pairs spanning distinct top-level directories.

---

## Architecture & Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — Pipeline phases, SIMD reduction, and crate breakdown.
- [docs/API.md](docs/API.md) — Axum REST endpoints and JSON schema specifications.
- [docs/FEATURE_MATRIX.md](docs/FEATURE_MATRIX.md) — Detailed capability and test coverage matrix.
- [docs/REQUIREMENTS.md](docs/REQUIREMENTS.md) — Performance SLAs, throughput bounds, and constraints.
- [docs/ROADMAP.md](docs/ROADMAP.md) — Enhancement proposals (EP-01 to EP-34) and release milestones.
- [docs/TODO.md](docs/TODO.md) — Active engineering tasks and technical debt registry.

---

## Performance Benchmarks

Scanning benchmark on a codebase of **500,000 LOC** across 1,200 files:

| Tool      | Engine Language | Scan Time  | Memory Usage | Type 3 AST Clones     |
| :-------- | :-------------- | :--------- | :----------- | :-------------------- |
| **CDDM**  | **Rust 2024**   | **180 ms** | **45 MB**    | **Yes (Tree-sitter)** |
| `jscpd`   | Node.js         | 4,200 ms   | 320 MB       | No                    |
| `PMD-CPD` | Java            | 6,800 ms   | 510 MB       | No                    |

---

## Contributing

We welcome community contributions! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) for our Git issue-driven development protocol, branch conventions, and testing guidelines.

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) and [Security Policy](SECURITY.md).

---

## License

Dual-licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
