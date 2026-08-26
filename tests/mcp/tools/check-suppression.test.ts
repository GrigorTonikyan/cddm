import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_check_suppression", () => {
  it("should evaluate file path against ignore rules", async () => {
    const res = await executeTool("cddm_check_suppression", {
      path: "tests/fixtures/sample.rs",
      ignore_tests: true,
    });

    expect(res).toBeDefined();
    expect(res.path).toBe("tests/fixtures/sample.rs");
    expect(typeof res.is_ignored).toBe("boolean");
    expect(typeof res.path_ignored).toBe("boolean");
  });

  it("should reject check when path is missing", async () => {
    await assertToolError("cddm_check_suppression", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
