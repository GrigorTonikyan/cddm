import { describe, expect, it } from "bun:test";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { callMcpStdio, startMcpSession } from "./helpers";

function toolNameToTestFilename(toolName: string): string {
  // Normalize "cddm_foo_bar" or "scan_codebase" to kebab-case
  const normalized = toolName.startsWith("cddm_") ? toolName.replace(/^cddm_/, "") : toolName;
  return `${normalized.replace(/_/g, "-")}.test.ts`;
}

describe("MCP Dynamic Discovery & 1:1 Test Suite Mapping", () => {
  it("should perform JSON-RPC 2.0 initialize handshake with 2026-07-28 protocol", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2026-07-28",
        capabilities: {},
        clientInfo: { name: "cddm-discovery-test", version: "4.2.0" },
      },
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.id).toBe(1);
    expect(res.error).toBeUndefined();
    const result = res.result as {
      protocolVersion?: string;
      serverInfo?: { name?: string; version?: string };
      capabilities?: { tasks?: { cancel?: boolean }; sampling?: unknown; roots?: unknown };
    };
    expect(result?.protocolVersion).toBe("2026-07-28");
    expect(result?.serverInfo?.name).toContain("CDDM");
    expect(result?.capabilities?.tasks?.cancel).toBe(true);
    expect(result?.capabilities?.sampling).toBeUndefined();
    expect(result?.capabilities?.roots).toBeUndefined();
  });

  it("should dynamically discover all 33 MCP tools and verify 1:1 test suite presence with caching annotations", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 2,
      method: "tools/list",
      params: {},
    });

    expect(res.jsonrpc).toBe("2.0");
    expect(res.error).toBeUndefined();
    const tools = (res.result as any)?.tools || [];
    expect(tools.length).toBeGreaterThanOrEqual(33);

    const toolsDir = join(import.meta.dir, "tools");
    const existingTestFiles = new Set(readdirSync(toolsDir));

    const missingTests: string[] = [];
    for (const tool of tools) {
      expect(tool.annotations).toBeDefined();
      expect(typeof tool.annotations.readOnlyHint).toBe("boolean");
      expect(typeof tool.annotations.destructiveHint).toBe("boolean");
      expect(typeof tool.annotations.idempotentHint).toBe("boolean");
      expect(typeof tool.annotations.ttlMs).toBe("number");
      expect(typeof tool.annotations.cacheScope).toBe("string");

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

  it("should return method not found for deprecated roots and sampling methods", async () => {
    for (const method of ["roots/list", "sampling/createMessage"]) {
      const res = await callMcpStdio({
        jsonrpc: "2.0",
        id: 3,
        method,
        params: {},
      });
      expect(res.jsonrpc).toBe("2.0");
      expect(res.error).toBeDefined();
      expect(res.error?.code).toBe(-32601);
    }
  });

  it("should support MCP 2026-07-28 Tasks framework (tasks/call, tasks/list, tasks/status, tasks/cancel)", async () => {
    const session = startMcpSession();
    try {
      // 1. tasks/call
      const callRes = await session.call({
        jsonrpc: "2.0",
        id: 4,
        method: "tasks/call",
        params: {
          name: "scan_monorepo",
          arguments: { directory: "crates/cddm-lsp" },
        },
      });
      expect(callRes.error).toBeUndefined();
      const taskId = (callRes.result as any)?.taskId;
      expect(typeof taskId).toBe("string");
      expect(taskId.startsWith("task-")).toBe(true);

      // 2. tasks/list
      const listRes = await session.call({
        jsonrpc: "2.0",
        id: 5,
        method: "tasks/list",
        params: {},
      });
      expect(listRes.error).toBeUndefined();
      const tasks = (listRes.result as any)?.tasks || [];
      expect(tasks.some((t: any) => t.taskId === taskId)).toBe(true);

      // 3. tasks/status
      const statusRes = await session.call({
        jsonrpc: "2.0",
        id: 6,
        method: "tasks/status",
        params: { taskId },
      });
      expect(statusRes.error).toBeUndefined();
      expect((statusRes.result as any)?.taskId).toBe(taskId);

      // 4. tasks/cancel
      const cancelRes = await session.call({
        jsonrpc: "2.0",
        id: 7,
        method: "tasks/cancel",
        params: { taskId },
      });
      expect(cancelRes.error).toBeUndefined();
      expect((cancelRes.result as any)?.cancelled).toBe(true);
    } finally {
      await session.close();
    }
  });

  it("should support header-based routing with Mcp-Method and Mcp-Name", async () => {
    const res = await callMcpStdio({
      jsonrpc: "2.0",
      id: 8,
      method: "mcp/dispatch",
      headers: {
        "Mcp-Method": "tools/call",
        "Mcp-Name": "scan_codebase",
      },
      params: {
        arguments: { directory: "crates/cddm-lsp" },
      },
    });

    expect(res.error).toBeUndefined();
    expect((res.result as any)?.content).toBeDefined();
    expect(Array.isArray((res.result as any)?.content)).toBe(true);
  });

  it("should handle resource subscriptions cleanly", async () => {
    const subRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 9,
      method: "resources/subscribe",
      params: { uri: "cddm://workspace/health" },
    });
    expect(subRes.jsonrpc).toBe("2.0");
    expect(subRes.error).toBeUndefined();
    expect((subRes.result as any)?.subscribed).toBe(true);

    const unsubRes = await callMcpStdio({
      jsonrpc: "2.0",
      id: 10,
      method: "resources/unsubscribe",
      params: { uri: "cddm://workspace/health" },
    });
    expect(unsubRes.jsonrpc).toBe("2.0");
    expect(unsubRes.error).toBeUndefined();
    expect((unsubRes.result as any)?.unsubscribed).toBe(true);
  });
});
