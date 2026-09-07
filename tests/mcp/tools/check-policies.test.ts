import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_check_policies", () => {
  it("should evaluate duplication quality gate policies", async () => {
    const res = await executeTool("cddm_check_policies", {
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.passed).toBe("boolean");
    expect(typeof res.total_violations).toBe("number");
    expect(Array.isArray(res.violations)).toBe(true);
  });

  it("should return compact policy evaluation when detail_level is compact", async () => {
    const res = await executeTool("cddm_check_policies", {
      directory: ".",
      min_tokens: 50,
      detail_level: "compact",
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.passed).toBe("boolean");
    expect(typeof res.total_violations).toBe("number");
    expect(Array.isArray(res.top_violations)).toBe(true);
    expect(res.violations).toBeUndefined();
  });
});
