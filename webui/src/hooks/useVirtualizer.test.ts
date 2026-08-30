import { renderHook } from "@testing-library/react";
import { describe, expect, it } from "vite-plus/test";
import { useVirtualizer } from "./useVirtualizer";

describe("useVirtualizer", () => {
  it("should return empty result for 0 count", () => {
    const { result } = renderHook(() =>
      useVirtualizer({
        count: 0,
        itemHeight: 50,
        containerHeight: 500,
        scrollTop: 0,
      }),
    );

    expect(result.current.virtualItems).toHaveLength(0);
    expect(result.current.totalHeight).toBe(0);
  });

  it("should calculate correct visible virtual items with overscan", () => {
    const { result } = renderHook(() =>
      useVirtualizer({
        count: 100,
        itemHeight: 50,
        containerHeight: 200,
        scrollTop: 200, // starting at index 4
        overscan: 2,
      }),
    );

    expect(result.current.totalHeight).toBe(5000);
    expect(result.current.startIndex).toBe(2);
    expect(result.current.endIndex).toBe(10);
    expect(result.current.virtualItems).toHaveLength(9);
    expect(result.current.virtualItems[0]?.index).toBe(2);
    expect(result.current.virtualItems[0]?.offsetTop).toBe(100);
  });

  it("should clamp endIndex at count - 1", () => {
    const { result } = renderHook(() =>
      useVirtualizer({
        count: 10,
        itemHeight: 50,
        containerHeight: 300,
        scrollTop: 400,
        overscan: 5,
      }),
    );

    expect(result.current.endIndex).toBe(9);
    expect(result.current.virtualItems[result.current.virtualItems.length - 1]?.index).toBe(9);
  });
});
