import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_diff_matrix", () => {
  it("should evaluate clone drift matrix across multiple git revisions", async () => {
    const res = await executeTool("cddm_diff_matrix", {
      branches: ["HEAD", "HEAD"],
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.branches)).toBe(true);
    expect(res.branches.length).toBe(2);
    expect(Array.isArray(res.matrix)).toBe(true);
    expect(res.matrix.length).toBe(2);
    expect(typeof res.matrix[0].base_dry_score).toBe("number");
    expect(typeof res.matrix[0].divergence_index).toBe("number");
    expect(typeof res.summary).toBe("string");
  });

  it("should reject diff matrix when branches parameter has fewer than 2 elements", async () => {
    await assertToolError("cddm_diff_matrix", { branches: ["HEAD"] }, RPC_ERRORS.INVALID_PARAMS);
  });

  it("should reject diff matrix when branches parameter is missing", async () => {
    await assertToolError("cddm_diff_matrix", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
