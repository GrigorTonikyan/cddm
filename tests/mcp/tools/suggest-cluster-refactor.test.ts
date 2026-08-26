import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_suggest_cluster_refactor", () => {
  it("should synthesize multi-site refactoring from explicit occurrences", async () => {
    const res = await executeTool("cddm_suggest_cluster_refactor", {
      occurrences: [
        { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        { file: "crates/cddm-cli/src/tui/views/refactor.rs", start_line: 1, end_line: 14 },
      ],
    });

    expect(res).toBeDefined();
    expect(res.cluster_id).toBe("cluster-custom");
    expect(res.suggested_function_name).toBeDefined();
    expect(Array.isArray(res.sites)).toBe(true);
    expect(typeof res.total_lines_saved).toBe("number");
  });

  it("should reject invocation with fewer than 2 occurrences", async () => {
    await assertToolError(
      "cddm_suggest_cluster_refactor",
      {
        occurrences: [
          { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        ],
      },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
