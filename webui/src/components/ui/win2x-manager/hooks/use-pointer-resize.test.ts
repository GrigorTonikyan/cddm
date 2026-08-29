import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerResize } from "./use-pointer-resize";
import { createDefaultHookOptions, createMockPointerEvent } from "./test-helpers";

describe("usePointerResize Hook (win2x-manager)", () => {
  const defaultOpts = createDefaultHookOptions({ onResizeEnd: vi.fn() });

  it("initializes handleResizePointerDown function", () => {
    const { result } = renderHook(() => usePointerResize(defaultOpts));
    expect(typeof result.current.handleResizePointerDown).toBe("function");
    expect(result.current.isResizing).toBe(false);
  });

  it("does not resize when maximized or disabled", () => {
    const { result } = renderHook(() =>
      usePointerResize({ ...defaultOpts, isMaximized: true, disabled: true }),
    );

    const mockEvent = createMockPointerEvent({ clientX: 500, clientY: 400, pointerId: 2 });

    act(() => {
      result.current.handleResizePointerDown("bottom-right", mockEvent);
    });

    expect(defaultOpts.onResizeEnd).not.toHaveBeenCalled();
    expect(result.current.isResizing).toBe(false);
  });
});
