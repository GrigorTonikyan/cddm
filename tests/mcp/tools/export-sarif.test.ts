import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_export_sarif", () => {
  it("should generate OASIS SARIF v2.1.0 report", async () => {
    const res = await executeTool("cddm_export_sarif", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(res.version).toBe("2.1.0");
    expect(res.$schema).toContain("sarif");
    expect(Array.isArray(res.runs)).toBe(true);
    expect(res.runs.length).toBeGreaterThan(0);
    expect(res.runs[0].tool.driver.name).toBe("CDDM");
  });
});
