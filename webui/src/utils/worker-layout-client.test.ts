import { describe, it, expect, afterEach } from "vite-plus/test";
import {
  computeTreemapLayoutSync,
  computeTreemapLayoutAsync,
  terminateLayoutWorker,
} from "./worker-layout-client";
import { ClonePair } from "../types/cddm-types";

describe("worker-layout-client", () => {
  afterEach(() => {
    terminateLayoutWorker();
  });

  const mockClonePairs: ClonePair[] = [
    {
      file_a: "src/utils/math.ts",
      start_line_a: 10,
      end_line_a: 20,
      file_b: "src/helpers/calc.ts",
      start_line_b: 15,
      end_line_b: 25,
      token_count: 50,
      similarity: 1.0,
      fragment_hash: "hash123",
      clone_type: "Exact",
    },
    {
      file_a: "src/utils/math.ts",
      start_line_a: 30,
      end_line_a: 40,
      file_b: "src/utils/other.ts",
      start_line_b: 5,
      end_line_b: 15,
      token_count: 40,
      similarity: 0.95,
      fragment_hash: "hash456",
      clone_type: "Renamed",
    },
  ];

  it("should compute layout synchronously with valid rectangles", () => {
    const result = computeTreemapLayoutSync({
      clonePairs: mockClonePairs,
      width: 800,
      height: 400,
      currentPath: "",
    });

    expect(result.fullHierarchy).toBeDefined();
    expect(result.fullHierarchy.children).toBeDefined();
    expect(result.layoutRects.length).toBeGreaterThan(0);
    expect(result.activeNode).toBe(result.fullHierarchy);

    for (const rect of result.layoutRects) {
      expect(rect.width).toBeGreaterThanOrEqual(0);
      expect(rect.height).toBeGreaterThanOrEqual(0);
      expect(rect.x + rect.width).toBeLessThanOrEqual(800.1);
      expect(rect.y + rect.height).toBeLessThanOrEqual(400.1);
    }
  });

  it("should navigate into nested directory path synchronously", () => {
    const result = computeTreemapLayoutSync({
      clonePairs: mockClonePairs,
      width: 800,
      height: 400,
      currentPath: "src/utils",
    });

    expect(result.activeNode.name).toBe("utils");
    expect(result.layoutRects.length).toBeGreaterThan(0);
  });

  it("should compute layout asynchronously (falling back cleanly in test env)", async () => {
    const result = await computeTreemapLayoutAsync({
      clonePairs: mockClonePairs,
      width: 800,
      height: 400,
      currentPath: "",
    });

    expect(result.fullHierarchy).toBeDefined();
    expect(result.layoutRects.length).toBeGreaterThan(0);
  });
});
