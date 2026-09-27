import React from "react";
import { CollapsibleCard, BADGE_VARIANTS } from "../ui";
import { Layers } from "lucide-react";
import { parsePath } from "../../utils/path-utils";
import { ClusterRefactorSuggestion } from "../../types/cddm-types";

interface RefactorClusterSitesProps {
  clusterSuggestion: ClusterRefactorSuggestion;
}

export const RefactorClusterSites: React.FC<RefactorClusterSitesProps> = ({
  clusterSuggestion,
}) => {
  return (
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
                  <span className="text-slate-100 font-semibold">{siteParsed.filename}</span>
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
  );
};
