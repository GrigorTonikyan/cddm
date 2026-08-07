# CDDM — Code De-Duplication Meister

> High-Performance Polyglot Code Clone Detection, DRY Health Analysis, AST Subtree Hasher & Embedded Studio WebUI.

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![Bun](https://img.shields.io/badge/bun-1.2+-black.svg)](https://bun.sh)
[![React](https://img.shields.io/badge/react-19.0-61dafb.svg)](https://react.dev)

---

## ⚡ Overview

**CDDM** (*Code De-Duplication Meister*) is an open-source, ultra-fast, multi-threaded polyglot code duplication engine built natively in Rust. It outperforms legacy copy/paste detectors with:

- **Winnowing $M_{61} = 2^{61}-1$ Rolling Hash Engine**: Ultra-fast $O(N)$ token fingerprinting across 30+ programming languages.
- **Tree-sitter AST Subtree Merkle Hasher**: Structural Type 3 (near-miss) and Type 4 (semantic) AST clone detection using `blake3`.
- **In-Process Git Blame Annotation**: Powered by `gix` (`gitoxide`) to annotate duplicate fragments with author names and line commit timestamps.
- **Embedded Studio WebUI (`cddm serve`)**: Served directly from the compiled single binary via `axum` and `rust-embed`.
- **Model Context Protocol (MCP) Server (`cddm-mcp`)**: Native AI agent integration via stdio JSON-RPC.
- **Dual Distribution**: Distributable via `cargo install cddm` and `npm install -g cddm`.

---

## 🚀 Installation & Usage

### Via Cargo
```bash
cargo install cddm
```

### Via npm
```bash
npm install -g cddm
```

---

## 💻 CLI Commands

### 1. Terminal Code Scan (`cddm scan`)
```bash
# Basic scan with ANSI table output
cddm scan ./src --min-tokens 50

# Output JSON report for CI/CD pipelines with failure threshold
cddm scan ./src --format json --fail-threshold 10.0

# Markdown report with in-process git blame annotations
cddm scan ./src --format markdown --git-blame
```

### 2. Embedded Studio WebUI (`cddm serve`)
```bash
# Launch interactive React WebUI in browser
cddm serve --port 3000 --open
```

### 3. Model Context Protocol Server (`cddm-mcp`)
```bash
# Launch MCP server for AI coding agents
cddm-mcp
```

---

## 🏗 Architecture & Workspace Layout

```text
x:\projects\cddm\
├── Cargo.toml                  # Workspace root (resolver = "2", edition = "2024")
├── crates/
│   ├── cddm-core/              # Pure library crate: Winnowing, Tree-sitter AST, gix blame, cache
│   ├── cddm-cli/               # CLI binary crate with clap & Axum embedded WebUI server
│   └── cddm-mcp/               # Model Context Protocol stdio binary server
│
├── webui/                      # React 19 + Vite + Bun WebUI app
│   └── dist/                   # Compiled static assets embedded via rust-embed
│
└── npm/                        # Cross-platform npm package wrapper & binary shim
```

---

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
