import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerResize } from "../hooks/use-pointer-resize";

describe("usePointerResize Hook (win2x-manager)", () => {
  it("initializes handleResizePointerDown function", () => {
    const containerRef = { current: document.createElement("div") };
    const onResizeEnd = vi.fn();

    const { result } = renderHook(() =>
      usePointerResize({
        containerRef,
        x: 100,
        y: 100,
        width: 500,
        height: 400,
        isMaximized: false,
        onResizeEnd,
      }),
    );

    expect(typeof result.current.handleResizePointerDown).toBe("function");
    expect(result.current.isResizing).toBe(false);
  });

  it("does not resize when maximized or disabled", () => {
    const containerRef = { current: document.createElement("div") };
    const onResizeEnd = vi.fn();

    const { result } = renderHook(() =>
      usePointerResize({
        containerRef,
        x: 100,
        y: 100,
        width: 500,
        height: 400,
        isMaximized: true,
        disabled: true,
        onResizeEnd,
      }),
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

    expect(onResizeEnd).not.toHaveBeenCalled();
    expect(result.current.isResizing).toBe(false);
  });
});
