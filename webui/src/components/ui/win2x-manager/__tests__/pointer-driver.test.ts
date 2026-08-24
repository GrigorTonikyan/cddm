import { describe, it, expect, vi, beforeEach, afterEach } from "vite-plus/test";
import { startPointerDrag, startPointerResize } from "../core/pointer-driver";

describe("Pointer Driver", () => {
  let captureElement: HTMLDivElement;
  let containerElement: HTMLDivElement;

  beforeEach(() => {
    captureElement = document.createElement("div");
    containerElement = document.createElement("div");
    document.body.appendChild(captureElement);
    document.body.appendChild(containerElement);
  });

  afterEach(() => {
    captureElement.remove();
    containerElement.remove();
  });

  it("initiates pointer drag session and commits on pointerup", () => {
    const onDragEnd = vi.fn();
    const onDragChange = vi.fn();

    const mockEvent = {
      clientX: 200,
      clientY: 150,
      pointerId: 1,
    } as unknown as PointerEvent;

    const cleanup = startPointerDrag(captureElement, mockEvent, {
      containerElement,
      initialX: 100,
      initialY: 80,
      width: 500,
      height: 400,
      onDragEnd,
      onDragChange,
    });

    expect(containerElement.getAttribute("data-moving")).toBe("true");

    // Simulate pointerup
    const upEvent = new Event("pointerup") as PointerEvent;
    captureElement.dispatchEvent(upEvent);

    expect(containerElement.getAttribute("data-moving")).toBeNull();
    expect(onDragEnd).toHaveBeenCalledTimes(1);

    cleanup();
  });

  it("initiates pointer resize session and commits on pointerup", () => {
    const onResizeEnd = vi.fn();

    const mockEvent = {
      clientX: 500,
      clientY: 400,
      pointerId: 2,
    } as unknown as PointerEvent;

    const cleanup = startPointerResize(captureElement, mockEvent, {
      containerElement,
      initialRect: { x: 100, y: 80, width: 500, height: 400 },
      direction: "bottom-right",
      onResizeEnd,
    });

    expect(containerElement.getAttribute("data-moving")).toBe("true");

    // Simulate pointerup
    const upEvent = new Event("pointerup") as PointerEvent;
    captureElement.dispatchEvent(upEvent);

    expect(containerElement.getAttribute("data-moving")).toBeNull();
    expect(onResizeEnd).toHaveBeenCalledTimes(1);

    cleanup();
  });
});
