import { describe, expect, it } from "bun:test";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { callMcpStdio } from "./helpers";

function toolNameToTestFilename(toolName: string): string {
  // Normalize "cddm_foo_bar" or "scan_codebase" to kebab-case
  const normalized = toolName.startsWith("cddm_") ? toolName.replace(/^cddm_/, "") : toolName;
  return `${normalized.replace(/_/g, "-")}.test.ts`;
}

describe("MCP Dynamic Discovery & 1:1 Test Suite Mapping", () => {
  it("should perform JSON-RPC 2.0 initialize handshake", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2024-11-05",
        capabilities: {},
        clientInfo: { name: "cddm-discovery-test", version: "1.7.0" },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(1);
    expect(res.error).toBeUndefined();
    const result = res.result as { serverInfo?: { name?: string; version?: string } };
    expect(result?.serverInfo?.name).toContain("CDDM");
  });

  it("should dynamically discover all 22 MCP tools and verify 1:1 test suite presence", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 2,
      method: "tools/list",
      params: {},
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();
    const tools = (res.result as any)?.tools || [];
    expect(tools.length).toBeGreaterThanOrEqual(22);

    const toolsDir = join(import.meta.dir, "tools");
    const existingTestFiles = new Set(readdirSync(toolsDir));

    const missingTests: string[] = [];
    for (const tool of tools) {
      const expectedFilename = toolNameToTestFilename(tool.name);
      if (!existingTestFiles.has(expectedFilename)) {
        missingTests.push(
          `Tool '${tool.name}' is missing test suite: tests/mcp/tools/${expectedFilename}`,
        );
      }
    }

    if (missingTests.length > 0) {
      throw new Error(`MCP Tool Test Parity Violation:\n${missingTests.join("\n")}`);
    }
  });
});
