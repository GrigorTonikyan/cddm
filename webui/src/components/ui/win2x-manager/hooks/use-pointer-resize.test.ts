import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerResize } from "./use-pointer-resize";

describe("usePointerResize Hook (win2x-manager)", () => {
  const defaultOpts = {
    containerRef: { current: document.createElement("div") },
    x: 100,
    y: 100,
    width: 500,
    height: 400,
    isMaximized: false,
    onResizeEnd: vi.fn(),
  };

  it("initializes handleResizePointerDown function", () => {
    const { result } = renderHook(() => usePointerResize(defaultOpts));
    expect(typeof result.current.handleResizePointerDown).toBe("function");
    expect(result.current.isResizing).toBe(false);
  });

  it("does not resize when maximized or disabled", () => {
    const { result } = renderHook(() =>
      usePointerResize({ ...defaultOpts, isMaximized: true, disabled: true }),
    );

    const mockEvent = {
      button: 0,
      clientX: 500,
      clientY: 400,
      pointerId: 2,
      target: document.createElement("div"),
      currentTarget: document.createElement("div"),
      preventDefault: vi.fn(),
      stopPropagation: vi.fn(),
    } as unknown as React.PointerEvent<HTMLElement>;

    act(() => {
      result.current.handleResizePointerDown("bottom-right", mockEvent);
    });

    expect(defaultOpts.onResizeEnd).not.toHaveBeenCalled();
    expect(result.current.isResizing).toBe(false);
  });
});
