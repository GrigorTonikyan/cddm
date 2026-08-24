# CDDM — System Architecture

## 1. High-Level Architecture

CDDM (_Code De-Duplication Meister_) is a multi-threaded Rust workspace consisting of three primary crates, an embedded React 19 WebUI, and cross-platform npm/Cargo package wrappers:

```mermaid
graph TD
    subgraph UI ["User Interfaces & APIs"]
        CLI["cddm CLI (clap)"]
        Serve["cddm serve (Axum + React 19)"]
        MCP["cddm-mcp (stdio JSON-RPC 2.0)"]
    end

    subgraph Core ["cddm-core Library Engine"]
        IO["Zero-Copy File I/O (memmap2)"]
        Grammar["Grammar Registry (30+ Languages)"]
        Tokenizer["Tokenization Engine (Normalizer)"]
        SIMD["SIMD Mersenne M61 Engine (AVX2/NEON)"]
        Winnow["Winnowing Engine (Mersenne M61)"]
        AST["Tree-sitter AST Hasher (Blake3 Merkle)"]
        Index["Fingerprint Index (HashMap)"]
        Detector["Parallel Detector (Rayon)"]
        Blame["Git Blame Annotator (gix)"]
        Cluster["N-Way Graph Clustering (Disjoint-Set Union-Find)"]
        Refactor["Multi-Site Consensus Refactoring Synthesizer"]
        Cache["SHA-256 Incremental Cache"]
        Watcher["FileSystem Watcher (notify)"]
    end

    CLI --> Core
    Serve --> Core
    MCP --> Core

    IO --> Tokenizer
    Grammar --> Tokenizer
    Tokenizer --> SIMD
    SIMD --> Winnow
    Winnow --> Index
    Index --> Detector
    AST --> Detector
    Detector --> Blame
    Detector --> Cluster
    Cluster --> Refactor
```

---

## 2. Scan Execution Pipeline

The `run_scan()` function in `detector.rs` orchestrates the full code clone detection pipeline:

```mermaid
flowchart LR
    A["Phase 1: Discovery<br/>WalkBuilder, Globs, Ignores"] --> B["Phase 2: Tokenization<br/>Rayon par_iter(), Normalization"]
    B --> C["Phase 3: Indexing<br/>Winnowing M61 Hash Indexing"]
    C --> D["Phase 4: Merging<br/>Pairwise Clone Pair Matching"]
    D --> E["Phase 5: Scoring<br/>DRY Health Score Computation"]
    E --> F["Phase 6: Output<br/>Console ANSI / JSON / Markdown / HTML"]
```

---

## 3. DRY Health Score Formula

CDDM measures overall codebase modularity and DRY health using a continuous non-linear scoring function:

```text
Score = max(0, min(100, (100 - 1.5 * Duplication_Percentage) * (1 - 0.25 * Cross_Module_Ratio)))
```

Where:

- **Duplication_Percentage**: Duplication percentage (`(Clone Tokens / Total Tokens) * 100`).
- **Cross_Module_Ratio**: Cross-module clone ratio (`Cross-Directory Clones / Total Clones`).

---

## 4. Winnowing Algorithm Parameters

The Winnowing fingerprinting engine uses Mersenne prime M61 = 2^61 - 1 for collision-resistant rolling hashing:

- **k-gram size**: `k = max(10, floor(min_tokens / 2))`
- **Window size**: `w = k + 5`
- **Hash bases**: `b1 = 313`, `b2 = 1000003` (dual-base hashing for collision resistance)
- **Rolling update**: `h_next = ((h_curr - old_token * b^(k-1)) * b + new_token) mod M61`

---

## 5. Crate Dependency Graph

```text
cddm-core (library crate)
  ├── blake3, sha2        (hashing)
  ├── rayon                (parallel CPU execution)
  ├── gix                  (in-process git blame)
  ├── ignore               (directory traversal & .gitignore parsing)
  ├── memmap2              (zero-copy memory-mapped file I/O)
  ├── tree-sitter-*        (AST CST parsing: 9 supported languages)
  ├── notify               (filesystem event watcher)
  ├── serde, serde_json    (serialization)
  └── tokio                (async runtime)

cddm-cli (binary crate) ──depends──→ cddm-core
  ├── clap                 (CLI flag parsing)
  ├── axum, tower-http     (HTTP server & static asset serving)
  ├── rust-embed           (static asset embedding)
  ├── comfy-table          (ANSI console tables)
  └── opener               (launching browser)

cddm-mcp (binary crate) ──depends──→ cddm-core
  ├── serde_json           (JSON-RPC 2.0 serialization)
  └── tokio                (async stdio transport)
```

---

## 6. WebUI Embedding Architecture

The React 19 WebUI is compiled to static assets at build time (`vp run build` in `webui/`) and embedded directly into the compiled Rust binary:

```text
Build Time:                          Runtime:
┌────────────────┐                  ┌────────────────────────┐
│ webui/src/     │   vp run build   │ cddm-cli binary        │
│ ├── App.tsx    │ ──────────────→  │ ┌────────────────────┐ │
│ ├── store/     │ Vite Plus → dist │ │ rust-embed Assets  │ │
│ └── components/│                  │ │ ├── index.html     │ │
└────────────────┘                  │ │ └── assets/*.js    │ │
                                    │ └────────┬───────────┘ │
                                    │          │ Axum routes  │
                                    │ GET /*   → static_asset │
                                    │ GET /    → index.html   │
                                    │ POST /api/scan → run()  │
                                    │ GET /api/events → SSE   │
                                    │ POST /api/apply-patch   │
                                    │ GET /api/health → ok    │
                                    └────────────────────────┘
```

---

## 7. Distribution Architecture

CDDM supports dual distribution channels:

1. **Cargo Crates (`crates.io`)**:
   - `cddm-core`: Library crate published for Rust projects.
   - `cddm-cli`: Binary crate installed via `cargo install cddm`.
   - `cddm-mcp`: MCP stdio server binary.

2. **npm Registry (`npmjs.com`)**:
   - `cddm`: Universal npm wrapper package with platform binary shims (`bin/cddm.js`).
   - `@cddm/win32-x64`, `@cddm/linux-x64`, `@cddm/darwin-x64`, `@cddm/darwin-arm64`: Native pre-built release binaries.

3. **GitHub Actions CI/CD Workflows**:
   - `.github/workflows/ci.yml`: Matrix build testing (Ubuntu, Windows, macOS), `clippy`, `rustfmt`, and WebUI tests.
   - `.github/workflows/release.yml`: Cross-compiling standalone release binaries on GitHub tag pushes (`v*`).

---

## 8. WebUI Component Architecture (`Atomic UI` & `win2x-manager`)

The WebUI frontend is split into two clean architectural layers:

1. **Shared Atomic UI Library (`webui/src/components/ui/`)**:
   - **Atoms**: `Portal`, `Backdrop`, `Badge`, `IconButton`.
   - **Molecules**: `CollapsibleCard`, `CodeBlock`.
   - **Design Tokens**: Parameterized `--cddm-ui-*` CSS custom properties with zero magic literals.

2. **Universal Window Manager (`webui/src/components/ui/win2x-manager/`)**:
   - **Pure Engine**: Zero application-specific UI, framework-agnostic mathematical geometry engine (`geometry-engine.ts`), and W3C hardware pointer driver (`pointer-driver.ts`).
   - **Components**: `TitleBar`, `WindowControls`, `ResizeHandle`, `ResizeHandleGroup`, `Win2xWindow`.
   - **Design Tokens**: Parameterized `--win2x-*` custom properties with modern nested CSS scoping.
   - **Compositor Pipeline**: 120fps hardware-accelerated movement via `transform: translate3d(x, y, 0)`, dynamic blur decoupling on `[data-moving="true"]`, and CSS containment (`contain: layout paint`).
