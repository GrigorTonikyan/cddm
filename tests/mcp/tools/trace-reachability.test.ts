import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_trace_reachability", () => {
  it("should trace cross-package call-graph reachability across the workspace", async () => {
    const res = await executeTool("cddm_trace_reachability", {
      directory: ".",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_packages).toBe("number");
    expect(Array.isArray(res.packages)).toBe(true);
    expect(typeof res.live_cross_package_symbols).toBe("number");
    expect(typeof res.live_internal_symbols).toBe("number");
    expect(typeof res.unused_exported_symbols).toBe("number");
    expect(typeof res.dead_internal_symbols).toBe("number");
    expect(typeof res.total_cross_package_calls).toBe("number");
    expect(Array.isArray(res.symbol_traces)).toBe(true);
  }, 30000);

  it("should support default parameters and return valid reachability payload", async () => {
    const res = await executeTool("cddm_trace_reachability", {});

    expect(res).toBeDefined();
    expect(typeof res.total_packages).toBe("number");
    expect(Array.isArray(res.packages)).toBe(true);
    expect(Array.isArray(res.symbol_traces)).toBe(true);
  }, 30000);
});
