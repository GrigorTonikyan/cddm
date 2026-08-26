import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_ast_refactor", () => {
  it("should generate AST-aware refactoring suggestion and replacement nodes", async () => {
    const res = await executeTool("cddm_ast_refactor", {
      occurrences: [
        { file: "crates/cddm-cli/src/tui/views/extract.rs", start_line: 1, end_line: 14 },
        { file: "crates/cddm-cli/src/tui/views/refactor.rs", start_line: 1, end_line: 14 },
      ],
      custom_function_name: "render_view_header",
    });

    expect(res).toBeDefined();
    expect(res.function_name).toBe("render_view_header");
    expect(typeof res.helper_function_code).toBe("string");
    expect(Array.isArray(res.rewritten_files)).toBe(true);
    expect(typeof res.syntax_valid).toBe("boolean");
  });

  it("should reject invocation with empty occurrences", async () => {
    await assertToolError("cddm_ast_refactor", { occurrences: [] }, RPC_ERRORS.INVALID_PARAMS);
  });
});
