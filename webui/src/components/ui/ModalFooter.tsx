import React from "react";

export interface ModalFooterProps {
  infoIcon?: React.ReactNode;
  infoText: string;
  onClose: () => void;
  closeText?: string;
  actionButton?: React.ReactNode;
}

export const ModalFooter: React.FC<ModalFooterProps> = ({
  infoIcon,
  infoText,
  onClose,
  closeText = "Close",
  actionButton,
}) => {
  return (
    <>
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        {infoIcon}
        <span>{infoText}</span>
      </div>
      <div className="flex items-center gap-2">
        {actionButton}
        <button
          type="button"
          onClick={onClose}
          className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
        >
          {closeText}
        </button>
      </div>
    </>
  );
};
