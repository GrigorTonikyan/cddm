import React from "react";
import { createPortal } from "react-dom";

export interface PortalProps {
  children: React.ReactNode;
  container?: Element | DocumentFragment;
}

/**
 * Universal atomic portal component for mounting UI outside the current DOM hierarchy.
 */
export const Portal: React.FC<PortalProps> = ({ children, container }) => {
  if (typeof document === "undefined") return null;
  const target = container || document.body;
  return createPortal(children, target);
};
