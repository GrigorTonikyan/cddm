/**
 * Pure mathematical geometry and coordinate engine for win2x-manager.
 * Zero DOM or React dependencies.
 */

import {
  ResizeDirection,
  SnapZone,
  WIN2X_DEFAULTS,
  WIN2X_RESIZE_DIRECTIONS,
  WIN2X_SNAP_DEFAULTS,
  WIN2X_SNAP_ZONES,
} from "../constants/win2x-constants";
import type { Win2xRect } from "./types";

export * from "./snap-layouts";

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
function snap1D(
  pos: number,
  size: number,
  otherPos: number,
  otherSize: number,
  threshold: number,
): { pos: number; snapped: boolean } {
  if (Math.abs(pos - (otherPos + otherSize)) <= threshold) {
    return { pos: otherPos + otherSize, snapped: true };
  }
  if (Math.abs(pos + size - otherPos) <= threshold) {
    return { pos: otherPos - size, snapped: true };
  }
  if (Math.abs(pos - otherPos) <= threshold) {
    return { pos: otherPos, snapped: true };
  }
  return { pos, snapped: false };
}

export function snapToNeighborWindows(
  movingRect: Win2xRect,
  otherWindows: Win2xRect[],
  threshold = WIN2X_SNAP_DEFAULTS.MAGNETIC_SNAP_THRESHOLD_PX,
): { x: number; y: number; snappedX: boolean; snappedY: boolean } {
  let nextX = movingRect.x;
  let nextY = movingRect.y;
  let snappedX = false;
  let snappedY = false;

  for (const win of otherWindows) {
    const sX = snap1D(nextX, movingRect.width, win.x, win.width, threshold);
    if (sX.snapped) {
      nextX = sX.pos;
      snappedX = true;
    }

    const sY = snap1D(nextY, movingRect.height, win.y, win.height, threshold);
    if (sY.snapped) {
      nextY = sY.pos;
      snappedY = true;
    }
  }

  return { x: nextX, y: nextY, snappedX, snappedY };
}

export const detectWindowToWindowSnap = snapToNeighborWindows;

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
