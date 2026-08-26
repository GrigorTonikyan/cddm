import { describe, it, expect } from "vite-plus/test";
import { computeGraphLayout, generateEdgePath, generateDataEdgePath } from "./graph-layout";
import type { ControlFlowGraph } from "./../types/cddm-types";

describe("graph-layout utility", () => {
  const mockCfg: ControlFlowGraph = {
    file_path: "src/calc.rs",
    function_name: "test_fn",
    line_start: 1,
    line_end: 10,
    nodes: [
      {
        id: 0,
        node_type: "Entry",
        label: "entry",
        statement_count: 1,
        line_start: 1,
        line_end: 1,
      },
      {
        id: 1,
        node_type: "Branch",
        label: "if x > 0",
        statement_count: 1,
        line_start: 2,
        line_end: 2,
      },
      {
        id: 2,
        node_type: "Return",
        label: "return x",
        statement_count: 1,
        line_start: 3,
        line_end: 3,
      },
    ],
    edges: [
      { from: 0, to: 1, edge_type: "Sequential" },
      { from: 1, to: 2, edge_type: "TrueBranch" },
    ],
    wl_hash: 123456789,
  };

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
