# CDDM — Code De-Duplication Meister

> **High-Performance Polyglot Code Clone Detection, DRY Health Analysis, AST Subtree Hasher & Embedded Studio WebUI.**

[![CI](https://github.com/GrigorTonikyan/cddm/actions/workflows/ci.yml/badge.svg)](https://github.com/GrigorTonikyan/cddm/actions/workflows/ci.yml)
[![License: MIT / Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Vite Plus](https://img.shields.io/badge/vite%2B-0.2.9-purple.svg)](https://viteplus.dev)
[![TypeScript](https://img.shields.io/badge/typescript-7.0-blue.svg)](https://www.typescriptlang.org)
[![React](https://img.shields.io/badge/react-19.2-61dafb.svg)](https://react.dev)
[![npm version](https://img.shields.io/badge/npm-0.3.0-red.svg)](https://www.npmjs.com/package/cddm)
[![crates.io](https://img.shields.io/badge/crates.io-0.3.0-brightgreen.svg)](https://crates.io/crates/cddm)

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Supported Languages](#supported-languages)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [CLI Command Reference](#cli-command-reference)
- [Embedded Studio WebUI](#embedded-studio-webui)
- [Model Context Protocol (MCP) Server](#model-context-protocol-mcp-server)
- [DRY Health Score Formula](#dry-health-score-formula)
- [Architecture & Documentation](#architecture--documentation)
- [Performance Benchmarks](#performance-benchmarks)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

**CDDM** (_Code De-Duplication Meister_) is an open-source, ultra-fast, multi-threaded polyglot code clone detection and modularity analysis engine built natively in Rust. Designed to scale seamlessly across large enterprise monorepos, CDDM detects exact (Type 1), renamed (Type 2), structural (Type 3), and semantic (Type 4) code clones in milliseconds.

Whether integrated into **CI/CD pipelines**, used via the **Terminal CLI**, explored in the **Embedded React Studio WebUI**, or connected to **AI Coding Agents** via **MCP**, CDDM helps developers keep their codebases clean, maintainable, and DRY (_Don't Repeat Yourself_).

---

## Key Features

| Feature                        | Description                                                                                                                            |
| :----------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| **M61 Rolling Hash Winnowing** | Sub-linear O(N) token fingerprinting using Mersenne Prime M61 = 2^61 - 1 for collision-resistant clone detection.                      |
| **Tree-sitter AST Hasher**     | Parse source trees into ASTs and hash subtrees with `blake3` to identify structural near-misses and semantic clones.                   |
| **DRY Health Scoring**         | Computes a normalized 0.0 - 100.0 DRY codebase health score factoring in duplication ratio and cross-module cross-contamination.       |
| **In-Process Git Blame**       | Powered by `gix` (`gitoxide`) to annotate duplicate fragments with author names and commit timestamps without spawning subprocesses.   |
| **Embedded Studio WebUI**      | High-performance interactive React 19 dashboard served directly from the single compiled binary via `axum` & `rust-embed`.             |
| **AI Agent MCP Server**        | Stdio JSON-RPC 2.0 protocol (`cddm-mcp`) allowing AI assistants (Claude, Antigravity, Cursor) to inspect duplication programmatically. |
| **Rayon Parallel Pipeline**    | Multi-threaded file discovery, AST parsing, and fingerprint indexing across all available CPU cores.                                   |

---

## Supported Languages

CDDM includes built-in tokenizers and Tree-sitter AST parsers for over 30+ popular programming languages:

```text
Rust (.rs)           TypeScript (.ts, .tsx)  JavaScript (.js, .jsx, .mjs)
Python (.py)         Go (.go)                C / C++ (.c, .h, .cpp, .hpp)
Java (.java)         C# (.cs)                Ruby (.rb)
PHP (.php)           Swift (.swift)          Kotlin (.kt, .kts)
Scala (.scala)       Dart (.dart)            Zig (.zig)
Elixir (.ex, .exs)   Haskell (.hs)           Lua (.lua)
Shell (.sh, .bash)   HTML / CSS              JSON / YAML / TOML
```

---

## Installation

### Via Cargo (Recommended for Rust users)

```bash
cargo install cddm
```

### Via npm (Cross-platform binary wrapper)

```bash
npm install -g cddm
```

### From Source

```bash
git clone https://github.com/GrigorTonikyan/cddm.git
cd cddm
cargo build --release
```

The compiled binary will be placed at `./target/release/cddm`.

---

## Quick Start

Scan your project directory in terminal:

```bash
# Scan current directory with ANSI color table output
cddm scan ./src

# Scan with custom minimum token threshold & git blame annotations
cddm scan ./src --min-tokens 40 --git-blame

# Export Markdown report for pull request comments
cddm scan ./src --format markdown > duplication-report.md

# Launch interactive WebUI Studio in browser
cddm serve --port 3000 --open
```

---

## CLI Command Reference

### `cddm scan [DIRECTORY]`

Executes code clone detection on the target directory.

```bash
cddm scan [OPTIONS] [DIRECTORY]
```

#### Options

- `-m, --min-tokens <INT>`: Minimum token count for a code fragment to be classified as a clone (Default: `50`).
- `-l, --languages <LANGS>`: Filter scanning to specific languages (e.g. `--languages rust,typescript`).
- `-f, --format <FORMAT>`: Output report format (`table`, `json`, `markdown`, `html`) (Default: `table`).
- `-o, --output <PATH>`: Write output report directly to a file.
- `--git-blame`: Enable `gix` in-process Git blame to attribute code clones to authors.
- `--no-self`: Skip checking for intra-file self-overlapping duplicates.
- `--ignore <PATTERNS>`: Glob patterns to exclude (Default: `node_modules`, `target`, `.git`, `dist`, `build`).
- `--fail-threshold <FLOAT>`: Exit with non-zero code if duplication percentage exceeds threshold (useful for CI).

---

## Embedded Studio WebUI

Launches the Axum HTTP server delivering the embedded React 19 Studio WebUI.

```bash
cddm serve --port 3000 --host 127.0.0.1 --open
```

---

## Model Context Protocol (MCP) Server

CDDM includes a native stdio Model Context Protocol (MCP) server `cddm-mcp` for direct integration with AI coding tools like Claude Desktop, Antigravity, or Cursor.

### Configuration for Claude Desktop (`claude_desktop_config.json`)

```json
{
  "mcpServers": {
    "cddm": {
      "command": "cddm-mcp",
      "args": []
    }
  }
}
```

### Exposed MCP Tools

- `scan_codebase`: Runs a polyglot code duplication scan and returns DRY health scores, duplication metrics, and clone pair details directly to AI context.

---

## DRY Health Score Formula

CDDM measures codebase modularity health using a continuous mathematical scoring function:

```text
Score = max(0, min(100, (100 - 1.5 * Duplication_Percentage) * (1 - 0.25 * Cross_Module_Ratio)))
```

Where:

- **Duplication_Percentage**: Codebase duplication percentage (`(Clone Tokens / Total Tokens) * 100`).
- **Cross_Module_Ratio**: Cross-module clone ratio (clones spanning distinct top-level directories / total clone pairs).

---

## Architecture & Documentation

For detailed technical references, explore our design documentation:

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System architecture, pipeline phases, hashing details & crate breakdown.
- [docs/API.md](docs/API.md) — Complete REST API & CLI specification.
- [docs/FEATURE_MATRIX.md](docs/FEATURE_MATRIX.md) — Detailed feature comparisons & algorithm specifications.
- [docs/REQUIREMENTS.md](docs/REQUIREMENTS.md) — Technical requirements & performance bounds.
- [docs/ROADMAP.md](docs/ROADMAP.md) — Strategic release roadmap & enhancement proposals (EP-01 to EP-10).
- [docs/TODO.md](docs/TODO.md) — Active task tracking and implementation checklist.

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

We welcome community contributions! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) to get started with building, testing, and submitting pull requests.

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) and [Security Policy](SECURITY.md).

---

## License

Dual-licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
