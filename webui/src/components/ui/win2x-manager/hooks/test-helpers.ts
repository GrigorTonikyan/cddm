import { vi } from "vite-plus/test";
import React from "react";

export function createDefaultHookOptions<T extends Record<string, unknown>>(
  extra: T,
): {
  containerRef: { current: HTMLDivElement };
  x: number;
  y: number;
  width: number;
  height: number;
  isMaximized: boolean;
} & T {
  return {
    containerRef: { current: document.createElement("div") },
    x: 100,
    y: 100,
    width: 500,
    height: 400,
    isMaximized: false,
    ...extra,
  };
}

export function createMockPointerEvent(
  overrides: Record<string, unknown> = {},
): React.PointerEvent<HTMLElement> {
  return {
    button: 0,
    clientX: 150,
    clientY: 150,
    pointerId: 1,
    target: document.createElement("div"),
    currentTarget: document.createElement("div"),
    preventDefault: vi.fn(),
    stopPropagation: vi.fn(),
    ...overrides,
  } as unknown as React.PointerEvent<HTMLElement>;
}
