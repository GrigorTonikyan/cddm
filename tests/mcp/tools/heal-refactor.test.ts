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

  it("should support cluster_id resolution for occurrences", async () => {
    const res = await executeTool("cddm_heal_refactor", {
      directory: ".",
      provider: "mock",
      cluster_id: 1,
      function_name: "healed_cluster_fn",
      verify: false,
      max_iterations: 1,
    });

    expect(res).toBeDefined();
    expect(typeof res.success).toBe("boolean");
    expect(res.iterations_run).toBeGreaterThanOrEqual(1);
  });

  it("should recognize anthropic provider and validate API key requirement", async () => {
    try {
      await executeTool("cddm_heal_refactor", {
        directory: ".",
        provider: "anthropic",
        cluster_id: 1,
        verify: false,
      });
      expect(true).toBe(false);
    } catch (err: unknown) {
      const errorMsg = String(err);
      expect(errorMsg).toContain("Anthropic API key");
    }
  });
});
