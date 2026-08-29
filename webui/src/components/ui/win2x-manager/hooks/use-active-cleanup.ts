import { useEffect, useRef, useState } from "react";

export function useActiveStateWithCleanup() {
  const [isActive, setIsActive] = useState(false);
  const cleanupRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    return () => {
      if (cleanupRef.current) {
        cleanupRef.current();
        cleanupRef.current = null;
      }
    };
  }, []);

  return { isActive, setIsActive, cleanupRef };
}
