#!/usr/bin/env bun
/**
 * Cross-platform Interface Feature Parity Policy Validator for CDDM.
 * Enforces:
 * 1. Complete feature parity across the 4 interaction pillars: CLI, WebUI Studio, MCP Server, and TUI Studio.
 * 2. Registration and integrity of all core capabilities in docs/FEATURE_PARITY.md.
 * 3. Existence of code handlers across crates/cddm-cli (CLI & TUI), webui/serve (WebUI), and crates/cddm-mcp (MCP).
 * 4. Concrete React component existence and active navigation linkage in webui/src/.
 * 5. Zero emojis across all diagnostic output.
 */

import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { reportViolationsAndExit } from "./lib/step-runner";

export interface FeatureParityCheck {
  id: string;
  name: string;
  cliCommandFile: string;
  mcpToolPattern: RegExp;
  axumRoutePattern: RegExp;
  tuiViewFile: string;
  webuiComponentFile: string;
  webuiNavPattern: RegExp;
}

const RAW_PARITY_MATRIX = [
  "scan|Codebase Scan|commands/scan.rs|scan_codebase|/api/scan|overview.rs|ScanResults.tsx|ScanResults",
  "diff|Differential Scan|commands/diff.rs|cddm_diff_scan|/api/diff|overview.rs|DiffScanResultsModal.tsx|DiffScanResultsModal",
  "cluster|Clone Graph Clustering|commands/refactor.rs|cddm_get_clone_cluster|/api/refactor-cluster|clones.rs|CloneClusterCard.tsx|CloneClusterCard",
  "diff_viewer|Split Diff Visualizer|formatters/scan.rs|cddm_get_clone_pair|/api/snippet|clones.rs|DiffViewer.tsx|DiffViewer",
  "semantic|Cross-Language Matching|commands/semantic.rs|cddm_scan_cross_language|/api/semantic|semantic.rs|SemanticGraphModal.tsx|SemanticGraphModal",
  "refactor|AST Refactoring Sandbox|commands/refactor.rs|cddm_ast_refactor|/api/refactor/ast|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "extract|Shared Module Extraction|commands/extract.rs|cddm_extract_shared_module|/api/extract|extract.rs|sandbox/ExtractModuleTab.tsx|ExtractModuleTab",
  "heal|AI Code Surgeon|commands/heal.rs|cddm_heal_refactor|/api/refactor/heal|refactor.rs|sandbox/AutoHealTab.tsx|AutoHealTab",
  "policy|Policy Engine|commands/rules.rs|cddm_check_policies|/api/policy|policy.rs|PolicyRulesModal.tsx|PolicyRulesModal",
  "suppression|AST Suppression|commands/ignore.rs|cddm_check_suppression|/api/suppression|policy.rs|SuppressionRulesModal.tsx|SuppressionRulesModal",
  "timeline|Git History Trends|commands/trend.rs|cddm_get_timeline|/api/timeline|timeline.rs|TimelineExplorerModal.tsx|TimelineExplorerModal",
  "sarif|SARIF Diagnostic Export|commands/scan.rs|cddm_export_sarif|/api/scan|overview.rs|ExportReportModal.tsx|ExportReportModal",
  "ai_prompt|AI Prompt Synthesis|commands/refactor.rs|cddm_generate_ai_prompt|/api/refactor/ai-prompt|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "monorepo|Monorepo Discovery|commands/monorepo.rs|cddm_scan_monorepo|/api/monorepo|overview.rs|MonorepoWorkspaceModal.tsx|MonorepoWorkspaceModal",
  "overlap|Ecosystem Library Overlap|commands/overlap.rs|cddm_detect_overlap|/api/overlap|overlap.rs|OverlapDetectorModal.tsx|OverlapDetectorModal",
  "hub_scan|Organization Federation Hub|commands/hub.rs|cddm_scan_hub|/api/hub|hub.rs|HubFederationModal.tsx|HubFederationModal",
  "hub_sync|Hub Federation Sync|commands/hub.rs|cddm_sync_hub|/api/hub/sync|hub.rs|HubFederationModal.tsx|HubFederationModal",
  "hub_extract|Hub Package Extraction|commands/hub.rs|cddm_extract_hub_package|/api/hub/extract|hub.rs|HubFederationModal.tsx|HubFederationModal",
  "coverage|Runtime Execution & Coverage|commands/coverage.rs|cddm_correlate_coverage|/api/coverage|coverage.rs|CoverageCorrelationModal.tsx|CoverageCorrelationModal",
  "dead_code|Polyglot Dead Code Detection|commands/dead_code.rs|cddm_detect_dead_code|/api/dead-code|dead_code.rs|DeadCodeExplorerModal.tsx|DeadCodeExplorerModal|DeadCodeStudioView",
  "dead_clones|Dead Clone Detection|commands/dead_code.rs|cddm_detect_dead_clones|/api/dead-code|dead_code.rs|DeadCodeExplorerModal.tsx|DeadCodeExplorerModal",
  "prune|Dead Clone Automated Pruning|commands/prune.rs|cddm_prune_dead_clones|/api/dead-code/prune|dead_code.rs|DeadCodeExplorerModal.tsx|DeadCodeExplorerModal",
  "reachability|Symbol Call Reachability|commands/dead_code.rs|cddm_trace_reachability|/api/dead-code/reachability|dead_code.rs|DeadCodeExplorerModal.tsx|DeadCodeExplorerModal",
  "neural|Neural Embeddings & Algorithmic Clones|commands/semantic.rs|cddm_semantic_neural_scan|/api/semantic/neural|semantic.rs|semantic/CrossLanguageExplorerTab.tsx|CrossLanguageExplorerTab",
  "matrix|Multi-Branch Drift Matrix|commands/diff.rs|cddm_diff_matrix|/api/diff/matrix|timeline.rs|TimelineExplorerModal.tsx|TimelineExplorerModal",
  "suggest_refactor|Refactoring Synthesis|commands/refactor.rs|cddm_suggest_refactor|/api/refactor|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "suggest_cluster|Cluster Refactoring Synthesis|commands/refactor.rs|cddm_suggest_cluster_refactor|/api/refactor-cluster|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "apply_cluster|Cluster Refactoring Application|commands/refactor.rs|cddm_apply_cluster_refactor|/api/refactor/apply|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "verify_refactor|Closed-Loop Refactor Verification|commands/refactor.rs|cddm_verify_refactor|/api/refactor/verify|refactor.rs|RefactorSandboxModal.tsx|RefactorSandboxModal",
  "cache_export|Offline Cache Export|commands/cache_pack.rs|cddm_export_cache_pack|/api/cache/export|overview.rs|ScanConfigPanel.tsx|ScanConfigPanel",
  "cache_import|Offline Cache Import|commands/cache_pack.rs|cddm_import_cache_pack|/api/cache/import|overview.rs|ScanConfigPanel.tsx|ScanConfigPanel",
  "semantic_graph|AST Semantic Graph Export|commands/semantic.rs|cddm_get_semantic_graph|/api/semantic-graph|semantic.rs|SemanticGraphModal.tsx|SemanticGraphModal",
  "compare_semantic_graphs|Semantic Graph Comparison|commands/semantic.rs|cddm_compare_semantic_graphs|/api/semantic-graph|semantic.rs|SemanticGraphModal.tsx|SemanticGraphModal",
] as const;

export const MANDATORY_PARITY_FEATURES: FeatureParityCheck[] = RAW_PARITY_MATRIX.map((entry) => {
  const parts = entry.split("|") as string[];
  const [
    id = "",
    name = "",
    cliRel = "",
    mcpPat = "",
    routePat = "",
    tuiRel = "",
    compRel = "",
    navPat = "",
  ] = parts;
  return {
    id,
    name,
    cliCommandFile: `crates/cddm-cli/src/${cliRel}`,
    mcpToolPattern: new RegExp(mcpPat),
    axumRoutePattern: new RegExp(routePat.replace(/\//g, "\\/")),
    tuiViewFile: `crates/cddm-cli/src/tui/views/${tuiRel}`,
    webuiComponentFile: compRel,
    webuiNavPattern: new RegExp(navPat),
  };
});

export interface ParityViolation {
  featureId: string;
  featureName: string;
  missingPillar: "CLI" | "WebUI" | "MCP" | "TUI" | "DOC";
  detail: string;
}

function readAllDirText(dirPath: string): string {
  if (!existsSync(dirPath)) return "";
  let result = "";
  try {
    const entries = readdirSync(dirPath);
    for (const entry of entries) {
      const full = join(dirPath, entry);
      const stat = statSync(full);
      if (stat.isDirectory()) {
        result += readAllDirText(full);
      } else if (
        stat.isFile() &&
        (entry.endsWith(".rs") || entry.endsWith(".ts") || entry.endsWith(".tsx"))
      ) {
        result += readFileSync(full, "utf-8");
      }
    }
  } catch {
    // Ignore errors
  }
  return result;
}

export function validateFeatureParity(workspaceRoot: string = process.cwd()): ParityViolation[] {
  const violations: ParityViolation[] = [];

  // 1. Verify docs/FEATURE_PARITY.md exists and contains all feature IDs
  const docPath = join(workspaceRoot, "docs/FEATURE_PARITY.md");
  if (!existsSync(docPath)) {
    violations.push({
      featureId: "all",
      featureName: "All Features",
      missingPillar: "DOC",
      detail: "Missing docs/FEATURE_PARITY.md documentation file",
    });
    return violations;
  }

  const docContent = readFileSync(docPath, "utf-8");

  // Verify that all registered MCP tools under tests/mcp/tools have corresponding entries
  const mcpTestDir = join(workspaceRoot, "tests/mcp/tools");
  if (existsSync(mcpTestDir)) {
    const testFiles = readdirSync(mcpTestDir).filter((f) => f.endsWith(".test.ts"));
    if (RAW_PARITY_MATRIX.length < testFiles.length) {
      violations.push({
        featureId: "mcp-coverage",
        featureName: "MCP Toolset Parity",
        missingPillar: "MCP",
        detail: `Expected parity matrix to cover at least ${testFiles.length} MCP tools, but only found ${RAW_PARITY_MATRIX.length}`,
      });
    }
  }

  // Read MCP codebase content across all tools and resources
  const mcpContent = readAllDirText(join(workspaceRoot, "crates/cddm-mcp/src"));

  // Read Axum serve router and handlers content
  const serveContent = readAllDirText(join(workspaceRoot, "crates/cddm-cli/src/serve"));

  // Read primary WebUI navigation entry files
  const primaryNavFiles = [
    join(workspaceRoot, "webui/src/App.tsx"),
    join(workspaceRoot, "webui/src/components/lazy-modals.ts"),
    join(workspaceRoot, "webui/src/components/ScanResults.tsx"),
    join(workspaceRoot, "webui/src/components/RefactorSandboxModal.tsx"),
    join(workspaceRoot, "webui/src/components/SemanticGraphModal.tsx"),
    join(workspaceRoot, "webui/src/components/ClonePairCard.tsx"),
  ];

  const navContent = primaryNavFiles
    .filter((p) => existsSync(p))
    .map((p) => readFileSync(p, "utf-8"))
    .join("\n");

  for (const feature of MANDATORY_PARITY_FEATURES) {
    // 1. Check Documentation
    if (!docContent.includes(feature.name)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "DOC",
        detail: `Feature '${feature.name}' not documented in docs/FEATURE_PARITY.md`,
      });
    }

    // 2. Check CLI
    const cliFilePath = join(workspaceRoot, feature.cliCommandFile);
    if (!existsSync(cliFilePath)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "CLI",
        detail: `Missing CLI command implementation: ${feature.cliCommandFile}`,
      });
    }

    // 3. Check MCP
    if (mcpContent && !feature.mcpToolPattern.test(mcpContent)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "MCP",
        detail: `Missing MCP tool/resource pattern: ${feature.mcpToolPattern}`,
      });
    }

    // 4. Check WebUI (Axum Route, React Component, and Navigation Linkage)
    if (serveContent && !feature.axumRoutePattern.test(serveContent)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "WebUI",
        detail: `Missing Axum REST route pattern: ${feature.axumRoutePattern}`,
      });
    }

    const componentPath = join(workspaceRoot, "webui/src/components", feature.webuiComponentFile);
    if (!existsSync(componentPath)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "WebUI",
        detail: `Missing frontend React component file: webui/src/components/${feature.webuiComponentFile}`,
      });
    }

    if (navContent && !feature.webuiNavPattern.test(navContent)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "WebUI",
        detail: `Component not wired into navigation (expected pattern: ${feature.webuiNavPattern})`,
      });
    }

    // 5. Check TUI View
    const tuiFilePath = join(workspaceRoot, feature.tuiViewFile);
    if (!existsSync(tuiFilePath)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "TUI",
        detail: `Missing TUI view implementation: ${feature.tuiViewFile}`,
      });
    }
  }

  return violations;
}

async function main() {
  console.log(
    "\x1b[36m--> Validating 4-Pillar Cross-Interface Feature Parity (CLI, WebUI, MCP, TUI)...\x1b[0m",
  );

  const violations = validateFeatureParity();

  reportViolationsAndExit(
    "Feature Parity Violation(s)",
    violations,
    (v) =>
      `  \x1b[31m[FAIL] [${v.missingPillar}] ${v.featureName} (${v.featureId}):\x1b[0m ${v.detail}`,
    `All ${MANDATORY_PARITY_FEATURES.length} core capabilities adhere to 4-Pillar Feature Parity!`,
    "All CDDM capabilities must be strictly supported across all 4 interface pillars!",
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal parity validation error:", err);
    process.exit(1);
  });
}
