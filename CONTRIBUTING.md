# Contributing to CDDM

First off, thank you for considering contributing to **CDDM** (*Code De-Duplication Meister*)! It is open-source software built for performance, polyglot developer experience, and code quality analysis.

---

## 🛠 Project Architecture Overview

CDDM is structured as a Rust cargo workspace with an embedded React 19 WebUI:

```text
cddm/
├── crates/
│   ├── cddm-core/      # Core algorithm library (Winnowing M₆₁, Tree-sitter AST, gix blame)
│   ├── cddm-cli/       # CLI application & Axum embedded WebUI server
│   └── cddm-mcp/       # Model Context Protocol stdio server for AI agents
├── webui/              # React 19 + Vite + Tailwind CSS studio frontend (Bun)
├── npm/                # npm cross-platform binary package distribution
└── docs/               # Architecture, API, Requirements & Feature Matrix docs
```

For a deep dive into internal design, read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/API.md](docs/API.md).

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: 2024 edition (1.85+ recommended). Install via [rustup.rs](https://rustup.rs).
- **Bun**: 1.2+ or **Node.js**: 20+. Install via [bun.sh](https://bun.sh).

### Setting Up Development Workspace

1. **Clone the repository**:
   ```bash
   git clone https://github.com/GrigorTonikyan/cddm.git
   cd cddm
   ```

2. **Build Rust crates**:
   ```bash
   cargo build
   ```

3. **Install WebUI dependencies**:
   ```bash
   cd webui
   bun install
   ```

---

## 🧪 Running Tests & Quality Checks

### Rust Crates

Before opening a pull request, ensure all Rust unit tests pass and code is formatted cleanly:

```bash
# Run all workspace unit & integration tests
cargo test

# Check formatting
cargo fmt --check

# Run Clippy lints
cargo clippy --workspace --all-targets -- -D warnings
```

### WebUI Frontend

```bash
cd webui

# Run unit tests (Vitest)
bun run test

# Check TypeScript types
bun run check

# Build production assets (embedded into rust-embed)
bun run build
```

---

## 💡 How to Contribute

### 1. Adding Support for a New Language
To add a new language grammar to `cddm-core`:
1. Add the corresponding `tree-sitter-<lang>` crate to `Cargo.toml` dependencies.
2. Register the extension mapping and language grammar in `crates/cddm-core/src/grammar.rs`.
3. Add tokenizer rules and tests in `crates/cddm-core/src/tokenizer.rs`.
4. Verify AST subtree hashing works in `crates/cddm-core/src/ast/parser.rs`.

### 2. Submitting Pull Requests

1. **Fork the repo** and create your branch from `main`:
   ```bash
   git checkout -b feature/my-cool-feature
   ```
2. Make your changes and write unit tests covering new functionality.
3. Commit with clear, descriptive commit messages following standard conventions:
   - `feat: add support for Go tree-sitter grammar`
   - `fix: handle edge case in winnowing window sliding`
   - `docs: update MCP setup guide`
4. Push to your fork and submit a Pull Request to `main`.

---

## 📜 Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## 📄 License

By contributing, you agree that your contributions will be dual-licensed under the [MIT License](LICENSE-MIT) and [Apache 2.0 License](LICENSE-APACHE).
