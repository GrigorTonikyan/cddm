import React, { useState } from "react";
import { DiffEditor } from "@monaco-editor/react";

export interface MonacoDiffViewerProps {
  original: string;
  modified: string;
  language?: string;
  renderSideBySide?: boolean;
  readOnly?: boolean;
  height?: string | number;
  onModifiedChange?: (newVal: string) => void;
}

export const MonacoDiffViewer: React.FC<MonacoDiffViewerProps> = ({
  original,
  modified,
  language = "typescript",
  renderSideBySide = true,
  readOnly = true,
  height = "320px",
  onModifiedChange,
}) => {
  const [isReady, setIsReady] = useState(false);

  // In test environments (Vitest/happy-dom/jsdom), avoid initializing canvas/WebGL
  const isTestEnv =
    typeof process !== "undefined" &&
    (process.env.NODE_ENV === "test" || process.env.VITEST === "true");

  if (isTestEnv) {
    return (
      <div
        data-testid="monaco-diff-viewer-test-fallback"
        className="p-3 bg-slate-950 font-mono text-xs text-slate-300 rounded border border-slate-800 space-y-2 overflow-auto max-h-80"
      >
        <div className="text-[11px] text-indigo-400 font-bold">
          Monaco Diff Test Fallback ({language} | {renderSideBySide ? "Split" : "Inline"})
        </div>
        <div className="grid grid-cols-2 gap-2">
          <pre className="p-2 bg-slate-900 rounded">{original}</pre>
          <pre className="p-2 bg-slate-900 rounded">{modified}</pre>
        </div>
      </div>
    );
  }

  return (
    <div
      className="relative w-full rounded-b-xl overflow-hidden border-t border-slate-800/80 bg-[#1e1e1e]"
      style={{ height }}
    >
      <DiffEditor
        height={height}
        language={language.toLowerCase()}
        original={original}
        modified={modified}
        theme="vs-dark"
        options={{
          readOnly,
          originalEditable: false,
          renderSideBySide,
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
          fontSize: 12,
          lineNumbers: "on",
          automaticLayout: true,
          renderOverviewRuler: false,
          folding: true,
        }}
        onMount={(editor) => {
          setIsReady(true);
          const modifiedModel = editor.getModifiedEditor().getModel();
          if (modifiedModel && onModifiedChange && !readOnly) {
            modifiedModel.onDidChangeContent(() => {
              onModifiedChange(modifiedModel.getValue());
            });
          }
        }}
      />
      {!isReady && (
        <div className="absolute inset-0 flex items-center justify-center bg-slate-950/80 text-xs font-mono text-slate-400">
          <span>Initializing Monaco Diff Engine...</span>
        </div>
      )}
    </div>
  );
};
