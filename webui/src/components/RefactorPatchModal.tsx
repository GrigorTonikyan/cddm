import React, { useEffect, useState, useCallback } from "react";
import { API_ROUTES } from "../constants/cddm-constants";
import { RefactorRequest, RefactorSuggestion } from "../types/cddm-types";
import { parsePath } from "../utils/path-utils";
import { Window, CollapsibleCard, CodeBlock } from "./ui";
import {
  Sparkles,
  Download,
  Copy,
  Check,
  RefreshCw,
  AlertCircle,
  FileCode2,
  GitBranch,
  Layers,
} from "lucide-react";

export interface RefactorPatchModalProps {
  isOpen: boolean;
  onClose: () => void;
  fileA: string;
  startLineA: number;
  endLineA: number;
  fileB: string;
  startLineB: number;
  endLineB: number;
}

export const RefactorPatchModal: React.FC<RefactorPatchModalProps> = ({
  isOpen,
  onClose,
  fileA,
  startLineA,
  endLineA,
  fileB,
  startLineB,
  endLineB,
}) => {
  const [suggestion, setSuggestion] = useState<RefactorSuggestion | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [copiedPatch, setCopiedPatch] = useState<boolean>(false);
  const [downloaded, setDownloaded] = useState<boolean>(false);

  const fetchSuggestion = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const payload: RefactorRequest = {
        file_a: fileA,
        start_line_a: startLineA,
        end_line_a: endLineA,
        file_b: fileB,
        start_line_b: startLineB,
        end_line_b: endLineB,
      };

      const res = await fetch(API_ROUTES.REFACTOR, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        throw new Error(`Refactoring analysis failed (${res.status}: ${await res.text()})`);
      }

      const data = (await res.json()) as RefactorSuggestion;
      setSuggestion(data);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "Failed to synthesize refactoring patch");
    } finally {
      setLoading(false);
    }
  }, [fileA, startLineA, endLineA, fileB, startLineB, endLineB]);

  useEffect(() => {
    if (isOpen) {
      void fetchSuggestion();
    }
  }, [isOpen, fetchSuggestion]);

  if (!isOpen) return null;

  const handleCopyPatch = () => {
    if (!suggestion) return;
    void navigator.clipboard.writeText(suggestion.unified_patch);
    setCopiedPatch(true);
    setTimeout(() => setCopiedPatch(false), 2000);
  };

  const handleDownloadPatch = () => {
    if (!suggestion) return;
    const blob = new Blob([suggestion.unified_patch], {
      type: "text/x-diff;charset=utf-8",
    });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `cddm-refactor-${parsePath(fileA).filename}.patch`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
    setDownloaded(true);
    setTimeout(() => setDownloaded(false), 2000);
  };

  const parsedA = parsePath(fileA);
  const parsedB = parsePath(fileB);

  const footerContent = (
    <>
      <div className="flex items-center gap-2 text-slate-500">
        <span>Apply with:</span>
        <code className="bg-slate-900 px-2 py-0.5 rounded border border-slate-800 text-slate-300">
          git apply cddm-refactor.patch
        </code>
      </div>
      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold transition-colors"
      >
        Close
      </button>
    </>
  );

  return (
    <Window
      isOpen={isOpen}
      onClose={onClose}
      title="Automated Refactoring Advisor"
      subtitle="Synthesized unified patch & deduplication recommendation"
      icon={<Sparkles className="w-4 h-4 text-indigo-400" />}
      footer={footerContent}
      initialWidth={920}
      initialHeight={680}
    >
      {loading ? (
        <div className="py-20 flex flex-col items-center justify-center gap-3 text-slate-400 font-mono text-xs">
          <RefreshCw className="w-7 h-7 animate-spin text-indigo-400" />
          <span>Analyzing clone invariant token streams and parameter variances...</span>
        </div>
      ) : error ? (
        <div className="p-4 bg-rose-950/40 border border-rose-900/60 rounded-xl text-xs font-mono space-y-3">
          <div className="flex items-center gap-2 text-rose-400 font-semibold">
            <AlertCircle className="w-4 h-4" />
            <span>Advisor Synthesis Notice</span>
          </div>
          <p className="text-slate-400">{error}</p>
          <button
            type="button"
            onClick={() => void fetchSuggestion()}
            className="px-3 py-1.5 rounded-lg bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200"
          >
            Retry Analysis
          </button>
        </div>
      ) : suggestion ? (
        <div className="space-y-4">
          {/* Section 1: Refactoring Strategy & Metrics Overview */}
          <CollapsibleCard
            icon={<GitBranch className="w-4 h-4" />}
            title="Strategy & Overview"
            badgeCount={`~${suggestion.lines_saved} lines saved`}
            badgeVariant="emerald"
            defaultOpen={true}
          >
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3.5">
              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <GitBranch className="w-3.5 h-3.5 text-indigo-400" />
                  Strategy
                </span>
                <div className="font-mono text-xs font-bold text-indigo-300">
                  {suggestion.strategy === "extract_function"
                    ? "Extract Function"
                    : "Parameterize Variables"}
                </div>
              </div>

              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <Layers className="w-3.5 h-3.5 text-emerald-400" />
                  Estimated Savings
                </span>
                <div className="font-mono text-xs font-bold text-emerald-400">
                  ~{suggestion.lines_saved} lines eliminated
                </div>
              </div>

              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <FileCode2 className="w-3.5 h-3.5 text-amber-400" />
                  Helper Name
                </span>
                <div className="font-mono text-xs font-bold text-slate-200 truncate">
                  {suggestion.suggested_function_name}()
                </div>
              </div>
            </div>
          </CollapsibleCard>

          {/* Section 2: Target Location & Extraction Destination */}
          <CollapsibleCard
            icon={<Layers className="w-4 h-4" />}
            title="Destination Target"
            defaultOpen={true}
          >
            <div className="flex items-center gap-3 text-xs font-mono text-slate-300">
              <span className="text-slate-500 shrink-0">Destination:</span>
              <span className="font-semibold text-indigo-300 truncate">
                {suggestion.target_module_hint}
              </span>
            </div>
          </CollapsibleCard>

          {/* Section 3: Parameter Differences Table */}
          {suggestion.parameter_differences.length > 0 && (
            <CollapsibleCard
              icon={<AlertCircle className="w-4 h-4" />}
              title="Parameter Variances"
              badgeCount={suggestion.parameter_differences.length}
              badgeVariant="amber"
              defaultOpen={true}
            >
              <div className="space-y-3">
                {suggestion.parameter_differences.map((diff, i) => (
                  <div
                    key={`param-diff-${diff.line_number_a}-${diff.line_number_b}-${i}`}
                    className="grid grid-cols-1 md:grid-cols-2 gap-3"
                  >
                    <CodeBlock
                      filename={parsedA.filename}
                      lineRange={`L${diff.line_number_a}`}
                      code={diff.fragment_a_code}
                      variant="removed"
                      showCopy={true}
                      maxHeightClass="max-h-40"
                      emptyPlaceholder="<empty>"
                    />
                    <CodeBlock
                      filename={parsedB.filename}
                      lineRange={`L${diff.line_number_b}`}
                      code={diff.fragment_b_code}
                      variant="added"
                      showCopy={true}
                      maxHeightClass="max-h-40"
                      emptyPlaceholder="<empty>"
                    />
                  </div>
                ))}
              </div>
            </CollapsibleCard>
          )}

          {/* Section 4: Synthesized Unified Patch Preview */}
          <CollapsibleCard
            icon={<FileCode2 className="w-4 h-4" />}
            title="Unified Diff (.patch)"
            badgeVariant="cyan"
            defaultOpen={true}
            actions={
              <>
                <button
                  type="button"
                  onClick={handleCopyPatch}
                  className="px-2.5 py-1 rounded-md bg-slate-950 border border-slate-800 hover:bg-slate-800 text-slate-300 text-[11px] flex items-center gap-1.5 transition-colors font-mono"
                >
                  {copiedPatch ? (
                    <>
                      <Check className="w-3 h-3 text-emerald-400" />
                      <span>Copied</span>
                    </>
                  ) : (
                    <>
                      <Copy className="w-3 h-3" />
                      <span>Copy Patch</span>
                    </>
                  )}
                </button>
                <button
                  type="button"
                  onClick={handleDownloadPatch}
                  className="px-2.5 py-1 rounded-md bg-indigo-600 hover:bg-indigo-500 text-white text-[11px] font-semibold flex items-center gap-1.5 transition-colors shadow-sm font-mono"
                >
                  {downloaded ? (
                    <>
                      <Check className="w-3 h-3" />
                      <span>Downloaded</span>
                    </>
                  ) : (
                    <>
                      <Download className="w-3 h-3" />
                      <span>Download .patch</span>
                    </>
                  )}
                </button>
              </>
            }
          >
            <div className="max-h-72 overflow-x-auto overflow-y-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 select-text">
              {suggestion.unified_patch.split("\n").map((line, idx) => {
                const isAdd = line.startsWith("+");
                const isDel = line.startsWith("-");
                const isHeader =
                  line.startsWith("@@") || line.startsWith("---") || line.startsWith("+++");

                return (
                  <div
                    key={`patch-line-${idx}-${line.slice(0, 10)}`}
                    className={`px-2 py-0.5 rounded ${
                      isAdd
                        ? "bg-emerald-950/40 text-emerald-300 font-semibold"
                        : isDel
                          ? "bg-rose-950/40 text-rose-300 font-semibold"
                          : isHeader
                            ? "text-indigo-300 font-bold"
                            : "text-slate-400"
                    }`}
                  >
                    <pre className="whitespace-pre overflow-visible">{line || " "}</pre>
                  </div>
                );
              })}
            </div>
          </CollapsibleCard>
        </div>
      ) : null}
    </Window>
  );
};
