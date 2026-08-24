import React from "react";
import styles from "./snap-ghost-guide.module.css";
import { SnapZone } from "../../core/types";
import { computeSnapRect } from "../../core/geometry-engine";
import {
  WIN2X_DATA_ATTRS,
  WIN2X_DEFAULTS,
  WIN2X_SNAP_ZONES,
  WIN2X_Z_INDEX,
} from "../../constants/win2x-constants";

export interface SnapGhostGuideProps {
  snapZone: SnapZone;
  viewportWidth?: number;
  viewportHeight?: number;
}

export const SnapGhostGuide: React.FC<SnapGhostGuideProps> = ({
  snapZone,
  viewportWidth = typeof window !== "undefined"
    ? window.innerWidth
    : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_WIDTH,
  viewportHeight = typeof window !== "undefined"
    ? window.innerHeight
    : WIN2X_DEFAULTS.FALLBACK_VIEWPORT_HEIGHT,
}) => {
  if (snapZone === WIN2X_SNAP_ZONES.NONE) return null;

  const rect = computeSnapRect(snapZone, viewportWidth, viewportHeight);
  if (!rect) return null;

  return (
    <div
      {...{ [WIN2X_DATA_ATTRS.SNAP_GHOST]: true }}
      className={styles.snapGhost}
      style={{
        transform: `translate3d(${rect.x}px, ${rect.y}px, 0)`,
        width: `${rect.width}px`,
        height: `${rect.height}px`,
        zIndex: WIN2X_Z_INDEX.SNAP_GHOST,
      }}
    />
  );
};
