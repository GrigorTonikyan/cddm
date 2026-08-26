import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerDrag } from "./use-pointer-drag";

describe("usePointerDrag Hook (win2x-manager)", () => {
  it("initializes handlePointerDown function", () => {
    const containerRef = { current: document.createElement("div") };
    const onDragEnd = vi.fn();

    const { result } = renderHook(() =>
      usePointerDrag({
        containerRef,
        x: 100,
        y: 100,
        width: 500,
        height: 400,
        isMaximized: false,
        onDragEnd,
      }),
    );

    expect(typeof result.current.handlePointerDown).toBe("function");
    expect(result.current.isDragging).toBe(false);
  });

  it("does not drag when maximized or disabled", () => {
    const containerRef = { current: document.createElement("div") };
    const onDragEnd = vi.fn();

    const { result } = renderHook(() =>
      usePointerDrag({
        containerRef,
        x: 100,
        y: 100,
        width: 500,
        height: 400,
        isMaximized: true,
        disabled: true,
        onDragEnd,
      }),
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

    expect(onDragEnd).not.toHaveBeenCalled();
    expect(result.current.isDragging).toBe(false);
  });
});
