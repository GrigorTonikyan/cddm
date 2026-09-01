# CDDM — Terminal UI (TUI) Studio Manual

> **High-Performance Keyboard-Driven Terminal Dashboard & Split Diffs**  
> **Interface Pillar 4 of 4**: Terminal TUI Engine (`crates/cddm-cli/src/tui/`)

---

## 1. Overview & Launching

CDDM TUI is built on `ratatui` and `crossterm`, providing a zero-overhead, responsive keyboard dashboard with 12 distinct views, live background watch rescan, and side-by-side terminal diffing.

```bash
# Launch interactive TUI in current directory
cddm tui

# Launch with live watch mode & custom token threshold
cddm tui ./src --min-tokens 40 --watch

# Launch with automated CI threshold gate
cddm tui . --fail-threshold 10.0
```

---

## 2. Exhaustive TUI Tabs Catalog (12 Tabs)

<!-- AUTOGEN:TUI_TABS:START -->

| Tab #  | Tab Title          |      Hotkey      | Description                                                                       |
| :----: | :----------------- | :--------------: | :-------------------------------------------------------------------------------- |
| **1**  | **Overview**       |     `1 or s`     | Workspace summary metrics, DRY health gauge, language breakdown & scan trigger    |
| **2**  | **Clones & Diffs** |   `2 or c / d`   | Clone pairs, N-way cluster trees, and split/unified Monaco-style diff viewer      |
| **3**  | **Semantic**       |       `3`        | Cross-language Weisfeiler-Lehman AST graph isomorphisms and neural embeddings     |
| **4**  | **Refactor**       | `4 or r / a / p` | AST-native refactoring sandbox, AI Prompt generator, and AI Code Surgeon          |
| **5**  | **Extract**        |     `5 or e`     | Standalone shared crate/package synthesizer with multi-ecosystem manifest updates |
| **6**  | **Policies**       |       `6`        | Architectural rules checker (.cddmrules.toml) and .cddmignore suppression manager |
| **7**  | **Timeline**       |       `7`        | Git commit history time-series trajectories and multi-branch clone drift matrix   |
| **8**  | **CI/CD & Hooks**  |       `8`        | Turnkey workflow generator (Gitea/GitHub/GitLab/Azure) and Git pre-commit hooks   |
| **9**  | **Overlap**        |       `9`        | Ecosystem library duplication detector for reimplemented utility functions        |
| **10** | **Hub**            |       `0`        | Multi-repository Organization Federation Hub viewer and cross-repo extractor      |
| **11** | **Coverage**       |     `C or v`     | Runtime execution trace correlation, hot-path analysis, and risk scoring          |
| **12** | **Dead Code**      |       `D`        | Unreferenced functions, unreachable code blocks, and 0-hit duplicate clones       |

<!-- AUTOGEN:TUI_TABS:END -->

---

## 3. Global Keyboard Shortcuts

| Shortcut                   | Action                                                |
| :------------------------- | :---------------------------------------------------- |
| `1` - `0`, `C`, `D`        | Directly jump to Tab 1 through 12                     |
| `Tab` / `Shift+Tab`        | Cycle forwards and backwards through tabs             |
| `j` / `k` or `Down` / `Up` | Navigate list items, clone pairs, and clusters        |
| `d`                        | Toggle Split Diff pane vs. Unified view in Clones tab |
| `c`                        | Toggle Pairwise vs. N-Way Cluster Tree mode           |
| `r`                        | Open AST Refactoring sandbox on selected clone        |
| `a`                        | Trigger Autonomous AI Code Surgeon                    |
| `p`                        | Copy formatted LLM Refactoring Prompt to clipboard    |
| `w`                        | Toggle Live Background Watcher rescan mode            |
| `s`                        | Trigger manual immediate rescanning of workspace      |
| `?`                        | Toggle popup Keyboard Help modal                      |
| `q` / `Esc` / `Ctrl+C`     | Exit TUI Studio                                       |
