# CDDM — API Reference

## 1. REST API (`cddm serve`)

The embedded Axum HTTP server exposes the following endpoints when running `cddm serve --port <PORT>`:

---

### `GET /api/health`

Health check endpoint.

**Response** (`200 OK`):
```json
{
  "status": "ok",
  "service": "CDDM Studio",
  "version": "0.1.0"
}
```

---

### `POST /api/scan`

Execute a code duplication scan.

**Request Body** (`application/json`):
```json
{
  "directory": "./src",
  "min_tokens": 50,
  "languages": [],
  "ignore_patterns": ["node_modules", "target", ".git", "dist", "build", ".logs"],
  "detect_type2": true,
  "scan_self": true,
  "enable_git_blame": false
}
```

| Field | Type | Required | Default | Description |
|:------|:-----|:---------|:--------|:------------|
| `directory` | `string` | Yes | `"."` | Root directory path to scan |
| `min_tokens` | `number` | No | `50` | Minimum token count for a clone |
| `languages` | `string[]` | No | `[]` | Filter by language names (empty = all) |
| `ignore_patterns` | `string[]` | No | See above | Glob patterns to exclude |
| `detect_type2` | `boolean` | No | `true` | Enable Type-2 (renamed) clone detection |
| `scan_self` | `boolean` | No | `true` | Find intra-file duplicates |
| `enable_git_blame` | `boolean` | No | `false` | Annotate clones with git author |

**Response** (`200 OK`):
```json
{
  "scan_id": "550e8400-e29b-41d4-a716-446655440000",
  "total_files": 42,
  "total_tokens": 15230,
  "total_clones": 7,
  "duplication_percentage": 4.85,
  "dry_health_score": 92.7,
  "clone_pairs": [
    {
      "file_a": "src/auth/login.rs",
      "start_line_a": 10,
      "end_line_a": 25,
      "file_b": "src/auth/register.rs",
      "start_line_b": 15,
      "end_line_b": 30,
      "token_count": 50,
      "similarity": 1.0,
      "fragment_hash": "a1b2c3-d4e5f6",
      "clone_type": "Exact",
      "author_a": null,
      "author_b": null
    }
  ],
  "duration_ms": 54,
  "language_breakdown": [
    { "language": "Rust", "files": 30, "tokens": 12000, "clones": 5 },
    { "language": "TypeScript", "files": 12, "tokens": 3230, "clones": 2 }
  ]
}
```

**Error Response** (`500 Internal Server Error`):
```text
<error message string>
```

---

### `GET /*` (Static Assets & SPA Fallback)

All other paths serve embedded static assets from the React WebUI bundle. Unknown paths fall back to `index.html` for client-side routing.

---

## 2. CLI Reference (`cddm`)

### `cddm scan [DIRECTORY]`

| Flag | Short | Type | Default | Description |
|:-----|:------|:-----|:--------|:------------|
| `--min-tokens` | `-m` | `usize` | `50` | Minimum token clone threshold |
| `--format` | `-f` | `console\|json\|markdown` | `console` | Output format |
| `--fail-threshold` | | `f64` | None | Exit code 1 if duplication % exceeds |
| `--languages` | `-l` | `String[]` | `[]` | Filter by language |
| `--ignore` | `-i` | `String[]` | `[]` | Additional ignore patterns |
| `--git-blame` | | `bool` | `false` | Enable git author annotation |

### `cddm serve`

| Flag | Short | Type | Default | Description |
|:-----|:------|:-----|:--------|:------------|
| `--port` | `-p` | `u16` | `3000` | HTTP server port |
| `--open` | `-o` | `bool` | `true` | Auto-open browser |

---

## 3. MCP Server Protocol (`cddm-mcp`)

The MCP server communicates over stdin/stdout using JSON-RPC 2.0 (one JSON object per line).

### `initialize`

**Request**:
```json
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
```

**Response**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": { "listChanged": false },
      "resources": { "subscribe": false, "listChanged": false }
    },
    "serverInfo": {
      "name": "CDDM Code De-Duplication Meister MCP Server",
      "version": "0.1.0"
    }
  }
}
```

### `tools/list`

**Request**:
```json
{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
```

**Response**: Returns array of available tools with JSON Schema input definitions.

### `tools/call` — `scan_codebase`

**Request**:
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "scan_codebase",
    "arguments": {
      "directory": "./src",
      "min_tokens": 50,
      "enable_git_blame": false
    }
  }
}
```

**Response**: Returns MCP content array with serialized `ScanResult` JSON.

---

## 4. Rust Library API (`cddm-core`)

### Primary Entry Point

```rust
pub async fn run_scan(
    config: ScanConfig,
    progress_tx: Sender<ScanProgress>,
    cancel_flag: Arc<AtomicBool>,
) -> Result<ScanResult, String>
```

### Key Types

| Type | Module | Description |
|:-----|:-------|:------------|
| `ScanConfig` | `types` | Configuration for a scan run |
| `ScanResult` | `types` | Complete scan output with metrics |
| `ClonePair` | `types` | A matched duplicate code fragment pair |
| `CloneType` | `types` | Enum: `Exact`, `Renamed`, `NearMiss`, `Semantic` |
| `ScanProgress` | `types` | Progress event for UI updates |
| `LanguageGrammar` | `grammar` | Language syntax definition |
| `Fingerprint` | `fingerprint` | Winnowed hash with source location |
| `AstSubtreeHash` | `ast::hasher` | Blake3 Merkle tree hash of AST subtree |
| `FingerprintCache` | `cache` | Incremental SHA-256 file cache |
| `CddmWatcher` | `watcher` | Real-time filesystem change listener |

### Utility Functions

| Function | Module | Description |
|:---------|:-------|:------------|
| `tokenize(source, grammar, normalize)` | `tokenizer` | Lexical tokenization |
| `winnow(tokens, k, w)` | `fingerprint` | Winnowing hash extraction |
| `fast_mod_m61(x)` | `fingerprint` | Mersenne prime modular reduction |
| `get_grammar_for_path(path)` | `grammar` | Extension → grammar lookup |
| `get_line_author(repo, file, line)` | `blame` | Git blame line annotation |
| `parse_ast_tree(source, ext)` | `ast::parser` | Tree-sitter CST parsing |
| `compute_ast_subtree_hashes(tree, depth)` | `ast::hasher` | Recursive Merkle hashing |
