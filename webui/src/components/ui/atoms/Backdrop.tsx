import React from "react";

export interface BackdropProps {
  isOpen: boolean;
  onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
  children?: React.ReactNode;
  className?: string;
  transparent?: boolean;
}

/**
 * Universal atomic backdrop overlay with acrylic blur and click-to-dismiss support.
 */
export const Backdrop: React.FC<BackdropProps> = ({
  isOpen,
  onClick,
  children,
  className = "",
  transparent = false,
}) => {
  if (!isOpen) return null;

  return (
    <div
      onClick={onClick}
      className={`fixed inset-0 z-[9990] transition-opacity duration-200 ${
        transparent
          ? "bg-transparent pointer-events-none"
          : "bg-slate-950/80 backdrop-blur-sm pointer-events-auto"
      } ${className}`}
    >
      {children}
    </div>
  );
};
