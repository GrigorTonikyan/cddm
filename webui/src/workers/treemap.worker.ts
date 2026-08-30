import { buildTreemapHierarchy, computeSquarifiedLayout } from "../utils/treemap-layout";
import { ClonePair, TreemapNode, TreemapRect } from "../types/cddm-types";

export interface LayoutWorkerRequest {
  id: number;
  type: "COMPUTE_LAYOUT";
  payload: {
    clonePairs: ClonePair[];
    width: number;
    height: number;
    currentPath: string;
  };
}

export interface LayoutWorkerSuccessResponse {
  id: number;
  type: "LAYOUT_SUCCESS";
  result: {
    fullHierarchy: TreemapNode;
    activeNode: TreemapNode;
    layoutRects: TreemapRect[];
  };
}

self.onmessage = (e: MessageEvent<LayoutWorkerRequest>) => {
  const { id, type, payload } = e.data;
  if (type === "COMPUTE_LAYOUT") {
    const { clonePairs, width, height, currentPath } = payload;
    const fullHierarchy = buildTreemapHierarchy(clonePairs);

    let activeNode = fullHierarchy;
    if (currentPath) {
      const segments = currentPath.split("/");
      let curr: TreemapNode | undefined = fullHierarchy;
      for (const seg of segments) {
        if (!curr?.children) break;
        curr = curr.children.find((c) => c.name === seg);
      }
      activeNode = curr || fullHierarchy;
    }

    const nodes = activeNode.children || [activeNode];
    const layoutRects = computeSquarifiedLayout(nodes, 0, 0, width, height);

    const response: LayoutWorkerSuccessResponse = {
      id,
      type: "LAYOUT_SUCCESS",
      result: { fullHierarchy, activeNode, layoutRects },
    };
    self.postMessage(response);
  }
};
