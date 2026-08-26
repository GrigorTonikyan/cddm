import { describe, it, expect } from "vite-plus/test";
import {
  clampToViewport,
  centerInViewport,
  constrainMinSize,
  computeResize,
  expandHandleToEdges,
  detectWindowToWindowSnap,
  getSnapLayoutDefinitions,
  computeSnapLayoutSlotRect,
} from "./geometry-engine";

describe("Geometry Engine", () => {
  it("clamps coordinates to keep titlebar accessible", () => {
    // Left boundary: minX = -width + 100 = -820
    const clampedFarLeft = clampToViewport(-1000, 100, 920, 680, 1920, 1080);
    expect(clampedFarLeft.x).toBe(-820);

    // Top boundary: minY = 0
    const clampedTop = clampToViewport(100, -50, 920, 680, 1920, 1080);
    expect(clampedTop.y).toBe(0);

    // Right boundary: maxX = 1920 - 100 = 1820
    const clampedFarRight = clampToViewport(2000, 100, 920, 680, 1920, 1080);
    expect(clampedFarRight.x).toBe(1820);

    // Bottom boundary: maxY = 1080 - 50 = 1030
    const clampedBottom = clampToViewport(100, 1100, 920, 680, 1920, 1080);
    expect(clampedBottom.y).toBe(1030);
  });

  it("calculates centered position within viewport", () => {
    const centered = centerInViewport(920, 680, 1920, 1080);
    expect(centered.x).toBe(500); // (1920 - 920) / 2 = 500
    expect(centered.y).toBe(200); // (1080 - 680) / 2 = 200
  });

  it("constrains minimum dimensions", () => {
    const constrained = constrainMinSize(200, 100, 460, 340);
    expect(constrained.width).toBe(460);
    expect(constrained.height).toBe(340);

    const normal = constrainMinSize(800, 600, 460, 340);
    expect(normal.width).toBe(800);
    expect(normal.height).toBe(600);
  });

  it("computes 8-way resize bounds accurately", () => {
    const initialRect = { x: 100, y: 100, width: 500, height: 400 };

    // Bottom-right expansion
    const brRes = computeResize(initialRect, "bottom-right", 50, 60, 460, 340);
    expect(brRes.width).toBe(550);
    expect(brRes.height).toBe(460);
    expect(brRes.x).toBe(100);
    expect(brRes.y).toBe(100);

    // Top-left contraction past minWidth
    const tlRes = computeResize(initialRect, "top-left", 200, 200, 460, 340);
    expect(tlRes.width).toBe(460);
    expect(tlRes.height).toBe(340);
    expect(tlRes.x).toBe(140); // 100 + (500 - 460)
    expect(tlRes.y).toBe(160); // 100 + (400 - 340)

    // Right edge expansion
    const rRes = computeResize(initialRect, "right", 80, 0, 460, 340);
    expect(rRes.width).toBe(580);
    expect(rRes.height).toBe(400);

    // Bottom edge expansion
    const bRes = computeResize(initialRect, "bottom", 0, 70, 460, 340);
    expect(bRes.height).toBe(470);
  });

  it("expands handle to viewport boundary or neighbor window", () => {
    const current = { x: 200, y: 200, width: 400, height: 300 };
    const neighborRight = [{ x: 800, y: 100, width: 400, height: 400 }];

    // Expand right into neighbor's left edge (x: 800)
    const expandedRight = expandHandleToEdges(current, "right", 1920, 1080, neighborRight);
    expect(expandedRight.width).toBe(600); // 800 - 200 = 600

    // Expand top to screen top (y: 0)
    const expandedTop = expandHandleToEdges(current, "top", 1920, 1080, []);
    expect(expandedTop.y).toBe(0);
    expect(expandedTop.height).toBe(500); // 300 + 200 = 500

    // Expand bottom to screen bottom (y: 1080)
    const expandedBottom = expandHandleToEdges(current, "bottom", 1920, 1080, []);
    expect(expandedBottom.height).toBe(880); // 1080 - 200 = 880
  });

  it("detects magnetic window-to-window snapping within threshold", () => {
    const dragging = { x: 490, y: 100, width: 300, height: 200 };
    const otherWindows = [{ x: 800, y: 100, width: 300, height: 200 }];

    // dragging.right = 790, other.left = 800. Difference is 10px <= 16px threshold -> should snap right edge to 800 (x = 500)
    const snapped = detectWindowToWindowSnap(dragging, otherWindows, 16);
    expect(snapped.x).toBe(500);
  });

  it("computes accurate geometry for all 6 Windows 11 Snap Layout presets", () => {
    const defs = getSnapLayoutDefinitions();
    expect(defs.length).toBe(6);

    // 50/50 Split (two-equal)
    const slot50Left = computeSnapLayoutSlotRect("two-equal", 0, 1920, 1080);
    expect(slot50Left?.width).toBe(960);
    expect(slot50Left?.height).toBe(1080);
    expect(slot50Left?.x).toBe(0);

    const slot50Right = computeSnapLayoutSlotRect("two-equal", 1, 1920, 1080);
    expect(slot50Right?.width).toBe(960);
    expect(slot50Right?.x).toBe(960);

    // 70/30 Unequal Split (two-unequal)
    const slot70Left = computeSnapLayoutSlotRect("two-unequal", 0, 1920, 1080);
    expect(slot70Left?.width).toBe(1286); // ~67% of 1920 = 1286

    // 4-Grid (four-grid)
    const gridTopRight = computeSnapLayoutSlotRect("four-grid", 1, 1920, 1080);
    expect(gridTopRight?.x).toBe(960);
    expect(gridTopRight?.y).toBe(0);
    expect(gridTopRight?.width).toBe(960);
    expect(gridTopRight?.height).toBe(540);
  });
});
