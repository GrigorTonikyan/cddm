import { Trash2 } from "lucide-react";
import React, { useEffect } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { DeadCodeStudioView } from "./scan-results/DeadCodeStudioView";
import { Win2xWindow } from "./ui/win2x-manager";

export interface DeadCodeExplorerModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DeadCodeExplorerModal: React.FC<DeadCodeExplorerModalProps> = ({
  isOpen,
  onClose,
}) => {
  const { deadCodeSummary, isDeadCodeLoading, scanDeadCode } = useCDDMStore();

  useEffect(() => {
    if (isOpen && !deadCodeSummary && !isDeadCodeLoading) {
      void scanDeadCode({ static_only: false });
    }
  }, [isOpen, deadCodeSummary, isDeadCodeLoading, scanDeadCode]);

  if (!isOpen) return null;

  return (
    <Win2xWindow
      id="cddm-dead-code-modal"
      title="Polyglot Dead Code Explorer & Safe Pruner"
      icon={<Trash2 className="w-4 h-4 text-rose-400" />}
      isOpen={isOpen}
      onClose={onClose}
      initialWidth={1050}
      initialHeight={720}
    >
      <div className="flex flex-col h-full bg-[#1e1e2e] text-slate-200 text-sm overflow-y-auto p-4">
        <DeadCodeStudioView />
      </div>
    </Win2xWindow>
  );
};

export default DeadCodeExplorerModal;
