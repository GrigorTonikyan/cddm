import React, { useEffect, useState } from "react";
import { createPortal } from "react-dom";

export interface PortalProps {
  children: React.ReactNode;
  container?: Element | DocumentFragment;
}

/**
 * Universal atomic portal mounting components to document.body.
 */
export const Portal: React.FC<PortalProps> = ({ children, container }) => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    return () => setMounted(false);
  }, []);

  if (!mounted) return null;

  const target = container || (typeof document !== "undefined" ? document.body : null);
  if (!target) return null;

  return createPortal(children, target);
};
