import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_scan_cross_language", () => {
  it("should run cross-language semantic clone detection", async () => {
    const res = await executeTool("cddm_scan_cross_language", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      threshold: 0.75,
    });

    expect(res).toBeDefined();
    expect(Array.isArray(res.pairs)).toBe(true);
    expect(typeof res.total_pairs).toBe("number");
  });
});
