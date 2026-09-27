import { describe, expect, it } from "bun:test";
import { startMcpSession } from "./helpers";

describe("MCP 2026-07-28 Asynchronous Tasks Framework", () => {
  it("should reject tasks/call when name is missing", async () => {
    const session = startMcpSession();
    try {
      const res = await session.call({
        jsonrpc: "2.0",
        id: 101,
        method: "tasks/call",
        params: {},
      });
      expect(res.error).toBeDefined();
      expect(res.error?.code).toBe(-32602);
      expect(res.error?.message).toContain("Missing required parameter");
    } finally {
      await session.close();
    }
  });

  it("should reject tasks/status when taskId is missing or invalid", async () => {
    const session = startMcpSession();
    try {
      const missingRes = await session.call({
        jsonrpc: "2.0",
        id: 102,
        method: "tasks/status",
        params: {},
      });
      expect(missingRes.error).toBeDefined();
      expect(missingRes.error?.code).toBe(-32602);

      const invalidRes = await session.call({
        jsonrpc: "2.0",
        id: 103,
        method: "tasks/status",
        params: { taskId: "task-nonexistent-12345" },
      });
      expect(invalidRes.error).toBeDefined();
      expect(invalidRes.error?.code).toBe(-32602);
      expect(invalidRes.error?.message).toContain("not found");
    } finally {
      await session.close();
    }
  });

  it("should reject tasks/cancel when taskId is missing or invalid", async () => {
    const session = startMcpSession();
    try {
      const missingRes = await session.call({
        jsonrpc: "2.0",
        id: 104,
        method: "tasks/cancel",
        params: {},
      });
      expect(missingRes.error).toBeDefined();
      expect(missingRes.error?.code).toBe(-32602);

      const invalidRes = await session.call({
        jsonrpc: "2.0",
        id: 105,
        method: "tasks/cancel",
        params: { taskId: "task-nonexistent-99999" },
      });
      expect(invalidRes.error).toBeDefined();
      expect(invalidRes.error?.code).toBe(-32602);
      expect(invalidRes.error?.message).toContain("not found");
    } finally {
      await session.close();
    }
  });

  it("should spawn task via header-based routing with Mcp-Method and Mcp-Name", async () => {
    const session = startMcpSession();
    try {
      const res = await session.call({
        jsonrpc: "2.0",
        id: 106,
        method: "mcp/dispatch",
        headers: {
          "Mcp-Method": "tasks/call",
          "Mcp-Name": "scan_codebase",
        },
        params: {
          arguments: { directory: "crates/cddm-lsp" },
        },
      });

      expect(res.error).toBeUndefined();
      const task = res.result as any;
      expect(task.taskId).toBeDefined();
      expect(task.name).toBe("scan_codebase");
      expect(task.status).toBe("running");

      // Verify task appears in list
      const listRes = await session.call({
        jsonrpc: "2.0",
        id: 107,
        method: "tasks/list",
        params: {},
      });
      expect(listRes.error).toBeUndefined();
      const tasks = (listRes.result as any)?.tasks || [];
      expect(tasks.some((t: any) => t.taskId === task.taskId)).toBe(true);
    } finally {
      await session.close();
    }
  });
});
