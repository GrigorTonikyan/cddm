import React from "react";
import {
  Save,
  Check,
  RotateCcw,
  FileCode2,
  Columns2,
  AlertCircle,
  FileText,
  Loader2,
} from "lucide-react";
import { useCDDMStore } from "../../store/cddm-store";

export interface EditorToolbarProps {
  onSave: () => void;
  onReload: () => void;
}

const SUPPORTED_LANGUAGES = [
  "typescript",
  "javascript",
  "rust",
  "python",
  "go",
  "json",
  "markdown",
  "html",
  "css",
  "toml",
  "yaml",
  "bash",
];

export const EditorToolbar: React.FC<EditorToolbarProps> = ({ onSave, onReload }) => {
  const {
    activeEditorFile,
    editorLanguage,
    editorIsDirty,
    editorIsLoading,
    editorError,
    editorSaveSuccess,
    editorSplitDiffMode,
    editorFilesList,
    setEditorLanguage,
    setEditorSplitDiffMode,
    openFileInEditor,
  } = useCDDMStore();

  const handleFileSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selected = e.target.value;
    if (selected && selected !== activeEditorFile) {
      void openFileInEditor(selected);
    }
  };

  const handleLanguageChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setEditorLanguage(e.target.value);
  };

  return (
    <div className="flex flex-col gap-2 p-3 bg-slate-900 border-b border-slate-800">
      <div className="flex flex-wrap items-center justify-between gap-3 text-xs font-mono">
        {/* Left: File Picker & Language Selector */}
        <div className="flex flex-wrap items-center gap-2 flex-1 min-w-0">
          <div className="flex items-center gap-1.5 bg-slate-950 px-2.5 py-1 rounded-lg border border-slate-800 min-w-0 max-w-md">
            <FileCode2 className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
            <select
              aria-label="Select file to edit"
              value={activeEditorFile ?? ""}
              onChange={handleFileSelect}
              className="bg-transparent text-slate-200 outline-none text-xs w-full truncate cursor-pointer font-mono"
            >
              {activeEditorFile && !editorFilesList.some((f) => f.path === activeEditorFile) ? (
                <option value={activeEditorFile} className="bg-slate-900 text-slate-200">
                  {activeEditorFile}
                </option>
              ) : null}
              {editorFilesList.length === 0 ? (
                <option value="" disabled className="bg-slate-900 text-slate-400">
                  {activeEditorFile ? activeEditorFile : "No workspace files discovered"}
                </option>
              ) : (
                editorFilesList.map((file) => (
                  <option key={file.path} value={file.path} className="bg-slate-900 text-slate-200">
                    {file.path} ({file.language ?? "plain"})
                  </option>
                ))
              )}
            </select>
          </div>

          {/* Language Selector */}
          <div className="flex items-center gap-1 bg-slate-950 px-2 py-1 rounded-lg border border-slate-800">
            <FileText className="w-3 h-3 text-slate-400 shrink-0" />
            <select
              aria-label="Select syntax language"
              value={editorLanguage}
              onChange={handleLanguageChange}
              className="bg-transparent text-slate-300 outline-none text-xs cursor-pointer font-mono uppercase"
            >
              {SUPPORTED_LANGUAGES.map((lang) => (
                <option key={lang} value={lang} className="bg-slate-900 text-slate-200">
                  {lang}
                </option>
              ))}
            </select>
          </div>

          {/* Split Diff Toggle */}
          <button
            type="button"
            onClick={() => setEditorSplitDiffMode(!editorSplitDiffMode)}
            className={`px-2.5 py-1 rounded-lg border text-xs font-mono flex items-center gap-1.5 transition-colors ${
              editorSplitDiffMode
                ? "bg-indigo-600/30 border-indigo-500/50 text-indigo-300"
                : "bg-slate-950 border-slate-800 text-slate-400 hover:text-slate-200"
            }`}
            title="Toggle Split Diff Mode"
          >
            <Columns2 className="w-3.5 h-3.5 text-indigo-400" />
            <span>{editorSplitDiffMode ? "Split Diff: ON" : "Split Diff: OFF"}</span>
          </button>
        </div>

        {/* Right: Dirty Status, Reload, Save Button */}
        <div className="flex items-center gap-2 shrink-0">
          {editorIsDirty ? (
            <span className="px-2 py-0.5 rounded text-[11px] bg-amber-500/20 text-amber-300 border border-amber-500/40 flex items-center gap-1 font-mono">
              <span className="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse" />
              Unsaved Changes
            </span>
          ) : (
            <span className="px-2 py-0.5 rounded text-[11px] bg-slate-800/80 text-slate-400 font-mono">
              Clean
            </span>
          )}

          <button
            type="button"
            onClick={onReload}
            disabled={editorIsLoading || !activeEditorFile}
            className="p-1.5 rounded-lg bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-400 hover:text-slate-200 disabled:opacity-50 transition-colors"
            title="Discard changes and reload from disk"
          >
            <RotateCcw className="w-3.5 h-3.5" />
          </button>

          <button
            type="button"
            onClick={onSave}
            disabled={editorIsLoading || !editorIsDirty || !activeEditorFile}
            className={`px-3 py-1 rounded-lg font-semibold text-xs flex items-center gap-1.5 transition-colors shadow-sm font-mono ${
              editorSaveSuccess
                ? "bg-emerald-600 text-white"
                : editorIsDirty
                  ? "bg-emerald-600 hover:bg-emerald-500 text-white"
                  : "bg-slate-800 text-slate-400 cursor-not-allowed opacity-60"
            }`}
            title="Save file to workspace (Ctrl+S)"
          >
            {editorIsLoading ? (
              <>
                <Loader2 className="w-3.5 h-3.5 animate-spin" />
                <span>Saving...</span>
              </>
            ) : editorSaveSuccess ? (
              <>
                <Check className="w-3.5 h-3.5" />
                <span>Saved</span>
              </>
            ) : (
              <>
                <Save className="w-3.5 h-3.5" />
                <span>Save</span>
                <span className="text-[10px] opacity-75 hidden sm:inline">(Ctrl+S)</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Error notification banner if any */}
      {editorError ? (
        <div className="flex items-center gap-2 p-2 bg-rose-950/40 border border-rose-900/60 rounded-lg text-rose-300 text-xs font-mono">
          <AlertCircle className="w-4 h-4 shrink-0 text-rose-400" />
          <span className="truncate">{editorError}</span>
        </div>
      ) : null}
    </div>
  );
};
