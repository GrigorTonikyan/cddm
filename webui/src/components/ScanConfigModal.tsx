import React from "react";
import { ScanConfigPanel } from "./ScanConfigPanel";
import { Win2xWindow } from "./ui/win2x-manager";
import { Sliders, Zap } from "lucide-react";

export interface ScanConfigModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const ScanConfigModal: React.FC<ScanConfigModalProps> = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  const footerContent = (
    <>
      <div className="flex items-center gap-2 text-xs font-mono text-slate-400">
        <Zap className="w-3.5 h-3.5 text-indigo-400" />
        <span>Winnowing M61 Token Algorithm</span>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </>
  );

  return (
    <Win2xWindow
      id="cddm-scan-config-window"
      windowType="scan-config"
      isOpen={isOpen}
      onClose={onClose}
      title="Scan Parameters & Engine Configuration"
      subtitle="Fine-tune token thresholds, similarity cutoff, ignore patterns, and cache"
      badge="M61 Engine"
      icon={<Sliders className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={860}
      initialHeight={620}
    >
      <div className="space-y-4">
        <ScanConfigPanel />
      </div>
    </Win2xWindow>
  );
};
