import React from "react";
import { SnapAssistSession, WindowRegistration } from "../../core/types";
import { computeSnapLayoutSlotRect, getSnapLayoutDefinitions } from "../../core/geometry-engine";
import { WIN2X_DATA_ATTRS } from "../../constants/win2x-constants";
import { X, LayoutTemplate } from "lucide-react";
import styles from "./snap-assist-modal.module.css";

export interface SnapAssistModalProps {
  session: SnapAssistSession;
  candidateWindows: WindowRegistration[];
  onSelectWindow: (windowId: string) => void;
  onDismiss: () => void;
}

export const SnapAssistModal: React.FC<SnapAssistModalProps> = ({
  session,
  candidateWindows,
  onSelectWindow,
  onDismiss,
}) => {
  const viewportW = typeof window !== "undefined" ? window.innerWidth : 1920;
  const viewportH = typeof window !== "undefined" ? window.innerHeight : 1080;

  const targetRect = computeSnapLayoutSlotRect(
    session.preset,
    session.activeSlotIndex,
    viewportW,
    viewportH,
  );

  if (!targetRect || candidateWindows.length === 0) return null;

  const def = getSnapLayoutDefinitions().find((d) => d.preset === session.preset);
  const slotLabel =
    def?.slots.find((s) => s.index === session.activeSlotIndex)?.label || "Next Slot";

  const containerStyle: React.CSSProperties = {
    left: `${targetRect.x}px`,
    top: `${targetRect.y}px`,
    width: `${targetRect.width}px`,
    height: `${targetRect.height}px`,
  };

  return (
    <div
      style={containerStyle}
      className={styles.snapAssistZone}
      {...{ [WIN2X_DATA_ATTRS.SNAP_ASSIST]: true }}
    >
      <div className={styles.header}>
        <div className={styles.titleArea}>
          <LayoutTemplate className="w-4 h-4 text-indigo-400 mr-2" />
          <span className={styles.title}>Snap Assist: Select window for {slotLabel}</span>
        </div>
        <button
          type="button"
          onClick={onDismiss}
          className={styles.dismissBtn}
          aria-label="Dismiss Snap Assist"
          title="Dismiss"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

      <div className={styles.grid}>
        {candidateWindows.map((win) => (
          <button
            key={win.id}
            type="button"
            className={styles.candidateCard}
            onClick={() => onSelectWindow(win.id)}
          >
            <div className={styles.cardIconWrapper}>{win.icon}</div>
            <div className={styles.cardInfo}>
              <span className={styles.cardTitle}>{win.title}</span>
              {win.subtitle && <span className={styles.cardSubtitle}>{win.subtitle}</span>}
            </div>
            {win.badge && <span className={styles.cardBadge}>{win.badge}</span>}
          </button>
        ))}
      </div>
    </div>
  );
};
