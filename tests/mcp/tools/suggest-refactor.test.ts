import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_suggest_refactor", () => {
  it("should generate structural refactoring suggestion and unified patch", async () => {
    const res = await executeTool("cddm_suggest_refactor", {
      file_a: "crates/cddm-cli/src/commands/diff.rs",
      start_line_a: 33,
      end_line_a: 66,
      file_b: "crates/cddm-cli/src/commands/scan.rs",
      start_line_b: 43,
      end_line_b: 76,
    });

    expect(res).toBeDefined();
    expect(res.strategy).toBeDefined();
    expect(typeof res.strategy).toBe("string");
    expect(typeof res.lines_saved).toBe("number");
    expect(typeof res.suggested_function_name).toBe("string");
  });

  it("should reject invocation with missing line bounds", async () => {
    await assertToolError("cddm_suggest_refactor", {}, RPC_ERRORS.INVALID_PARAMS);
  });
});
