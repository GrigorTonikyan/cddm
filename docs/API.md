# CDDM — API Reference

## 1. REST API (`cddm serve`)

The embedded Axum HTTP server exposes the following endpoints when running `cddm serve --port <PORT>`:

---

### `GET /api/health`

Health check endpoint for monitoring and studio readiness.

**Response** (`200 OK`):

```json
{
  "status": "ok",
  "service": "CDDM Studio",
  "version": "0.1.2"
}
```

---

### `POST /api/scan`

Execute a code duplication scan asynchronously.

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

| Field              | Type       | Required | Default   | Description                                      |
| :----------------- | :--------- | :------- | :-------- | :----------------------------------------------- |
| `directory`        | `string`   | Yes      | `"."`     | Root directory path to scan                      |
| `min_tokens`       | `number`   | No       | `50`      | Minimum token count for a clone fragment         |
| `languages`        | `string[]` | No       | `[]`      | Filter by language names (empty = all)           |
| `ignore_patterns`  | `string[]` | No       | See above | Glob patterns to exclude                         |
| `detect_type2`     | `boolean`  | No       | `true`    | Enable Type-2 (renamed) identifier normalization |
| `scan_self`        | `boolean`  | No       | `true`    | Find intra-file self-overlapping duplicates      |
| `enable_git_blame` | `boolean`  | No       | `false`   | Annotate clone pairs with `gix` git author       |

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
      "author_a": "Grigor Tonikyan (line 10, 2026-08-08)",
      "author_b": "Grigor Tonikyan (line 15, 2026-08-08)"
    }
  ],
  "duration_ms": 54,
  "language_breakdown": [
    { "language": "Rust", "files": 30, "tokens": 12000, "clones": 5 },
    { "language": "TypeScript", "files": 12, "tokens": 3230, "clones": 2 }
  ]
}
```

---

## 2. CLI Reference (`cddm`)

### Command: `cddm scan [DIRECTORY]`

Executes terminal clone detection with configurable reporters.

| Flag               | Short | Type                            | Default   | Description                                    |
| :----------------- | :---- | :------------------------------ | :-------- | :--------------------------------------------- |
| `--min-tokens`     | `-m`  | `usize`                         | `50`      | Minimum token clone threshold                  |
| `--format`         | `-f`  | `console\|json\|markdown\|html` | `console` | Output reporter format                         |
| `--fail-threshold` |       | `f64`                           | None      | Exit code 1 if duplication % exceeds threshold |
| `--languages`      | `-l`  | `String[]`                      | `[]`      | Filter scan by language names                  |
| `--ignore`         | `-i`  | `String[]`                      | `[]`      | Additional ignore glob patterns                |
| `--git-blame`      |       | `bool`                          | `false`   | Enable `gix` git author annotations            |
| `--no-self`        |       | `bool`                          | `false`   | Skip intra-file clone checking                 |
| `--output`         | `-o`  | `String`                        | None      | Save report directly to file                   |

### Command: `cddm serve`

Launches the Axum server delivering the interactive WebUI.

| Flag     | Short | Type     | Default     | Description               |
| :------- | :---- | :------- | :---------- | :------------------------ |
| `--port` | `-p`  | `u16`    | `3000`      | HTTP server port          |
| `--host` |       | `String` | `127.0.0.1` | Host binding address      |
| `--open` | `-o`  | `bool`   | `true`      | Auto-open default browser |

---

## 3. MCP Protocol (`cddm-mcp`)

The MCP server communicates over stdio using JSON-RPC 2.0.

### Tool: `scan_codebase`

Runs a polyglot code duplication scan and returns structured JSON metrics and clone pair details for AI context.

| Parameter          | Type      | Required | Default | Description                              |
| :----------------- | :-------- | :------- | :------ | :--------------------------------------- |
| `directory`        | `string`  | Yes      | `"."`   | Target directory path to analyze         |
| `min_tokens`       | `number`  | No       | `50`    | Minimum token clone threshold            |
| `enable_git_blame` | `boolean` | No       | `false` | Annotate duplicate lines with git author |

---

## 4. Rust Library API (`cddm-core`)

```rust
use cddm_core::{run_scan, ScanConfig, ScanResult};

let config = ScanConfig {
    directory: "./src".to_string(),
    min_tokens: 50,
    enable_git_blame: true,
    ..Default::default()
};

let (tx, _rx) = tokio::sync::mpsc::channel(32);
let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

let result: ScanResult = run_scan(config, tx, cancel_flag).await.unwrap();
println!("DRY Health Score: {:.1}", result.dry_health_score);
```
