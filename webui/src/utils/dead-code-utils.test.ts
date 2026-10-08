import { describe, expect, it } from "vite-plus/test";
import type { CloneLocation } from "../types/cddm-types";
import type { DeadCodeItem } from "../types/dead-code-types";
import {
  findMatchingDeadCodeItem,
  isCloneClusterDead,
  isClonePairDead,
  normalizePath,
  pathsMatch,
} from "./dead-code-utils";

describe("dead-code-utils", () => {
  const sampleItems: DeadCodeItem[] = [
    {
      id: 1,
      file_path: "src/legacy/utils.ts",
      symbol_name: "legacyHelper",
      kind: "unreferenced_function",
      line_start: 10,
      line_end: 25,
      token_count: 50,
      estimated_lines_saved: 16,
      reason: "No references found across workspace",
      confidence: 0.95,
    },
    {
      id: 2,
      file_path: "crates/core/src/dead_clone.rs",
      symbol_name: "unused_clone",
      kind: "dead_clone",
      line_start: 40,
      line_end: 60,
      token_count: 80,
      estimated_lines_saved: 21,
      reason: "Duplicate block in dead path",
      confidence: 0.9,
    },
  ];

  describe("normalizePath & pathsMatch", () => {
    it("should normalize backslashes and casing", () => {
      expect(normalizePath("src\\Legacy\\Utils.ts")).toBe("src/legacy/utils.ts");
      expect(normalizePath("./src/legacy/utils.ts")).toBe("src/legacy/utils.ts");
    });

    it("should match identical and subpath file paths", () => {
      expect(pathsMatch("src/legacy/utils.ts", "src/legacy/utils.ts")).toBe(true);
      expect(pathsMatch("X:/project/src/legacy/utils.ts", "src/legacy/utils.ts")).toBe(true);
      expect(pathsMatch("src/legacy/utils.ts", "X:/project/src/legacy/utils.ts")).toBe(true);
      expect(pathsMatch("src/other/utils.ts", "src/legacy/utils.ts")).toBe(false);
    });
  });

  describe("findMatchingDeadCodeItem", () => {
    it("should return undefined for empty items or missing file", () => {
      expect(findMatchingDeadCodeItem(undefined, "src/legacy/utils.ts")).toBeUndefined();
      expect(findMatchingDeadCodeItem([], "src/legacy/utils.ts")).toBeUndefined();
      expect(findMatchingDeadCodeItem(sampleItems, "")).toBeUndefined();
    });

    it("should find item by file match alone when line bounds omitted", () => {
      const match = findMatchingDeadCodeItem(sampleItems, "src/legacy/utils.ts");
      expect(match).toBeDefined();
      expect(match?.id).toBe(1);
    });

    it("should match overlapping line ranges", () => {
      // Overlaps lines 10-25
      const match1 = findMatchingDeadCodeItem(sampleItems, "src/legacy/utils.ts", 15, 30);
      expect(match1).toBeDefined();
      expect(match1?.id).toBe(1);

      // Outside lines 10-25
      const match2 = findMatchingDeadCodeItem(sampleItems, "src/legacy/utils.ts", 30, 45);
      expect(match2).toBeUndefined();
    });
  });

  describe("isClonePairDead", () => {
    it("should detect when either side of a clone pair overlaps dead code", () => {
      const result = isClonePairDead(
        sampleItems,
        "src/legacy/utils.ts",
        [12, 20],
        "src/active/component.ts",
        [100, 108],
      );
      expect(result.isDead).toBe(true);
      expect(result.itemA?.id).toBe(1);
      expect(result.itemB).toBeUndefined();
    });

    it("should return false when neither side overlaps dead code", () => {
      const result = isClonePairDead(
        sampleItems,
        "src/active/foo.ts",
        [1, 10],
        "src/active/bar.ts",
        [20, 30],
      );
      expect(result.isDead).toBe(false);
    });
  });

  describe("isCloneClusterDead", () => {
    it("should detect dead locations in a clone cluster", () => {
      const locations: CloneLocation[] = [
        { file: "src/legacy/utils.ts", start_line: 15, end_line: 22 },
        { file: "crates/core/src/dead_clone.rs", start_line: 45, end_line: 55 },
        { file: "src/active/main.ts", start_line: 1, end_line: 10 },
      ];

      const result = isCloneClusterDead(sampleItems, locations);
      expect(result.isDead).toBe(true);
      expect(result.deadLocationsCount).toBe(2);
      expect(result.matchedItems.length).toBe(2);
    });

    it("should return false for empty or unreferenced locations", () => {
      expect(isCloneClusterDead(sampleItems, [])).toEqual({
        isDead: false,
        deadLocationsCount: 0,
        matchedItems: [],
      });
      expect(isCloneClusterDead(undefined, [{ file: "a.ts", start_line: 1, end_line: 5 }])).toEqual(
        {
          isDead: false,
          deadLocationsCount: 0,
          matchedItems: [],
        },
      );
    });
  });
});
