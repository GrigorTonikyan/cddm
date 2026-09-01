# CDDM — Command-Line Interface (CLI) Reference Manual

> **High-Performance Polyglot Code Clone Detection & Refactoring Engine**  
> **Interface Pillar 1 of 4**: Terminal CLI Engine (`crates/cddm-cli`)

---

## 1. Executive Summary

The CDDM CLI provides 22 subcommands for high-speed terminal-based duplication analysis, differential CI/CD verification, automated AST refactorings, and multi-repository federation.

### Global Options

- `-v, --verbose`: Increase verbosity level (`-v` for debug, `-vv` for trace).
- `-q, --quiet`: Suppress all non-error console output.
- `--log-level <LEVEL>`: Set explicit log level (`trace`, `debug`, `info`, `warn`, `error`, `off`).
- `--log-file <PATH>`: Write structured application logs to a file.
- `-h, --help`: Print command help and options.
- `-V, --version`: Print version information.

---

## 2. Exhaustive CLI Subcommands Directory

<!-- AUTOGEN:CLI:START -->

| Command                             | Usage                                         | Description                                                                       | Key Options                                                                                                                                                |
| :---------------------------------- | :-------------------------------------------- | :-------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`cddm scan`**                     | `cddm scan [OPTIONS] [DIRECTORY]`             | Scan target directory for code duplication, clone pairs, and DRY health score     | `--min-tokens`, `--format`, `--fail-threshold`, `--languages`, `--ignore`, `--git-blame`, `--cross-language`, `--rules`, `--enforce-policies`, `--threads` |
| **`cddm dead-code`** (alias `dead`) | `cddm dead-code [OPTIONS] [DIRECTORY]`        | Detect unreferenced functions, unreachable code blocks, and dead duplicate clones | `--min-tokens`, `--format`, `--coverage`, `--static-only`, `--languages`                                                                                   |
| **`cddm diff`**                     | `cddm diff [OPTIONS] <BASE_REF> [TARGET_REF]` | Differential clone scan comparing working tree against Git base revisions         | `--matrix`, `--cross-language`, `--fail-threshold`, `--git-blame`, `--rules`                                                                               |
| **`cddm semantic`**                 | `cddm semantic [OPTIONS] [DIRECTORY]`         | Analyze cross-language semantic clones and dense neural algorithmic equivalences  | `--threshold`, `--neural`, `--neural-threshold`, `--min-tokens`, `--threads`                                                                               |
| **`cddm refactor`**                 | `cddm refactor [OPTIONS]`                     | Synthesize deduplication refactoring patches, AST rewrites, and AI prompts        | `--pair`, `--cluster`, `--ast`, `--output`, `--prompt`, `--verify`, `--test-cmd`, `--apply-branch`                                                         |
| **`cddm extract`**                  | `cddm extract [OPTIONS]`                      | Extract duplicate clone clusters into standalone shared packages or crates        | `--cluster`, `--pkg-name`, `--pkg-type`, `--target-dir`, `--dry-run`                                                                                       |
| **`cddm serve`**                    | `cddm serve [OPTIONS]`                        | Launch the embedded React 19 Studio WebUI dashboard in browser                    | `--port`, `--open`                                                                                                                                         |
| **`cddm watch`**                    | `cddm watch [OPTIONS] [DIRECTORY]`            | Continuous file watcher with real-time incremental rescanning on save             | `--min-tokens`, `--debounce-ms`, `--serve`, `--open`, `--fail-threshold`                                                                                   |
| **`cddm lsp`**                      | `cddm lsp [OPTIONS] [DIRECTORY]`              | Start Language Server Protocol (LSP 3.17) daemon for real-time IDE diagnostics    | `--min-tokens`                                                                                                                                             |
| **`cddm trend`**                    | `cddm trend [OPTIONS] [DIRECTORY]`            | Analyze historical duplication trajectories and DRY score across Git commits      | `--max-samples`, `--min-tokens`, `--format`                                                                                                                |
| **`cddm hook`**                     | `cddm hook <install                           | uninstall                                                                         | status> [OPTIONS]`                                                                                                                                         | Manage automated Git pre-commit and pre-push duplication gate enforcement hooks | `--type`, `--fail-threshold`, `--min-tokens`                                     |
| **`cddm ignore`**                   | `cddm ignore <init                            | check> [OPTIONS]`                                                                 | Manage .cddmignore suppression rules and inspect file/line suppression status                                                                              | `--force`, `--line`, `--ignore-tests`, `--ignore-mocks`, `--ignore-generated`   |
| **`cddm rules`**                    | `cddm rules <init                             | check> [OPTIONS]`                                                                 | Manage architectural boundary policies and zero-duplication zones (.cddmrules.toml)                                                                        | `--rules`, `--enforce-policies`, `--format`, `--force`                          |
| **`cddm init`**                     | `cddm init <gitea                             | github                                                                            | gitlab                                                                                                                                                     | azure> [OPTIONS]`                                                               | Generate turnkey CI/CD workflows for Gitea Actions, GitHub, GitLab, and Azure    | `--fail-threshold`, `--min-tokens`, `--output`, `--write` |
| **`cddm comment`**                  | `cddm comment [OPTIONS] [DIRECTORY]`          | Generate formatted Markdown DRY health tables for Pull / Merge Request comments   | `--platform`, `--fail-threshold`, `--min-tokens`, `--output`                                                                                               |
| **`cddm heal`**                     | `cddm heal [OPTIONS]`                         | Autonomous AI Code Surgeon refactoring with closed-loop test repair loop          | `--cluster`, `--pair`, `--provider`, `--model`, `--api-key`, `--verify`, `--test-cmd`, `--branch`, `--max-iterations`                                      |
| **`cddm cache`**                    | `cddm cache <export                           | import> [OPTIONS]`                                                                | Manage persistent fingerprint cache and export/import portable .cddmpack bundles                                                                           | `--cache-dir`, `--output`, `--pack-file`, `--target-dir`                        |
| **`cddm monorepo`**                 | `cddm monorepo [OPTIONS] [DIRECTORY]`         | Discover and scan multi-package monorepos for cross-package duplicates            | `--min-tokens`                                                                                                                                             |
| **`cddm tui`**                      | `cddm tui [OPTIONS] [DIRECTORY]`              | Launch the interactive 12-tab Terminal UI (TUI) Studio dashboard                  | `--watch`, `--fail-threshold`, `--min-tokens`, `--languages`, `--ignore`                                                                                   |
| **`cddm overlap`**                  | `cddm overlap [OPTIONS] [DIRECTORY]`          | Detect reimplemented standard library and ecosystem package algorithms            | `--threshold`, `--format`                                                                                                                                  |
| **`cddm hub`**                      | `cddm hub <init                               | scan                                                                              | extract> [OPTIONS]`                                                                                                                                        | Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)    | `--config`, `--targets`, `--cluster`, `--pkg-name`, `--pkg-type`, `--target-dir` |
| **`cddm coverage`**                 | `cddm coverage [OPTIONS]`                     | Correlate runtime execution coverage reports with duplicate code clones           | `--report`, `--dead-code-only`, `--min-hits`, `--risk-threshold`, `--format`                                                                               |

<!-- AUTOGEN:CLI:END -->

---

## 3. Key Command Recipes

### Fast Workspace Duplication Scan

```bash
cddm scan ./src --min-tokens 40 --format console
```

### CI/CD Quality Gate Enforcement

```bash
cddm diff main --fail-threshold 3.0 --format sarif --output results.sarif
```

### AI Autonomous Refactoring Loop

```bash
cddm heal --cluster 1 --provider gemini --model gemini-2.5-pro --verify --test-cmd "cargo test"
```

### Turnkey Gitea Actions Workflow Generation

```bash
cddm init gitea --fail-threshold 5.0 --write
```
