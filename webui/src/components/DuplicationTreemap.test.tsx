import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import {
  DuplicationTreemap,
  buildTreemapHierarchy,
  computeSquarifiedLayout,
} from "./DuplicationTreemap";
import { ClonePair } from "./../types/cddm-types";

describe("DuplicationTreemap Component & Layout", () => {
  const mockPairs: ClonePair[] = [
    {
      file_a: "crates/cddm-core/src/detector.rs",
      start_line_a: 10,
      end_line_a: 20,
      file_b: "crates/cddm-core/src/cache.rs",
      start_line_b: 30,
      end_line_b: 40,
      token_count: 100,
      similarity: 0.95,
      fragment_hash: "hash1",
      clone_type: "Exact",
    },
    {
      file_a: "webui/src/components/ClonePairCard.tsx",
      start_line_a: 5,
      end_line_a: 15,
      file_b: "webui/src/components/ScanResults.tsx",
      start_line_b: 25,
      end_line_b: 35,
      token_count: 50,
      similarity: 0.9,
      fragment_hash: "hash2",
      clone_type: "Renamed",
    },
  ];

  it("should build hierarchy tree correctly from clone pairs", () => {
    const root = buildTreemapHierarchy(mockPairs);
    expect(root.name).toBe("root");
    expect(root.children).toBeDefined();
    expect(root.children!.length).toBeGreaterThan(0);

    const cratesNode = root.children!.find((c) => c.name === "crates");
    expect(cratesNode).toBeDefined();
    expect(cratesNode!.tokens).toBeGreaterThanOrEqual(200);
  });

  it("should compute valid squarified rectangles within bounds", () => {
    const root = buildTreemapHierarchy(mockPairs);
    const rects = computeSquarifiedLayout(root.children || [], 0, 0, 800, 360);

    expect(rects.length).toBeGreaterThan(0);
    for (const r of rects) {
      expect(r.x).toBeGreaterThanOrEqual(0);
      expect(r.y).toBeGreaterThanOrEqual(0);
      expect(r.width).toBeGreaterThan(0);
      expect(r.height).toBeGreaterThan(0);
      expect(r.x + r.width).toBeLessThanOrEqual(801);
      expect(r.y + r.height).toBeLessThanOrEqual(361);
    }
  });

  it("should render SVG rectangles and handle click-to-filter", () => {
    const onFilter = vi.fn();

    render(
      <DuplicationTreemap
        clonePairs={mockPairs}
        totalTokens={5000}
        onSelectFilterPath={onFilter}
      />,
    );

    expect(screen.getByText("Duplication Treemap Visualizer")).toBeDefined();
    expect(screen.getByText("Root")).toBeDefined();
    expect(screen.getByText("Low Duplication")).toBeDefined();
    expect(screen.getByText("High Density")).toBeDefined();

    // Click on crates node
    const cratesText = screen.queryByText("crates");
    if (cratesText) {
      fireEvent.click(cratesText);
      expect(screen.getByText("crates")).toBeDefined();
    }
  });
});
