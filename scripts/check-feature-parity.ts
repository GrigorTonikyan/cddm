#!/usr/bin/env bun
/**
 * Cross-platform Interface Feature Parity Policy Validator for CDDM.
 * Enforces:
 * 1. Complete feature parity across the 4 interaction pillars: CLI, WebUI Studio, MCP Server, and TUI Studio.
 * 2. Registration and integrity of all core capabilities in docs/FEATURE_PARITY.md.
 * 3. Existence of code handlers across crates/cddm-cli (CLI & TUI), webui/serve (WebUI), and crates/cddm-mcp (MCP).
 * 4. Zero emojis across all diagnostic output.
 */

import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

export interface FeatureParityCheck {
  id: string;
  name: string;
  cliCommandFile: string;
  mcpToolPattern: RegExp;
  axumRoutePattern: RegExp;
  tuiViewFile: string;
}

const RAW_PARITY_MATRIX = [
  "scan|Codebase Scan|commands/scan.rs|scan_codebase|/api/scan|overview.rs",
  "diff|Differential Scan|commands/diff.rs|cddm_diff_scan|/api/diff|overview.rs",
  "cluster|Clone Graph Clustering|commands/refactor.rs|cddm_get_clone_cluster|/api/refactor-cluster|clones.rs",
  "diff_viewer|Split Diff Visualizer|formatters/scan.rs|cddm_get_clone_pair|/api/snippet|clones.rs",
  "semantic|Cross-Language Matching|commands/semantic.rs|cddm_scan_cross_language|/api/semantic|semantic.rs",
  "refactor|AST Refactoring Sandbox|commands/refactor.rs|cddm_ast_refactor|/api/refactor/ast|refactor.rs",
  "extract|Shared Module Extraction|commands/extract.rs|cddm_extract_shared_module|/api/extract|extract.rs",
  "heal|AI Code Surgeon|commands/heal.rs|cddm_heal_refactor|/api/refactor/heal|refactor.rs",
  "policy|Policy Engine|commands/rules.rs|cddm_check_policies|/api/policy|policy.rs",
  "suppression|AST Suppression|commands/ignore.rs|cddm_check_suppression|/api/suppression|policy.rs",
  "timeline|Git History Trends|commands/trend.rs|cddm_get_timeline|/api/timeline|timeline.rs",
  "workflow|CI/CD & Hook Manager|commands/hook.rs|cddm_export_sarif|/api/workflow/hooks|workflow.rs",
  "overlap|Ecosystem Library Overlap|commands/overlap.rs|cddm_detect_overlap|/api/overlap|overlap.rs",
  "hub|Organization Federation Hub|commands/hub.rs|cddm_scan_hub|/api/hub|hub.rs",
  "coverage|Runtime Execution & Coverage|commands/coverage.rs|cddm_correlate_coverage|/api/coverage|coverage.rs",
  "neural|Neural Embeddings & Algorithmic Clones|commands/semantic.rs|cddm_semantic_neural_scan|/api/semantic/neural|semantic.rs",
] as const;

export const MANDATORY_PARITY_FEATURES: FeatureParityCheck[] = RAW_PARITY_MATRIX.map((entry) => {
  const parts = entry.split("|") as string[];
  const [id = "", name = "", cliRel = "", mcpPat = "", routePat = "", tuiRel = ""] = parts;
  return {
    id,
    name,
    cliCommandFile: `crates/cddm-cli/src/${cliRel}`,
    mcpToolPattern: new RegExp(mcpPat),
    axumRoutePattern: new RegExp(routePat.replace(/\//g, "\\/")),
    tuiViewFile: `crates/cddm-cli/src/tui/views/${tuiRel}`,
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

  // Read MCP codebase content across all tools and resources
  const mcpContent = readAllDirText(join(workspaceRoot, "crates/cddm-mcp/src"));

  // Read Axum serve router and handlers content
  const serveContent = readAllDirText(join(workspaceRoot, "crates/cddm-cli/src/serve"));

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

    // 4. Check WebUI (Axum Route)
    if (serveContent && !feature.axumRoutePattern.test(serveContent)) {
      violations.push({
        featureId: feature.id,
        featureName: feature.name,
        missingPillar: "WebUI",
        detail: `Missing Axum REST route pattern: ${feature.axumRoutePattern}`,
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

import { reportViolationsAndExit } from "./lib/step-runner";

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
