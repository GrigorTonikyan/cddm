import React from "react";
import type { CachePackSummary } from "../../types/scan-types";
import { CheckCircle2 } from "lucide-react";

export interface CachePackCardProps {
  title: string;
  description: string;
  icon: React.ReactNode;
  firstFieldLabel: string;
  firstFieldValue: string;
  firstFieldId: string;
  onFirstFieldChange: (val: string) => void;
  secondFieldLabel: string;
  secondFieldValue: string;
  secondFieldId: string;
  onSecondFieldChange: (val: string) => void;
  summary: CachePackSummary | null;
  summaryTheme: "indigo" | "purple";
  isSubmitting: boolean;
  submitButtonText: string;
  submittingText: string;
  onSubmit: () => void;
  submitButtonTheme: "indigo" | "purple";
  submitIcon: React.ReactNode;
}

export const CachePackCard: React.FC<CachePackCardProps> = ({
  title,
  description,
  icon,
  firstFieldLabel,
  firstFieldValue,
  firstFieldId,
  onFirstFieldChange,
  secondFieldLabel,
  secondFieldValue,
  secondFieldId,
  onSecondFieldChange,
  summary,
  summaryTheme,
  isSubmitting,
  submitButtonText,
  submittingText,
  onSubmit,
  submitButtonTheme,
  submitIcon,
}) => {
  const summaryBg =
    summaryTheme === "indigo"
      ? "bg-indigo-950/60 border-indigo-800/60 text-indigo-200"
      : "bg-purple-950/60 border-purple-800/60 text-purple-200";

  const btnBg =
    submitButtonTheme === "indigo"
      ? "bg-indigo-600 hover:bg-indigo-500"
      : "bg-purple-600 hover:bg-purple-500";

  return (
    <div className="bg-slate-900/70 border border-slate-800 rounded-xl p-5 flex flex-col justify-between space-y-4">
      <div>
        <div className="flex items-center gap-2 font-bold text-sm text-slate-200 mb-2">
          {icon}
          <span>{title}</span>
        </div>
        <p className="text-xs text-slate-400 mb-4">{description}</p>

        <div className="space-y-3">
          <div>
            <label htmlFor={firstFieldId} className="block text-xs text-slate-400 mb-1">
              {firstFieldLabel}
            </label>
            <input
              id={firstFieldId}
              type="text"
              value={firstFieldValue}
              onChange={(e) => onFirstFieldChange(e.target.value)}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
            />
          </div>
          <div>
            <label htmlFor={secondFieldId} className="block text-xs text-slate-400 mb-1">
              {secondFieldLabel}
            </label>
            <input
              id={secondFieldId}
              type="text"
              value={secondFieldValue}
              onChange={(e) => onSecondFieldChange(e.target.value)}
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
            />
          </div>
        </div>
      </div>

      <div>
        {summary && (
          <div className={`mb-3 p-2.5 border rounded-lg text-xs font-mono space-y-1 ${summaryBg}`}>
            <div className="flex items-center gap-1.5 text-emerald-400 font-semibold">
              <CheckCircle2 className="w-3.5 h-3.5" />
              <span>{summary.message || "Operation Completed"}</span>
            </div>
            <div>Entries: {summary.entry_count}</div>
            <div className="truncate">File: {summary.pack_file}</div>
          </div>
        )}

        <button
          type="button"
          onClick={onSubmit}
          disabled={isSubmitting}
          className={`w-full py-2 px-4 ${btnBg} text-white rounded-lg text-xs font-semibold transition-colors flex items-center justify-center gap-2 disabled:opacity-50`}
        >
          {submitIcon}
          <span>{isSubmitting ? submittingText : submitButtonText}</span>
        </button>
      </div>
    </div>
  );
};
