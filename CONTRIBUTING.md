# Contributing to CDDM

First off, thank you for considering contributing to **CDDM** (_Code De-Duplication Meister_)! It is open-source software built for performance, polyglot developer experience, and code quality analysis.

---

## Project Architecture Overview

CDDM is structured as a Rust cargo workspace with an embedded React 19 WebUI:

```text
cddm/
├── .vite-hooks/        # Native Vite Plus pre-commit, pre-push, and commit-msg quality hooks
├── .vscode/            # Aligned IDE settings, formatters, tasks, and debug profiles
├── crates/
│   ├── cddm-core/      # Core algorithm library (Winnowing M61, Tree-sitter AST, gix blame)
│   ├── cddm-cli/       # CLI application & Axum embedded WebUI server
│   └── cddm-mcp/       # Model Context Protocol stdio server for AI agents
├── webui/              # React 19 + Vite Plus + Tailwind CSS studio frontend
├── npm/                # npm cross-platform binary package distribution
├── scripts/            # Setup and verification automation scripts
└── docs/               # Architecture, API, Requirements & Feature Matrix docs
```

For a deep dive into internal design, read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/API.md](docs/API.md).

---

## Getting Started

### Prerequisites

- **Rust**: 2024 edition (1.85+ recommended). Install via [rustup.rs](https://rustup.rs).
- **Vite Plus**: `0.2.9` via [viteplus.dev](https://viteplus.dev) or **Bun**: `1.4.0` via [bun.sh](https://bun.sh).

### Setting Up Development Workspace

1. **Clone the repository**:

   ```bash
   git clone https://github.com/GrigorTonikyan/cddm.git
   cd cddm
   ```

2. **Configure Git Hooks**:

   CDDM includes native Git hooks in `.vite-hooks/` to ensure formatters and linters run automatically on every commit:

   ```bash
   # Option A: via workspace prepare script
   vp run prepare

   # Option B: via Vite Plus config command directly
   vp config
   ```

3. **Build Rust crates**:

   ```bash
   cargo build
   ```

4. **Install WebUI dependencies**:

   ```bash
   cd webui
   vp install
   ```

---

## Running Tests & Quality Checks

### Master Workspace Runners

You can run the complete quality pipeline, auto-fix, clean, or workspace reset across both Rust backend and React WebUI with single cross-platform commands:

```bash
# 1. Run all 11 checks, tests, typechecks, lints, builds, and dogfood self-scan (Read-Only)
vp run verify
# or directly:
bun scripts/verify.ts

# 2. Automatically fix all auto-fixable formatting & lints, then run full verification
vp run fix
# or directly:
bun scripts/fix.ts

# 3. Deep clean all build artifacts, temp files, caches, test reports, and lockfiles
vp run clean
# or directly:
bun scripts/clean.ts

# 4. Deep clean, reinstall dependencies, configure hooks, build, and verify workspace
vp run reset
# or directly:
bun scripts/reset.ts
```

### Individual Subsystems

#### Rust Crates

```bash
# Check formatting
cargo fmt --check

# Run Clippy lints (zero warning policy)
cargo clippy --workspace --all-targets -- -D warnings

# Run all 38 workspace unit & integration tests
cargo test --workspace
```

#### WebUI Frontend

```bash
cd webui

# Check TypeScript types
vp run check

# Run Vite Plus linter
vp run lint

# Check code formatting with Vite Plus
vp run format:check

# Run unit tests (Vitest - 24 tests)
vp run test

# Build production assets (embedded into rust-embed)
vp run build
```

---

## How to Contribute

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
3. Commit with clear, descriptive messages following Conventional Commits:
   - `feat(core): add Go tree-sitter grammar support`
   - `fix(webui): correct slider token threshold calculation`
   - `docs: update MCP setup guide`
   - `feat(api)!: breaking change to scan endpoint`
4. Commit messages are automatically checked via `@commitlint/cli` and `commitlint.config.ts`.
5. Run `vp run verify` to confirm all 11 quality checks pass.
6. Push to your fork and submit a Pull Request to `main`.

### 3. Releasing & Semantic Versioning

For maintainers creating new releases:

```bash
# Preview calculated version and changelog entry (dry-run)
vp run version:check

# Bump version and synchronize Cargo.toml, package manifests, and CHANGELOG.md
vp run bump

# Complete release: bumps versions, regenerates changelog, creates git commit and tag
vp run version:release
```

---

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## License

By contributing, you agree that your contributions will be dual-licensed under the [MIT License](LICENSE-MIT) and [Apache 2.0 License](LICENSE-APACHE).
