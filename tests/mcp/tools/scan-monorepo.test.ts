import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_scan_monorepo", () => {
  it("should discover packages and scan monorepo boundaries", async () => {
    const res = await executeTool("cddm_scan_monorepo", {
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.workspaces)).toBe(true);
    expect(typeof res.total_workspaces).toBe("number");
    expect(typeof res.total_clones).toBe("number");
  });
});
