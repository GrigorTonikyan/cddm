import React from "react";
import { LanguageStats } from "../types/cddm-types";
import { getLanguageStyle } from "../utils/path-utils";
import { Win2xWindow } from "./ui/win2x-manager";
import { PieChart, Sparkles, Layers, FileCode } from "lucide-react";

import { ModalFooter } from "./ui/ModalFooter";

export interface LanguageAnalyticsModalProps {
  isOpen: boolean;
  onClose: () => void;
  languages: LanguageStats[];
  totalTokens: number;
  totalFiles: number;
}

export const LanguageAnalyticsModal: React.FC<LanguageAnalyticsModalProps> = ({
  isOpen,
  onClose,
  languages,
  totalTokens,
  totalFiles,
}) => {
  if (!isOpen) return null;

  const totalTokensAllLangs =
    totalTokens > 0 ? totalTokens : languages.reduce((sum, item) => sum + item.tokens, 0);

  return (
    <Win2xWindow
      id="cddm-language-analytics-window"
      windowType="language-analytics"
      isOpen={isOpen}
      onClose={onClose}
      title="Language & Architectural Composition"
      subtitle="Multi-language token distribution, file densities, and duplication metrics"
      badge={`${languages.length} Languages`}
      icon={<PieChart className="w-4 h-4 text-indigo-400" />}
      footer={
        <ModalFooter
          infoIcon={<Sparkles className="w-3.5 h-3.5 text-indigo-400" />}
          infoText={`${languages.length} programming language ecosystem composition`}
          onClose={onClose}
        />
      }
      initialWidth={920}
      initialHeight={650}
    >
      <div className="space-y-5">
        {/* Top Summary Metrics */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
          <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
            <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
              <PieChart className="w-3.5 h-3.5 text-indigo-400" />
              Languages Detected
            </span>
            <div className="font-mono text-lg font-bold text-indigo-300">
              {languages.length} ecosystems
            </div>
          </div>

          <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
            <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
              <FileCode className="w-3.5 h-3.5 text-emerald-400" />
              Total Source Files
            </span>
            <div className="font-mono text-lg font-bold text-emerald-400">
              {totalFiles.toLocaleString()} files
            </div>
          </div>

          <div className="bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 space-y-1">
            <span className="text-[11px] font-mono text-slate-500 uppercase tracking-wider flex items-center gap-1.5">
              <Layers className="w-3.5 h-3.5 text-amber-400" />
              Indexed Tokens
            </span>
            <div className="font-mono text-lg font-bold text-slate-200">
              {totalTokensAllLangs.toLocaleString()} tokens
            </div>
          </div>
        </div>

        {/* Segmented Distribution Bar */}
        <div className="bg-slate-950/80 p-4 rounded-xl border border-slate-800 space-y-3">
          <div className="flex items-center justify-between text-xs font-mono text-slate-400">
            <span className="font-bold uppercase tracking-wider text-slate-300">
              Token Share Distribution
            </span>
            <span>100% Codebase Coverage</span>
          </div>

          <div className="w-full h-4 bg-slate-900 rounded-full overflow-hidden flex border border-slate-800 shadow-inner">
            {languages.map((item) => {
              const style = getLanguageStyle(item.language);
              const pct = totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;
              return (
                <div
                  key={item.language}
                  className={`h-full ${style.bar} transition-all duration-300`}
                  style={{ width: `${pct}%` }}
                  title={`${item.language}: ${item.files} files (${pct.toFixed(1)}% tokens)`}
                />
              );
            })}
          </div>
        </div>

        {/* Detailed Table */}
        <div className="bg-slate-950/80 rounded-xl border border-slate-800 overflow-hidden">
          <div className="px-4 py-3 bg-slate-900/60 border-b border-slate-800 text-xs font-mono font-bold text-slate-300 uppercase tracking-wider">
            Language Composition Breakdown
          </div>

          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-slate-900/40 text-slate-400 border-b border-slate-800/60">
                <tr>
                  <th className="px-4 py-2.5">Language</th>
                  <th className="px-4 py-2.5">Files</th>
                  <th className="px-4 py-2.5">Tokens</th>
                  <th className="px-4 py-2.5">Token %</th>
                  <th className="px-4 py-2.5">Clones</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800/50 text-slate-300">
                {languages.map((item) => {
                  const style = getLanguageStyle(item.language);
                  const pct =
                    totalTokensAllLangs > 0 ? (item.tokens / totalTokensAllLangs) * 100 : 0;

                  return (
                    <tr key={item.language} className="hover:bg-slate-900/40 transition-colors">
                      <td className="px-4 py-2.5 flex items-center gap-2">
                        <span className={`w-2.5 h-2.5 rounded-full ${style.bar}`} />
                        <span className="font-semibold text-slate-100">{item.language}</span>
                      </td>
                      <td className="px-4 py-2.5">{item.files.toLocaleString()}</td>
                      <td className="px-4 py-2.5">{item.tokens.toLocaleString()}</td>
                      <td className="px-4 py-2.5 font-semibold text-indigo-300">
                        {pct.toFixed(1)}%
                      </td>
                      <td className="px-4 py-2.5">
                        <span className="px-2 py-0.5 rounded bg-slate-900 border border-slate-800 text-slate-300">
                          {item.clones.toLocaleString()}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </Win2xWindow>
  );
};
