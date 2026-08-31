import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_detect_dead_code", () => {
  it("should run polyglot dead code detection on workspace", async () => {
    const res = await executeTool("cddm_detect_dead_code", {
      directory: ".",
      min_tokens: 50,
      static_only: true,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_dead_items).toBe("number");
    expect(typeof res.dead_functions).toBe("number");
    expect(typeof res.unreachable_blocks).toBe("number");
    expect(typeof res.dead_clones).toBe("number");
    expect(typeof res.total_dead_lines).toBe("number");
    expect(typeof res.estimated_savings_pct).toBe("number");
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);

  it("should support running with default parameters", async () => {
    const res = await executeTool("cddm_detect_dead_code", {});

    expect(res).toBeDefined();
    expect(typeof res.total_dead_items).toBe("number");
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);
});
