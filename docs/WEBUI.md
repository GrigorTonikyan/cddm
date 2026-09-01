# CDDM — Embedded React 19 Studio WebUI Manual

> **High-Performance Visual Dashboard, Diffs & Interactive Refactoring Studio**  
> **Interface Pillar 2 of 4**: WebUI Studio (`webui/` + `cddm-cli::serve`)

---

## 1. Overview & Architecture

CDDM embeds a high-performance, single-binary React 19 visual studio served natively via Axum and `rust-embed`. It features real-time SSE live updates, split Monaco diffs, hierarchical D3 treemaps, and 19 interactive analysis modals.

### Launching the Studio

```bash
# Launch server on port 3000 and open default browser
cddm serve --port 3000 --open

# Launch live watcher with embedded WebUI sync
cddm watch ./src --serve 3000 --open
```

---

## 2. Interactive Modals & Visual Surfaces (19 Modals)

<!-- AUTOGEN:WEBUI_MODALS:START -->

| Modal / View                   | Trigger / Shortcut           | Description                                                                   |
| :----------------------------- | :--------------------------- | :---------------------------------------------------------------------------- |
| **`DiffScanResultsModal`**     | `Header 'Diff Scan'`         | Side-by-side branch comparison, new clone alerts, and drift metrics           |
| **`CoverageCorrelationModal`** | `Header 'Coverage' / Key C`  | Runtime execution trace correlation, heatmaps, and hot-path risk score        |
| **`DeadCodeExplorerModal`**    | `Header 'Dead Code' / Key D` | Interactive unreferenced function and dead clone block viewer                 |
| **`HubFederationModal`**       | `Header 'Hub' / Key 0`       | Multi-repository organization federation overview and cross-repo clusters     |
| **`OverlapDetectorModal`**     | `Header 'Overlap' / Key 9`   | Reimplemented ecosystem library algorithm catalog and replacement suggestions |
| **`PolicyRulesModal`**         | `Header 'Policies' / Key 6`  | Architectural boundary rules, zero-duplication zones, and policy violations   |
| **`SuppressionRulesModal`**    | `Header 'Suppression'`       | .cddmignore rule editor, regex patterns, and live suppression testing         |
| **`RefactorSandboxModal`**     | `Cluster Card 'Refactor'`    | Interactive AST refactoring sandbox, AI Prompt generator, and AI Surgeon      |
| **`RefactorPatchModal`**       | `Clone Card 'View Patch'`    | Unified .patch diff synthesizer and multi-file consensus viewer               |
| **`TimelineExplorerModal`**    | `Header 'Timeline' / Key 7`  | Historical Git trajectory charts, commit churn, and branch drift matrix       |
| **`TreemapExplorerModal`**     | `Header 'Treemap'`           | Hierarchical D3 file-tree duplication area visualization                      |
| **`SemanticGraphModal`**       | `Header 'Semantic' / Key 3`  | Interactive CFG/PDG graph visualizer and WL kernel isomorphism viewer         |
| **`MonorepoWorkspaceModal`**   | `Header 'Monorepo'`          | Multi-package workspace package dependency graph and cross-package clones     |
| **`HookManagerModal`**         | `Header 'Hooks' / Key 8`     | Git pre-commit/pre-push hooks and turnkey CI/CD workflow generator            |
| **`ScanConfigModal`**          | `Header 'Configure Scan'`    | Real-time token thresholds, language filters, and worker thread ceilings      |
| **`ExportReportModal`**        | `Header 'Export'`            | One-click export to JSON, Markdown, SARIF 2.1.0, and HTML                     |
| **`HealthAuditModal`**         | `DRY Health Gauge Click`     | Mathematical score breakdown, penalty factors, and modularity ratings         |
| **`LanguageAnalyticsModal`**   | `Language Bar Click`         | Polyglot volume breakdown, token percentages, and duplicate lines by language |
| **`ClonePairDiffModal`**       | `Clone Pair Card Click`      | Split Monaco diff viewer with syntax highlighting and git blame annotations   |

<!-- AUTOGEN:WEBUI_MODALS:END -->

---

## 3. Axum REST & SSE API Catalog

- `GET /api/health`: Studio server readiness probe.
- `POST /api/scan`: Run full asynchronous duplication scan.
- `POST /api/diff`: Run differential branch scan.
- `POST /api/diff/matrix`: Multi-branch and worktree clone drift matrix.
- `GET /api/events`: Server-Sent Events (SSE) live incremental scan feed.
- `GET /api/snippet`: Fetch highlighted source line spans and Git blame metadata.
- `POST /api/refactor/ast`: Tree-sitter AST transformation generator.
- `POST /api/refactor/heal`: Closed-loop autonomous AI repair execution.
- `GET/POST /api/policy/rules`: Read or mutate `.cddmrules.toml`.
- `GET/POST /api/suppression/rules`: Read or mutate `.cddmignore`.
- `GET /api/timeline`: Historical Git commit trend series.
- `POST /api/coverage/correlate`: Correlate runtime LCOV/Cobertura traces.
- `POST /api/dead-code/scan`: Unreferenced function and dead clone detector.
- `GET/POST /api/hub/config` & `POST /api/hub/scan`: Organization Federation Hub.
