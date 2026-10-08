/**
 * MCP and WebUI Metadata Catalogs for CDDM Interface Documentation.
 */

import { existsSync, readdirSync, statSync } from "node:fs";
import { basename, join, resolve } from "node:path";

export interface McpToolDef {
  name: string;
  category: string;
  summary: string;
  keyParams: string;
}

export interface McpResourceDef {
  uri: string;
  name: string;
  summary: string;
}

export interface WebUiModalDef {
  modal: string;
  trigger: string;
  description: string;
}

export const MCP_TOOLS_CATALOG: McpToolDef[] = [
  {
    name: "scan_codebase",
    category: "Core Scan",
    summary: "Runs full polyglot duplicate code clone detection & DRY scoring",
    keyParams: "directory, min_tokens, languages, ignore_patterns",
  },
  {
    name: "cddm_diff_scan",
    category: "Differential",
    summary: "Differential clone scan comparing working tree against Git revisions",
    keyParams: "base_ref, target_ref, directory, min_tokens",
  },
  {
    name: "cddm_get_clone_pair",
    category: "Inspection",
    summary: "Retrieves localized source snippet lines, token counts, and Git blame",
    keyParams: "pair_id, file_a, file_b",
  },
  {
    name: "cddm_suggest_refactor",
    category: "Refactoring",
    summary: "Performs invariant analysis and synthesizes unified .patch diffs",
    keyParams: "pair_id, directory",
  },
  {
    name: "cddm_get_clone_cluster",
    category: "Clustering",
    summary: "Retrieves all occurrences and statistics for an N-way equivalence cluster",
    keyParams: "cluster_id, directory",
  },
  {
    name: "cddm_suggest_cluster_refactor",
    category: "Refactoring",
    summary: "Performs multi-site consensus refactoring across an N-way cluster",
    keyParams: "cluster_id, directory",
  },
  {
    name: "cddm_export_sarif",
    category: "CI/CD",
    summary: "Generates OASIS SARIF v2.1.0 report on demand for code scanning",
    keyParams: "directory, min_tokens",
  },
  {
    name: "cddm_get_timeline",
    category: "Git History",
    summary: "Samples Git history and evaluates time-series DRY Health trajectory",
    keyParams: "directory, max_samples, min_tokens",
  },
  {
    name: "cddm_check_suppression",
    category: "Suppression",
    summary: "Checks if paths or lines are suppressed by .cddmignore rules",
    keyParams: "path, line, ignore_tests, ignore_mocks",
  },
  {
    name: "cddm_apply_cluster_refactor",
    category: "Refactoring",
    summary: "Applies synthesized refactoring patch with optional Git branch creation",
    keyParams: "cluster_id, branch_name, directory",
  },
  {
    name: "cddm_generate_ai_prompt",
    category: "AI Workflows",
    summary: "Synthesizes structured Markdown prompts for LLM refactoring agents",
    keyParams: "pair_id, cluster_id, directory",
  },
  {
    name: "cddm_ast_refactor",
    category: "AST Engine",
    summary: "Synthesizes Tree-sitter AST transformations and helper signatures",
    keyParams: "cluster_id, fn_name, target_module",
  },
  {
    name: "cddm_verify_refactor",
    category: "Closed-Loop",
    summary: "Executes closed-loop test suite verification on refactored branch",
    keyParams: "test_cmd, branch",
  },
  {
    name: "cddm_check_policies",
    category: "Policies",
    summary: "Evaluates architectural boundary rules and zero-duplication zones",
    keyParams: "directory, rules_file, enforce",
  },
  {
    name: "cddm_heal_refactor",
    category: "AI Surgeon",
    summary: "Autonomous AI refactoring with closed-loop iterative test healing",
    keyParams: "cluster_id, provider, model, test_cmd",
  },
  {
    name: "cddm_export_cache_pack",
    category: "Caching",
    summary: "Exports persistent fingerprint cache into portable .cddmpack archive",
    keyParams: "cache_dir, output_path",
  },
  {
    name: "cddm_import_cache_pack",
    category: "Caching",
    summary: "Imports .cddmpack archive into persistent fingerprint database",
    keyParams: "pack_file, target_dir",
  },
  {
    name: "cddm_scan_monorepo",
    category: "Monorepos",
    summary: "Scans multi-package workspaces and identifies cross-package clones",
    keyParams: "directory, min_tokens",
  },
  {
    name: "cddm_get_semantic_graph",
    category: "Semantic AST",
    summary: "Extracts Control Flow Graphs (CFG) and Program Dependence Graphs (PDG)",
    keyParams: "file_path, language",
  },
  {
    name: "cddm_compare_semantic_graphs",
    category: "Semantic AST",
    summary: "Computes Weisfeiler-Lehman graph kernel isomorphism scores",
    keyParams: "graph_a, graph_b",
  },
  {
    name: "cddm_scan_cross_language",
    category: "Polyglot",
    summary: "Detects cross-language semantic clone pairs across language barriers",
    keyParams: "directory, threshold, min_tokens",
  },
  {
    name: "cddm_extract_shared_module",
    category: "Extraction",
    summary: "Synthesizes a new standalone shared crate/package from clone clusters",
    keyParams: "cluster_id, pkg_name, pkg_type, target_dir",
  },
  {
    name: "cddm_detect_overlap",
    category: "Overlap",
    summary: "Detects reimplemented standard and third-party library algorithms",
    keyParams: "directory, threshold",
  },
  {
    name: "cddm_scan_hub",
    category: "Federation",
    summary: "Scans multi-repository Organization Federation Hub for cross-repo clones",
    keyParams: "config_path, targets, min_tokens",
  },
  {
    name: "cddm_extract_hub_package",
    category: "Federation",
    summary: "Extracts cross-repository duplicate clusters into federated packages",
    keyParams: "cluster_id, pkg_name, pkg_type, target_dir",
  },
  {
    name: "cddm_sync_hub",
    category: "Federation",
    summary: "Synchronizes blind fingerprints and distributed cache with remote peer",
    keyParams: "remote_endpoint, salt, min_tokens, fail_threshold",
  },
  {
    name: "cddm_correlate_coverage",
    category: "Coverage",
    summary: "Correlates code clones with runtime test execution hit counts",
    keyParams: "coverage_report, directory, min_tokens",
  },
  {
    name: "cddm_detect_dead_clones",
    category: "Coverage",
    summary: "Filters duplicate clones with 0 runtime hits across all sites",
    keyParams: "coverage_report, directory",
  },
  {
    name: "cddm_detect_dead_code",
    category: "Dead Code",
    summary: "Detects unreferenced functions, dead blocks, and orphan clones",
    keyParams: "directory, min_tokens, static_only",
  },
  {
    name: "cddm_prune_dead_clones",
    category: "Dead Code",
    summary: "Safely prunes unreachable dead clone clusters and unreferenced code",
    keyParams: "directory, min_tokens, dry_run, safe_only, threshold, item_ids",
  },
  {
    name: "cddm_trace_reachability",
    category: "Dead Code",
    summary: "Computes cross-package call-graph reachability for polyglot monorepos",
    keyParams: "directory, min_tokens",
  },
  {
    name: "cddm_semantic_neural_scan",

    category: "Neural",
    summary: "Dense subword embedding scan for algorithmic equivalence clones",
    keyParams: "directory, neural_threshold",
  },
  {
    name: "cddm_diff_matrix",
    category: "Differential",
    summary: "Evaluates multi-branch and worktree clone drift matrix",
    keyParams: "base_ref, branches, directory",
  },
];

export const MCP_RESOURCES_CATALOG: McpResourceDef[] = [
  {
    uri: "cddm://workspace/health",
    name: "Workspace DRY Health Score",
    summary: "Real-time DRY Health Index, file metrics, and language statistics",
  },
  {
    uri: "cddm://workspace/clones",
    name: "Workspace Code Clones",
    summary: "Registry of active duplicate code clones across repository files",
  },
  {
    uri: "cddm://workspace/clusters",
    name: "Workspace Code Clone Clusters",
    summary: "N-way equivalence classes of duplicated logic across repository files",
  },
  {
    uri: "cddm://workspace/timeline",
    name: "Workspace Duplication Trend",
    summary: "Historical DRY Health trajectories and commit snapshots across Git history",
  },
  {
    uri: "cddm://workspace/suppressions",
    name: "Workspace Suppression Rules",
    summary: "Active .cddmignore suppression rules and filter directives",
  },
  {
    uri: "cddm://workspace/policies",
    name: "Workspace Policy Rules",
    summary: "Active .cddmrules.toml boundary and anti-duplication policy rules",
  },
  {
    uri: "cddm://workspace/semantic-graph",
    name: "Workspace Semantic Graph",
    summary: "Control Flow and Program Dependence Graph metadata",
  },
  {
    uri: "cddm://workspace/cross-language-clones",
    name: "Workspace Cross-Language Clones",
    summary: "Cross-language semantic clone pairs detected via WL graph kernels",
  },
  {
    uri: "cddm://workspace/watch-status",
    name: "Workspace Live Watch Status",
    summary: "Real-time status of directory watcher daemon and incremental delta metrics",
  },
  {
    uri: "cddm://workspace/overlap",
    name: "Workspace Ecosystem Overlap",
    summary: "Reimplemented standard and community package utilities detected",
  },
  {
    uri: "cddm://workspace/hub",
    name: "Organization Federation Hub",
    summary: "Multi-repository organization duplication metrics and cross-repo clusters",
  },
  {
    uri: "cddm://workspace/coverage",
    name: "Workspace Coverage Correlation",
    summary: "Runtime execution hit counts, dead duplicates, and hot-path risk analysis",
  },
  {
    uri: "cddm://workspace/dead-code",
    name: "Workspace Dead Code Inventory",
    summary: "Detected unreferenced functions, unreachable blocks, and dead clones",
  },
  {
    uri: "cddm://workspace/neural-embeddings",
    name: "Workspace Neural Embeddings",
    summary: "Dense subword embedding vectors and algorithmic equivalence pairs",
  },
];

export interface WebUiModalMeta {
  trigger: string;
  description: string;
  relPath?: string;
}

export const WEBUI_MODAL_METADATA_REGISTRY: Record<string, WebUiModalMeta> = {
  ClonePairDiffModal: {
    trigger: "Clone Pair Card Click",
    description: "Split Monaco diff viewer with syntax highlighting and git blame annotations",
  },
  CodeEditorModal: {
    trigger: "Header 'Code Editor' / Clone Card 'Studio Editor'",
    description:
      "Integrated Monaco code editor and side-by-side split diff viewer with workspace file editing",
  },
  CoverageCorrelationModal: {
    trigger: "Header 'Coverage' / Key C",
    description: "Runtime execution trace correlation, heatmaps, and hot-path risk score",
  },
  DeadCodeExplorerModal: {
    trigger: "Header 'Dead Code' / Key D",
    description: "Interactive unreferenced function and dead clone block viewer",
  },
  DiffScanResultsModal: {
    trigger: "Header 'Diff Scan'",
    description: "Side-by-side branch comparison, new clone alerts, and drift metrics",
  },
  ExportReportModal: {
    trigger: "Header 'Export'",
    description: "One-click export to JSON, Markdown, SARIF 2.1.0, and HTML",
  },
  HealthAuditModal: {
    trigger: "DRY Health Gauge Click",
    description: "Mathematical score breakdown, penalty factors, and modularity ratings",
  },
  HookManagerModal: {
    trigger: "Header 'Hooks' / Key 8",
    description: "Git pre-commit/pre-push hooks and turnkey CI/CD workflow generator",
  },
  HubFederationModal: {
    trigger: "Header 'Hub' / Key 0",
    description: "Multi-repository organization federation overview and cross-repo clusters",
  },
  LanguageAnalyticsModal: {
    trigger: "Language Bar Click",
    description: "Polyglot volume breakdown, token percentages, and duplicate lines by language",
  },
  LiveEventInspectorModal: {
    trigger: "Live Watch HUD Click",
    description: "Real-time daemon status, file change events, and incremental rescan logs",
    relPath: "watch/LiveEventInspectorModal.tsx",
  },
  McpAppsPreviewModal: {
    trigger: "Header 'MCP Apps'",
    description: "Interactive HTML preview widgets, diff split-views, and cluster treemaps",
  },
  MonorepoWorkspaceModal: {
    trigger: "Header 'Monorepo'",
    description: "Multi-package workspace package dependency graph and cross-package clones",
  },
  OverlapDetectorModal: {
    trigger: "Header 'Overlap' / Key 9",
    description: "Reimplemented ecosystem library algorithm catalog and replacement suggestions",
  },
  PolicyRulesModal: {
    trigger: "Header 'Policies' / Key 6",
    description: "Architectural boundary rules, zero-duplication zones, and policy violations",
  },
  RefactorPatchModal: {
    trigger: "Clone Card 'View Patch'",
    description: "Unified .patch diff synthesizer and multi-file consensus viewer",
  },
  RefactorSandboxModal: {
    trigger: "Cluster Card 'Refactor'",
    description: "Interactive AST refactoring sandbox, AI Prompt generator, and AI Surgeon",
  },
  ScanConfigModal: {
    trigger: "Header 'Configure Scan'",
    description: "Real-time token thresholds, language filters, and worker thread ceilings",
  },
  SemanticGraphModal: {
    trigger: "Header 'Semantic' / Key 3",
    description: "Interactive CFG/PDG graph visualizer and WL kernel isomorphism viewer",
  },
  SuppressionRulesModal: {
    trigger: "Header 'Suppression'",
    description: ".cddmignore rule editor, regex patterns, and live suppression testing",
  },
  TimelineExplorerModal: {
    trigger: "Header 'Timeline' / Key 7",
    description: "Historical Git trajectory charts, commit churn, and branch drift matrix",
  },
  TreemapExplorerModal: {
    trigger: "Header 'Treemap'",
    description: "Hierarchical D3 file-tree duplication area visualization",
  },
};

function getWorkspaceRoot(explicitRoot?: string): string {
  if (explicitRoot && existsSync(join(explicitRoot, "webui/src/components"))) {
    return explicitRoot;
  }
  if (existsSync(join(process.cwd(), "webui/src/components"))) {
    return process.cwd();
  }
  return resolve(import.meta.dir, "../..");
}

export function discoverWebUiModals(workspaceRoot?: string): WebUiModalDef[] {
  const root = getWorkspaceRoot(workspaceRoot);
  const componentsDir = join(root, "webui/src/components");
  if (!existsSync(componentsDir)) {
    throw new Error(`WebUI components directory not found: ${componentsDir}`);
  }

  const discoveredFiles: string[] = [];
  const scan = (dir: string) => {
    for (const e of readdirSync(dir)) {
      const full = join(dir, e);
      if (statSync(full).isDirectory()) {
        if (e !== "ui") scan(full);
      } else if (
        e.endsWith("Modal.tsx") &&
        !e.endsWith(".test.tsx") &&
        e !== "ModalFooter.tsx" &&
        e !== "ModalTabs.tsx"
      ) {
        discoveredFiles.push(full);
      }
    }
  };
  scan(componentsDir);

  const modals: WebUiModalDef[] = discoveredFiles.map((file) => {
    const modalName = basename(file).replace(/\.tsx$/, "");
    const meta = WEBUI_MODAL_METADATA_REGISTRY[modalName];
    if (!meta) {
      throw new Error(
        `Discovered modal '${modalName}' at '${file}' lacks metadata in WEBUI_MODAL_METADATA_REGISTRY.`,
      );
    }
    return { modal: modalName, trigger: meta.trigger, description: meta.description };
  });

  for (const [name, meta] of Object.entries(WEBUI_MODAL_METADATA_REGISTRY)) {
    const expected = meta.relPath
      ? join(componentsDir, meta.relPath)
      : join(componentsDir, `${name}.tsx`);
    if (!existsSync(expected)) {
      throw new Error(
        `Phantom modal in registry: '${name}' does not exist on disk at '${expected}'.`,
      );
    }
  }

  return modals.sort((a, b) => a.modal.localeCompare(b.modal));
}

export const WEBUI_MODALS_CATALOG: WebUiModalDef[] = discoverWebUiModals();

export function generateMcpToolsMarkdownTable(): string {
  const header =
    "| Tool Name | Category | Description | Key Parameters |\n| :--- | :--- | :--- | :--- |\n";
  const rows = MCP_TOOLS_CATALOG.map(
    (t) => `| **\`${t.name}\`** | ${t.category} | ${t.summary} | \`${t.keyParams}\` |`,
  ).join("\n");
  return `${header}${rows}\n`;
}

export function generateMcpResourcesMarkdownTable(): string {
  const header = "| Resource URI | Name | Description |\n| :--- | :--- | :--- |\n";
  const rows = MCP_RESOURCES_CATALOG.map(
    (r) => `| \`${r.uri}\` | **${r.name}** | ${r.summary} |`,
  ).join("\n");
  return `${header}${rows}\n`;
}

export function generateWebUiModalsMarkdownTable(workspaceRoot?: string): string {
  const header = "| Modal / View | Trigger / Shortcut | Description |\n| :--- | :--- | :--- |\n";
  const rows = discoverWebUiModals(workspaceRoot)
    .map((m) => `| **\`${m.modal}\`** | \`${m.trigger}\` | ${m.description} |`)
    .join("\n");
  return `${header}${rows}\n`;
}
