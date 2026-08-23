import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { useResizable } from "../useResizable";

describe("useResizable Hook", () => {
  it("should initialize handleResizeMouseDown function", () => {
    const onResizeEnd = vi.fn();
    const { result } = renderHook(() =>
      useResizable({
        x: 100,
        y: 100,
        width: 600,
        height: 400,
        isMaximized: false,
        onResizeEnd,
      }),
    );

    expect(typeof result.current.handleResizeMouseDown).toBe("function");
  });

  it("should ignore resize mouse down if maximized or disabled", () => {
    const onResizeEnd = vi.fn();
    const { result } = renderHook(() =>
      useResizable({
        x: 100,
        y: 100,
        width: 600,
        height: 400,
        isMaximized: true,
        disabled: true,
        onResizeEnd,
      }),
    );

    const mockEvent = {
      button: 0,
      clientX: 700,
      clientY: 500,
      preventDefault: vi.fn(),
      stopPropagation: vi.fn(),
    } as unknown as React.MouseEvent;

    act(() => {
      result.current.handleResizeMouseDown("bottom-right", mockEvent);
    });

    expect(onResizeEnd).not.toHaveBeenCalled();
  });
});
