# CDDM — System Architecture

## 1. High-Level Architecture

CDDM is a pure Rust workspace consisting of three crates plus an embedded React 19 WebUI:

```
┌─────────────────────────────────────────────────────────┐
│                    User Interfaces                       │
│  ┌──────────┐  ┌──────────────┐  ┌───────────────────┐  │
│  │ cddm CLI │  │ cddm serve   │  │ cddm-mcp (stdio)  │  │
│  │ (clap)   │  │ (Axum+React) │  │ (JSON-RPC 2.0)    │  │
│  └────┬─────┘  └──────┬───────┘  └─────────┬─────────┘  │
│       │               │                    │             │
│       └───────────────┼────────────────────┘             │
│                       ▼                                  │
│  ┌─────────────────────────────────────────────────────┐ │
│  │                   cddm-core                         │ │
│  │  ┌──────────┐ ┌────────────┐ ┌──────────────────┐   │ │
│  │  │ grammar  │→│ tokenizer  │→│ fingerprint      │   │ │
│  │  │ (12 lang)│ │ (normalize)│ │ (Winnow M₆₁)    │   │ │
│  │  └──────────┘ └────────────┘ └────────┬─────────┘   │ │
│  │                                       ▼             │ │
│  │  ┌──────────┐ ┌────────────┐ ┌──────────────────┐   │ │
│  │  │ blame    │←│ detector   │←│ index HashMap    │   │ │
│  │  │ (gix)    │ │ (rayon par)│ │ (hash → locs)   │   │ │
│  │  └──────────┘ └────────────┘ └──────────────────┘   │ │
│  │                                                     │ │
│  │  ┌──────────┐ ┌────────────┐ ┌──────────────────┐   │ │
│  │  │ ast      │ │ cache      │ │ watcher          │   │ │
│  │  │(tree-sit)│ │ (SHA-256)  │ │ (notify crate)   │   │ │
│  │  └──────────┘ └────────────┘ └──────────────────┘   │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

---

## 2. Scan Execution Pipeline

The `run_scan()` function in `detector.rs` orchestrates the full clone detection pipeline:

```
Phase 1: Discovery         Phase 2: Tokenization       Phase 3: Indexing
┌──────────────────┐      ┌──────────────────────┐    ┌───────────────────┐
│ WalkBuilder      │      │ Rayon par_iter()      │    │ HashMap<(u64,u64),│
│ → File paths     │ ───→ │ → grammar lookup     │ ──→│   Vec<Location>>  │
│ → Extension match│      │ → tokenize()         │    │ Fingerprint index │
│ → Ignore filter  │      │ → winnow()           │    └────────┬──────────┘
└──────────────────┘      └──────────────────────┘             │
                                                               ▼
Phase 4: Merging            Phase 5: Scoring         Phase 6: Output
┌──────────────────┐      ┌──────────────────────┐    ┌───────────────────┐
│ Pairwise compare │      │ DRY Health Score      │    │ ScanResult JSON   │
│ → ClonePair emit │ ───→ │ = (100 - dup% * 1.5) │ ──→│ → Console table   │
│ → Git blame ann. │      │   * (1 - 0.25 * xmod)│    │ → JSON reporter   │
│ → scan_self flag │      │ Clamped [0.0, 100.0]  │    │ → Markdown report │
└──────────────────┘      └──────────────────────┘    └───────────────────┘
```

---

## 3. DRY Health Score Formula

$$S_{\text{DRY}} = \max\!\bigl(0,\; \min\!\bigl(100,\; (100 - 1.5 \cdot D_\%) \cdot (1 - 0.25 \cdot R_{\text{cross}})\bigr)\bigr)$$

Where:
- $D_\%$ = duplication percentage (clone tokens / total tokens × 100)
- $R_{\text{cross}}$ = cross-module clone ratio (clones across different top-level dirs / total clones)

---

## 4. Winnowing Algorithm Parameters

The Winnowing fingerprinting engine uses Mersenne prime $M_{61} = 2^{61} - 1$ for collision-resistant rolling hash:

- **k-gram size**: $k = \max(10, \lfloor \text{min\_tokens} / 2 \rfloor)$
- **Window size**: $w = k + 5$
- **Hash bases**: $b_1 = 313$, $b_2 = 1{,}000{,}003$ (dual-hash for collision resistance)
- **Rolling update**: $h' = ((h - \text{old} \cdot b^{k-1}) \cdot b + \text{new}) \bmod M_{61}$

---

## 5. Crate Dependency Graph

```
cddm-core (library)
  ├── blake3, sha2        (hashing)
  ├── rayon                (parallelism)
  ├── gix                  (git blame)
  ├── ignore               (file discovery)
  ├── tree-sitter-*        (AST parsing)
  ├── notify               (file watching)
  ├── serde, serde_json    (serialization)
  └── tokio                (async runtime)

cddm-cli (binary) ──depends──→ cddm-core
  ├── clap                 (CLI parsing)
  ├── axum, tower-http     (HTTP server)
  ├── rust-embed           (static assets)
  ├── comfy-table          (ANSI tables)
  └── opener               (browser launch)

cddm-mcp (binary) ──depends──→ cddm-core
  ├── serde_json           (JSON-RPC)
  └── tokio                (async runtime)
```

---

## 6. WebUI Embedding Architecture

The React 19 WebUI is compiled to static assets at build time and embedded into the Rust binary:

```
Build Time:                          Runtime:
┌────────────────┐                  ┌────────────────────────┐
│ webui/src/     │  bun run build   │ cddm-cli binary        │
│ ├── App.tsx    │ ──────────────→  │ ┌────────────────────┐ │
│ ├── store/     │   Vite → dist/   │ │ rust-embed Assets  │ │
│ └── components/│                  │ │ ├── index.html     │ │
└────────────────┘                  │ │ └── assets/*.js    │ │
                                    │ └────────┬───────────┘ │
                                    │          │ Axum routes  │
                                    │ GET /*  → static_asset  │
                                    │ GET /   → index.html    │
                                    │ POST /api/scan → run()  │
                                    │ GET /api/health → ok    │
                                    └────────────────────────┘
```

---

## 7. MCP Server Protocol

The `cddm-mcp` binary implements Model Context Protocol v2024-11-05 over stdin/stdout:

| Method | Direction | Description |
|:-------|:----------|:------------|
| `initialize` | Client → Server | Handshake, returns capabilities |
| `tools/list` | Client → Server | Returns `scan_codebase` tool schema |
| `tools/call` | Client → Server | Executes `scan_codebase`, returns `ScanResult` |

Tool schema for `scan_codebase`:
```json
{
  "name": "scan_codebase",
  "inputSchema": {
    "type": "object",
    "properties": {
      "directory": { "type": "string" },
      "min_tokens": { "type": "number" },
      "enable_git_blame": { "type": "boolean" }
    },
    "required": ["directory"]
  }
}
```
