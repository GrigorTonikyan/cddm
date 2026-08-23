import { useEffect } from "react";

let activeModalCount = 0;
let originalOverflowStyle = "";

/**
 * Headless atomic hook to lock background scrolling while a modal/window is open.
 * Uses reference counting so multiple simultaneous modals don't interfere with restoration.
 */
export function useBodyScrollLock(isLocked: boolean): void {
  useEffect(() => {
    if (!isLocked || typeof document === "undefined") return;

    if (activeModalCount === 0) {
      originalOverflowStyle = document.body.style.overflow;
      document.body.style.overflow = "hidden";
    }
    activeModalCount++;

    return () => {
      activeModalCount = Math.max(0, activeModalCount - 1);
      if (activeModalCount === 0) {
        document.body.style.overflow = originalOverflowStyle;
      }
    };
  }, [isLocked]);
}
