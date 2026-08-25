import React from "react";
import type { AstRewriteResult } from "../../types/cddm-types";
import { AlertCircle, RefreshCw, Sparkles } from "lucide-react";

export interface AstRewritePreviewProps {
  astRewriteResult: AstRewriteResult | null;
  isAstLoading: boolean;
  astError: string | null;
}

export const AstRewritePreview: React.FC<AstRewritePreviewProps> = ({
  astRewriteResult,
  isAstLoading,
  astError,
}) => {
  return (
    <div className="space-y-3 font-mono text-xs text-slate-300">
      {isAstLoading ? (
        <div className="py-16 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs bg-slate-950/60 border border-slate-800 rounded-xl">
          <RefreshCw className="w-6 h-6 animate-spin text-purple-400" />
          <span>Synthesizing Tree-sitter AST refactoring and inferring types...</span>
        </div>
      ) : astError ? (
        <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono text-rose-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-rose-400 flex-shrink-0" />
          <span>{astError}</span>
        </div>
      ) : astRewriteResult ? (
        <div className="space-y-3">
          {/* Extracted Helper Header */}
          <div className="p-3.5 bg-slate-900/90 border border-purple-800/40 rounded-xl space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-purple-300 font-bold text-xs flex items-center gap-2">
                <Sparkles className="w-3.5 h-3.5 text-purple-400" />
                Extracted Shared Helper: {astRewriteResult.function_name}()
              </span>
              <span className="text-slate-400 text-[11px] font-mono">
                Target: {astRewriteResult.target_module_path}
              </span>
            </div>

            {astRewriteResult.inferred_parameters.length > 0 && (
              <div className="flex flex-wrap items-center gap-1.5 pt-1">
                <span className="text-[11px] text-slate-400 mr-1">Inferred Parameters:</span>
                {astRewriteResult.inferred_parameters.map((param, pIdx) => (
                  <span
                    key={pIdx}
                    className="px-2 py-0.5 bg-purple-950/60 border border-purple-800/60 text-purple-300 rounded font-mono text-[11px]"
                  >
                    {param.name}: <span className="text-amber-300">{param.inferred_type}</span>
                  </span>
                ))}
              </div>
            )}
          </div>

          {/* Synthesized Helper Code Block */}
          <div className="border border-slate-800 rounded-xl bg-slate-950 overflow-hidden">
            <div className="px-3.5 py-2 bg-slate-900/60 border-b border-slate-800 text-[11px] text-slate-400 font-mono">
              Synthesized Function Implementation
            </div>
            <pre className="p-3.5 text-xs font-mono text-slate-200 overflow-x-auto max-h-[160px] leading-relaxed">
              {astRewriteResult.helper_function_code}
            </pre>
          </div>

          {/* Rewritten Source Files List */}
          <div className="space-y-2">
            <span className="text-slate-300 font-semibold text-xs">
              Transformed Source Files ({astRewriteResult.rewritten_files.length})
            </span>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {astRewriteResult.rewritten_files.map((rf, rfIdx) => (
                <div
                  key={rfIdx}
                  className="p-2.5 bg-slate-900/60 border border-slate-800 rounded-lg space-y-1 text-[11px]"
                >
                  <div className="flex items-center justify-between text-slate-200 font-medium">
                    <span className="truncate">{rf.file_path}</span>
                    <span className="text-emerald-400 font-mono text-[10px]">
                      {rf.call_sites_count} call site(s)
                    </span>
                  </div>
                  <div className="text-slate-400 text-[10px] flex items-center justify-between font-mono">
                    <span>
                      {rf.original_line_count} -&gt; {rf.new_line_count} lines
                    </span>
                    {rf.imports_added.length > 0 && (
                      <span className="text-indigo-400">+{rf.imports_added.length} import</span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      ) : (
        <div className="p-8 bg-slate-950/60 border border-slate-800 rounded-xl text-center text-slate-500 font-mono text-xs">
          Click &quot;Re-Simulate Sandbox&quot; to synthesize Tree-sitter AST refactoring.
        </div>
      )}
    </div>
  );
};
