import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_get_semantic_graph", () => {
  it("should extract Control Flow Graph and Program Dependence Graph from source code", async () => {
    const res = await executeTool("cddm_get_semantic_graph", {
      code: "fn compute(x: i32) -> i32 { if x > 0 { x * 2 } else { 0 } }",
      language: "Rust",
    });

    expect(res).toBeDefined();
    expect(res.cfg_count).toBeGreaterThanOrEqual(1);
    expect(res.pdg_count).toBeGreaterThanOrEqual(1);
    expect(Array.isArray(res.cfgs)).toBe(true);
    expect(res.cfgs[0].nodes.length).toBeGreaterThan(0);
    expect(res.cfgs[0].wl_hash).toBeDefined();
  });

  it("should reject invocation when code or file is missing", async () => {
    await assertToolError("cddm_get_semantic_graph", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
