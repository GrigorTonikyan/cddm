import { describe, expect, it } from "bun:test";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { callMcpStdio } from "../../tests/mcp/helpers";

const exeName = process.platform === "win32" ? "cddm-mcp.exe" : "cddm-mcp";
const hasBinary =
  existsSync(join(import.meta.dir, "../../target/release", exeName)) ||
  existsSync(join(import.meta.dir, "../../target/debug", exeName));

const itIfBinary = hasBinary ? it : it.skip;

describe("MCP Server Live Multi-Tool Fidelity & Response Audit", () => {
  itIfBinary("should initialize cleanly and negotiate capabilities", async () => {
    const initRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2026-07-28",
        capabilities: {},
        clientInfo: { name: "ManualAuditClient", version: "1.0.0" },
      },
    });
    expect(initRes.error).toBeUndefined();
    expect(initRes.result).toBeDefined();
    const result = initRes.result as {
      protocolVersion: string;
      serverInfo: { name: string; version: string };
    };
    expect(result.protocolVersion).toBe("2026-07-28");
    expect(result.serverInfo.name).toContain("CDDM");
  });

  itIfBinary("should list all 33 tools with semantic category metadata", async () => {
    const toolsListRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 2,
      method: "tools/list",
      params: {},
    });
    expect(toolsListRes.error).toBeUndefined();
    const tools = (
      toolsListRes.result as { tools: Array<{ name: string; "x-cddm-category"?: string }> }
    ).tools;
    expect(tools.length).toBeGreaterThanOrEqual(33);
    for (const t of tools) {
      expect(t["x-cddm-category"]).toBeDefined();
    }
  });

  itIfBinary("should extract semantic control flow graphs and compare them", async () => {
    const compareRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 3,
      method: "tools/call",
      params: {
        name: "cddm_compare_semantic_graphs",
        arguments: {
          code_a: "fn foo(x: i32) -> i32 { x * 2 }",
          language_a: "rust",
          code_b: "def foo(x):\n    return x * 2",
          language_b: "python",
        },
      },
    });
    expect(compareRes.error).toBeUndefined();
    const result = compareRes.result as { content: Array<{ text: string }> };
    expect(result).toBeDefined();
    const content = result.content[0]?.text ?? "{}";
    const parsed = JSON.parse(content);
    expect(parsed.is_semantic_clone).toBe(true);
    expect(parsed.is_cross_language).toBe(true);
  });

  itIfBinary(
    "should read workspace health resource",
    async () => {
      const resRead = await callMcpStdio({
        jsonrpc: "2.0",
        id: 4,
        method: "resources/read",
        params: { uri: "cddm://workspace/health" },
      });
      expect(resRead.error).toBeUndefined();
      const result = resRead.result as { contents: Array<{ text: string }> };
      expect(result).toBeDefined();
      const content = result.contents[0]?.text ?? "{}";
      const parsed = JSON.parse(content);
      expect(parsed.dry_health_score).toBeGreaterThan(0);
    },
    30000,
  );
});
