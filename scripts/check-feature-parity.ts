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

export const MANDATORY_PARITY_FEATURES: FeatureParityCheck[] = [
  {
    id: "scan",
    name: "Codebase Scan",
    cliCommandFile: "crates/cddm-cli/src/commands/scan.rs",
    mcpToolPattern: /scan_codebase/,
    axumRoutePattern: /\/api\/scan/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/overview.rs",
  },
  {
    id: "diff",
    name: "Differential Scan",
    cliCommandFile: "crates/cddm-cli/src/commands/diff.rs",
    mcpToolPattern: /cddm_diff_scan/,
    axumRoutePattern: /\/api\/diff/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/overview.rs",
  },
  {
    id: "cluster",
    name: "Clone Graph Clustering",
    cliCommandFile: "crates/cddm-cli/src/commands/refactor.rs",
    mcpToolPattern: /cddm_get_clone_cluster/,
    axumRoutePattern: /\/api\/refactor-cluster/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/clones.rs",
  },
  {
    id: "diff_viewer",
    name: "Split Diff Visualizer",
    cliCommandFile: "crates/cddm-cli/src/formatters/scan.rs",
    mcpToolPattern: /cddm_get_clone_pair/,
    axumRoutePattern: /\/api\/snippet/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/clones.rs",
  },
  {
    id: "semantic",
    name: "Cross-Language Matching",
    cliCommandFile: "crates/cddm-cli/src/commands/semantic.rs",
    mcpToolPattern: /cddm_scan_cross_language/,
    axumRoutePattern: /\/api\/semantic/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/semantic.rs",
  },
  {
    id: "refactor",
    name: "AST Refactoring Sandbox",
    cliCommandFile: "crates/cddm-cli/src/commands/refactor.rs",
    mcpToolPattern: /cddm_ast_refactor/,
    axumRoutePattern: /\/api\/refactor\/ast/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/refactor.rs",
  },
  {
    id: "extract",
    name: "Shared Module Extraction",
    cliCommandFile: "crates/cddm-cli/src/commands/extract.rs",
    mcpToolPattern: /cddm_extract_shared_module/,
    axumRoutePattern: /\/api\/extract/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/extract.rs",
  },
  {
    id: "heal",
    name: "AI Code Surgeon",
    cliCommandFile: "crates/cddm-cli/src/commands/heal.rs",
    mcpToolPattern: /cddm_heal_refactor/,
    axumRoutePattern: /\/api\/refactor\/heal/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/refactor.rs",
  },
  {
    id: "policy",
    name: "Policy Engine",
    cliCommandFile: "crates/cddm-cli/src/commands/rules.rs",
    mcpToolPattern: /cddm_check_policies/,
    axumRoutePattern: /\/api\/policy/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/policy.rs",
  },
  {
    id: "suppression",
    name: "AST Suppression",
    cliCommandFile: "crates/cddm-cli/src/commands/ignore.rs",
    mcpToolPattern: /cddm_check_suppression/,
    axumRoutePattern: /\/api\/suppression/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/policy.rs",
  },
  {
    id: "timeline",
    name: "Git History Trends",
    cliCommandFile: "crates/cddm-cli/src/commands/trend.rs",
    mcpToolPattern: /cddm_get_timeline/,
    axumRoutePattern: /\/api\/timeline/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/timeline.rs",
  },
  {
    id: "workflow",
    name: "CI/CD & Hook Manager",
    cliCommandFile: "crates/cddm-cli/src/commands/hook.rs",
    mcpToolPattern: /cddm_export_sarif|cddm:\/\/workspace\/hooks/,
    axumRoutePattern: /\/api\/workflow\/hooks/,
    tuiViewFile: "crates/cddm-cli/src/tui/views/workflow.rs",
  },
];

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

async function main() {
  console.log(
    "\x1b[36m--> Validating 4-Pillar Cross-Interface Feature Parity (CLI, WebUI, MCP, TUI)...\x1b[0m",
  );

  const violations = validateFeatureParity();

  if (violations.length > 0) {
    console.error(
      `\n\x1b[31m[ERROR] Found ${violations.length} Feature Parity Violation(s):\x1b[0m\n`,
    );
    for (const v of violations) {
      console.error(
        `  \x1b[31m[FAIL] [${v.missingPillar}] ${v.featureName} (${v.featureId}):\x1b[0m ${v.detail}`,
      );
    }
    console.error(
      "\n\x1b[31mAll CDDM capabilities must be strictly supported across all 4 interface pillars!\x1b[0m\n",
    );
    process.exit(1);
  }

  console.log(
    `\x1b[32m[PASS] All ${MANDATORY_PARITY_FEATURES.length} core capabilities adhere to 4-Pillar Feature Parity!\x1b[0m\n`,
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal parity validation error:", err);
    process.exit(1);
  });
}
