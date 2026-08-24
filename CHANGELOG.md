# Changelog

All notable changes to **CDDM** (_Code De-Duplication Meister_) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.4.0] - 2026-08-24

### Features

- **suppression**: implement intelligent AST-aware suppression engine with `.cddmignore` glob rules, per-path threshold overrides, inline comment directives (`// cddm:ignore`, `/* cddm:ignore-start */`), and test/mock/generated file auto-detection
- **refactor**: implement interactive auto-refactor sandbox studio with customized function signatures, destination module placement, and transactional Git branch application (`gix`)
- **cli**: add `cddm ignore init` and `cddm ignore check` subcommands, plus `--cddmignore`, `--ignore-tests`, `--ignore-mocks`, and `--ignore-generated` flags
- **mcp**: add `cddm_check_suppression` and `cddm_apply_cluster_refactor` tools, and `cddm://workspace/suppressions` resource
- **webui**: add `SuppressionRulesModal` and `RefactorSandboxModal` with live parameter customization, colorized diff preview, and one-click Git branch deployment
- **timeline**: implement git duplication trends and turnkey ci generator (`da3ed61`)
- **lsp**: implement real-time language server engine and vscode extension (`14f7bdb`)
- **core**: implement live watch SSE sync, workspace patch application, and IDE deeplinks (`b917475`)
- **core**: add N-way clone graph clustering and multi-site deduplication synthesis (`b0735f9`)
- **core**: implement high-throughput zero-copy mmap and simd rolling hash vectorization (`d582f32`)

## [0.6.0] - 2026-08-24

### Features

- **core**: expand polyglot tree-sitter grammars and integrate ast merkle clone classifier (`964aeb7`)
- **core**: implement unified diff parser and atomic workspace patch applier
- **cli**: implement real-time live watch subcommand `cddm watch` with delta reporting
- **cli**: implement Server-Sent Events `/api/events` and patch application endpoint `/api/apply-patch`
- **webui**: implement live watch push sync, IDE protocol deeplinks, and direct refactor application

### Refactoring

- **webui**: implement modular atomic UI primitives and win2x window manager (`b551fd9`)

### Documentation

- update feature matrix, roadmap, and architecture for milestone v0.5.0 (`21d9de1`)

## [0.5.0] - 2026-08-23

### Features

- **webui**: implement atomic windowing system, diff viewer, duplication treemap, and refactor modal (`6142cc8`)

## [0.4.0] - 2026-08-23

### Features

- **core**: implement redb caching, git differential scans, and refactor advisor (`0d24083`)

## [0.3.0] - 2026-08-23

### Features

- **core**: implement SARIF exporter, refactor advisor, and MCP tools (`db8517d`)

### Tooling & Maintenance

- **vscode**: replace eslint extension with vite-plus extension pack (`5d2da2f`)

## [0.2.0] - 2026-08-23

### Features

- **scripts**: sync README badges and Cargo.lock on version bump (`83f324c`)
- **scripts**: add workspace clean and reset runners with full verification (`47ad3ee`)
- **tooling**: enforce zero-emoji policy and eradicate emojis across codebase (`c499090`)

### Bug Fixes

- **vscode**: remove duplicate flags from rust-analyzer extraArgs (`cdaf76f`)
- **ci**: enforce workspace-wide formatting in CI and configure markdownlint MD024 (`f300cab`)

### Documentation

- **agents**: add zero-downgrade dependency policy to prime directives (`e345352`)
- add comprehensive strategic roadmap, enhancement proposals, and active todo tracker (`27d108f`)

### Tooling & Maintenance

- **deps**: upgrade rust dependencies to latest versions preserving precision (`8c5f20a`)
- **cargo**: enforce missing_debug_implementations = deny workspace-wide (`37dfab5`)
- **deps**: update vitest to 4.1.11 in webui (`16d7dec`)
- **docs**: integrate doc integrity and roadmap sync into verification pipeline (`e333809`)

## [0.1.2] - 2026-08-23

### Features

- **mcp**: add `cddm-mcp` stdio JSON-RPC 2.0 server for AI coding agents
- **ast**: add Tree-sitter AST subtree hashing with Blake3 and filesystem watcher
- **webui**: embedded React 19 Studio WebUI served natively from Axum binary
- **blame**: in-process `gix` Git blame author annotation without subprocesses
- **core**: Winnowing M61 rolling hash clone detection engine in Rust 2024

### Bug Fixes

- merge overlapping clone pairs to prevent combinatorial explosion
- resolve Axum thread starvation and large DOM render freezing
- enforce strict typing in Zustand store and resolve floating promises

### Documentation

- comprehensive system architecture, API specifications, and feature matrix
- exhaustively verified requirements and performance benchmark tables

### Tooling & Maintenance

- unified workspace-wide toolchain with Vite Plus (`vp`) and TypeScript 7.0.2
- automated cross-platform Conventional Commits and Semantic Versioning engine
