import { describe, it, expect, beforeEach } from "vite-plus/test";
import { renderHook } from "@testing-library/react";
import { useBodyScrollLock } from "../hooks/use-body-scroll-lock";

describe("useBodyScrollLock Hook (win2x-manager)", () => {
  beforeEach(() => {
    document.body.style.overflow = "";
  });

  it("locks document.body overflow when true", () => {
    const { unmount } = renderHook(() => useBodyScrollLock(true));
    expect(document.body.style.overflow).toBe("hidden");

    unmount();
    expect(document.body.style.overflow).toBe("");
  });

  it("does not lock document.body overflow when false", () => {
    renderHook(() => useBodyScrollLock(false));
    expect(document.body.style.overflow).toBe("");
  });

  it("handles multiple concurrent locks via reference counting", () => {
    const hook1 = renderHook(() => useBodyScrollLock(true));
    const hook2 = renderHook(() => useBodyScrollLock(true));

    expect(document.body.style.overflow).toBe("hidden");

    hook1.unmount();
    expect(document.body.style.overflow).toBe("hidden");

    hook2.unmount();
    expect(document.body.style.overflow).toBe("");
  });
});
