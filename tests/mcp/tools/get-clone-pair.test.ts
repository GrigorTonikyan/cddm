import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_get_clone_pair", () => {
  it("should fetch localized source lines for a valid clone pair", async () => {
    const res = await executeTool("cddm_get_clone_pair", {
      file_a: "crates/cddm-core/src/extract/mod.rs",
      start_line_a: 39,
      end_line_a: 50,
      file_b: "crates/cddm-core/src/refactor/ast.rs",
      start_line_b: 39,
      end_line_b: 50,
    });

    expect(res).toBeDefined();
    expect(res.fragment_a).toBeDefined();
    expect(res.fragment_b).toBeDefined();
    expect(res.fragment_a.file).toContain("extract");
    expect(res.fragment_b.file).toContain("ast");
    expect(res.fragment_a.line_count).toBe(12);
    expect(res.fragment_b.line_count).toBe(12);
    expect(Array.isArray(res.fragment_a.lines)).toBe(true);
    expect(Array.isArray(res.fragment_b.lines)).toBe(true);
  });

  it("should return INVALID_PARAMS when required range arguments are missing", async () => {
    await assertToolError(
      "cddm_get_clone_pair",
      { file_a: "crates/cddm-core/src/extract/mod.rs" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
