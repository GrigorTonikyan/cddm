import { describe, expect, it } from "bun:test";
import { assertToolError, executeTool, RPC_ERRORS } from "../helpers";

describe("MCP Tool: cddm_compare_semantic_graphs", () => {
  it("should compute cross-language hybrid similarity score between CFGs", async () => {
    const res = await executeTool("cddm_compare_semantic_graphs", {
      code_a: "fn sum(a: i32, b: i32) -> i32 { a + b }",
      code_b: "function sum(a, b) { return a + b; }",
      language_a: "Rust",
      language_b: "JavaScript",
    });

    expect(res).toBeDefined();
    expect(typeof res.similarity).toBe("number");
    expect(res.similarity).toBeGreaterThan(0.7);
    expect(res.is_semantic_clone).toBe(true);
    expect(res.is_cross_language).toBe(true);
  });

  it("should reject invocation when code snippets are missing", async () => {
    await assertToolError(
      "cddm_compare_semantic_graphs",
      { code_a: "fn a() {}" },
      RPC_ERRORS.INVALID_PARAMS,
    );
  });
});
