import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_diff_scan", () => {
  it("should run differential scan between git refs", async () => {
    const res = await executeTool("cddm_diff_scan", {
      base_ref: "HEAD",
      target_ref: "HEAD",
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(res.scan_id).toBeDefined();
    expect(res.summary).toBeDefined();
    expect(Array.isArray(res.diff_clones)).toBe(true);
    expect(typeof res.duration_ms).toBe("number");
  });

  it("should reject diff scan when base_ref is missing", async () => {
    await assertToolError("cddm_diff_scan", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
