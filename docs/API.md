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
  "version": "0.6.0"
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

### `GET /api/snippet`

Retrieves source lines for a given file path and line span with configurable surrounding context and syntax language detection.

**Query Parameters**:

| Parameter | Type     | Required | Default | Description                                |
| :-------- | :------- | :------- | :------ | :----------------------------------------- |
| `file`    | `string` | Yes      | None    | Relative or absolute file path             |
| `start`   | `number` | Yes      | None    | 1-based start line of duplicate fragment   |
| `end`     | `number` | Yes      | None    | 1-based end line of duplicate fragment     |
| `context` | `number` | No       | `3`     | Context lines before and after (max: `20`) |

**Response** (`200 OK`):

```json
{
  "file": "src/auth/login.rs",
  "start_line": 10,
  "end_line": 12,
  "context_start_line": 7,
  "context_end_line": 15,
  "lines": [
    { "line_number": 7, "content": "use anyhow::Result;", "is_target": false },
    { "line_number": 10, "content": "pub fn login() {", "is_target": true }
  ],
  "total_lines": 120,
  "language": "Rust"
}
```

---

### `POST /api/refactor`

Synthesizes on-demand deduplication recommendations, parameter variance analysis, and unified `.patch` format diffs.

**Request Body** (`application/json`):

```json
{
  "file_a": "src/a.rs",
  "start_line_a": 10,
  "end_line_a": 25,
  "file_b": "src/b.rs",
  "start_line_b": 15,
  "end_line_b": 30
}
```

**Response** (`200 OK`):

```json
{
  "suggested_function_name": "extracted_shared_helper",
  "strategy": "extract_function",
  "common_body_lines": ["let x = 1;", "let y = 2;"],
  "parameter_differences": [],
  "target_module_hint": "Shared utility module or common crate",
  "unified_patch": "--- a/src/a.rs\n+++ b/src/a.rs\n@@ -10,2 +10,1 @@\n-let x = 1;\n+    extracted_shared_helper();",
  "lines_saved": 8
}
```

---

## 2. CLI Reference (`cddm`)

### Command: `cddm scan [DIRECTORY]`

Executes terminal clone detection with configurable reporters.

| Flag               | Short | Type                             | Default   | Description                                    |
| :----------------- | :---- | :------------------------------- | :-------- | :--------------------------------------------- |
| `--min-tokens`     | `-m`  | `usize`                          | `50`      | Minimum token clone threshold                  |
| `--format`         | `-f`  | `console\|json\|markdown\|sarif` | `console` | Output reporter format                         |
| `--fail-threshold` |       | `f64`                            | None      | Exit code 1 if duplication % exceeds threshold |
| `--languages`      | `-l`  | `String[]`                       | `[]`      | Filter scan by language names                  |
| `--ignore`         | `-i`  | `String[]`                       | `[]`      | Additional ignore glob patterns                |
| `--git-blame`      |       | `bool`                           | `false`   | Enable `gix` git author annotations            |
| `--cache-dir`      |       | `PathBuf`                        | None      | Custom path for persistent redb cache database |
| `--no-cache`       |       | `bool`                           | `false`   | Bypass persistent disk cache                   |
| `--clear-cache`    |       | `bool`                           | `false`   | Clear existing cache database before scanning  |

### Command: `cddm diff <BASE_REF> [TARGET_REF]`

Executes differential duplication scanning comparing current changes against a Git base revision.

| Flag               | Short | Type                             | Default   | Description                                    |
| :----------------- | :---- | :------------------------------- | :-------- | :--------------------------------------------- |
| `--directory`      | `-d`  | `PathBuf`                        | `"."`     | Target Git repository directory path           |
| `--min-tokens`     | `-m`  | `usize`                          | `50`      | Minimum token clone threshold                  |
| `--format`         | `-f`  | `console\|json\|markdown\|sarif` | `console` | Output report format                           |
| `--fail-threshold` |       | `f64`                            | None      | Exit code 1 if new clones exceed threshold     |
| `--languages`      | `-l`  | `String[]`                       | `[]`      | Filter scan by language names                  |
| `--ignore`         | `-i`  | `String[]`                       | `[]`      | Additional ignore glob patterns                |
| `--git-blame`      |       | `bool`                           | `false`   | Enable `gix` git author annotations            |
| `--cache-dir`      |       | `PathBuf`                        | None      | Custom path for persistent redb cache database |
| `--no-cache`       |       | `bool`                           | `false`   | Bypass persistent disk cache                   |

### Command: `cddm refactor [OPTIONS]`

Generates automated refactoring patch recommendations for duplicate code clones.

| Flag           | Short | Type      | Default | Description                                 |
| :------------- | :---- | :-------- | :------ | :------------------------------------------ |
| `--pair`       | `-p`  | `usize`   | `1`     | Target clone pair 1-based index to refactor |
| `--output`     | `-o`  | `PathBuf` | None    | Write generated unified patch to file       |
| `--directory`  |       | `PathBuf` | `"."`   | Target codebase directory path              |
| `--min-tokens` | `-m`  | `usize`   | `50`    | Minimum token clone threshold               |

### Command: `cddm serve`

Launches the Axum server delivering the interactive WebUI.

| Flag     | Short | Type   | Default | Description               |
| :------- | :---- | :----- | :------ | :------------------------ |
| `--port` | `-p`  | `u16`  | `3000`  | HTTP server port          |
| `--open` | `-o`  | `bool` | `false` | Auto-open default browser |

---

## 3. MCP Protocol (`cddm-mcp`)

The MCP server communicates over stdio using JSON-RPC 2.0 and supports Tools, Resources, and Prompts.

### Tools

#### `scan_codebase`

Runs a polyglot code duplication scan and returns structured JSON metrics and clone pair details for AI context.

| Parameter          | Type      | Required | Default | Description                              |
| :----------------- | :-------- | :------- | :------ | :--------------------------------------- |
| `directory`        | `string`  | Yes      | `"."`   | Target directory path to analyze         |
| `min_tokens`       | `number`  | No       | `50`    | Minimum token clone threshold            |
| `enable_git_blame` | `boolean` | No       | `false` | Annotate duplicate lines with git author |

#### `cddm_diff_scan`

Runs differential code clone detection comparing working changes against a Git base revision.

| Parameter    | Type     | Required | Default  | Description                                      |
| :----------- | :------- | :------- | :------- | :----------------------------------------------- |
| `base_ref`   | `string` | Yes      | None     | Base Git revision to compare against (e.g. main) |
| `target_ref` | `string` | No       | `"HEAD"` | Target Git revision                              |
| `directory`  | `string` | No       | `"."`    | Target Git repository directory path             |
| `min_tokens` | `number` | No       | `50`     | Minimum token clone threshold                    |

#### `cddm_get_clone_pair`

Retrieves localized source snippet lines, token counts, and blame context for a clone pair.

| Parameter      | Type     | Required | Description                      |
| :------------- | :------- | :------- | :------------------------------- |
| `file_a`       | `string` | Yes      | File path of fragment A          |
| `start_line_a` | `number` | Yes      | 1-based start line of fragment A |
| `end_line_a`   | `number` | Yes      | 1-based end line of fragment A   |
| `file_b`       | `string` | Yes      | File path of fragment B          |
| `start_line_b` | `number` | Yes      | 1-based start line of fragment B |
| `end_line_b`   | `number` | Yes      | 1-based end line of fragment B   |

#### `cddm_suggest_refactor`

Performs invariant LCS analysis and produces a structural deduplication recommendation with unified `.patch` format.

| Parameter      | Type     | Required | Description                      |
| :------------- | :------- | :------- | :------------------------------- |
| `file_a`       | `string` | Yes      | File path of fragment A          |
| `start_line_a` | `number` | Yes      | 1-based start line of fragment A |
| `end_line_a`   | `number` | Yes      | 1-based end line of fragment A   |
| `file_b`       | `string` | Yes      | File path of fragment B          |
| `start_line_b` | `number` | Yes      | 1-based start line of fragment B |
| `end_line_b`   | `number` | Yes      | 1-based end line of fragment B   |

#### `cddm_export_sarif`

Executes duplication scan and returns OASIS SARIF v2.1.0 report for GitHub Code Scanning integration.

| Parameter    | Type     | Required | Default | Description                      |
| :----------- | :------- | :------- | :------ | :------------------------------- |
| `directory`  | `string` | Yes      | `"."`   | Target directory path to analyze |
| `min_tokens` | `number` | No       | `50`    | Minimum token threshold          |

### Resources

| URI                       | MIME Type          | Description                                                   |
| :------------------------ | :----------------- | :------------------------------------------------------------ |
| `cddm://workspace/health` | `application/json` | Real-time DRY Health Index, file metrics, and language stats. |
| `cddm://workspace/clones` | `application/json` | Registry of active duplicate code clones across files.        |

### Prompts

| Prompt Name           | Description                                                            |
| :-------------------- | :--------------------------------------------------------------------- |
| `audit_dry_health`    | Pre-configured prompt to audit codebase DRY health and top hotspots.   |
| `refactor_clone_pair` | Pre-configured prompt to extract duplicate fragments into shared code. |

---

## 4. Rust Library API (`cddm-core`)

```rust
use cddm_core::{
    run_scan, ScanConfig, ScanResult, generate_sarif_report, analyze_clone_refactoring,
};

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

// Generate OASIS SARIF 2.1.0 Report
let sarif_report = generate_sarif_report(&result);

// Analyze Clone Refactoring
let suggestion = analyze_clone_refactoring("src/a.rs", (10, 25), "src/b.rs", (15, 30)).unwrap();
println!("Patch:\n{}", suggestion.unified_patch);
```
