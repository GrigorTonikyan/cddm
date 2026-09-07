import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: scan_codebase", () => {
  it("should perform polyglot code duplication scan on directory", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_files).toBe("number");
    expect(typeof res.total_tokens).toBe("number");
    expect(typeof res.total_clones).toBe("number");
    expect(typeof res.dry_health_score).toBe("number");
    expect(res.total_files).toBeGreaterThan(0);
    expect(res.dry_health_score).toBeGreaterThanOrEqual(0);
    expect(res.dry_health_score).toBeLessThanOrEqual(100);
  }, 30000);

  it("should handle default directory and token arguments", async () => {
    const res = await executeTool("scan_codebase", {});
    expect(res).toBeDefined();
    expect(res.total_files).toBeGreaterThan(0);
  }, 30000);

  it("should support detect_type3 toggle in scan_codebase", async () => {
    const resWith = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type3: true,
    });
    const resWithout = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type3: false,
    });

    expect(resWith).toBeDefined();
    expect(resWithout).toBeDefined();
    expect(resWithout.total_files).toBe(resWith.total_files);
  }, 30000);

  it("should support detect_type4 toggle in scan_codebase", async () => {
    const resWith = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type4: true,
    });
    const resWithout = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      detect_type4: false,
    });

    expect(resWith).toBeDefined();
    expect(resWithout).toBeDefined();
    expect(resWithout.total_files).toBe(resWith.total_files);
  }, 30000);

  it("should return compact summary when summary_only is true", async () => {
    const res = await executeTool("scan_codebase", {
      directory: "crates/cddm-lsp",
      min_tokens: 50,
      summary_only: true,
    });

    expect(res).toBeDefined();
    expect(res.summary_mode).toBe(true);
    expect(typeof res.total_files).toBe("number");
    expect(typeof res.dry_health_score).toBe("number");
    expect(Array.isArray(res.top_clusters)).toBe(true);
    expect(res.clone_pairs).toBeUndefined();
  }, 30000);
});
