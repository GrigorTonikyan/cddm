import { describe, it, expect, beforeEach } from "vite-plus/test";
import { renderHook } from "@testing-library/react";
import { useBodyScrollLock } from "../useBodyScrollLock";

describe("useBodyScrollLock Hook", () => {
  beforeEach(() => {
    document.body.style.overflow = "";
  });

  it("should lock body scroll when active and restore on unmount", () => {
    expect(document.body.style.overflow).toBe("");

    const { unmount } = renderHook(() => useBodyScrollLock(true));
    expect(document.body.style.overflow).toBe("hidden");

    unmount();
    expect(document.body.style.overflow).toBe("");
  });

  it("should not lock body scroll when disabled", () => {
    renderHook(() => useBodyScrollLock(false));
    expect(document.body.style.overflow).toBe("");
  });

  it("should handle nested lock calls with reference counting", () => {
    const { unmount: unmountA } = renderHook(() => useBodyScrollLock(true));
    const { unmount: unmountB } = renderHook(() => useBodyScrollLock(true));
    expect(document.body.style.overflow).toBe("hidden");

    unmountA();
    expect(document.body.style.overflow).toBe("hidden");

    unmountB();
    expect(document.body.style.overflow).toBe("");
  });
});
