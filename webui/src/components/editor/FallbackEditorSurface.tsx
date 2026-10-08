import React from "react";
import { useCDDMStore } from "../../store/cddm-store";

export interface FallbackEditorSurfaceProps {
  onSave: () => void;
}

export const FallbackEditorSurface: React.FC<FallbackEditorSurfaceProps> = ({ onSave }) => {
  const {
    activeEditorContent,
    setActiveEditorContent,
    editorDiffContentA,
    editorSplitDiffMode,
    editorDiffFileA,
    activeEditorFile,
  } = useCDDMStore();

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "s") {
      e.preventDefault();
      onSave();
    }
  };

  if (editorSplitDiffMode) {
    return (
      <div
        data-testid="monaco-editor-diff"
        className="grid grid-cols-1 md:grid-cols-2 gap-2 h-full min-h-[400px] p-3 bg-slate-950 font-mono text-xs"
      >
        {/* Original Side (Read Only) */}
        <div className="flex flex-col border border-slate-800 rounded-lg overflow-hidden bg-slate-900/60">
          <div className="px-3 py-1.5 bg-slate-950 border-b border-slate-800 text-[11px] text-slate-400 font-semibold truncate">
            Original: {editorDiffFileA ?? "Base Revision"}
          </div>
          <textarea
            aria-label="Original file content"
            readOnly
            value={editorDiffContentA}
            className="flex-1 w-full p-3 bg-transparent text-slate-400 resize-none outline-none font-mono text-xs leading-relaxed select-text"
          />
        </div>

        {/* Modified Side (Editable) */}
        <div className="flex flex-col border border-slate-800 rounded-lg overflow-hidden bg-slate-900/60">
          <div className="px-3 py-1.5 bg-slate-950 border-b border-slate-800 text-[11px] text-emerald-400 font-semibold truncate flex items-center justify-between">
            <span>Modified: {activeEditorFile ?? "Target Revision"}</span>
            <span className="text-[10px] text-slate-500">(Editable)</span>
          </div>
          <textarea
            aria-label="Modified file content"
            data-testid="code-editor-textarea"
            value={activeEditorContent}
            onChange={(e) => setActiveEditorContent(e.target.value)}
            onKeyDown={handleKeyDown}
            className="flex-1 w-full p-3 bg-transparent text-slate-100 resize-none outline-none font-mono text-xs leading-relaxed select-text focus:bg-slate-900/40"
          />
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full min-h-[400px] p-3 bg-slate-950 font-mono text-xs">
      <textarea
        aria-label="Code Editor Content"
        data-testid="code-editor-textarea"
        value={activeEditorContent}
        onChange={(e) => setActiveEditorContent(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Select or open a file to start editing..."
        className="flex-1 w-full p-4 bg-slate-900/70 border border-slate-800 rounded-xl text-slate-100 resize-none outline-none font-mono text-xs leading-relaxed select-text focus:border-indigo-500/50"
      />
    </div>
  );
};
