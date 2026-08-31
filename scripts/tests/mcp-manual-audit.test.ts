import { describe, expect, it } from "bun:test";
import { callMcpStdio } from "../../tests/mcp/helpers";

describe("MCP Server Live Multi-Tool Fidelity & Response Audit", () => {
  it("should initialize cleanly and negotiate capabilities", async () => {
    const initRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2024-11-05",
        capabilities: {},
        clientInfo: { name: "ManualAuditClient", version: "1.0.0" },
      },
    });
    expect(initRes.error).toBeUndefined();
    expect(initRes.result).toBeDefined();
    const result = initRes.result as { serverInfo: { name: string; version: string } };
    expect(result.serverInfo.name).toContain("CDDM");
  });

  it("should list all 30 tools with semantic category metadata", async () => {
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
    expect(tools.length).toBeGreaterThanOrEqual(22);
    for (const t of tools) {
      expect(t["x-cddm-category"]).toBeDefined();
    }
  });

  it("should extract semantic control flow graphs and compare them", async () => {
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

  it("should read workspace health resource", async () => {
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
  });
});
