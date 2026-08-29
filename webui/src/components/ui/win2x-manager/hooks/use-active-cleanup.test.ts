import { describe, expect, it, vi } from "vite-plus/test";
import { renderHook } from "@testing-library/react";
import { useActiveStateWithCleanup } from "./use-active-cleanup";

describe("useActiveStateWithCleanup", () => {
  it("initializes with false and cleans up on unmount", () => {
    const cleanupFn = vi.fn();
    const { result, unmount } = renderHook(() => useActiveStateWithCleanup());

    expect(result.current.isActive).toBe(false);
    result.current.cleanupRef.current = cleanupFn;

    unmount();
    expect(cleanupFn).toHaveBeenCalledTimes(1);
    expect(result.current.cleanupRef.current).toBeNull();
  });
});
