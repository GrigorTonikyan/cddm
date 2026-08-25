import { describe, expect, it } from "bun:test";
import { join } from "node:path";

interface JsonRpcResponse<T = unknown> {
  jsonrpc: string;
  id?: number | string;
  result?: T;
  error?: {
    code: number;
    message: string;
  };
}

async function callMcpStdio(request: Record<string, unknown>): Promise<JsonRpcResponse> {
  const binaryPath = join(
    import.meta.dir,
    "../../target/debug",
    process.platform === "win32" ? "cddm-mcp.exe" : "cddm-mcp",
  );

  const proc = Bun.spawn([binaryPath], {
    stdin: "pipe",
    stdout: "pipe",
    stderr: "pipe",
  });

  const payload = JSON.stringify(request) + "\n";
  void proc.stdin.write(payload);
  void proc.stdin.flush();
  void proc.stdin.end();

  const text = await new Response(proc.stdout).text();
  await proc.exited;

  const line = text
    .split("\n")
    .map((l) => l.trim())
    .find((l) => l.startsWith("{") && l.endsWith("}"));
  if (!line) {
    throw new Error(`No JSON output received from cddm-mcp. Raw stdout: ${text}`);
  }

  return JSON.parse(line) as JsonRpcResponse;
}

describe("CDDM Model Context Protocol (MCP) Stdio Server E2E", () => {
  it("should handle initialize handshake", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2024-11-05",
        capabilities: {},
        clientInfo: { name: "test-client", version: "1.0.0" },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(1);
    const result = res.result as { serverInfo?: { name?: string; version?: string } };
    expect(result?.serverInfo?.name).toContain("CDDM");
    expect(result?.serverInfo?.version).toBe("1.7.0");
  });

  it("should list all 20 MCP tools with full schemas", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 2,
      method: "tools/list",
      params: {},
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(2);
    const result = res.result as { tools?: Array<{ name: string; description: string }> };
    expect(result?.tools).toBeDefined();
    expect(result?.tools?.length).toBeGreaterThanOrEqual(18);

    const toolNames = result?.tools?.map((t) => t.name) || [];
    expect(toolNames).toContain("scan_codebase");
    expect(toolNames).toContain("cddm_check_policies");
    expect(toolNames).toContain("cddm_check_suppression");
    expect(toolNames).toContain("cddm_export_sarif");
    expect(toolNames).toContain("cddm_scan_monorepo");
    expect(toolNames).toContain("cddm_heal_refactor");
  });

  it("should execute cddm_check_policies tool", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 3,
      method: "tools/call",
      params: {
        name: "cddm_check_policies",
        arguments: {
          directory: ".",
        },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(3);
    const result = res.result as { content?: Array<{ type: string; text: string }> };
    expect(result?.content).toBeDefined();
    expect(result?.content?.[0]?.type).toBe("text");
    expect(result?.content?.[0]?.text).toContain("total_violations");
  });

  it("should execute cddm_scan_monorepo tool", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 4,
      method: "tools/call",
      params: {
        name: "cddm_scan_monorepo",
        arguments: {
          directory: ".",
        },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(4);
    const result = res.result as { content?: Array<{ type: string; text: string }> };
    expect(result?.content).toBeDefined();
    expect(result?.content?.[0]?.text).toContain("workspaces");
  });

  it("should list and read MCP resources", async () => {
    const listRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 5,
      method: "resources/list",
      params: {},
    });

    expect(listRes.jsonrpc).toBe("2.0");
    const listResult = listRes.result as { resources?: Array<{ uri: string }> };
    const uris = listResult?.resources?.map((r) => r.uri) || [];
    expect(uris).toContain("cddm://workspace/clusters");
    expect(uris).toContain("cddm://workspace/policies");

    const readRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 6,
      method: "resources/read",
      params: {
        uri: "cddm://workspace/policies",
      },
    });

    expect(readRes.jsonrpc).toBe("2.0");
    const readResult = readRes.result as { contents?: Array<{ uri: string; text: string }> };
    expect(readResult?.contents?.[0]?.uri).toBe("cddm://workspace/policies");
  });
});
