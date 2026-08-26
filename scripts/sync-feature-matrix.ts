#!/usr/bin/env bun
/**
 * Dynamic Feature Matrix & Test Inventory Synchronizer for CDDM.
 *
 * Automatically discovers all test suites across:
 * - Rust Crates (crates/)
 * - React 19 WebUI Studio (webui/src/)
 * - Repository Tooling Scripts (scripts/)
 * - Model Context Protocol 1:1 Suites (tests/mcp/)
 *
 * Generates and synchronizes the real-time test matrix into docs/FEATURE_MATRIX.md,
 * eliminating manual hardcoding and guaranteeing documentation truth.
 */

import { execSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { syncFeatureMatrixFile } from "./lib/test-matrix-generator";

const isCheckMode = process.argv.includes("--check");
const repoRoot = process.cwd();

console.log("--> Discovering all polyglot test suites dynamically...");
const { matrix, updatedContent, hasChanges } = syncFeatureMatrixFile(repoRoot);

console.log(`Discovered:
- Rust Engine: ${matrix.rustTestCount} #[test] units
- WebUI Studio: ${matrix.webuiTestCount} unit tests across ${matrix.webuiSuites.length} suites
- Tooling Scripts: ${matrix.scriptTestCount} unit tests across ${matrix.scriptSuites.length} suites
- MCP Protocol: ${matrix.mcpTestCount} unit tests across ${matrix.mcpSuites.length} suites
Total: ${matrix.rustTestCount + matrix.webuiTestCount + matrix.scriptTestCount + matrix.mcpTestCount} verified test cases
`);

if (isCheckMode) {
  if (hasChanges) {
    console.error("[FAIL] docs/FEATURE_MATRIX.md is out of sync with discovered test suites!");
    console.error("Run `bun scripts/sync-feature-matrix.ts` to synchronize automatically.");
    process.exit(1);
  } else {
    console.log("[PASS] docs/FEATURE_MATRIX.md is 100% synchronized with all test suites.");
  }
} else {
  const matrixPath = join(repoRoot, "docs/FEATURE_MATRIX.md");
  writeFileSync(matrixPath, updatedContent, "utf8");
  try {
    execSync("vp fmt docs/FEATURE_MATRIX.md", { stdio: "ignore" });
  } catch {
    // Ignore if vp is not in path
  }
  console.log(
    `[SUCCESS] Synchronized docs/FEATURE_MATRIX.md (${hasChanges ? "updated" : "already up to date"}).`,
  );
}
