import React from "react";
import { Check, Copy, ExternalLink, FileCode } from "lucide-react";
import { useCDDMStore } from "../../store/cddm-store";

export interface DiffFragmentHeaderProps {
  filePath: string;
  parsed: { directory: string; filename: string };
  startLine: number;
  endLine: number;
  ideLink: string;
  editorName: string;
  copied: boolean;
  onCopy: () => void;
  onOpenInEditor?: () => void;
}

export const DiffFragmentHeader: React.FC<DiffFragmentHeaderProps> = ({
  filePath,
  parsed,
  startLine,
  endLine,
  ideLink,
  editorName,
  copied,
  onCopy,
  onOpenInEditor,
}) => {
  const { openFileInEditor } = useCDDMStore();

  const handleOpenStudio = () => {
    if (onOpenInEditor) {
      onOpenInEditor();
    } else {
      void openFileInEditor(filePath, startLine);
    }
  };

  return (
    <div className="px-3 py-1.5 bg-slate-900/60 border-b border-slate-800/60 flex items-center justify-between text-xs font-mono">
      <span className="text-slate-300 truncate" title={filePath}>
        <span className="text-slate-500">{parsed.directory}</span>
        <span className="font-bold text-indigo-300">{parsed.filename}</span>
        <span className="text-slate-500 ml-1.5">
          (L{startLine}–{endLine})
        </span>
      </span>
      <div className="flex items-center gap-1">
        <button
          type="button"
          onClick={handleOpenStudio}
          title="Open in CDDM Studio Code Editor"
          className="p-1 hover:bg-slate-800 text-slate-400 hover:text-cyan-300 rounded transition-colors flex items-center gap-1 text-[11px]"
        >
          <FileCode className="w-3.5 h-3.5 text-cyan-400" />
          <span className="hidden sm:inline">Studio</span>
        </button>
        <a
          href={ideLink}
          title={`Open in ${editorName} at line ${startLine}`}
          className="p-1 hover:bg-slate-800 text-slate-400 hover:text-indigo-300 rounded transition-colors"
        >
          <ExternalLink className="w-3.5 h-3.5" />
        </a>
        <button
          type="button"
          onClick={onCopy}
          className="p-1 hover:bg-slate-800 text-slate-400 hover:text-slate-200 rounded transition-colors"
          title="Copy duplicate code"
        >
          {copied ? (
            <Check className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <Copy className="w-3.5 h-3.5" />
          )}
        </button>
      </div>
    </div>
  );
};
