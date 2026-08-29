import React from "react";
import { GitCommit, Tag } from "lucide-react";
import type { TimelineSnapshot } from "../../types/cddm-types";

export interface CommitHistoryTableProps {
  snapshots: TimelineSnapshot[];
  hoveredSnapshot: TimelineSnapshot | null;
  setHoveredSnapshot: (s: TimelineSnapshot | null) => void;
}

export const CommitHistoryTable: React.FC<CommitHistoryTableProps> = ({
  snapshots,
  hoveredSnapshot,
  setHoveredSnapshot,
}) => {
  return (
    <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl overflow-hidden shadow-lg">
      <div className="px-4 py-3 border-b border-slate-800 flex items-center justify-between">
        <span className="text-xs font-bold text-slate-300 uppercase tracking-wider">
          Commit Checkpoints History
        </span>
        <span className="text-[11px] font-mono text-slate-500">
          {snapshots.length} Commits Sampled
        </span>
      </div>
      <div className="max-h-56 overflow-y-auto">
        <table className="w-full text-left text-xs font-mono">
          <thead className="bg-slate-950/80 text-slate-400 uppercase text-[10px] tracking-wider sticky top-0 border-b border-slate-800">
            <tr>
              <th className="py-2.5 px-3">Commit</th>
              <th className="py-2.5 px-3">Date</th>
              <th className="py-2.5 px-3">Author</th>
              <th className="py-2.5 px-3">Message</th>
              <th className="py-2.5 px-3 text-right">DRY Score</th>
              <th className="py-2.5 px-3 text-right">Duplication</th>
              <th className="py-2.5 px-3 text-right">Clones</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/50 text-slate-300">
            {snapshots.map((s) => {
              const isHovered = hoveredSnapshot?.commit_hash === s.commit_hash;
              return (
                <tr
                  key={s.commit_hash}
                  onMouseEnter={() => setHoveredSnapshot(s)}
                  onMouseLeave={() => setHoveredSnapshot(null)}
                  className={`transition-colors cursor-pointer ${
                    isHovered ? "bg-slate-800/60 text-slate-100" : "hover:bg-slate-800/30"
                  }`}
                >
                  <td className="py-2 px-3">
                    <div className="flex items-center gap-1.5 font-bold text-indigo-300">
                      <GitCommit className="w-3 h-3 text-indigo-400" />
                      <span>{s.short_hash}</span>
                      {s.tag && (
                        <span className="bg-purple-950 text-purple-300 text-[10px] px-1.5 py-0.2 rounded border border-purple-800/50 flex items-center gap-0.5">
                          <Tag className="w-2.5 h-2.5" />
                          {s.tag}
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="py-2 px-3 text-slate-400 text-[11px]">{s.formatted_date}</td>
                  <td className="py-2 px-3 text-slate-400 truncate max-w-[100px]">{s.author}</td>
                  <td className="py-2 px-3 text-slate-200 truncate max-w-[180px]">{s.message}</td>
                  <td className="py-2 px-3 text-right font-bold text-emerald-400">
                    {s.dry_health_score.toFixed(1)}
                  </td>
                  <td className="py-2 px-3 text-right text-rose-400">
                    {s.duplication_percentage.toFixed(1)}%
                  </td>
                  <td className="py-2 px-3 text-right text-slate-400">{s.total_clones}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
};
