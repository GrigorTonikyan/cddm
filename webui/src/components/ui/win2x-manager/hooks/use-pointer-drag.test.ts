import { describe, it, expect, vi } from "vite-plus/test";
import { renderHook, act } from "@testing-library/react";
import { usePointerDrag } from "./use-pointer-drag";
import { createDefaultHookOptions, createMockPointerEvent } from "./test-helpers";

describe("usePointerDrag Hook (win2x-manager)", () => {
  const defaultOpts = createDefaultHookOptions({ onDragEnd: vi.fn() });

  it("initializes handlePointerDown function", () => {
    const { result } = renderHook(() => usePointerDrag(defaultOpts));
    expect(typeof result.current.handlePointerDown).toBe("function");
    expect(result.current.isDragging).toBe(false);
  });

  it("does not drag when maximized or disabled", () => {
    const { result } = renderHook(() =>
      usePointerDrag({ ...defaultOpts, isMaximized: true, disabled: true }),
    );

    const mockEvent = createMockPointerEvent();

    act(() => {
      result.current.handlePointerDown(mockEvent);
    });

    expect(defaultOpts.onDragEnd).not.toHaveBeenCalled();
    expect(result.current.isDragging).toBe(false);
  });
});
