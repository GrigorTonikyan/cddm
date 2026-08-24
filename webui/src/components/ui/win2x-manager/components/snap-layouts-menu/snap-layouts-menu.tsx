import React, { useState } from "react";
import { SnapLayoutPreset, WIN2X_DATA_ATTRS } from "../../constants/win2x-constants";
import { getSnapLayoutDefinitions } from "../../core/geometry-engine";
import styles from "./snap-layouts-menu.module.css";

export interface SnapLayoutsMenuProps {
  isOpen: boolean;
  onSelect: (preset: SnapLayoutPreset, slotIndex: number) => void;
  onClose?: () => void;
  className?: string;
}

export const SnapLayoutsMenu: React.FC<SnapLayoutsMenuProps> = ({
  isOpen,
  onSelect,
  className = "",
}) => {
  const [hoveredSlot, setHoveredSlot] = useState<{
    preset: SnapLayoutPreset;
    slotIndex: number;
  } | null>(null);

  if (!isOpen) return null;

  const definitions = getSnapLayoutDefinitions();

  return (
    <div
      className={`${styles.menuContainer || ""} ${className}`.trim()}
      {...{ [WIN2X_DATA_ATTRS.SNAP_LAYOUTS_MENU]: true }}
      onClick={(e) => e.stopPropagation()}
    >
      <div className={styles.grid}>
        {definitions.map((def) => (
          <div key={def.preset} className={styles.layoutCard} title={def.title}>
            <div className={styles.slotsWrapper}>
              {def.slots.map((slot) => {
                const isHovered =
                  hoveredSlot?.preset === def.preset && hoveredSlot?.slotIndex === slot.index;

                // Preview coordinates calculated relative to 100x64 preview box
                const previewRect = slot.rect(100, 64);

                const slotStyle: React.CSSProperties = {
                  left: `${previewRect.x}%`,
                  top: `${previewRect.y}%`,
                  width: `${previewRect.width}%`,
                  height: `${previewRect.height}%`,
                };

                return (
                  <button
                    key={`${def.preset}-${slot.index}`}
                    type="button"
                    style={slotStyle}
                    className={`${styles.slotButton} ${isHovered ? styles.slotHovered : ""}`}
                    onMouseEnter={() =>
                      setHoveredSlot({ preset: def.preset, slotIndex: slot.index })
                    }
                    onMouseLeave={() => setHoveredSlot(null)}
                    onClick={() => {
                      onSelect(def.preset, slot.index);
                    }}
                    aria-label={`${def.title} - ${slot.label}`}
                    title={`${def.title} - ${slot.label}`}
                  >
                    <span className={styles.slotIndicator} />
                  </button>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
