import type { CfgNode, ControlFlowGraph } from "../types/cddm-types";

export interface NodePosition {
  id: number;
  x: number;
  y: number;
  node: CfgNode;
}

export interface GraphLayoutResult {
  width: number;
  height: number;
  positions: Map<number, NodePosition>;
}

/**
 * Computes a layered topological layout for Control Flow Graph nodes inside an SVG canvas.
 */
export function computeGraphLayout(
  cfg: ControlFlowGraph,
  containerWidth: number = 380,
): GraphLayoutResult {
  const positions = new Map<number, NodePosition>();
  const nodes = cfg.nodes;
  if (nodes.length === 0) {
    return { width: containerWidth, height: 200, positions };
  }

  // Calculate in-degree to find entry and layer levels
  const inDegree = new Map<number, number>();
  const adj = new Map<number, number[]>();

  for (const node of nodes) {
    inDegree.set(node.id, 0);
    adj.set(node.id, []);
  }

  for (const edge of cfg.edges) {
    const curIn = inDegree.get(edge.to) ?? 0;
    inDegree.set(edge.to, curIn + 1);
    const neighbors = adj.get(edge.from) ?? [];
    neighbors.push(edge.to);
    adj.set(edge.from, neighbors);
  }

  // Assign layers using BFS from roots (or nodes with in-degree 0)
  const nodeLayer = new Map<number, number>();
  const queue: number[] = [];

  for (const node of nodes) {
    if ((inDegree.get(node.id) ?? 0) === 0 || node.node_type === "Entry") {
      nodeLayer.set(node.id, 0);
      queue.push(node.id);
    }
  }

  // Fallback if graph is cyclic with no 0-in-degree node
  if (queue.length === 0 && nodes.length > 0 && nodes[0]) {
    nodeLayer.set(nodes[0].id, 0);
    queue.push(nodes[0].id);
  }

  const visited = new Set<number>();
  while (queue.length > 0) {
    const curr = queue.shift()!;
    if (visited.has(curr)) continue;
    visited.add(curr);

    const currLayer = nodeLayer.get(curr) ?? 0;
    const neighbors = adj.get(curr) ?? [];
    for (const next of neighbors) {
      const existingLayer = nodeLayer.get(next) ?? 0;
      nodeLayer.set(next, Math.max(existingLayer, currLayer + 1));
      if (!visited.has(next)) {
        queue.push(next);
      }
    }
  }

  // Group nodes by layer
  const layers = new Map<number, number[]>();
  let maxLayer = 0;

  for (const node of nodes) {
    const layer = nodeLayer.get(node.id) ?? 0;
    maxLayer = Math.max(maxLayer, layer);
    const list = layers.get(layer) ?? [];
    list.push(node.id);
    layers.set(layer, list);
  }

  const layerSpacingY = 70;
  const paddingTop = 40;
  const paddingBottom = 40;
  const totalHeight = Math.max(260, (maxLayer + 1) * layerSpacingY + paddingTop + paddingBottom);

  for (let layer = 0; layer <= maxLayer; layer++) {
    const layerNodes = layers.get(layer) ?? [];
    const count = layerNodes.length;
    const y = paddingTop + layer * layerSpacingY;

    for (let i = 0; i < count; i++) {
      const nodeId = layerNodes[i];
      if (nodeId === undefined) continue;
      const node = nodes.find((n) => n.id === nodeId);
      if (!node) continue;

      const spacingX = containerWidth / (count + 1);
      const x = spacingX * (i + 1);

      positions.set(nodeId, {
        id: nodeId,
        x,
        y,
        node,
      });
    }
  }

  return {
    width: containerWidth,
    height: totalHeight,
    positions,
  };
}

/**
 * Generates an SVG path string for a directional CFG edge.
 */
export function generateEdgePath(
  fromPos: NodePosition,
  toPos: NodePosition,
  isLoopBack: boolean = false,
): string {
  const fx = fromPos.x;
  const fy = fromPos.y;
  const tx = toPos.x;
  const ty = toPos.y;

  if (isLoopBack) {
    // Loopback curve around the left side
    const curveOffset = Math.min(fx, tx) - 50;
    return `M ${fx} ${fy} C ${curveOffset} ${fy}, ${curveOffset} ${ty}, ${tx} ${ty}`;
  }

  if (Math.abs(fx - tx) < 5) {
    // Straight vertical line
    return `M ${fx} ${fy + 18} L ${tx} ${ty - 18}`;
  }

  // Smooth cubic bezier S-curve
  const midY = (fy + ty) / 2;
  return `M ${fx} ${fy + 18} C ${fx} ${midY}, ${tx} ${midY}, ${tx} ${ty - 18}`;
}

/**
 * Generates an SVG path string for a PDG data dependency edge (dashed curve).
 */
export function generateDataEdgePath(fromPos: NodePosition, toPos: NodePosition): string {
  const fx = fromPos.x;
  const fy = fromPos.y;
  const tx = toPos.x;
  const ty = toPos.y;

  // Arc around right side
  const offset = Math.max(fx, tx) + 40;
  return `M ${fx + 40} ${fy} C ${offset} ${fy}, ${offset} ${ty}, ${tx + 40} ${ty}`;
}
