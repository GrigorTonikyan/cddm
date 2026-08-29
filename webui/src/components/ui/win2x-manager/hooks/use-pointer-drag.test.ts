import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerDrag } from "./use-pointer-drag";

describe("usePointerDrag Hook (win2x-manager)", () => {
  const defaultOpts = {
    containerRef: { current: document.createElement("div") },
    x: 100,
    y: 100,
    width: 500,
    height: 400,
    isMaximized: false,
    onDragEnd: vi.fn(),
  };

  it("initializes handlePointerDown function", () => {
    const { result } = renderHook(() => usePointerDrag(defaultOpts));
    expect(typeof result.current.handlePointerDown).toBe("function");
    expect(result.current.isDragging).toBe(false);
  });

  it("does not drag when maximized or disabled", () => {
    const { result } = renderHook(() =>
      usePointerDrag({ ...defaultOpts, isMaximized: true, disabled: true }),
    );

    const mockEvent = {
      button: 0,
      clientX: 150,
      clientY: 150,
      pointerId: 1,
      target: document.createElement("div"),
      currentTarget: document.createElement("div"),
    } as unknown as React.PointerEvent<HTMLElement>;

    act(() => {
      result.current.handlePointerDown(mockEvent);
    });

    expect(defaultOpts.onDragEnd).not.toHaveBeenCalled();
    expect(result.current.isDragging).toBe(false);
  });
});
