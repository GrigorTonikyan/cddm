# CDDM — Model Context Protocol (MCP) Server Specification

> **AI Coding Agent Stdio Protocol Specification**  
> **Interface Pillar 3 of 4**: MCP JSON-RPC 2.0 Server (`crates/cddm-mcp`)

---

## 1. Overview & Setup

The CDDM MCP Server (`cddm-mcp`) exposes the entire clone analysis, AST refactoring, policy checking, and federation engine to AI assistants (Antigravity, Claude Desktop, Cursor) over JSON-RPC 2.0 stdio.

### Claude Desktop Configuration (`claude_desktop_config.json`)

```json
{
  "mcpServers": {
    "cddm": {
      "command": "cddm-mcp",
      "args": []
    }
  }
}
```

---

## 2. Exposed MCP Tools Directory (30 Tools)

<!-- AUTOGEN:MCP_TOOLS:START -->

| Tool Name                           | Category     | Description                                                               | Key Parameters                                                   |
| :---------------------------------- | :----------- | :------------------------------------------------------------------------ | :--------------------------------------------------------------- |
| **`scan_codebase`**                 | Core Scan    | Runs full polyglot duplicate code clone detection & DRY scoring           | `directory, min_tokens, languages, ignore_patterns`              |
| **`cddm_diff_scan`**                | Differential | Differential clone scan comparing working tree against Git revisions      | `base_ref, target_ref, directory, min_tokens`                    |
| **`cddm_get_clone_pair`**           | Inspection   | Retrieves localized source snippet lines, token counts, and Git blame     | `pair_id, file_a, file_b`                                        |
| **`cddm_suggest_refactor`**         | Refactoring  | Performs invariant analysis and synthesizes unified .patch diffs          | `pair_id, directory`                                             |
| **`cddm_get_clone_cluster`**        | Clustering   | Retrieves all occurrences and statistics for an N-way equivalence cluster | `cluster_id, directory`                                          |
| **`cddm_suggest_cluster_refactor`** | Refactoring  | Performs multi-site consensus refactoring across an N-way cluster         | `cluster_id, directory`                                          |
| **`cddm_export_sarif`**             | CI/CD        | Generates OASIS SARIF v2.1.0 report on demand for code scanning           | `directory, min_tokens`                                          |
| **`cddm_get_timeline`**             | Git History  | Samples Git history and evaluates time-series DRY Health trajectory       | `directory, max_samples, min_tokens`                             |
| **`cddm_check_suppression`**        | Suppression  | Checks if paths or lines are suppressed by .cddmignore rules              | `path, line, ignore_tests, ignore_mocks`                         |
| **`cddm_apply_cluster_refactor`**   | Refactoring  | Applies synthesized refactoring patch with optional Git branch creation   | `cluster_id, branch_name, directory`                             |
| **`cddm_generate_ai_prompt`**       | AI Workflows | Synthesizes structured Markdown prompts for LLM refactoring agents        | `pair_id, cluster_id, directory`                                 |
| **`cddm_ast_refactor`**             | AST Engine   | Synthesizes Tree-sitter AST transformations and helper signatures         | `cluster_id, fn_name, target_module`                             |
| **`cddm_verify_refactor`**          | Closed-Loop  | Executes closed-loop test suite verification on refactored branch         | `test_cmd, branch`                                               |
| **`cddm_check_policies`**           | Policies     | Evaluates architectural boundary rules and zero-duplication zones         | `directory, rules_file, enforce`                                 |
| **`cddm_heal_refactor`**            | AI Surgeon   | Autonomous AI refactoring with closed-loop iterative test healing         | `cluster_id, provider, model, test_cmd`                          |
| **`cddm_export_cache_pack`**        | Caching      | Exports persistent fingerprint cache into portable .cddmpack archive      | `cache_dir, output_path`                                         |
| **`cddm_import_cache_pack`**        | Caching      | Imports .cddmpack archive into persistent fingerprint database            | `pack_file, target_dir`                                          |
| **`cddm_scan_monorepo`**            | Monorepos    | Scans multi-package workspaces and identifies cross-package clones        | `directory, min_tokens`                                          |
| **`cddm_get_semantic_graph`**       | Semantic AST | Extracts Control Flow Graphs (CFG) and Program Dependence Graphs (PDG)    | `file_path, language`                                            |
| **`cddm_compare_semantic_graphs`**  | Semantic AST | Computes Weisfeiler-Lehman graph kernel isomorphism scores                | `graph_a, graph_b`                                               |
| **`cddm_scan_cross_language`**      | Polyglot     | Detects cross-language semantic clone pairs across language barriers      | `directory, threshold, min_tokens`                               |
| **`cddm_extract_shared_module`**    | Extraction   | Synthesizes a new standalone shared crate/package from clone clusters     | `cluster_id, pkg_name, pkg_type, target_dir`                     |
| **`cddm_detect_overlap`**           | Overlap      | Detects reimplemented standard and third-party library algorithms         | `directory, threshold`                                           |
| **`cddm_scan_hub`**                 | Federation   | Scans multi-repository Organization Federation Hub for cross-repo clones  | `config_path, targets, min_tokens`                               |
| **`cddm_extract_hub_package`**      | Federation   | Extracts cross-repository duplicate clusters into federated packages      | `cluster_id, pkg_name, pkg_type, target_dir`                     |
| **`cddm_correlate_coverage`**       | Coverage     | Correlates code clones with runtime test execution hit counts             | `coverage_report, directory, min_tokens`                         |
| **`cddm_detect_dead_clones`**       | Coverage     | Filters duplicate clones with 0 runtime hits across all sites             | `coverage_report, directory`                                     |
| **`cddm_detect_dead_code`**         | Dead Code    | Detects unreferenced functions, dead blocks, and orphan clones            | `directory, min_tokens, static_only`                             |
| **`cddm_prune_dead_clones`**        | Dead Code    | Safely prunes unreachable dead clone clusters and unreferenced code       | `directory, min_tokens, dry_run, safe_only, threshold, item_ids` |
| **`cddm_semantic_neural_scan`**     | Neural       | Dense subword embedding scan for algorithmic equivalence clones           | `directory, neural_threshold`                                    |
| **`cddm_diff_matrix`**              | Differential | Evaluates multi-branch and worktree clone drift matrix                    | `base_ref, branches, directory`                                  |

<!-- AUTOGEN:MCP_TOOLS:END -->

---

## 3. Exposed MCP Resources & URI Templates

<!-- AUTOGEN:MCP_RESOURCES:START -->

| Resource URI                             | Name                                | Description                                                                |
| :--------------------------------------- | :---------------------------------- | :------------------------------------------------------------------------- |
| `cddm://workspace/health`                | **Workspace DRY Health Score**      | Real-time DRY Health Index, file metrics, and language statistics          |
| `cddm://workspace/clones`                | **Workspace Code Clones**           | Registry of active duplicate code clones across repository files           |
| `cddm://workspace/clusters`              | **Workspace Code Clone Clusters**   | N-way equivalence classes of duplicated logic across repository files      |
| `cddm://workspace/timeline`              | **Workspace Duplication Trend**     | Historical DRY Health trajectories and commit snapshots across Git history |
| `cddm://workspace/suppressions`          | **Workspace Suppression Rules**     | Active .cddmignore suppression rules and filter directives                 |
| `cddm://workspace/policies`              | **Workspace Policy Rules**          | Active .cddmrules.toml boundary and anti-duplication policy rules          |
| `cddm://workspace/semantic-graph`        | **Workspace Semantic Graph**        | Control Flow and Program Dependence Graph metadata                         |
| `cddm://workspace/cross-language-clones` | **Workspace Cross-Language Clones** | Cross-language semantic clone pairs detected via WL graph kernels          |
| `cddm://workspace/watch-status`          | **Workspace Live Watch Status**     | Real-time status of directory watcher daemon and incremental delta metrics |
| `cddm://workspace/overlap`               | **Workspace Ecosystem Overlap**     | Reimplemented standard and community package utilities detected            |
| `cddm://workspace/hub`                   | **Organization Federation Hub**     | Multi-repository organization duplication metrics and cross-repo clusters  |
| `cddm://workspace/coverage`              | **Workspace Coverage Correlation**  | Runtime execution hit counts, dead duplicates, and hot-path risk analysis  |
| `cddm://workspace/dead-code`             | **Workspace Dead Code Inventory**   | Detected unreferenced functions, unreachable blocks, and dead clones       |
| `cddm://workspace/neural-embeddings`     | **Workspace Neural Embeddings**     | Dense subword embedding vectors and algorithmic equivalence pairs          |

<!-- AUTOGEN:MCP_RESOURCES:END -->

### Dynamic URI Templates

- `cddm://file/{path}/clones`: Filter duplicate clones involving a specific file.
- `cddm://cluster/{cluster_id}/details`: Detailed occurrences and consensus refactoring recommendation for cluster.
- `cddm://file/{path}/tokens`: Token boundaries and token spans for syntax inspection.

---

## 4. MCP Prompts

1. `audit_dry_health`: Audits codebase DRY health and prioritizes duplication hotspots.
2. `refactor_clone_pair`: Formulates extract-method prompts for duplicate code fragments.
3. `audit_cross_language`: Detects isomorphic logic across different programming languages.
