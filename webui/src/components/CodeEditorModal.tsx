import React, { useEffect, useCallback } from "react";
import { Code2, FileCode2, Save } from "lucide-react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import { EditorToolbar } from "./editor/EditorToolbar";
import { EditorSurface } from "./editor/EditorSurface";

export interface CodeEditorModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const CodeEditorModal: React.FC<CodeEditorModalProps> = ({ isOpen, onClose }) => {
  const {
    activeEditorFile,
    editorIsDirty,
    editorIsLoading,
    editorSaveSuccess,
    saveActiveEditorFile,
    openFileInEditor,
    fetchEditorFiles,
    editorFilesList,
  } = useCDDMStore();

  useEffect(() => {
    if (isOpen && editorFilesList.length === 0) {
      void fetchEditorFiles();
    }
  }, [isOpen, editorFilesList.length, fetchEditorFiles]);

  const handleSave = useCallback(async () => {
    if (!editorIsDirty || editorIsLoading || !activeEditorFile) return;
    await saveActiveEditorFile();
  }, [editorIsDirty, editorIsLoading, activeEditorFile, saveActiveEditorFile]);

  const handleReload = useCallback(() => {
    if (activeEditorFile) {
      void openFileInEditor(activeEditorFile);
    }
  }, [activeEditorFile, openFileInEditor]);

  // Global keybinding for Ctrl+S / Cmd+S when modal is active
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
        e.preventDefault();
        void handleSave();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, handleSave]);

  const footerContent = (
    <div className="flex items-center justify-between w-full text-xs font-mono text-slate-400">
      <div className="flex items-center gap-2 truncate">
        <FileCode2 className="w-3.5 h-3.5 text-indigo-400 shrink-0" />
        <span className="truncate text-slate-300">
          {activeEditorFile ? activeEditorFile : "Ready — choose a file to begin"}
        </span>
      </div>

      <div className="flex items-center gap-2 shrink-0">
        <button
          type="button"
          onClick={() => void handleSave()}
          disabled={editorIsLoading || !editorIsDirty || !activeEditorFile}
          className={`px-3 py-1 rounded font-semibold text-xs flex items-center gap-1.5 transition-colors ${
            editorSaveSuccess
              ? "bg-emerald-600 text-white"
              : editorIsDirty
                ? "bg-emerald-600 hover:bg-emerald-500 text-white"
                : "bg-slate-800 text-slate-500 cursor-not-allowed"
          }`}
        >
          <Save className="w-3 h-3" />
          <span>{editorSaveSuccess ? "Saved" : "Save Changes"}</span>
        </button>

        <button
          type="button"
          onClick={onClose}
          className="px-3 py-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  );

  return (
    <Win2xWindow
      id="code-editor-studio"
      windowType="code-editor"
      isOpen={isOpen}
      onClose={onClose}
      title="Integrated Code Editor & Split Diff Studio"
      subtitle={activeEditorFile ?? "Workspace File Editor"}
      badge={editorIsDirty ? "Modified" : "Clean"}
      icon={<Code2 className="w-4 h-4 text-emerald-400" />}
      footer={footerContent}
      initialWidth={1150}
      initialHeight={750}
    >
      <div className="flex flex-col h-full min-h-[500px] overflow-hidden rounded-xl border border-slate-800 bg-slate-950">
        <EditorToolbar onSave={() => void handleSave()} onReload={handleReload} />
        <div className="flex-1 min-h-0 relative">
          <EditorSurface onSave={() => void handleSave()} />
        </div>
      </div>
    </Win2xWindow>
  );
};

export default CodeEditorModal;
