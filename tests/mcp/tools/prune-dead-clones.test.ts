import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_prune_dead_clones", () => {
  it("should run dead clone pruning in dry-run mode on workspace", async () => {
    const res = await executeTool("cddm_prune_dead_clones", {
      directory: ".",
      min_tokens: 50,
      dry_run: true,
      safe_only: true,
      threshold: 0.9,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_candidates).toBe("number");
    expect(typeof res.pruned_items).toBe("number");
    expect(typeof res.skipped_items).toBe("number");
    expect(typeof res.total_lines_removed).toBe("number");
    expect(res.dry_run).toBe(true);
    expect(Array.isArray(res.files_affected)).toBe(true);
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);

  it("should support default parameters and return valid schema payload", async () => {
    const res = await executeTool("cddm_prune_dead_clones", {
      dry_run: true,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_candidates).toBe("number");
    expect(typeof res.pruned_items).toBe("number");
    expect(res.dry_run).toBe(true);
    expect(Array.isArray(res.items)).toBe(true);
  }, 30000);
});
