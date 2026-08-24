/**
 * Pure mathematical geometry and coordinate engine for win2x-manager.
 * Zero DOM or React dependencies.
 */

import {
  ResizeDirection,
  SnapLayoutPreset,
  SnapZone,
  WIN2X_DEFAULTS,
  WIN2X_RESIZE_DIRECTIONS,
  WIN2X_SNAP_DEFAULTS,
  WIN2X_SNAP_LAYOUT_PRESETS,
  WIN2X_SNAP_ZONES,
} from "../constants/win2x-constants";
import { SnapLayoutDefinition, Win2xRect } from "./types";

/**
 * Clamps coordinates to keep the window title bar accessible within the viewport.
 */
export function clampToViewport(
  x: number,
  y: number,
  width: number,
  _height: number,
  viewportWidth: number,
  viewportHeight: number,
): { x: number; y: number } {
  const minX = -width + WIN2X_DEFAULTS.MIN_VISIBLE_X_OFFSET;
  const maxX = Math.max(0, viewportWidth - WIN2X_DEFAULTS.MIN_VISIBLE_X_OFFSET);
  const minY = 0;
  const maxY = Math.max(0, viewportHeight - WIN2X_DEFAULTS.MIN_VISIBLE_Y_OFFSET);

  return {
    x: Math.max(minX, Math.min(x, maxX)),
    y: Math.max(minY, Math.min(y, maxY)),
  };
}

/**
 * Centers a window of given dimensions within the viewport.
 */
export function centerInViewport(
  width: number,
  height: number,
  viewportWidth: number,
  viewportHeight: number,
): { x: number; y: number } {
  return {
    x: Math.max(WIN2X_DEFAULTS.CENTER_SAFETY_OFFSET_X, Math.round((viewportWidth - width) / 2)),
    y: Math.max(WIN2X_DEFAULTS.CENTER_SAFETY_OFFSET_Y, Math.round((viewportHeight - height) / 2)),
  };
}

/**
 * Constrains dimensions to minimum allowed size.
 */
export function constrainMinSize(
  width: number,
  height: number,
  minWidth: number = WIN2X_DEFAULTS.MIN_WIDTH,
  minHeight: number = WIN2X_DEFAULTS.MIN_HEIGHT,
): { width: number; height: number } {
  return {
    width: Math.max(minWidth, width),
    height: Math.max(minHeight, height),
  };
}

/**
 * Computes the next rectangular bounds during an 8-direction resize operation.
 */
export function computeResize(
  initialRect: Win2xRect,
  direction: ResizeDirection,
  deltaX: number,
  deltaY: number,
  minWidth: number = WIN2X_DEFAULTS.MIN_WIDTH,
  minHeight: number = WIN2X_DEFAULTS.MIN_HEIGHT,
): Win2xRect {
  let nextX = initialRect.x;
  let nextY = initialRect.y;
  let nextW = initialRect.width;
  let nextH = initialRect.height;

  if (
    direction === WIN2X_RESIZE_DIRECTIONS.RIGHT ||
    direction === WIN2X_RESIZE_DIRECTIONS.TOP_RIGHT ||
    direction === WIN2X_RESIZE_DIRECTIONS.BOTTOM_RIGHT
  ) {
    nextW = Math.max(minWidth, initialRect.width + deltaX);
  }
  if (
    direction === WIN2X_RESIZE_DIRECTIONS.BOTTOM ||
    direction === WIN2X_RESIZE_DIRECTIONS.BOTTOM_LEFT ||
    direction === WIN2X_RESIZE_DIRECTIONS.BOTTOM_RIGHT
  ) {
    nextH = Math.max(minHeight, initialRect.height + deltaY);
  }
  if (
    direction === WIN2X_RESIZE_DIRECTIONS.LEFT ||
    direction === WIN2X_RESIZE_DIRECTIONS.TOP_LEFT ||
    direction === WIN2X_RESIZE_DIRECTIONS.BOTTOM_LEFT
  ) {
    const rawW = initialRect.width - deltaX;
    const clampedW = Math.max(minWidth, rawW);
    nextX = initialRect.x + (initialRect.width - clampedW);
    nextW = clampedW;
  }
  if (
    direction === WIN2X_RESIZE_DIRECTIONS.TOP ||
    direction === WIN2X_RESIZE_DIRECTIONS.TOP_LEFT ||
    direction === WIN2X_RESIZE_DIRECTIONS.TOP_RIGHT
  ) {
    const rawH = initialRect.height - deltaY;
    const clampedH = Math.max(minHeight, rawH);
    nextY = initialRect.y + (initialRect.height - clampedH);
    nextH = clampedH;
  }

  return { x: nextX, y: nextY, width: nextW, height: nextH };
}

/**
 * Double-click handle expansion: expands window in the given direction
 * until touching the viewport edge or an adjacent window's edge.
 */
export function expandHandleToEdges(
  currentRect: Win2xRect,
  direction: ResizeDirection,
  viewportWidth: number,
  viewportHeight: number,
  otherWindows: Win2xRect[] = [],
): Win2xRect {
  let { x, y, width, height } = currentRect;

  const expandUp = () => {
    let topLimit = 0;
    for (const win of otherWindows) {
      const winBottom = win.y + win.height;
      const overlapsX = x < win.x + win.width && x + width > win.x;
      if (overlapsX && winBottom <= y && winBottom > topLimit) {
        topLimit = winBottom;
      }
    }
    const newY = topLimit;
    const newH = height + (y - newY);
    y = newY;
    height = newH;
  };

  const expandDown = () => {
    let bottomLimit = viewportHeight;
    for (const win of otherWindows) {
      const winTop = win.y;
      const overlapsX = x < win.x + win.width && x + width > win.x;
      if (overlapsX && winTop >= y + height && winTop < bottomLimit) {
        bottomLimit = winTop;
      }
    }
    height = Math.max(WIN2X_DEFAULTS.MIN_HEIGHT, bottomLimit - y);
  };

  const expandLeft = () => {
    let leftLimit = 0;
    for (const win of otherWindows) {
      const winRight = win.x + win.width;
      const overlapsY = y < win.y + win.height && y + height > win.y;
      if (overlapsY && winRight <= x && winRight > leftLimit) {
        leftLimit = winRight;
      }
    }
    const newX = leftLimit;
    const newW = width + (x - newX);
    x = newX;
    width = newW;
  };

  const expandRight = () => {
    let rightLimit = viewportWidth;
    for (const win of otherWindows) {
      const winLeft = win.x;
      const overlapsY = y < win.y + win.height && y + height > win.y;
      if (overlapsY && winLeft >= x + width && winLeft < rightLimit) {
        rightLimit = winLeft;
      }
    }
    width = Math.max(WIN2X_DEFAULTS.MIN_WIDTH, rightLimit - x);
  };

  switch (direction) {
    case WIN2X_RESIZE_DIRECTIONS.TOP:
      expandUp();
      break;
    case WIN2X_RESIZE_DIRECTIONS.BOTTOM:
      expandDown();
      break;
    case WIN2X_RESIZE_DIRECTIONS.LEFT:
      expandLeft();
      break;
    case WIN2X_RESIZE_DIRECTIONS.RIGHT:
      expandRight();
      break;
    case WIN2X_RESIZE_DIRECTIONS.TOP_LEFT:
      expandUp();
      expandLeft();
      break;
    case WIN2X_RESIZE_DIRECTIONS.TOP_RIGHT:
      expandUp();
      expandRight();
      break;
    case WIN2X_RESIZE_DIRECTIONS.BOTTOM_LEFT:
      expandDown();
      expandLeft();
      break;
    case WIN2X_RESIZE_DIRECTIONS.BOTTOM_RIGHT:
      expandDown();
      expandRight();
      break;
  }

  return { x, y, width, height };
}

/**
 * Magnetic window-to-window snapping: checks if moving window's edges
 * are within threshold of neighboring windows and snaps smoothly.
 */
export function detectWindowToWindowSnap(
  movingRect: Win2xRect,
  otherWindows: Win2xRect[],
  threshold = WIN2X_SNAP_DEFAULTS.MAGNETIC_SNAP_THRESHOLD_PX,
): { x: number; y: number; snappedX: boolean; snappedY: boolean } {
  let nextX = movingRect.x;
  let nextY = movingRect.y;
  let snappedX = false;
  let snappedY = false;

  for (const win of otherWindows) {
    // Snap X to neighbor's right edge (adjacent left)
    if (Math.abs(movingRect.x - (win.x + win.width)) <= threshold) {
      nextX = win.x + win.width;
      snappedX = true;
    }
    // Snap right edge to neighbor's left edge (adjacent right)
    else if (Math.abs(movingRect.x + movingRect.width - win.x) <= threshold) {
      nextX = win.x - movingRect.width;
      snappedX = true;
    }
    // Snap X alignment (left-left)
    else if (Math.abs(movingRect.x - win.x) <= threshold) {
      nextX = win.x;
      snappedX = true;
    }

    // Snap Y to neighbor's bottom edge (adjacent below)
    if (Math.abs(movingRect.y - (win.y + win.height)) <= threshold) {
      nextY = win.y + win.height;
      snappedY = true;
    }
    // Snap bottom edge to neighbor's top edge (adjacent above)
    else if (Math.abs(movingRect.y + movingRect.height - win.y) <= threshold) {
      nextY = win.y - movingRect.height;
      snappedY = true;
    }
    // Snap Y alignment (top-top)
    else if (Math.abs(movingRect.y - win.y) <= threshold) {
      nextY = win.y;
      snappedY = true;
    }
  }

  return { x: nextX, y: nextY, snappedX, snappedY };
}

/**
 * Screen edge snap zone detector.
 */
export function detectSnapZone(
  pointerX: number,
  pointerY: number,
  viewportWidth: number,
  viewportHeight: number,
  edgeThreshold = WIN2X_SNAP_DEFAULTS.EDGE_THRESHOLD_PX,
  cornerThreshold = WIN2X_SNAP_DEFAULTS.CORNER_THRESHOLD_PX,
): SnapZone {
  const isTop = pointerY <= edgeThreshold;
  const isLeft = pointerX <= edgeThreshold;
  const isRight = pointerX >= viewportWidth - edgeThreshold;

  const isCornerTop = pointerY <= cornerThreshold;
  const isCornerBottom = pointerY >= viewportHeight - cornerThreshold;
  const isCornerLeft = pointerX <= cornerThreshold;
  const isCornerRight = pointerX >= viewportWidth - cornerThreshold;

  if (isCornerTop && isCornerLeft) return WIN2X_SNAP_ZONES.TOP_LEFT;
  if (isCornerTop && isCornerRight) return WIN2X_SNAP_ZONES.TOP_RIGHT;
  if (isCornerBottom && isCornerLeft) return WIN2X_SNAP_ZONES.BOTTOM_LEFT;
  if (isCornerBottom && isCornerRight) return WIN2X_SNAP_ZONES.BOTTOM_RIGHT;

  if (isTop) return WIN2X_SNAP_ZONES.TOP_MAXIMIZE;
  if (isLeft) return WIN2X_SNAP_ZONES.LEFT_HALF;
  if (isRight) return WIN2X_SNAP_ZONES.RIGHT_HALF;

  return WIN2X_SNAP_ZONES.NONE;
}

/**
 * Computes standard edge-snapped bounding rectangle.
 */
export function computeSnapRect(
  snapZone: SnapZone,
  viewportWidth: number,
  viewportHeight: number,
): Win2xRect | null {
  if (snapZone === WIN2X_SNAP_ZONES.NONE) return null;

  const halfW = Math.round(viewportWidth / 2);
  const halfH = Math.round(viewportHeight / 2);

  switch (snapZone) {
    case WIN2X_SNAP_ZONES.TOP_MAXIMIZE:
      return { x: 0, y: 0, width: viewportWidth, height: viewportHeight };
    case WIN2X_SNAP_ZONES.LEFT_HALF:
      return { x: 0, y: 0, width: halfW, height: viewportHeight };
    case WIN2X_SNAP_ZONES.RIGHT_HALF:
      return { x: halfW, y: 0, width: viewportWidth - halfW, height: viewportHeight };
    case WIN2X_SNAP_ZONES.TOP_LEFT:
      return { x: 0, y: 0, width: halfW, height: halfH };
    case WIN2X_SNAP_ZONES.TOP_RIGHT:
      return { x: halfW, y: 0, width: viewportWidth - halfW, height: halfH };
    case WIN2X_SNAP_ZONES.BOTTOM_LEFT:
      return { x: 0, y: halfH, width: halfW, height: viewportHeight - halfH };
    case WIN2X_SNAP_ZONES.BOTTOM_RIGHT:
      return { x: halfW, y: halfH, width: viewportWidth - halfW, height: viewportHeight - halfH };
    default:
      return null;
  }
}

/**
 * Returns the 6 Windows 11 Snap Layout preset definitions.
 */
export function getSnapLayoutDefinitions(): SnapLayoutDefinition[] {
  return [
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.TWO_EQUAL,
      title: "50/50 Split",
      slots: [
        {
          index: 0,
          label: "Left Half",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw / 2), height: vh }),
        },
        {
          index: 1,
          label: "Right Half",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            return { x: hw, y: 0, width: vw - hw, height: vh };
          },
        },
      ],
    },
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.TWO_UNEQUAL,
      title: "70/30 Unequal Split",
      slots: [
        {
          index: 0,
          label: "Left Wide",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw * 0.67), height: vh }),
        },
        {
          index: 1,
          label: "Right Narrow",
          rect: (vw, vh) => {
            const lw = Math.round(vw * 0.67);
            return { x: lw, y: 0, width: vw - lw, height: vh };
          },
        },
      ],
    },
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.THREE_LEFT_MAIN,
      title: "3-Pane (Left Main)",
      slots: [
        {
          index: 0,
          label: "Left Main",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw / 2), height: vh }),
        },
        {
          index: 1,
          label: "Right Top",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            return { x: hw, y: 0, width: vw - hw, height: Math.round(vh / 2) };
          },
        },
        {
          index: 2,
          label: "Right Bottom",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            const hh = Math.round(vh / 2);
            return { x: hw, y: hh, width: vw - hw, height: vh - hh };
          },
        },
      ],
    },
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.THREE_RIGHT_MAIN,
      title: "3-Pane (Right Main)",
      slots: [
        {
          index: 0,
          label: "Left Top",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw / 2), height: Math.round(vh / 2) }),
        },
        {
          index: 1,
          label: "Left Bottom",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            const hh = Math.round(vh / 2);
            return { x: 0, y: hh, width: hw, height: vh - hh };
          },
        },
        {
          index: 2,
          label: "Right Main",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            return { x: hw, y: 0, width: vw - hw, height: vh };
          },
        },
      ],
    },
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.FOUR_GRID,
      title: "4-Pane Grid",
      slots: [
        {
          index: 0,
          label: "Top Left",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw / 2), height: Math.round(vh / 2) }),
        },
        {
          index: 1,
          label: "Top Right",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            return { x: hw, y: 0, width: vw - hw, height: Math.round(vh / 2) };
          },
        },
        {
          index: 2,
          label: "Bottom Left",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            const hh = Math.round(vh / 2);
            return { x: 0, y: hh, width: hw, height: vh - hh };
          },
        },
        {
          index: 3,
          label: "Bottom Right",
          rect: (vw, vh) => {
            const hw = Math.round(vw / 2);
            const hh = Math.round(vh / 2);
            return { x: hw, y: hh, width: vw - hw, height: vh - hh };
          },
        },
      ],
    },
    {
      preset: WIN2X_SNAP_LAYOUT_PRESETS.THREE_COLUMNS,
      title: "3-Column Split",
      slots: [
        {
          index: 0,
          label: "Left Column",
          rect: (vw, vh) => ({ x: 0, y: 0, width: Math.round(vw * 0.25), height: vh }),
        },
        {
          index: 1,
          label: "Center Column",
          rect: (vw, vh) => {
            const lw = Math.round(vw * 0.25);
            const cw = Math.round(vw * 0.5);
            return { x: lw, y: 0, width: cw, height: vh };
          },
        },
        {
          index: 2,
          label: "Right Column",
          rect: (vw, vh) => {
            const lw = Math.round(vw * 0.25);
            const cw = Math.round(vw * 0.5);
            return { x: lw + cw, y: 0, width: vw - (lw + cw), height: vh };
          },
        },
      ],
    },
  ];
}

/**
 * Computes exact rectangle for a specific snap layout slot.
 */
export function computeSnapLayoutSlotRect(
  preset: SnapLayoutPreset,
  slotIndex: number,
  viewportWidth: number,
  viewportHeight: number,
): Win2xRect | null {
  const defs = getSnapLayoutDefinitions();
  const found = defs.find((d) => d.preset === preset);
  if (!found) return null;
  const slot = found.slots.find((s) => s.index === slotIndex);
  if (!slot) return null;
  return slot.rect(viewportWidth, viewportHeight);
}

/**
 * Cascades windows with a given step offset across the viewport.
 */
export function calculateCascadePositions(
  windowIds: string[],
  viewportWidth: number,
  viewportHeight: number,
  stepOffset = WIN2X_DEFAULTS.CASCADE_STEP,
): Map<string, { x: number; y: number }> {
  const result = new Map<string, { x: number; y: number }>();
  let currentX = stepOffset;
  let currentY = stepOffset;

  for (const id of windowIds) {
    result.set(id, { x: currentX, y: currentY });
    currentX += stepOffset;
    currentY += stepOffset;

    if (currentX > viewportWidth / 2 || currentY > viewportHeight / 2) {
      currentX = stepOffset;
      currentY = stepOffset;
    }
  }
  return result;
}

/**
 * Computes grid tiled rectangular positions for all windows.
 */
export function calculateTileGridPositions(
  windowIds: string[],
  viewportWidth: number,
  viewportHeight: number,
): Map<string, Win2xRect> {
  const result = new Map<string, Win2xRect>();
  const count = windowIds.length;
  if (count === 0) return result;

  const cols = Math.ceil(Math.sqrt(count));
  const rows = Math.ceil(count / cols);

  const cellWidth = Math.floor(viewportWidth / cols);
  const cellHeight = Math.floor(viewportHeight / rows);

  windowIds.forEach((id, index) => {
    const col = index % cols;
    const row = Math.floor(index / cols);

    result.set(id, {
      x: col * cellWidth,
      y: row * cellHeight,
      width: cellWidth,
      height: cellHeight,
    });
  });

  return result;
}

/**
 * Computes horizontal or vertical split positions.
 */
export function calculateTileSplitPositions(
  windowIds: string[],
  viewportWidth: number,
  viewportHeight: number,
  orientation: "horizontal" | "vertical",
): Map<string, Win2xRect> {
  const result = new Map<string, Win2xRect>();
  const count = windowIds.length;
  if (count === 0) return result;

  if (orientation === "horizontal") {
    const cellWidth = Math.floor(viewportWidth / count);
    windowIds.forEach((id, index) => {
      result.set(id, {
        x: index * cellWidth,
        y: 0,
        width: cellWidth,
        height: viewportHeight,
      });
    });
  } else {
    const cellHeight = Math.floor(viewportHeight / count);
    windowIds.forEach((id, index) => {
      result.set(id, {
        x: 0,
        y: index * cellHeight,
        width: viewportWidth,
        height: cellHeight,
      });
    });
  }

  return result;
}
