import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_get_clone_cluster", () => {
  it("should fetch cluster details and occurrence locations", async () => {
    const res = await executeTool("cddm_get_clone_cluster", {
      cluster_id: 1,
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(res.cluster_id).toBe(1);
    expect(Array.isArray(res.occurrences)).toBe(true);
  });

  it("should return error when cluster_id is missing", async () => {
    await assertToolError("cddm_get_clone_cluster", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
