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
  "total_clusters": 3,
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
  "clone_clusters": [
    {
      "id": 1,
      "occurrences": [
        {
          "file": "src/auth/login.rs",
          "start_line": 10,
          "end_line": 25,
          "author": "Grigor Tonikyan"
        },
        {
          "file": "src/auth/register.rs",
          "start_line": 15,
          "end_line": 30,
          "author": "Grigor Tonikyan"
        },
        {
          "file": "src/auth/reset.rs",
          "start_line": 5,
          "end_line": 20,
          "author": "Grigor Tonikyan"
        }
      ],
      "token_count": 50,
      "similarity": 1.0,
      "clone_type": "Exact",
      "fragment_hash": "a1b2c3-d4e5f6"
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

### `POST /api/refactor-cluster`

Synthesizes a multi-site consensus deduplication patch across $N \ge 2$ duplicate occurrences within an equivalence cluster.

**Request Body** (`application/json`):

```json
{
  "occurrences": [
    { "file": "src/auth/login.rs", "start_line": 10, "end_line": 25 },
    { "file": "src/auth/register.rs", "start_line": 15, "end_line": 30 },
    { "file": "src/auth/reset.rs", "start_line": 5, "end_line": 20 }
  ]
}
```

**Response** (`200 OK`):

```json
{
  "suggested_function_name": "extracted_shared_helper",
  "strategy": "extract_multi_site_function",
  "common_body_lines": ["let token = authenticate(&req)?;"],
  "sites": [
    { "file": "src/auth/login.rs", "start_line": 10, "end_line": 25, "parameter_differences": [] },
    {
      "file": "src/auth/register.rs",
      "start_line": 15,
      "end_line": 30,
      "parameter_differences": []
    },
    { "file": "src/auth/reset.rs", "start_line": 5, "end_line": 20, "parameter_differences": [] }
  ],
  "target_module_hint": "Shared utility module or common crate",
  "unified_patch": "--- a/src/auth/login.rs\n+++ b/src/auth/login.rs\n...\n--- a/src/auth/register.rs\n+++ b/src/auth/register.rs\n...",
  "lines_saved": 24
}
```

---

### `POST /api/apply-patch`

Applies a synthesized unified diff refactoring patch directly and atomically to the workspace filesystem.

**Request Body** (`application/json`):

```json
{
  "patch": "--- a/src/auth/login.rs\n+++ b/src/auth/login.rs\n@@ -10,15 +10,1 @@\n...",
  "dry_run": false
}
```

| Field     | Type      | Required | Default | Description                                              |
| :-------- | :-------- | :------- | :------ | :------------------------------------------------------- |
| `patch`   | `string`  | Yes      | N/A     | Standard unified diff patch string                       |
| `dry_run` | `boolean` | No       | `false` | When true, validates patch application without modifying |

**Response** (`200 OK`):

```json
{
  "success": true,
  "modified_files": ["src/auth/login.rs", "src/auth/register.rs"],
  "hunks_applied": 2,
  "message": "Successfully applied 2 patch hunks across 2 file(s)."
}
```

---

### `GET /api/suppression/rules`

Retrieves active `.cddmignore` suppression rules and global category filters.

**Response** (`200 OK`):

```json
{
  "rules": [
    {
      "pattern": "**/tests/**",
      "rule_type": "ignore",
      "min_tokens": null,
      "ignored_clone_types": [],
      "line_number": 3
    }
  ],
  "ignore_tests": false,
  "ignore_mocks": false,
  "ignore_generated": true,
  "raw_cddmignore": "**/tests/**\n"
}
```

---

### `POST /api/suppression/rules`

Updates active suppression rules, category toggles, and persists `.cddmignore` to disk.

**Request Body** (`application/json`):

```json
{
  "rules": [],
  "ignore_tests": true,
  "ignore_mocks": false,
  "ignore_generated": true,
  "raw_cddmignore": "**/tests/**\n[threshold] legacy/** min_tokens=100\n"
}
```

---

### `POST /api/refactor/sandbox`

Simulates customized invariant extraction and parameterization with live diff preview across occurrence sites.

**Request Body** (`application/json`):

```json
{
  "cluster_id": 1,
  "occurrences": [
    { "file": "src/auth/login.rs", "start_line": 10, "end_line": 25 },
    { "file": "src/auth/register.rs", "start_line": 15, "end_line": 30 }
  ],
  "custom_function_name": "validate_credentials",
  "target_module_path": "src/auth/common.rs"
}
```

**Response** (`200 OK`):

```json
{
  "cluster_id": 1,
  "function_name": "validate_credentials",
  "target_module_path": "src/auth/common.rs",
  "unified_patch": "--- a/src/auth/login.rs\n+++ b/src/auth/login.rs\n...",
  "total_lines_saved": 24,
  "sites_count": 2,
  "affected_files": ["src/auth/login.rs", "src/auth/register.rs"]
}
```

---

### `POST /api/refactor/apply-branch`

Applies a synthesized refactoring patch directly to the workspace with optional transactional Git branch creation.

**Request Body** (`application/json`):

```json
{
  "patch": "--- a/src/auth/login.rs\n+++ b/src/auth/login.rs\n...",
  "branch_name": "cddm/refactor-cluster-1",
  "create_branch": true
}
```

**Response** (`200 OK`):

```json
{
  "success": true,
  "branch_created": "cddm/refactor-cluster-1",
  "modified_files": ["src/auth/login.rs", "src/auth/register.rs"],
  "hunks_applied": 2,
  "message": "Refactoring patch successfully applied to branch 'cddm/refactor-cluster-1'."
}
```

---

### `POST /api/refactor/ai-prompt`

Synthesizes a structured AI refactoring prompt specification tailored for AI coding assistants.

**Request Body** (`application/json`):

```json
{
  "clone_type": "Renamed",
  "similarity": 0.95,
  "token_count": 120,
  "lines_saved_est": 25,
  "function_name": "validate_credentials",
  "target_module": "src/auth/common.rs",
  "occurrences": [
    {
      "path": "src/auth/login.rs",
      "span": { "line_start": 10, "line_end": 25, "byte_offset": 0 },
      "snippet": "..."
    }
  ],
  "invariant_body": "...",
  "parameters": ["username: &str", "password: &str"],
  "custom_instructions": "Ensure zero unsafe code."
}
```

**Response** (`200 OK`):

```json
{
  "prompt": "# Code De-Duplication & Refactoring Specification\n..."
}
```

---

### `POST /api/refactor/ast`

Synthesizes a Tree-sitter AST-native refactoring transformation with inferred parameter types, module import generation, and CST node substitutions.

**Request Body** (`application/json`):

```json
{
  "cluster_id": 1,
  "occurrences": [
    { "file": "src/auth/login.rs", "start_line": 10, "end_line": 25 },
    { "file": "src/auth/register.rs", "start_line": 15, "end_line": 30 }
  ],
  "custom_function_name": "validate_credentials",
  "target_module_path": "src/auth/common.rs",
  "custom_parameter_names": ["username", "password"]
}
```

**Response** (`200 OK`):

```json
{
  "cluster_id": 1,
  "function_name": "validate_credentials",
  "target_module_path": "src/auth/common.rs",
  "helper_signature": "pub fn validate_credentials(username: &str, password: &str)",
  "helper_function_code": "pub fn validate_credentials(username: &str, password: &str) {\n    ...\n}",
  "inferred_parameters": [
    {
      "name": "username",
      "inferred_type": "&str",
      "original_values": ["user_a", "user_b"]
    }
  ],
  "rewritten_files": [
    {
      "file_path": "src/auth/login.rs",
      "original_line_count": 120,
      "new_line_count": 105,
      "call_sites_count": 1,
      "rewritten_source": "...",
      "imports_added": ["use crate::auth::common::validate_credentials;"]
    }
  ],
  "unified_patch": "--- a/src/auth/login.rs\n+++ b/src/auth/login.rs\n...",
  "total_lines_saved": 24,
  "syntax_valid": true
}
```

---

### `POST /api/refactor/verify`

Runs closed-loop test suite verification against the workspace or a dedicated refactored branch to ensure zero behavioral regressions.

**Request Body** (`application/json`):

```json
{
  "directory": ".",
  "test_command": "cargo test --workspace",
  "branch_name": "cddm/refactor-cluster-1",
  "timeout_seconds": 60
}
```

**Response** (`200 OK`):

```json
{
  "success": true,
  "exit_code": 0,
  "duration_ms": 1420,
  "command_executed": "cargo test --workspace",
  "stdout_snippet": "test result: ok. 178 passed; 0 failed",
  "stderr_snippet": "",
  "message": "Test suite verification passed with exit code 0"
}
```

---

### `GET /api/events`

Subscribes to live Server-Sent Events (SSE) stream for real-time background file change notifications, progress tracking, and re-scan results.

**Event Types**:

- `scan_started`: Emitted when an incremental or manual scan begins (`{ "type": "scan_started", "payload": { "scan_id": "..." } }`).
- `scan_progress`: Emitted with real-time phase and file progress.
- `scan_complete`: Emitted when duplication analysis finishes, carrying updated `ScanResult`.
- `patch_applied`: Emitted when a refactoring patch is applied to the workspace.

---

## 2. CLI Reference (`cddm`)

### Command: `cddm scan [DIRECTORY]`

Executes terminal clone detection with configurable reporters.

| Flag                 | Short | Type                             | Default   | Description                                    |
| :------------------- | :---- | :------------------------------- | :-------- | :--------------------------------------------- |
| `--min-tokens`       | `-m`  | `usize`                          | `50`      | Minimum token clone threshold                  |
| `--format`           | `-f`  | `console\|json\|markdown\|sarif` | `console` | Output reporter format                         |
| `--fail-threshold`   |       | `f64`                            | None      | Exit code 1 if duplication % exceeds threshold |
| `--languages`        | `-l`  | `String[]`                       | `[]`      | Filter scan by language names                  |
| `--ignore`           | `-i`  | `String[]`                       | `[]`      | Additional ignore glob patterns                |
| `--cddmignore`       |       | `PathBuf`                        | None      | Custom path to `.cddmignore` file              |
| `--ignore-tests`     |       | `bool`                           | `false`   | Suppress test directories and files            |
| `--ignore-mocks`     |       | `bool`                           | `false`   | Suppress mock and fixture files                |
| `--ignore-generated` |       | `bool`                           | `true`    | Suppress `@generated` and `DO NOT EDIT` files  |
| `--git-blame`        |       | `bool`                           | `false`   | Enable `gix` git author annotations            |
| `--cache-dir`        |       | `PathBuf`                        | None      | Custom path for persistent redb cache database |
| `--no-cache`         |       | `bool`                           | `false`   | Bypass persistent disk cache                   |
| `--clear-cache`      |       | `bool`                           | `false`   | Clear existing cache database before scanning  |

### Command: `cddm diff <BASE_REF> [TARGET_REF]`

Executes differential duplication scanning comparing current changes against a Git base revision.

| Flag                 | Short | Type                             | Default   | Description                                    |
| :------------------- | :---- | :------------------------------- | :-------- | :--------------------------------------------- |
| `--directory`        | `-d`  | `PathBuf`                        | `"."`     | Target Git repository directory path           |
| `--min-tokens`       | `-m`  | `usize`                          | `50`      | Minimum token clone threshold                  |
| `--format`           | `-f`  | `console\|json\|markdown\|sarif` | `console` | Output report format                           |
| `--fail-threshold`   |       | `f64`                            | None      | Exit code 1 if new clones exceed threshold     |
| `--languages`        | `-l`  | `String[]`                       | `[]`      | Filter scan by language names                  |
| `--ignore`           | `-i`  | `String[]`                       | `[]`      | Additional ignore glob patterns                |
| `--cddmignore`       |       | `PathBuf`                        | None      | Custom path to `.cddmignore` file              |
| `--ignore-tests`     |       | `bool`                           | `false`   | Suppress test directories and files            |
| `--ignore-mocks`     |       | `bool`                           | `false`   | Suppress mock and fixture files                |
| `--ignore-generated` |       | `bool`                           | `true`    | Suppress `@generated` and `DO NOT EDIT` files  |
| `--git-blame`        |       | `bool`                           | `false`   | Enable `gix` git author annotations            |
| `--cache-dir`        |       | `PathBuf`                        | None      | Custom path for persistent redb cache database |
| `--no-cache`         |       | `bool`                           | `false`   | Bypass persistent disk cache                   |

### Command: `cddm ignore <SUBCOMMAND>`

Manages `.cddmignore` rules and inspects file path suppression status.

- `cddm ignore init [--force]`: Initializes a default `.cddmignore` template in the workspace root.
- `cddm ignore check <PATH> [--line <N>] [--cddmignore <FILE>] [--ignore-tests] [--ignore-mocks] [--ignore-generated]`: Checks whether a file path or specific source line is suppressed by rules or inline directives.

### Command: `cddm watch [DIRECTORY]`

Continuously watches workspace for source modifications and automatically executes real-time incremental duplication analysis with live terminal status updates.

| Flag               | Short | Type       | Default | Description                                    |
| :----------------- | :---- | :--------- | :------ | :--------------------------------------------- |
| `--min-tokens`     | `-m`  | `usize`    | `50`    | Minimum token clone threshold                  |
| `--languages`      | `-l`  | `String[]` | `[]`    | Filter scan by language names                  |
| `--ignore`         | `-i`  | `String[]` | `[]`    | Additional ignore glob patterns                |
| `--debounce-ms`    |       | `u64`      | `250`   | Debounce interval in milliseconds              |
| `--fail-threshold` |       | `f64`      | None    | Log warning if duplication % exceeds threshold |
| `--git-blame`      |       | `bool`     | `false` | Enable `gix` git author annotations            |
| `--cache-dir`      |       | `PathBuf`  | None    | Custom path for persistent redb cache database |
| `--no-cache`       |       | `bool`     | `false` | Bypass persistent disk cache                   |

### Command: `cddm refactor [OPTIONS]`

Generates automated refactoring patch recommendations for duplicate code clones via textual diffs or Tree-sitter AST transformations.

| Flag              | Short | Type      | Default | Description                                            |
| :---------------- | :---- | :-------- | :------ | :----------------------------------------------------- |
| `--pair`          | `-p`  | `usize`   | None    | Target clone pair 1-based index to refactor            |
| `--cluster`       | `-c`  | `usize`   | None    | Target clone cluster 1-based index to refactor         |
| `--ast`           |       | `bool`    | `false` | Enable Tree-sitter AST-native typed rewrite engine     |
| `--fn-name`       |       | `String`  | None    | Custom extracted function name                         |
| `--target-module` |       | `PathBuf` | None    | Custom destination file path for extracted helper      |
| `--apply-branch`  |       | `String`  | None    | Apply refactoring to a dedicated Git branch            |
| `--verify`        |       | `bool`    | `false` | Run closed-loop test suite verification after refactor |
| `--test-cmd`      |       | `String`  | None    | Custom test command for verification                   |
| `--prompt`        |       | `bool`    | `false` | Synthesize structured AI refactoring prompt spec       |
| `--output`        | `-o`  | `PathBuf` | None    | Write generated unified patch or prompt to file        |
| `--directory`     |       | `PathBuf` | `"."`   | Target codebase directory path                         |
| `--min-tokens`    | `-m`  | `usize`   | `50`    | Minimum token clone threshold                          |

### Command: `cddm comment [DIRECTORY]`

Scans the repository and outputs a formatted Markdown summary table with DRY health metrics ready for CI pull request / merge request comments.

| Flag               | Short | Type                    | Default  | Description                                    |
| :----------------- | :---- | :---------------------- | :------- | :--------------------------------------------- |
| `--fail-threshold` |       | `f64`                   | `15.0`   | Exit code 1 if duplication % exceeds threshold |
| `--platform`       |       | `github\|gitlab\|azure` | `github` | Target CI workflow platform                    |
| `--output`         | `-o`  | `PathBuf`               | None     | Write generated Markdown comment to file       |
| `--min-tokens`     | `-m`  | `usize`                 | `50`     | Minimum token clone threshold                  |

### Command: `cddm serve`

Launches the Axum server delivering the interactive WebUI.

| Flag     | Short | Type   | Default | Description               |
| :------- | :---- | :----- | :------ | :------------------------ |
| `--port` | `-p`  | `u16`  | `3001`  | HTTP server port          |
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

#### `cddm_get_clone_cluster`

Retrieves source snippet lines, token counts, and occurrences context for an N-way clone equivalence cluster.

| Parameter    | Type     | Required | Default | Description                      |
| :----------- | :------- | :------- | :------ | :------------------------------- |
| `cluster_id` | `number` | Yes      | None    | 1-based cluster ID               |
| `directory`  | `string` | No       | `"."`   | Target directory path to analyze |
| `min_tokens` | `number` | No       | `50`    | Minimum token clone threshold    |

#### `cddm_suggest_cluster_refactor`

Performs multi-site consensus invariant analysis across an equivalence cluster and generates a unified multi-file `.patch`.

| Parameter     | Type       | Required | Default | Description                                        |
| :------------ | :--------- | :------- | :------ | :------------------------------------------------- |
| `cluster_id`  | `number`   | No       | None    | Target 1-based cluster ID                          |
| `occurrences` | `object[]` | No       | None    | Explicit array of `{ file, start_line, end_line }` |
| `directory`   | `string`   | No       | `"."`   | Target directory path to analyze                   |
| `min_tokens`  | `number`   | No       | `50`    | Minimum token clone threshold                      |

#### `cddm_check_suppression`

Checks whether a specified file path or line number is suppressed by `.cddmignore` glob rules, category filters, or inline directives.

| Parameter          | Type      | Required | Default | Description                                             |
| :----------------- | :-------- | :------- | :------ | :------------------------------------------------------ |
| `path`             | `string`  | Yes      | None    | File path to check                                      |
| `line`             | `number`  | No       | None    | Optional 1-based line number to check inline directives |
| `cddmignore_path`  | `string`  | No       | None    | Path to custom `.cddmignore` file                       |
| `ignore_tests`     | `boolean` | No       | `false` | Suppress test files                                     |
| `ignore_mocks`     | `boolean` | No       | `false` | Suppress mock files                                     |
| `ignore_generated` | `boolean` | No       | `true`  | Suppress auto-generated files                           |

#### `cddm_apply_cluster_refactor`

Applies a synthesized refactoring patch directly to the workspace filesystem with optional automated Git branch creation.

| Parameter       | Type      | Required | Default | Description                                          |
| :-------------- | :-------- | :------- | :------ | :--------------------------------------------------- |
| `patch`         | `string`  | Yes      | None    | Unified diff patch to apply                          |
| `branch_name`   | `string`  | No       | None    | Git branch name to create and check out              |
| `create_branch` | `boolean` | No       | `false` | Whether to create a dedicated branch before applying |

#### `cddm_export_sarif`

Executes duplication scan and returns OASIS SARIF v2.1.0 report for GitHub Code Scanning integration.

| Parameter    | Type     | Required | Default | Description                      |
| :----------- | :------- | :------- | :------ | :------------------------------- |
| `directory`  | `string` | Yes      | `"."`   | Target directory path to analyze |
| `min_tokens` | `number` | No       | `50`    | Minimum token threshold          |

#### `cddm_get_timeline`

Samples Git repository commit history and returns historical duplication trajectory, DRY health score delta, and file churn hotspots.

| Parameter     | Type     | Required | Default | Description                                  |
| :------------ | :------- | :------- | :------ | :------------------------------------------- |
| `directory`   | `string` | No       | `"."`   | Target repository directory path             |
| `max_samples` | `number` | No       | `10`    | Maximum number of historical commits to walk |
| `min_tokens`  | `number` | No       | `50`    | Minimum token clone threshold                |

#### `cddm_generate_ai_prompt`

Synthesizes a structured AI refactoring prompt specification detailing clone locations, invariant bodies, and parameter variations for AI assistants.

| Parameter             | Type       | Required | Default | Description                                        |
| :-------------------- | :--------- | :------- | :------ | :------------------------------------------------- |
| `function_name`       | `string`   | No       | None    | Proposed helper function name                      |
| `target_module`       | `string`   | No       | None    | Proposed destination file path                     |
| `invariant_body`      | `string`   | No       | None    | Common logic extracted across occurrences          |
| `parameters`          | `string[]` | No       | `[]`    | Identified parameter names and variations          |
| `occurrences`         | `object[]` | No       | `[]`    | Explicit array of `{ file, start_line, end_line }` |
| `custom_instructions` | `string`   | No       | None    | Optional architectural constraints for the AI      |

#### `cddm_ast_refactor`

Synthesizes a Tree-sitter AST-native refactoring transformation with inferred types, import synthesis, and concrete syntax tree node substitutions.

| Parameter                | Type       | Required | Default | Description                                          |
| :----------------------- | :--------- | :------- | :------ | :--------------------------------------------------- |
| `occurrences`            | `object[]` | Yes      | None    | Array of `{ path, start_line, end_line }` to rewrite |
| `custom_function_name`   | `string`   | No       | None    | Optional extracted helper function name              |
| `target_module_path`     | `string`   | No       | None    | Optional destination module path                     |
| `custom_parameter_names` | `string[]` | No       | `[]`    | Optional customized parameter names                  |

#### `cddm_verify_refactor`

Executes closed-loop automated test suite verification against the workspace or a specified Git branch.

| Parameter         | Type     | Required | Default | Description                               |
| :---------------- | :------- | :------- | :------ | :---------------------------------------- |
| `directory`       | `string` | No       | `"."`   | Workspace directory path                  |
| `test_command`    | `string` | No       | None    | Custom test command (e.g. `cargo test`)   |
| `branch_name`     | `string` | No       | None    | Optional Git branch to verify             |
| `timeout_seconds` | `number` | No       | `60`    | Maximum test execution timeout in seconds |

### Resources

| URI                             | MIME Type          | Description                                                     |
| :------------------------------ | :----------------- | :-------------------------------------------------------------- |
| `cddm://workspace/health`       | `application/json` | Real-time DRY Health Index, file metrics, and language stats.   |
| `cddm://workspace/clones`       | `application/json` | Registry of active duplicate code clones across files.          |
| `cddm://workspace/clusters`     | `application/json` | Disjoint-set partitioned N-way clone equivalence clusters.      |
| `cddm://workspace/timeline`     | `application/json` | Historical commit snapshots, DRY trajectory, and churn metrics. |
| `cddm://workspace/suppressions` | `application/json` | Active `.cddmignore` glob patterns and category filters.        |

### Prompts

| Prompt Name           | Description                                                            |
| :-------------------- | :--------------------------------------------------------------------- |
| `audit_dry_health`    | Pre-configured prompt to audit codebase DRY health and top hotspots.   |
| `refactor_clone_pair` | Pre-configured prompt to extract duplicate fragments into shared code. |

---

## 4. Rust Library API (`cddm-core`)

```rust
use cddm_core::{
    run_scan, ScanConfig, ScanResult, generate_sarif_report,
    analyze_clone_refactoring, analyze_cluster_refactoring,
    preview_cluster_refactor, apply_cluster_refactor_branch,
    suppression::SuppressionEngine, cluster::cluster_clone_pairs,
    CloneLocation,
};

let config = ScanConfig {
    directory: "./src".to_string(),
    min_tokens: 50,
    enable_git_blame: true,
    ignore_tests: true,
    ignore_generated: true,
    ..Default::default()
};

let (tx, _rx) = tokio::sync::mpsc::channel(32);
let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

let result: ScanResult = run_scan(config, tx, cancel_flag).await.unwrap();
println!("DRY Health Score: {:.1}", result.dry_health_score);
println!("Equivalence Clusters: {}", result.total_clusters);

// AST Suppression Engine Usage
let engine = SuppressionEngine::with_options(true, true, true);
let is_ignored = engine.is_path_ignored(std::path::Path::new("tests/mock.rs"), None);
println!("Is path ignored: {}", is_ignored);

// Interactive Refactor Sandbox Simulation
let sandbox_res = preview_cluster_refactor(
    &[
        CloneLocation { file: "src/a.rs".to_string(), start_line: 10, end_line: 25, author: None },
        CloneLocation { file: "src/b.rs".to_string(), start_line: 15, end_line: 30, author: None },
    ],
    Some("custom_helper"),
    Some("src/common.rs"),
    None,
).unwrap();
println!("Lines saved: {}", sandbox_res.total_lines_saved);

// Transactional Git Branch Patch Application
let branch_apply = apply_cluster_refactor_branch(
    std::path::Path::new("."),
    &sandbox_res.unified_patch,
    Some("cddm/refactor-custom"),
    true,
).unwrap();
println!("Branch created: {:?}", branch_apply.branch_created);
```

---

## 5. Language Server Protocol (`cddm-lsp`)

The `cddm-lsp` engine implements the standard Language Server Protocol (LSP 3.17) over JSON-RPC 2.0 Stdio:

### Methods

| LSP Method                        | Direction        | Description                                                                                        |
| :-------------------------------- | :--------------- | :------------------------------------------------------------------------------------------------- |
| `initialize`                      | Client -> Server | Initializes LSP session and advertises server capabilities                                         |
| `textDocument/didOpen`            | Client -> Server | Notifies server of newly opened document and updates diagnostics                                   |
| `textDocument/didChange`          | Client -> Server | Ingests document buffer modifications and triggers debounced scan                                  |
| `textDocument/didSave`            | Client -> Server | Triggers immediate workspace scan and updates diagnostics                                          |
| `textDocument/publishDiagnostics` | Server -> Client | Publishes clone warnings with `CDDM-Exact`, `CDDM-Renamed`, `CDDM-NearMiss`, `CDDM-Semantic` codes |
| `textDocument/codeAction`         | Client -> Server | Synthesizes `WorkspaceEdit` with `TextEdit`s to extract duplicate functions                        |
| `textDocument/hover`              | Client -> Server | Returns rich Markdown card with clone metrics, similarity, and counterpart links                   |
| `textDocument/definition`         | Client -> Server | Navigates from clone site A to counterpart clone site B                                            |
| `workspace/executeCommand`        | Client -> Server | Executes custom commands (`cddm.rescanWorkspace`)                                                  |
