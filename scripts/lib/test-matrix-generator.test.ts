import { describe, it, expect } from "bun:test";
import {
  discoverTestMatrix,
  generateWebUITable,
  generateScriptsAndMcpTable,
  TestSuiteEntry,
} from "./test-matrix-generator";

describe("test-matrix-generator utility", () => {
  it("generates WebUI markdown table correctly", () => {
    const mockSuites: TestSuiteEntry[] = [
      {
        category: "WebUI",
        name: "Duplication Treemap",
        filePath: "webui/src/components/DuplicationTreemap.test.tsx",
        testCount: 4,
        status: "PASS",
      },
    ];

    const table = generateWebUITable(mockSuites, 4);
    expect(table).toContain("Duplication Treemap");
    expect(table).toContain("webui/src/components/DuplicationTreemap.test.tsx");
    expect(table).toContain("4 tests");
  });

  it("generates scripts and MCP markdown tables accurately", () => {
    const mockScripts: TestSuiteEntry[] = [
      {
        category: "Scripts",
        name: "Clean & Reset",
        filePath: "scripts/tests/clean-reset.test.ts",
        testCount: 27,
        status: "PASS",
      },
    ];
    const mockMcp: TestSuiteEntry[] = [
      {
        category: "MCP",
        name: "scan_codebase",
        filePath: "tests/mcp/tools/scan-codebase.test.ts",
        testCount: 3,
        status: "PASS",
      },
    ];

    const table = generateScriptsAndMcpTable(mockScripts, mockMcp, 27, 3);
    expect(table).toContain("Clean & Reset");
    expect(table).toContain("scan_codebase");
    expect(table).toContain("30 tests across 2 suites");
  });

  it("discovers all test suites from the workspace without error", async () => {
    const summary = await discoverTestMatrix(process.cwd());
    expect(summary.rustTestCount).toBeGreaterThan(200);
    expect(summary.webuiSuites.length).toBeGreaterThan(50);
    expect(summary.scriptSuites.length).toBeGreaterThan(10);
    expect(summary.mcpSuites.length).toBeGreaterThan(25);
  });
});
