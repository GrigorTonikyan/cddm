import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_detect_dead_clones", () => {
  const sampleLcov = `
SF:src/auth.ts
DA:10,0
DA:11,0
end_of_record
SF:src/helpers.ts
DA:1,0
DA:2,0
end_of_record
`;

  it("should detect dead code clones with 0 executions", async () => {
    const res = await executeTool("cddm_detect_dead_clones", {
      report_content: sampleLcov,
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_clone_pairs).toBe("number");
    expect(typeof res.dead_code_clones).toBe("number");
    expect(Array.isArray(res.metrics)).toBe(true);
  });

  it("should reject invocation when non-existent report_path is provided", async () => {
    await assertToolError(
      "cddm_detect_dead_clones",
      { report_path: "non/existent/lcov.info" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
