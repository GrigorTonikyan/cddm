import React, { useState, useEffect } from "react";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Building2,
  FolderGit2,
  Package,
  Layers,
  Sparkles,
  CheckCircle2,
  RefreshCw,
} from "lucide-react";
import type { CrossRepoCluster, HubExtractResult, HubScanSummary } from "../types/cddm-types";
import { useCDDMStore } from "../store/cddm-store";

export interface HubFederationModalProps {
  isOpen: boolean;
  onClose: () => void;
  initialSummary?: HubScanSummary | null;
}

export const HubFederationModal: React.FC<HubFederationModalProps> = ({
  isOpen,
  onClose,
  initialSummary = null,
}) => {
  const { hubSummary, isHubLoading, runHubScan, extractHubPackage } = useCDDMStore();
  const [activeTab, setActiveTab] = useState<"repos" | "matrix" | "clusters">("repos");
  const [extractResult, setExtractResult] = useState<HubExtractResult | null>(null);
  const [extractingClusterId, setExtractingClusterId] = useState<number | null>(null);

  const summary = hubSummary || initialSummary;

  useEffect(() => {
    if (isOpen && !summary) {
      void runHubScan();
    }
  }, [isOpen, summary, runHubScan]);

  if (!isOpen) return null;

  const handleExtract = async (cluster: CrossRepoCluster) => {
    setExtractingClusterId(cluster.id);
    try {
      const res = await extractHubPackage({
        cluster_id: cluster.id,
        target_package_name: cluster.suggested_package || "@org/shared-utils",
        package_type: "npm",
        target_dir: "./packages/shared-extracted",
        dry_run: true,
      });
      setExtractResult(res);
    } catch {
      // handled by store
    } finally {
      setExtractingClusterId(null);
    }
  };

  return (
    <Win2xWindow
      id="cddm-hub-federation-window"
      windowType="hub-federation"
      title="Organization Federation Hub (.cddmhub.toml)"
      subtitle="Cross-repository duplication analysis, inter-repo correlation matrix, and standalone package synthesis"
      isOpen={isOpen}
      onClose={onClose}
    >
      <div className="flex flex-col h-full bg-[#1e1e1e] text-gray-200 text-xs font-mono select-none">
        {/* Top Metric Bar */}
        <div className="flex items-center justify-between px-4 py-3 bg-[#252526] border-b border-[#333333]">
          <div className="flex items-center gap-3">
            <Building2 className="w-5 h-5 text-indigo-400" />
            <div>
              <div className="text-sm font-semibold text-white">
                {summary?.hub_name || "Federation Hub"}
              </div>
              <div className="text-[11px] text-gray-400">
                {summary
                  ? `${summary.total_repos} Connected Repositories • ${summary.total_files} Total Files`
                  : "Loading organization graph..."}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-4">
            {summary && (
              <div className="text-right">
                <div className="text-[10px] text-gray-400 uppercase">Org DRY Score</div>
                <div className="text-sm font-bold text-emerald-400">
                  {(summary.organization_dry_score ?? 100.0).toFixed(1)} / 100.0
                </div>
              </div>
            )}
            <button
              onClick={() => void runHubScan()}
              disabled={isHubLoading}
              className="flex items-center gap-1.5 px-2.5 py-1.5 bg-[#333333] hover:bg-[#3e3e3e] text-white rounded border border-[#444] transition-colors disabled:opacity-50"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isHubLoading ? "animate-spin" : ""}`} />
              Rescan Hub
            </button>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="flex border-b border-[#333333] bg-[#222222] px-3 pt-2 gap-2">
          <button
            onClick={() => setActiveTab("repos")}
            className={`px-3 py-1.5 text-xs font-medium rounded-t border-t border-x transition-colors ${
              activeTab === "repos"
                ? "bg-[#1e1e1e] text-white border-[#444444]"
                : "text-gray-400 border-transparent hover:text-gray-200"
            }`}
          >
            <span className="flex items-center gap-1.5">
              <FolderGit2 className="w-3.5 h-3.5 text-blue-400" />
              Member Repositories ({summary?.repos?.length || 0})
            </span>
          </button>

          <button
            onClick={() => setActiveTab("matrix")}
            className={`px-3 py-1.5 text-xs font-medium rounded-t border-t border-x transition-colors ${
              activeTab === "matrix"
                ? "bg-[#1e1e1e] text-white border-[#444444]"
                : "text-gray-400 border-transparent hover:text-gray-200"
            }`}
          >
            <span className="flex items-center gap-1.5">
              <Layers className="w-3.5 h-3.5 text-purple-400" />
              Duplication Matrix ({summary?.duplication_matrix?.length || 0})
            </span>
          </button>

          <button
            onClick={() => setActiveTab("clusters")}
            className={`px-3 py-1.5 text-xs font-medium rounded-t border-t border-x transition-colors ${
              activeTab === "clusters"
                ? "bg-[#1e1e1e] text-white border-[#444444]"
                : "text-gray-400 border-transparent hover:text-gray-200"
            }`}
          >
            <span className="flex items-center gap-1.5">
              <Package className="w-3.5 h-3.5 text-emerald-400" />
              Cross-Repo Extraction ({summary?.clusters?.length || 0})
            </span>
          </button>
        </div>

        {/* Tab Content */}
        <div className="flex-1 p-4 overflow-y-auto">
          {activeTab === "repos" && (
            <div className="space-y-3">
              {summary?.repos?.map((repo) => (
                <div
                  key={repo.name}
                  className="flex items-center justify-between p-3 bg-[#252526] rounded border border-[#333333] hover:border-[#444444] transition-colors"
                >
                  <div className="flex items-center gap-3">
                    <FolderGit2 className="w-4 h-4 text-blue-400" />
                    <div>
                      <div className="text-xs font-semibold text-white">{repo.name}</div>
                      <div className="text-[10px] text-gray-400">
                        {repo.path} • {repo.tech_stack}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4 text-right">
                    <div>
                      <div className="text-[10px] text-gray-400">Files</div>
                      <div className="text-xs font-semibold text-gray-200">{repo.total_files}</div>
                    </div>
                    <div>
                      <div className="text-[10px] text-gray-400">Cross-Dup</div>
                      <div className="text-xs font-semibold text-amber-400">
                        {(repo.cross_repo_duplication_percentage ?? 0.0).toFixed(1)}%
                      </div>
                    </div>
                    <div>
                      <div className="text-[10px] text-gray-400">DRY Score</div>
                      <div className="text-xs font-semibold text-emerald-400">
                        {(repo.dry_health_score ?? 100.0).toFixed(1)}
                      </div>
                    </div>
                  </div>
                </div>
              ))}
              {(!summary?.repos || summary.repos.length === 0) && (
                <div className="text-center py-10 text-gray-500">
                  No repositories configured in .cddmhub.toml
                </div>
              )}
            </div>
          )}

          {activeTab === "matrix" && (
            <div className="space-y-3">
              <div className="text-xs text-gray-400 mb-2">
                Shared clone clusters and token overlap correlated across repository boundaries:
              </div>
              {summary?.duplication_matrix.map((pair, idx) => (
                <div
                  key={idx}
                  className="flex items-center justify-between p-3 bg-[#252526] rounded border border-[#333333]"
                >
                  <div className="flex items-center gap-2">
                    <span className="font-semibold text-blue-400">{pair.repo_a}</span>
                    <span className="text-gray-500">⇄</span>
                    <span className="font-semibold text-purple-400">{pair.repo_b}</span>
                  </div>
                  <div className="flex items-center gap-4">
                    <span className="px-2 py-0.5 bg-indigo-950 text-indigo-300 rounded border border-indigo-800 text-[10px]">
                      {pair.shared_clones} Shared Clusters
                    </span>
                    <span className="text-gray-400 text-[11px]">
                      {pair.shared_tokens} Duplicate Tokens
                    </span>
                  </div>
                </div>
              ))}
              {(!summary?.duplication_matrix || summary.duplication_matrix.length === 0) && (
                <div className="text-center py-10 text-gray-500">
                  Zero cross-repository duplication detected across federation repositories.
                </div>
              )}
            </div>
          )}

          {activeTab === "clusters" && (
            <div className="space-y-4">
              {extractResult && (
                <div className="p-3 bg-emerald-950/40 border border-emerald-800 rounded text-emerald-200">
                  <div className="flex items-center gap-2 font-semibold text-emerald-300">
                    <CheckCircle2 className="w-4 h-4" />
                    Package Extraction Synthesized: {extractResult.package_name} (
                    {extractResult.package_type})
                  </div>
                  <div className="mt-1 text-[11px] text-gray-300">
                    {extractResult.summary} • {extractResult.lines_saved} lines consolidated across{" "}
                    {extractResult.repo_updates.length} repositories.
                  </div>
                </div>
              )}

              {summary?.clusters?.map((cluster) => (
                <div
                  key={cluster.id}
                  className="p-3 bg-[#252526] rounded border border-[#333333] space-y-2"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-0.5 bg-blue-900 text-blue-200 rounded font-bold text-[10px]">
                        Cluster #{cluster.id}
                      </span>
                      <span className="text-gray-300 font-medium">{cluster.suggested_package}</span>
                      <span className="text-gray-500 text-[10px]">
                        ({cluster.token_count} tokens •{" "}
                        {((cluster.similarity ?? 1.0) * 100).toFixed(0)}% similarity)
                      </span>
                    </div>

                    <button
                      onClick={() => void handleExtract(cluster)}
                      disabled={extractingClusterId === cluster.id}
                      className="flex items-center gap-1 px-2.5 py-1 bg-emerald-700 hover:bg-emerald-600 text-white rounded text-[11px] font-medium transition-colors disabled:opacity-50"
                    >
                      <Sparkles className="w-3 h-3" />
                      {extractingClusterId === cluster.id
                        ? "Synthesizing..."
                        : "Extract Shared Package"}
                    </button>
                  </div>

                  <div className="space-y-1 pl-2 border-l-2 border-indigo-800">
                    {cluster.occurrences.map((occ, oIdx) => (
                      <div key={oIdx} className="text-[11px] text-gray-400">
                        <span className="font-semibold text-gray-300">{occ.repo_name}:</span>{" "}
                        {occ.file_path}:{occ.start_line}-{occ.end_line}
                      </div>
                    ))}
                  </div>
                </div>
              ))}

              {(!summary?.clusters || summary.clusters.length === 0) && (
                <div className="text-center py-10 text-gray-500">
                  No cross-repository clusters requiring standalone package extraction.
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </Win2xWindow>
  );
};
