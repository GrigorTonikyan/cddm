import { describe, expect, it } from "bun:test";
import {
  discoverTestMatrix,
  generateScriptsAndMcpTable,
  generateWebUITable,
  syncFeatureMatrixFile,
} from "../lib/test-matrix-generator";

describe("Feature Matrix Dynamic Discovery & Generator", () => {
  it("should dynamically discover all polyglot test tiers", () => {
    const matrix = discoverTestMatrix();

    expect(matrix.rustTestCount).toBeGreaterThanOrEqual(170);
    expect(matrix.webuiSuites.length).toBe(41);
    expect(matrix.webuiTestCount).toBeGreaterThanOrEqual(160);
    expect(matrix.scriptSuites.length).toBeGreaterThanOrEqual(8);
    expect(matrix.scriptTestCount).toBeGreaterThanOrEqual(40);
    expect(matrix.mcpSuites.length).toBe(23);
    expect(matrix.mcpTestCount).toBeGreaterThanOrEqual(39);
  });

  it("should generate valid WebUI markdown table", () => {
    const matrix = discoverTestMatrix();
    const table = generateWebUITable(matrix.webuiSuites, matrix.webuiTestCount);

    expect(table).toContain("## 2. WebUI Frontend");
    expect(table).toContain("| Module | Test Suite File | Test Cases | Status |");
    expect(table).toContain("webui/src/App.test.tsx");
    expect(table).toContain("webui/src/components/ScanResults.test.tsx");
  });

  it("should generate valid Scripts and MCP markdown table", () => {
    const matrix = discoverTestMatrix();
    const table = generateScriptsAndMcpTable(
      matrix.scriptSuites,
      matrix.mcpSuites,
      matrix.scriptTestCount,
      matrix.mcpTestCount,
    );

    expect(table).toContain("## 3. Repository Scripts & MCP Protocol");
    expect(table).toContain("### Repository Tooling & Automation Suites");
    expect(table).toContain("### Model Context Protocol (MCP) 1:1 Tool Test Suites");
    expect(table).toContain("tests/mcp/discovery.test.ts");
    expect(table).toContain("tests/mcp/tools/scan-codebase.test.ts");
  });

  it("should verify that docs/FEATURE_MATRIX.md is currently in sync", () => {
    const { hasChanges } = syncFeatureMatrixFile();
    expect(hasChanges).toBe(false);
  });
});
