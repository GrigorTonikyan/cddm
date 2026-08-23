import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { useDraggable } from "../useDraggable";

describe("useDraggable Hook", () => {
  it("should initialize handleMouseDown function", () => {
    const onDragEnd = vi.fn();
    const { result } = renderHook(() =>
      useDraggable({
        x: 100,
        y: 100,
        width: 500,
        height: 400,
        isMaximized: false,
        onDragEnd,
      }),
    );

    expect(typeof result.current.handleMouseDown).toBe("function");
  });

  it("should not drag when maximized or disabled", () => {
    const onDragEnd = vi.fn();
    const { result } = renderHook(() =>
      useDraggable({
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
      target: document.createElement("div"),
    } as unknown as React.MouseEvent;

    act(() => {
      result.current.handleMouseDown(mockEvent);
    });

    expect(onDragEnd).not.toHaveBeenCalled();
  });
});
