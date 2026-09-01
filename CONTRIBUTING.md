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
│   ├── cddm-cli/       # CLI application, TUI dashboard & Axum embedded WebUI server
│   ├── cddm-lsp/       # Language Server Protocol 3.17 daemon for IDE diagnostics
│   └── cddm-mcp/       # Model Context Protocol stdio server for AI agents
├── webui/              # React 19 + Vite Plus + Tailwind CSS studio frontend
├── npm/                # npm cross-platform binary package distribution
├── scripts/            # Setup and verification automation scripts
└── docs/               # Interface references (CLI, MCP, WebUI, TUI), Architecture & Feature Matrix
```

For comprehensive manuals on specific interaction surfaces, consult:

- [docs/CLI.md](docs/CLI.md) — CLI Command Reference & Flags
- [docs/MCP.md](docs/MCP.md) — Model Context Protocol AI Agent Tools & Resources
- [docs/WEBUI.md](docs/WEBUI.md) — Embedded React 19 Studio WebUI Guide
- [docs/TUI.md](docs/TUI.md) — Terminal UI Keyboard Shortcuts & Views
- [docs/FEATURE_PARITY.md](docs/FEATURE_PARITY.md) — 4-Pillar Cross-Interface Feature Parity Matrix
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System Architecture & Crate Breakdown

---

## Getting Started

### Prerequisites

- **Rust**: 2024 edition (1.85+ recommended). Install via [rustup.rs](https://rustup.rs).
- **Vite Plus**: `0.3.0` via [viteplus.dev](https://viteplus.dev) or **Bun**: `1.4.0` via [bun.sh](https://bun.sh).

### Setting Up Development Workspace

1. **Clone the repository**:

   ```bash
   git clone https://git.gt-web-dev.com/gt-dev/cddm.git
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

You can run the complete quality pipeline, auto-fix, clean, or workspace reset across both Rust backend, WebUI, and scripts with single cross-platform commands:

```bash
# 1. Run all 18 checks, tests, typechecks, lints, builds, and dogfood self-scan (Read-Only)
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

# Run all workspace unit & integration tests
cargo test --workspace
```

#### WebUI Frontend

```bash
cd webui

# Check TypeScript types and linting with Vite Plus
vp check

# Run unit and component tests (Vitest)
vp run test

# Build production assets (embedded into rust-embed)
vp run build
```

---

## How to Contribute

### 1. Gitea SSoT Issue Discovery

CDDM enforces [Gitea (`git.gt-web-dev.com`)](https://git.gt-web-dev.com/gt-dev/cddm) as the authoritative Single Source of Truth for issues, roadmaps, and PRs. GitHub is strictly a downstream replica mirror.

1. Always check existing [Gitea Issues](https://git.gt-web-dev.com/gt-dev/cddm/issues) before starting work.
2. If no issue exists, create a new issue on Gitea (e.g. `Issue #19`) to establish the primary authoritative tracking record.

### 2. Canonical Branching & Development

1. Create a working branch derived strictly from the primary Gitea issue number:

   ```bash
   # Feature branch
   git checkout -b feat/issue-19-my-feature-description

   # Bugfix branch
   git checkout -b fix/issue-19-my-fix-description
   ```

2. Make changes end-to-end with unit tests covering all 4 interface pillars where applicable (CLI, WebUI, MCP, TUI).
3. Commit with clear, descriptive messages following Conventional Commits referencing the primary Gitea issue:
   - `feat(core): add Go tree-sitter grammar support (#19)`
   - `fix(webui): correct slider token threshold calculation (#19)`
   - `docs(mcp): update MCP tool setup guide (#19)`
4. Commit messages are automatically validated via `@commitlint/cli` and `commitlint.config.ts`.
5. Run `vp run verify` to confirm all 18 quality checks pass.

### 3. Pull Requests, Auto-Closing & API Merge

1. Push your branch to `origin` (Gitea) first, then mirror to `github`:

   ```bash
   git push origin feat/issue-19-my-feature-description
   ```

2. Open the primary Pull Request on [Gitea](https://git.gt-web-dev.com/gt-dev/cddm/pulls) merging into `main`.
3. Include closing keywords (`Fixes #19` or `Closes #19`) in the PR description and assign the target Milestone (e.g. `v1.11.0`).
4. Merges into `main` are executed via the official Gitea REST API (`POST /repos/{owner}/{repo}/pulls/{id}/merge`), automatically marking the PR as merged, closing the linked issue, and deleting the feature branch.

### 4. Automated Semantic Releases

Releases synchronize all 10 project manifests (`package.json`, `Cargo.toml`, `webui/package.json`, NPM packages, VS Code extension, Homebrew, Scoop, Winget, and README badges) and trigger multi-platform CI cross-compilation:

```bash
# Preview calculated version and changelog entry (dry-run)
vp run version:check

# Synchronize all 10 manifests and tag release (e.g. v1.11.0)
vp run version:release
# or:
vp run bump
```

---

## Code of Conduct

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## License

By contributing, you agree that your contributions will be dual-licensed under the [MIT License](LICENSE-MIT) and [Apache 2.0 License](LICENSE-APACHE).
