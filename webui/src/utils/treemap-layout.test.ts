import { describe, it, expect } from "vite-plus/test";
import { buildTreemapHierarchy, computeSquarifiedLayout } from "./treemap-layout";
import { ClonePair, TreemapNode } from "../types/cddm-types";

describe("treemap-layout utilities", () => {
  const mockClonePairs: ClonePair[] = [
    {
      fragment_hash: "hash-1",
      file_a: "src/engine/lexer.rs",
      start_line_a: 10,
      end_line_a: 30,
      file_b: "src/parser/lexer_helper.rs",
      start_line_b: 15,
      end_line_b: 35,
      token_count: 120,
      similarity: 95.0,
      clone_type: "Renamed",
      author_a: "Alice",
      author_b: "Bob",
    },
    {
      fragment_hash: "hash-2",
      file_a: "src/engine/lexer.rs",
      start_line_a: 40,
      end_line_a: 60,
      file_b: "src/engine/tokens.rs",
      start_line_b: 5,
      end_line_b: 25,
      token_count: 80,
      similarity: 100.0,
      clone_type: "Exact",
      author_a: "Alice",
      author_b: "Alice",
    },
  ];

  it("builds a hierarchical directory tree correctly", () => {
    const hierarchy = buildTreemapHierarchy(mockClonePairs);
    expect(hierarchy.name).toBe("root");
    expect(hierarchy.children).toBeDefined();
    expect(hierarchy.children!.length).toBeGreaterThan(0);

    const srcNode = hierarchy.children!.find((c) => c.name === "src");
    expect(srcNode).toBeDefined();
    expect(srcNode?.tokens).toBeGreaterThan(0);
    expect(srcNode?.clones).toBeGreaterThanOrEqual(2);
  });

  it("handles empty or invalid clone pair inputs gracefully", () => {
    const emptyHierarchy = buildTreemapHierarchy([]);
    expect(emptyHierarchy.name).toBe("root");
    expect(emptyHierarchy.tokens).toBe(1);
    expect(emptyHierarchy.clones).toBe(0);
    expect(emptyHierarchy.children).toBeUndefined();
  });

  it("computes squarified layout rectangle positions accurately", () => {
    const nodes: TreemapNode[] = [
      { name: "moduleA", path: "src/moduleA", tokens: 500, clones: 4, duplicationPercentage: 40 },
      { name: "moduleB", path: "src/moduleB", tokens: 300, clones: 2, duplicationPercentage: 25 },
      { name: "moduleC", path: "src/moduleC", tokens: 200, clones: 1, duplicationPercentage: 15 },
    ];

    const rects = computeSquarifiedLayout(nodes, 0, 0, 800, 600);
    expect(rects).toHaveLength(3);

    for (const rect of rects) {
      expect(rect.width).toBeGreaterThan(0);
      expect(rect.height).toBeGreaterThan(0);
      expect(rect.x).toBeGreaterThanOrEqual(0);
      expect(rect.y).toBeGreaterThanOrEqual(0);
      expect(rect.x + rect.width).toBeLessThanOrEqual(801);
      expect(rect.y + rect.height).toBeLessThanOrEqual(601);
    }
  });

  it("handles zero dimension or empty node cases safely", () => {
    expect(computeSquarifiedLayout([], 0, 0, 800, 600)).toEqual([]);
    expect(
      computeSquarifiedLayout(
        [{ name: "a", path: "a", tokens: 10, clones: 1, duplicationPercentage: 10 }],
        0,
        0,
        0,
        600,
      ),
    ).toEqual([]);
    expect(
      computeSquarifiedLayout(
        [{ name: "a", path: "a", tokens: 10, clones: 1, duplicationPercentage: 10 }],
        0,
        0,
        800,
        0,
      ),
    ).toEqual([]);
  });
});
