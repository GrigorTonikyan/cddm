import { describe, it, expect } from "vite-plus/test";
import { computeGraphLayout, generateEdgePath, generateDataEdgePath } from "./graph-layout";
import { createMockControlFlowGraph } from "../test/test-helpers";

describe("graph-layout utility", () => {
  const mockCfg = createMockControlFlowGraph();

  it("should compute layout with valid coordinates for all nodes", () => {
    const layout = computeGraphLayout(mockCfg, 400);
    expect(layout.width).toBe(400);
    expect(layout.height).toBeGreaterThan(150);
    expect(layout.positions.size).toBe(3);

    const pos0 = layout.positions.get(0);
    const pos1 = layout.positions.get(1);
    const pos2 = layout.positions.get(2);

    expect(pos0).toBeDefined();
    expect(pos1).toBeDefined();
    expect(pos2).toBeDefined();
    expect(pos0!.y).toBeLessThan(pos1!.y);
    expect(pos1!.y).toBeLessThan(pos2!.y);
  });

  it("should generate valid edge paths", () => {
    const layout = computeGraphLayout(mockCfg, 400);
    const pos0 = layout.positions.get(0)!;
    const pos1 = layout.positions.get(1)!;

    const path = generateEdgePath(pos0, pos1, false);
    expect(path).toContain("M ");
    expect(path.length).toBeGreaterThan(5);

    const loopPath = generateEdgePath(pos1, pos0, true);
    expect(loopPath).toContain("C ");
  });

  it("should generate valid PDG data dependency arc paths", () => {
    const layout = computeGraphLayout(mockCfg, 400);
    const pos0 = layout.positions.get(0)!;
    const pos2 = layout.positions.get(2)!;

    const dataPath = generateDataEdgePath(pos0, pos2);
    expect(dataPath).toContain("M ");
    expect(dataPath).toContain("C ");
  });
});
