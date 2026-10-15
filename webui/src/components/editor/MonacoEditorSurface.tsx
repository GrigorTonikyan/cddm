import React from "react";
import Editor, { DiffEditor } from "@monaco-editor/react";
import { useCDDMStore } from "../../store/cddm-store";

export interface MonacoEditorSurfaceProps {
  onSave: () => void;
}

export const MonacoEditorSurface: React.FC<MonacoEditorSurfaceProps> = ({ onSave }) => {
  const {
    activeEditorContent,
    setActiveEditorContent,
    editorDiffContentA,
    editorSplitDiffMode,
    editorLanguage,
    targetLine,
    targetRangeA,
    targetRangeB,
  } = useCDDMStore();

  const commonOptions = {
    minimap: { enabled: true },
    scrollBeyondLastLine: false,
    fontSize: 13,
    automaticLayout: true,
    fontFamily:
      "'JetBrains Mono', 'Fira Code', 'Cascadia Code', Menlo, Monaco, Consolas, monospace",
    lineNumbers: "on" as const,
    renderWhitespace: "selection" as const,
    tabSize: 2,
  };

  if (editorSplitDiffMode) {
    return (
      <div className="w-full h-full min-h-[400px] bg-slate-950">
        <DiffEditor
          height="100%"
          language={editorLanguage}
          original={editorDiffContentA}
          modified={activeEditorContent}
          theme="vs-dark"
          options={{
            ...commonOptions,
            readOnly: false,
            originalEditable: false,
            renderSideBySide: true,
          }}
          onMount={(diffEditor, monaco) => {
            const modifiedEditor = diffEditor.getModifiedEditor();
            const originalEditor = diffEditor.getOriginalEditor();
            modifiedEditor.onDidChangeModelContent(() => {
              setActiveEditorContent(modifiedEditor.getValue());
            });
            modifiedEditor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
              onSave();
            });
            if (targetRangeB) {
              modifiedEditor.revealLineInCenter(targetRangeB[0]);
              modifiedEditor.setPosition({ lineNumber: targetRangeB[0], column: 1 });
            } else if (targetLine) {
              modifiedEditor.revealLineInCenter(targetLine);
              modifiedEditor.setPosition({ lineNumber: targetLine, column: 1 });
            }
            if (targetRangeA) {
              originalEditor.revealLineInCenter(targetRangeA[0]);
            }
          }}
        />
      </div>
    );
  }

  return (
    <div className="w-full h-full min-h-[400px] bg-slate-950">
      <Editor
        height="100%"
        language={editorLanguage}
        value={activeEditorContent}
        theme="vs-dark"
        options={commonOptions}
        onChange={(val) => setActiveEditorContent(val ?? "")}
        onMount={(editor, monaco) => {
          editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
            onSave();
          });
          if (targetLine) {
            editor.revealLineInCenter(targetLine);
            editor.setPosition({ lineNumber: targetLine, column: 1 });
          }
        }}
      />
    </div>
  );
};
