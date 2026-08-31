import { useState, useMemo, useCallback, useEffect } from "react";
import { ClonePair, TreemapNode, TreemapRect } from "../types/cddm-types";
import {
  computeTreemapLayoutSync,
  computeTreemapLayoutAsync,
  ComputeLayoutResult,
} from "../utils/worker-layout-client";

export interface BreadcrumbItem {
  name: string;
  path: string;
}

export interface UseTreemapLayoutOptions {
  clonePairs: ClonePair[];
  width?: number;
  height?: number;
}

export interface UseTreemapLayoutResult {
  fullHierarchy: TreemapNode;
  activeNode: TreemapNode;
  breadcrumbs: BreadcrumbItem[];
  layoutRects: TreemapRect[];
  currentPath: string;
  setCurrentPath: (path: string) => void;
  navigateTo: (path: string) => void;
  navigateUp: () => void;
  resetToRoot: () => void;
}

export const useTreemapLayout = ({
  clonePairs,
  width = 800,
  height = 360,
}: UseTreemapLayoutOptions): UseTreemapLayoutResult => {
  const [currentPath, setCurrentPath] = useState<string>("");
  const [asyncLayout, setAsyncLayout] = useState<ComputeLayoutResult | null>(null);

  // Synchronous immediate calculation for zero-flicker initial render
  const syncLayout = useMemo(() => {
    return computeTreemapLayoutSync({
      clonePairs,
      width,
      height,
      currentPath,
    });
  }, [clonePairs, width, height, currentPath]);

  // Offload heavy layout calculations to Web Worker when available in browser
  useEffect(() => {
    if (typeof window === "undefined" || typeof Worker === "undefined") {
      return;
    }
    let active = true;
    void computeTreemapLayoutAsync({
      clonePairs,
      width,
      height,
      currentPath,
    })
      .then((result) => {
        if (active && result) {
          setAsyncLayout(result);
        }
      })
      .catch(() => {});
    return () => {
      active = false;
    };
  }, [clonePairs, width, height, currentPath]);

  const activeResult = asyncLayout || syncLayout;
  const { fullHierarchy, activeNode, layoutRects } = activeResult;

  const breadcrumbs = useMemo(() => {
    if (!currentPath) return [{ name: "Root", path: "" }];
    const segments = currentPath.split("/");
    const crumbs: BreadcrumbItem[] = [{ name: "Root", path: "" }];
    let accum = "";
    for (const seg of segments) {
      accum = accum ? `${accum}/${seg}` : seg;
      crumbs.push({ name: seg, path: accum });
    }
    return crumbs;
  }, [currentPath]);

  const navigateTo = useCallback((path: string) => {
    setCurrentPath(path);
  }, []);

  const navigateUp = useCallback(() => {
    if (!currentPath) return;
    const segments = currentPath.split("/");
    segments.pop();
    setCurrentPath(segments.join("/"));
  }, [currentPath]);

  const resetToRoot = useCallback(() => {
    setCurrentPath("");
  }, []);

  return {
    fullHierarchy,
    activeNode,
    breadcrumbs,
    layoutRects,
    currentPath,
    setCurrentPath,
    navigateTo,
    navigateUp,
    resetToRoot,
  };
};
