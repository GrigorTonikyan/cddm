import { describe, expect, it } from "bun:test";
import { executeTool } from "../helpers";

describe("MCP Tool: cddm_semantic_neural_scan", () => {
  it("should run in-process neural code embedding scan on workspace", async () => {
    const res = await executeTool("cddm_semantic_neural_scan", {
      directory: "crates/cddm-lsp",
      threshold: 0.85,
      dimension: 256,
    });

    expect(res).toBeDefined();
    expect(typeof res.total_blocks_embedded).toBe("number");
    expect(typeof res.total_neural_pairs).toBe("number");
    expect(Array.isArray(res.pairs)).toBe(true);
  });

  it("should compare two code snippets with cosine similarity", async () => {
    const res = await executeTool("cddm_semantic_neural_scan", {
      code_a: "pub fn compute_sum(a: i32, b: i32) -> i32 { a + b }",
      language_a: "rs",
      code_b: "def compute_sum(a, b):\n    return a + b",
      language_b: "py",
      threshold: 0.7,
    });

    expect(res).toBeDefined();
    expect(typeof res.cosine_similarity).toBe("number");
    expect(typeof res.is_equivalent).toBe("boolean");
    expect(res.cosine_similarity).toBeGreaterThan(0.4);
  });

  it("should return error for non-existent workspace directory", async () => {
    try {
      await executeTool("cddm_semantic_neural_scan", {
        directory: "non_existent_folder_xyz_123",
      });
      expect(true).toBe(false);
    } catch (e: any) {
      expect(e).toBeDefined();
    }
  });
});
