import { useMemo } from "react";

export interface UseVirtualizerOptions {
  count: number;
  itemHeight: number;
  containerHeight: number;
  scrollTop: number;
  overscan?: number;
}

export interface VirtualItem {
  index: number;
  offsetTop: number;
  height: number;
}

export interface VirtualizerResult {
  virtualItems: VirtualItem[];
  totalHeight: number;
  startIndex: number;
  endIndex: number;
}

/**
 * Lightweight pure virtualizer hook for rendering high-throughput lists in CDDM WebUI Studio.
 * Computes visible start/end indices and item offsets with configurable overscan.
 */
export function useVirtualizer({
  count,
  itemHeight,
  containerHeight,
  scrollTop,
  overscan = 3,
}: UseVirtualizerOptions): VirtualizerResult {
  return useMemo(() => {
    if (count === 0 || itemHeight <= 0 || containerHeight <= 0) {
      return {
        virtualItems: [],
        totalHeight: 0,
        startIndex: 0,
        endIndex: 0,
      };
    }

    const totalHeight = count * itemHeight;
    const rawStartIndex = Math.floor(scrollTop / itemHeight);
    const visibleCount = Math.ceil(containerHeight / itemHeight);

    const startIndex = Math.max(0, rawStartIndex - overscan);
    const endIndex = Math.min(count - 1, rawStartIndex + visibleCount + overscan);

    const virtualItems: VirtualItem[] = [];
    for (let i = startIndex; i <= endIndex; i++) {
      virtualItems.push({
        index: i,
        offsetTop: i * itemHeight,
        height: itemHeight,
      });
    }

    return {
      virtualItems,
      totalHeight,
      startIndex,
      endIndex,
    };
  }, [count, itemHeight, containerHeight, scrollTop, overscan]);
}
