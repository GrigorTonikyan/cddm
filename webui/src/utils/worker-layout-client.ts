import { ClonePair, TreemapNode, TreemapRect } from "../types/cddm-types";
import { buildTreemapHierarchy, computeSquarifiedLayout } from "./treemap-layout";

export interface ComputeLayoutOptions {
  clonePairs: ClonePair[];
  width: number;
  height: number;
  currentPath: string;
}

export interface ComputeLayoutResult {
  fullHierarchy: TreemapNode;
  activeNode: TreemapNode;
  layoutRects: TreemapRect[];
}

/**
 * Synchronous layout computation for fallback, testing, and SSR environments.
 */
export function computeTreemapLayoutSync(options: ComputeLayoutOptions): ComputeLayoutResult {
  const { clonePairs, width, height, currentPath } = options;
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

  return { fullHierarchy, activeNode, layoutRects };
}

let workerInstance: Worker | null = null;
let requestId = 0;
const pendingRequests = new Map<number, (res: ComputeLayoutResult) => void>();

function getWorker(): Worker | null {
  if (typeof window === "undefined" || typeof Worker === "undefined") {
    return null;
  }
  if (!workerInstance) {
    try {
      workerInstance = new Worker(new URL("../workers/treemap.worker.ts", import.meta.url), {
        type: "module",
      });
      workerInstance.onmessage = (e: MessageEvent) => {
        const { id, type, result } = e.data;
        if (type === "LAYOUT_SUCCESS" && pendingRequests.has(id)) {
          const resolver = pendingRequests.get(id);
          pendingRequests.delete(id);
          resolver?.(result);
        }
      };
      workerInstance.onerror = () => {
        workerInstance = null;
      };
    } catch {
      workerInstance = null;
    }
  }
  return workerInstance;
}

/**
 * Computes treemap hierarchy and squarified layout rectangles asynchronously via Web Worker when available,
 * gracefully falling back to synchronous computation in non-worker environments.
 */
export async function computeTreemapLayoutAsync(
  options: ComputeLayoutOptions,
): Promise<ComputeLayoutResult> {
  const worker = getWorker();
  if (!worker) {
    return computeTreemapLayoutSync(options);
  }

  return new Promise((resolve) => {
    const id = ++requestId;
    pendingRequests.set(id, resolve);
    worker.postMessage({
      id,
      type: "COMPUTE_LAYOUT",
      payload: options,
    });
  });
}

/**
 * Terminates the background worker instance if active.
 */
export function terminateLayoutWorker(): void {
  if (workerInstance) {
    workerInstance.terminate();
    workerInstance = null;
    pendingRequests.clear();
  }
}
