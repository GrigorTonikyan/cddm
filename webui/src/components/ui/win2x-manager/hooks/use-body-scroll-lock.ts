import { useEffect } from "react";

let lockCount = 0;
let originalOverflow = "";

/**
 * Headless hook that locks scrolling on document.body with reference counting.
 */
export function useBodyScrollLock(isLocked: boolean): void {
  useEffect(() => {
    if (!isLocked || typeof document === "undefined") {
      return;
    }

    if (lockCount === 0) {
      originalOverflow = document.body.style.overflow;
      document.body.style.overflow = "hidden";
    }
    lockCount += 1;

    return () => {
      lockCount = Math.max(0, lockCount - 1);
      if (lockCount === 0) {
        document.body.style.overflow = originalOverflow;
      }
    };
  }, [isLocked]);
}
