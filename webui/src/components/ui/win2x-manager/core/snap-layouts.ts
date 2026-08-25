/**
 * Snap layout definitions and multi-window tiling layouts for win2x-manager.
 */

import {
  SnapLayoutPreset,
  WIN2X_DEFAULTS,
  WIN2X_SNAP_LAYOUT_PRESETS,
} from "../constants/win2x-constants";
import type { SnapLayoutDefinition, Win2xRect } from "./types";

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
