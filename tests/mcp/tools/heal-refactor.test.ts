import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_heal_refactor", () => {
  it("should run autonomous surgeon loop with mock AI provider", async () => {
    const res = await executeTool("cddm_heal_refactor", {
      directory: ".",
      provider: "mock",
      pair_id: 1,
      function_name: "healed_fn",
      verify: false,
    });

    expect(res).toBeDefined();
    expect(typeof res.success).toBe("boolean");
    expect(res.iterations_run).toBeGreaterThanOrEqual(1);
    expect(Array.isArray(res.iterations)).toBe(true);
  });
});
