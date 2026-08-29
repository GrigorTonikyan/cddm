import React, { useEffect, useState, useCallback } from "react";
import { API_ROUTES } from "../constants/cddm-constants";
import {
  CloneCluster,
  ClusterRefactorRequest,
  ClusterRefactorSuggestion,
  RefactorRequest,
  RefactorSuggestion,
} from "../types/cddm-types";
import { parsePath } from "../utils/path-utils";
import { downloadTextFile } from "../utils/file-download";
import { CollapsibleCard, CodeBlock, BADGE_VARIANTS, CODE_BLOCK_VARIANTS } from "./ui";
import { Win2xWindow } from "./ui/win2x-manager";
import { useCDDMStore } from "../store/cddm-store";
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
  Wrench,
} from "lucide-react";

export interface RefactorPatchModalProps {
  isOpen: boolean;
  onClose: () => void;
  cluster?: CloneCluster;
  fileA?: string;
  startLineA?: number;
  endLineA?: number;
  fileB?: string;
  startLineB?: number;
  endLineB?: number;
}

export const RefactorPatchModal: React.FC<RefactorPatchModalProps> = ({
  isOpen,
  onClose,
  cluster,
  fileA,
  startLineA,
  endLineA,
  fileB,
  startLineB,
  endLineB,
}) => {
  const { applyPatch, isPatching } = useCDDMStore();
  const [pairSuggestion, setPairSuggestion] = useState<RefactorSuggestion | null>(null);
  const [clusterSuggestion, setClusterSuggestion] = useState<ClusterRefactorSuggestion | null>(
    null,
  );
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [copiedPatch, setCopiedPatch] = useState<boolean>(false);
  const [downloaded, setDownloaded] = useState<boolean>(false);
  const [applied, setApplied] = useState<boolean>(false);

  const isClusterMode = Boolean(cluster);

  const fetchSuggestion = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      if (cluster) {
        const payload: ClusterRefactorRequest = {
          cluster_id: String(cluster.id),
          occurrences: cluster.occurrences,
        };

        const res = await fetch(API_ROUTES.REFACTOR_CLUSTER, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(payload),
        });

        if (!res.ok) {
          const errorText = await res.text().catch(() => res.statusText);
          throw new Error(
            `Cluster refactoring analysis failed (${res.status}): ${errorText || res.statusText}`,
          );
        }

        const data = (await res.json()) as ClusterRefactorSuggestion;
        setClusterSuggestion(data);
        setPairSuggestion(null);
      } else if (
        fileA &&
        fileB &&
        startLineA !== undefined &&
        endLineA !== undefined &&
        startLineB !== undefined &&
        endLineB !== undefined
      ) {
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
          const errorText = await res.text().catch(() => res.statusText);
          throw new Error(
            `Refactoring analysis failed (${res.status}): ${errorText || res.statusText}`,
          );
        }

        const data = (await res.json()) as RefactorSuggestion;
        setPairSuggestion(data);
        setClusterSuggestion(null);
      }
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "Failed to synthesize refactoring patch");
    } finally {
      setLoading(false);
    }
  }, [cluster, fileA, startLineA, endLineA, fileB, startLineB, endLineB]);

  useEffect(() => {
    if (isOpen) {
      void fetchSuggestion();
    }
  }, [isOpen, fetchSuggestion]);

  if (!isOpen) return null;

  const currentPatch = clusterSuggestion?.unified_patch || pairSuggestion?.unified_patch || "";

  const handleCopyPatch = () => {
    if (!currentPatch) return;
    void navigator.clipboard.writeText(currentPatch);
    setCopiedPatch(true);
    setTimeout(() => setCopiedPatch(false), 2000);
  };

  const handleDownloadPatch = () => {
    if (!currentPatch) return;
    const filename = cluster
      ? `cddm-cluster-${cluster.id}-refactor.patch`
      : `cddm-refactor-${parsePath(fileA || "code").filename}.patch`;
    downloadTextFile(currentPatch, filename, "text/x-diff;charset=utf-8");
    setDownloaded(true);
    setTimeout(() => setDownloaded(false), 2000);
  };

  const handleApplyPatch = async () => {
    if (!currentPatch) return;
    try {
      await applyPatch(currentPatch, false);
      setApplied(true);
      setTimeout(() => {
        onClose();
        setApplied(false);
      }, 1000);
    } catch {
      // Error handled by store
    }
  };

  const parsedA = parsePath(fileA || "");
  const parsedB = parsePath(fileB || "");

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

  const windowTitle = isClusterMode
    ? "Multi-Site Cluster Refactoring Advisor"
    : "Automated Refactoring Advisor";
  const windowSubtitle = isClusterMode
    ? `Cluster #${cluster?.id} (${cluster?.occurrences.length} Sites)`
    : `${parsedA.filename}:${startLineA} <-> ${parsedB.filename}:${startLineB}`;
  const windowBadge = isClusterMode
    ? `${cluster?.occurrences.length} Occurrences`
    : `L${startLineA}-${endLineA}`;

  return (
    <Win2xWindow
      id={`refactor-patch-${cluster ? `cluster-${cluster.id}` : `${fileA}:${startLineA}-${endLineA}_${fileB}:${startLineB}-${endLineB}`}`}
      windowType="refactor-advisor"
      isOpen={isOpen}
      onClose={onClose}
      title={windowTitle}
      subtitle={windowSubtitle}
      badge={windowBadge}
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
      ) : clusterSuggestion || pairSuggestion ? (
        <div className="space-y-4">
          {/* Section 1: Refactoring Strategy & Metrics Overview */}
          <CollapsibleCard
            icon={<GitBranch className="w-4 h-4" />}
            title="Strategy & Overview"
            badgeCount={`~${clusterSuggestion?.total_lines_saved ?? pairSuggestion?.lines_saved ?? 0} lines saved`}
            badgeVariant={BADGE_VARIANTS.EMERALD}
            defaultOpen={true}
          >
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3.5">
              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <GitBranch className="w-3.5 h-3.5 text-indigo-400" />
                  Strategy
                </span>
                <div className="font-mono text-xs font-bold text-indigo-300">
                  {(clusterSuggestion?.strategy ?? pairSuggestion?.strategy) === "extract_function"
                    ? "Extract Function"
                    : "Multi-Site Extraction"}
                </div>
              </div>

              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <Layers className="w-3.5 h-3.5 text-emerald-400" />
                  Estimated Savings
                </span>
                <div className="font-mono text-xs font-bold text-emerald-400">
                  ~{clusterSuggestion?.total_lines_saved ?? pairSuggestion?.lines_saved ?? 0} lines
                  eliminated
                </div>
              </div>

              <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
                <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
                  <FileCode2 className="w-3.5 h-3.5 text-amber-400" />
                  Helper Name
                </span>
                <div className="font-mono text-xs font-bold text-slate-200 truncate">
                  {clusterSuggestion?.suggested_function_name ??
                    pairSuggestion?.suggested_function_name}
                  ()
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
                {clusterSuggestion?.target_module_hint ?? pairSuggestion?.target_module_hint}
              </span>
            </div>
          </CollapsibleCard>

          {/* Section 3: Sites & Parameter Variances */}
          {clusterSuggestion ? (
            <CollapsibleCard
              icon={<Layers className="w-4 h-4" />}
              title="Cluster Occurrence Sites"
              badgeCount={`${clusterSuggestion.sites.length} Sites`}
              badgeVariant={BADGE_VARIANTS.INDIGO}
              defaultOpen={true}
            >
              <div className="space-y-3">
                {clusterSuggestion.sites.map((site, i) => {
                  const siteParsed = parsePath(site.file);
                  return (
                    <div
                      key={`cluster-site-${site.file}-${site.start_line}-${i}`}
                      className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-2"
                    >
                      <div className="flex items-center justify-between text-xs font-mono">
                        <div className="flex items-center gap-2">
                          <span className="w-5 h-5 rounded bg-purple-950 text-purple-300 text-[10px] flex items-center justify-center font-bold border border-purple-800/60">
                            {i + 1}
                          </span>
                          <span className="text-slate-100 font-semibold">
                            {siteParsed.filename}
                          </span>
                          <span className="text-slate-500">
                            L{site.start_line}-{site.end_line}
                          </span>
                        </div>
                        <span className="text-[11px] px-2 py-0.5 rounded bg-slate-900 text-slate-400 border border-slate-800">
                          {site.parameter_differences.length} variances
                        </span>
                      </div>
                      <div className="font-mono text-xs bg-slate-900/90 p-2.5 rounded-lg border border-slate-800/80 text-emerald-300">
                        <span className="text-slate-500 text-[11px] select-none block mb-1">
                          Call-site replacement:
                        </span>
                        <code>{site.call_site_replacement}</code>
                      </div>
                    </div>
                  );
                })}
              </div>
            </CollapsibleCard>
          ) : pairSuggestion && pairSuggestion.parameter_differences.length > 0 ? (
            <CollapsibleCard
              icon={<AlertCircle className="w-4 h-4" />}
              title="Parameter Variances"
              badgeCount={pairSuggestion.parameter_differences.length}
              badgeVariant={BADGE_VARIANTS.AMBER}
              defaultOpen={true}
            >
              <div className="space-y-3">
                {pairSuggestion.parameter_differences.map((diff, i) => (
                  <div
                    key={`param-diff-${diff.line_number_a}-${diff.line_number_b}-${i}`}
                    className="grid grid-cols-1 md:grid-cols-2 gap-3"
                  >
                    <CodeBlock
                      filename={parsedA.filename}
                      lineRange={`L${diff.line_number_a}`}
                      code={diff.fragment_a_code}
                      variant={CODE_BLOCK_VARIANTS.REMOVED}
                      showCopy={true}
                      emptyPlaceholder="<empty>"
                    />
                    <CodeBlock
                      filename={parsedB.filename}
                      lineRange={`L${diff.line_number_b}`}
                      code={diff.fragment_b_code}
                      variant={CODE_BLOCK_VARIANTS.ADDED}
                      showCopy={true}
                      emptyPlaceholder="<empty>"
                    />
                  </div>
                ))}
              </div>
            </CollapsibleCard>
          ) : null}

          {/* Section 4: Synthesized Unified Patch Preview */}
          <CollapsibleCard
            icon={<FileCode2 className="w-4 h-4" />}
            title="Unified Diff (.patch)"
            badgeVariant={BADGE_VARIANTS.CYAN}
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
                <button
                  type="button"
                  onClick={handleApplyPatch}
                  disabled={isPatching || applied}
                  className="px-2.5 py-1 rounded-md bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white text-[11px] font-semibold flex items-center gap-1.5 transition-colors shadow-sm font-mono"
                >
                  {isPatching ? (
                    <>
                      <RefreshCw className="w-3 h-3 animate-spin" />
                      <span>Applying...</span>
                    </>
                  ) : applied ? (
                    <>
                      <Check className="w-3 h-3 text-white" />
                      <span>Applied!</span>
                    </>
                  ) : (
                    <>
                      <Wrench className="w-3 h-3" />
                      <span>Apply to Workspace</span>
                    </>
                  )}
                </button>
              </>
            }
          >
            <div className="max-h-72 overflow-x-auto overflow-y-auto p-3.5 font-mono text-xs leading-relaxed bg-slate-950 rounded-xl border border-slate-800 select-text">
              {currentPatch.split("\n").map((line, idx) => {
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
    </Win2xWindow>
  );
};
