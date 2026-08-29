# CDDM — 4-Pillar Cross-Interface Feature Parity Matrix

> **Governance Standard**: This matrix defines the required feature parity across all four primary CDDM interaction interfaces: **CLI Engine**, **WebUI Studio**, **MCP Server**, and **TUI Studio**.
> **Enforcement**: Validated automatically via `vp run check:parity` (`bun scripts/check-feature-parity.ts`) in CI.

---

## 1. Executive Summary & Interface Pillars

CDDM ensures that every engineering capability is first-class and accessible in any working context:

```text
+----------------------------------------------------------------------------------------------------+
|                                    CDDM Unified Core Engine                                        |
+----------------------------------------------------------------------------------------------------+
|  1. CLI Engine        |  2. WebUI Studio       |  3. MCP Server         |  4. TUI Studio           |
|  - Headless CI/CD     |  - Visual React 19     |  - AI Agents (JSON-RPC)|  - Terminal Power-Users  |
|  - Scriptable stdout  |  - win2x-manager       |  - Antigravity/Cursor  |  - ratatui / crossterm   |
|  - Exit codes & flags |  - Monaco Split Diffs  |  - Tools & Resources   |  - Split Diff Keyboard UI|
+----------------------------------------------------------------------------------------------------+
```

---

## 2. Exhaustive Feature Parity Matrix (19 Core Capabilities)

| Capability Area                                | 1. CLI Command                 | 2. WebUI Studio (REST/SSE + UI)                                                    | 3. MCP Tool & Resource                                                   | 4. TUI Studio (`cddm tui`)                                      |
| :--------------------------------------------- | :----------------------------- | :--------------------------------------------------------------------------------- | :----------------------------------------------------------------------- | :-------------------------------------------------------------- |
| **1. Codebase Scan**                           | `cddm scan [DIR] [FLAGS]`      | `POST /api/scan`<br/>`ScanConfigPanel`, `ScanResults`                              | Tool: `scan_codebase`<br/>Res: `cddm://workspace/health`                 | **Tab 1: Overview**<br/>Key `s` to trigger rescan               |
| **2. Differential Scan**                       | `cddm diff <BASE> [TARGET]`    | `POST /api/diff`<br/>`DiffScanResultsModal`                                        | Tool: `cddm_diff_scan`<br/>Res: `cddm://workspace/diff`                  | **Tab 1: Overview** / Diff Mode                                 |
| **3. Clone Graph Clustering**                  | `cddm refactor --cluster <ID>` | `POST /api/refactor-cluster`<br/>`CloneClusterCard`                                | Tool: `cddm_get_clone_cluster`<br/>Res: `cddm://workspace/clusters`      | **Tab 2: Clone Explorer**<br/>Key `c` toggle Clusters view      |
| **4. Split Diff Visualizer**                   | `cddm scan --format console`   | `GET /api/snippet`<br/>`DiffViewer`, `ClonePairCard`                               | Tool: `cddm_get_clone_pair`<br/>(Returns snippets & blame)               | **Tab 2: Clone Explorer**<br/>Key `d` toggle Split Diff pane    |
| **5. Cross-Language Matching**                 | `cddm semantic [DIR]`          | `POST /api/semantic/scan`<br/>`SemanticGraphModal`                                 | Tool: `cddm_scan_cross_language`<br/>Res: `cross_language_clones`        | **Tab 3: Cross-Language Explorer**                              |
| **6. AST Refactoring Sandbox**                 | `cddm refactor --ast [FLAGS]`  | `POST /api/refactor/ast`<br/>`RefactorSandboxModal`                                | Tool: `cddm_ast_refactor`                                                | **Tab 4: Refactor Sandbox**<br/>Key `r` to open refactor        |
| **7. Shared Module Extraction**                | `cddm extract [FLAGS]`         | `POST /api/extract/preview`<br/>`ExtractModuleTab`                                 | Tool: `cddm_extract_shared_module`                                       | **Tab 5: Shared Module Extractor**<br/>Key `e` to open extract  |
| **8. AI Code Surgeon**                         | `cddm heal [FLAGS]`            | `POST /api/refactor/heal`<br/>`AutoHealTab`                                        | Tool: `cddm_heal_refactor`                                               | **Tab 4: Refactor Sandbox**<br/>Key `a` to trigger AI Heal      |
| **9. Policy Engine**                           | `cddm rules check/init`        | `GET/POST /api/policy/rules`<br/>`PolicyRulesModal`                                | Tool: `cddm_check_policies`<br/>Res: `cddm://workspace/policies`         | **Tab 6: Policy & Suppression**                                 |
| **10. AST Suppression**                        | `cddm ignore check/init`       | `GET/POST /api/suppression/rules`<br/>`SuppressionRulesModal`                      | Tool: `cddm_check_suppression`<br/>Res: `workspace/suppressions`         | **Tab 6: Policy & Suppression**                                 |
| **11. Git History Trends**                     | `cddm trend [DIR]`             | `GET /api/timeline`<br/>`TimelineExplorerModal`                                    | Tool: `cddm_get_timeline`<br/>Res: `cddm://workspace/timeline`           | **Tab 7: Git History Timeline**                                 |
| **12. CI/CD & Hook Manager**                   | `cddm hook install/init`       | `GET/POST /api/workflow/hooks`<br/>`HookManagerModal`                              | Resource: `cddm://workspace/hooks`                                       | **Tab 8: CI/CD & Git Hook Manager**                             |
| **13. AI Refactor Prompt**                     | `cddm refactor --prompt`       | `POST /api/refactor/ai-prompt`<br/>"Copy AI Prompt" button                         | Tool: `cddm_generate_ai_prompt`<br/>Prompt: `refactor_clone_pair`        | **Tab 4: Refactor Sandbox**<br/>Key `p` to copy AI prompt       |
| **14. Monorepo Discovery**                     | `cddm monorepo [DIR]`          | `GET /api/monorepo`<br/>`MonorepoWorkspaceModal`                                   | Tool: `cddm_scan_monorepo`<br/>Res: `cddm://workspace/monorepo`          | **Tab 1: Overview** (Monorepo view)                             |
| **15. Live Watch Sync**                        | `cddm watch [FLAGS]`           | `GET /api/events` (SSE)<br/>`LiveWatch` indicator                                  | Resource: `cddm://workspace/watch_status`                                | **Live Watch Mode**<br/>Key `w` to toggle live watcher          |
| **16. Ecosystem Library Overlap**              | `cddm overlap [FLAGS]`         | `GET /api/overlap/catalog`<br/>`POST /api/overlap/scan`<br/>`OverlapDetectorModal` | Tool: `cddm_detect_overlap`<br/>Res: `cddm://workspace/overlap`          | **Tab 9: Ecosystem Overlap**<br/>Key `9` to open overlap        |
| **17. Organization Federation Hub**            | `cddm hub [FLAGS]`             | `GET/POST /api/hub/config`<br/>`POST /api/hub/scan`<br/>`HubFederationModal`       | Tool: `cddm_scan_hub`<br/>Res: `cddm://workspace/hub`                    | **Tab 10: Organization Hub**<br/>Key `0` to open hub            |
| **18. Runtime Execution & Coverage**           | `cddm coverage [FLAGS]`        | `POST /api/coverage/correlate`<br/>`CoverageCorrelationModal`                      | Tool: `cddm_correlate_coverage`<br/>Res: `cddm://workspace/coverage`     | **Tab 11: Runtime Coverage**<br/>Key `C` / `v` to open coverage |
| **19. Neural Embeddings & Algorithmic Clones** | `cddm semantic --neural`       | `POST /api/semantic/neural`<br/>`CrossLanguageExplorerTab`                         | Tool: `cddm_semantic_neural_scan`<br/>Res: `workspace/neural_embeddings` | **Tab 3: Cross-Language Explorer** (Neural View)                |

---

## 3. Parity Enforcement Standards for Contributors & AI Agents

1. **New Engine Capabilities**: Whenever a new analysis algorithm, metric, or transformation is added to `cddm-core`, the implementing engineer or AI agent MUST submit:
   - CLI command/flag handler in `crates/cddm-cli/src/commands/`.
   - Axum REST handler in `crates/cddm-cli/src/serve/` and React view in `webui/`.
   - MCP tool/resource in `crates/cddm-mcp/src/`.
   - TUI view/keybinding in `crates/cddm-cli/src/tui/`.
2. **Automated Verification Gate**:
   - `vp run check:parity` or `bun scripts/check-feature-parity.ts` runs on every PR and during `vp run verify`.
