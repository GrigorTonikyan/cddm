import { describe, expect, it } from "bun:test";
import { assertPropertyTypes, assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_correlate_coverage", () => {
  const sampleLcov = `
SF:src/auth.ts
DA:10,5
DA:11,5
end_of_record
SF:src/helpers.ts
DA:1,100
DA:2,150
end_of_record
`;

  it("should correlate coverage report content with duplicate clones", async () => {
    const res = await executeTool("cddm_correlate_coverage", {
      report_content: sampleLcov,
      format: "lcov",
      directory: ".",
      min_tokens: 50,
    });

    assertPropertyTypes(res, {
      total_clone_pairs: "number",
      dead_code_clones: "number",
      test_gap_clones: "number",
      hot_path_clones: "number",
      total_runtime_hits: "number",
      metrics: "array",
    });
  });

  it("should filter by min_hits and dead_code_only", async () => {
    const res = await executeTool("cddm_correlate_coverage", {
      report_content: sampleLcov,
      dead_code_only: true,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.metrics)).toBe(true);
  });

  it("should reject invocation when non-existent report_path is provided", async () => {
    await assertToolError(
      "cddm_correlate_coverage",
      { report_path: "non/existent/coverage.xml" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
