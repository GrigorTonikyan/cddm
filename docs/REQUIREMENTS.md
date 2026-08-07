# CDDM (Code De-Duplication Meister) — System Requirements Document

## 1. Overview & Vision
CDDM (Code De-Duplication Meister) is a standalone, ultra-fast, multi-threaded polyglot code clone detection, DRY health index analysis, and AST refactoring engine written in pure Rust 2024 edition with an embedded React 19 WebUI and Model Context Protocol (MCP) server.

---

## 2. Functional Requirements

### 2.1 Code Clone Detection Engine
- **FR-1.1 Polyglot Support**: Must support 12+ primary programming languages out of the box (Rust, TypeScript, JavaScript, Python, Go, Java, C, C++, C#, CSS/SCSS, HTML, JSON) with extensibility for 30+ via Tree-sitter.
- **FR-1.2 Type-1 (Exact) Clone Detection**: Detect identical code fragments after stripping formatting and comments using Winnowing $M_{61} = 2^{61}-1$ rolling hash.
- **FR-1.3 Type-2 (Renamed/Parameterized) Clone Detection**: Detect structural clones where identifiers, variable names, and literals are renamed.
- **FR-1.4 Type-3 (Near-Miss) Clone Detection**: Detect subtrees with added, removed, or modified statements using Tree-sitter concrete syntax trees.
- **FR-1.5 Type-4 (Semantic) Clone Detection**: Detect functionally equivalent logic with different syntactic structures using Blake3 Merkle subtree hashing.
- **FR-1.6 Configurable Token Threshold**: Allow users to configure minimum clone token size ($N \in [10, 500]$).
- **FR-1.7 Intra-File & Cross-File Scanning**: Support toggling self-scanning within the same file vs. cross-file clone discovery.

### 2.2 Git Blame & Author Attribution
- **FR-2.1 In-Process Git Blame**: Annotate duplicate code lines with author name and commit age using `gix` (`gitoxide`) without invoking external `git` binary.
- **FR-2.2 Non-Git Fallback**: Gracefully degrade when scanning non-version-controlled directories.

### 2.3 Incremental Scanning & Caching
- **FR-3.1 Content Hash Caching**: Store Sha256 content hashes of scanned files to avoid re-tokenizing unchanged files.
- **FR-3.2 Differential File Watcher**: Monitor OS file system events via `notify` to update fingerprint indices in $<1\text{ms}$ on active file modification.

### 2.4 Reporting & CI/CD Integration
- **FR-4.1 Console Table Reporter**: Formatted ANSI colored table output with line spans, token counts, and similarity scores.
- **FR-4.2 JSON Reporter**: Structured JSON output suitable for automated CI/CD parsing.
- **FR-4.3 Markdown Reporter**: GitHub-flavored markdown report with table formatting.
- **FR-4.4 Failure Threshold**: Exit CLI process with exit code `1` when duplication percentage exceeds `--fail-threshold <PCT>`.

### 2.5 Studio WebUI (`cddm serve`)
- **FR-5.1 Embedded Static Assets**: Bundle production React WebUI assets inside Rust binary using `rust-embed`.
- **FR-5.2 Axum HTTP Server**: Serve static files and REST API endpoints (`/api/scan`, `/api/health`) at `http://localhost:<PORT>`.
- **FR-5.3 Automatic Browser Launch**: Automatically open default browser when `--open` flag is set.
- **FR-5.4 Interactive Split Diff Viewer**: Render side-by-side clone comparison cards with line numbers and author tags.

### 2.6 Model Context Protocol (MCP) Server (`cddm-mcp`)
- **FR-6.1 Stdio JSON-RPC 2.0 Protocol**: Support MCP standard over stdio.
- **FR-6.2 Tool Exposure**: Expose `scan_codebase` tool for AI agents (Claude, Antigravity, Cursor) to execute scans programmatically.

---

## 3. Non-Functional Requirements

### 3.1 Performance & Memory
- **NFR-1.1 Execution Speed**: Process $>100,000$ lines of code per second across multiple CPU cores using `rayon`.
- **NFR-1.2 Memory Efficiency**: Streaming winnowing sliding window with bounded memory overhead.

### 3.2 Reliability & Safety
- **NFR-2.1 Zero Panics**: Complete error handling using `Result<T, E>` without unhandled panics.
- **NFR-2.2 Strict Typing**: 100% type safety across Rust (`#![forbid(unsafe_code)]` where possible) and TypeScript (`strict: true`, zero `any`).

---

## 4. Distribution Channels
- **CARGO**: `cargo install cddm`
- **NPM**: `npm install -g cddm` (native platform binary optionalDependencies)
