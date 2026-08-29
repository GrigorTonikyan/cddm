import React from "react";
import { DEFAULT_FAIL_THRESHOLD } from "../constants/cddm-constants";
import { ScanResult } from "../types/cddm-types";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  Award,
  ShieldCheck,
  AlertTriangle,
  CheckCircle2,
  Copy,
  Layers,
  ArrowRight,
} from "lucide-react";
import { ModalFooter } from "./ui/ModalFooter";

export interface HealthAuditModalProps {
  isOpen: boolean;
  onClose: () => void;
  results: ScanResult;
}

export const HealthAuditModal: React.FC<HealthAuditModalProps> = ({ isOpen, onClose, results }) => {
  if (!isOpen) return null;

  const isHealthy = results.dry_health_score >= 80;
  const isModerate = results.dry_health_score >= 60;

  const scoreColor = isHealthy
    ? "text-emerald-400 border-emerald-500/40 bg-emerald-950/20"
    : isModerate
      ? "text-amber-400 border-amber-500/40 bg-amber-950/20"
      : "text-rose-400 border-rose-500/40 bg-rose-950/20";

  const qualityGatePass = results.duplication_percentage <= DEFAULT_FAIL_THRESHOLD;

  return (
    <Win2xWindow
      id="cddm-health-audit-window"
      windowType="health-audit"
      isOpen={isOpen}
      onClose={onClose}
      title="DRY Health Score Audit & Diagnostics"
      subtitle="Architectural health rating, redundancy penalties, and quality gate analysis"
      badge={`Score: ${results.dry_health_score.toFixed(1)}/100`}
      icon={<Award className="w-4 h-4 text-indigo-400" />}
      footer={
        <ModalFooter
          infoIcon={<ShieldCheck className="w-3.5 h-3.5 text-indigo-400" />}
          infoText={`Target Quality Gate: < ${DEFAULT_FAIL_THRESHOLD.toFixed(1)}% duplication`}
          onClose={onClose}
        />
      }
      initialWidth={880}
      initialHeight={640}
    >
      <div className="space-y-5">
        {/* Main Score & Quality Gate Banner */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* DRY Health Card */}
          <div className={`p-5 rounded-xl border flex flex-col justify-between ${scoreColor}`}>
            <div>
              <div className="flex items-center justify-between text-xs font-bold uppercase tracking-wider">
                <span>Architectural DRY Health</span>
                <Award className="w-5 h-5" />
              </div>
              <div className="mt-3 flex items-baseline gap-2">
                <span className="text-4xl font-extrabold font-mono">
                  {results.dry_health_score.toFixed(1)}
                </span>
                <span className="text-sm opacity-60">/ 100</span>
              </div>
              <p className="text-xs text-slate-400 mt-2">
                {isHealthy
                  ? "Excellent health rating. Codebase maintains high modularity with minimal duplication."
                  : isModerate
                    ? "Moderate health rating. Notable duplication hotspots identified for refactoring."
                    : "Low health rating. Significant copy-paste redundancy detected across subsystems."}
              </p>
            </div>

            <div className="w-full bg-slate-900/60 rounded-full h-2 mt-4 overflow-hidden border border-slate-700/30">
              <div
                className={`h-full transition-all duration-500 ${
                  isHealthy ? "bg-emerald-400" : isModerate ? "bg-amber-400" : "bg-rose-400"
                }`}
                style={{
                  width: `${Math.min(100, Math.max(0, results.dry_health_score))}%`,
                }}
              />
            </div>
          </div>

          {/* Quality Gate Status Card */}
          <div className="bg-slate-950/80 p-5 rounded-xl border border-slate-800 flex flex-col justify-between">
            <div>
              <div className="flex items-center justify-between text-xs font-bold uppercase tracking-wider text-slate-400">
                <span>CI Quality Gate Status</span>
                {qualityGatePass ? (
                  <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                ) : (
                  <AlertTriangle className="w-5 h-5 text-rose-400" />
                )}
              </div>

              <div className="mt-3 flex items-center gap-2">
                <span
                  className={`text-xs font-mono font-bold px-3 py-1 rounded-lg border ${
                    qualityGatePass
                      ? "bg-emerald-950/80 text-emerald-300 border-emerald-800/60"
                      : "bg-rose-950/80 text-rose-300 border-rose-800/60"
                  }`}
                >
                  {qualityGatePass ? "[PASS]" : "[FAIL]"} Threshold
                </span>
                <span className="font-mono text-sm text-slate-300">
                  {results.duplication_percentage.toFixed(2)}% Duplication
                </span>
              </div>

              <p className="text-xs text-slate-400 mt-2">
                {qualityGatePass
                  ? `Codebase satisfies the strict <= ${DEFAULT_FAIL_THRESHOLD.toFixed(1)}% dogfooding quality threshold standard.`
                  : `Duplication exceeds the ${DEFAULT_FAIL_THRESHOLD.toFixed(1)}% threshold standard. Automated CI scans will fail.`}
              </p>
            </div>

            <div className="pt-3 border-t border-slate-800/60 flex items-center justify-between text-xs font-mono text-slate-400">
              <span>Threshold Standard:</span>
              <span className="text-indigo-300 font-bold">
                &le; {DEFAULT_FAIL_THRESHOLD.toFixed(1)}% max
              </span>
            </div>
          </div>
        </div>

        {/* Breakdown Factors Grid */}
        <div className="bg-slate-950/80 p-4 rounded-xl border border-slate-800 space-y-3">
          <h4 className="text-xs font-bold uppercase tracking-wider text-slate-300 font-mono flex items-center gap-2">
            <Layers className="w-4 h-4 text-indigo-400" />
            <span>Audit Metrics & Remediation Priorities</span>
          </h4>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 text-xs font-mono">
            <div className="bg-slate-900/60 p-3 rounded-lg border border-slate-800 space-y-1">
              <span className="text-slate-500">Duplicate Clones</span>
              <div className="font-bold text-slate-200">
                {results.total_clones.toLocaleString()} clone pairs
              </div>
            </div>

            <div className="bg-slate-900/60 p-3 rounded-lg border border-slate-800 space-y-1">
              <span className="text-slate-500">Redundant Tokens</span>
              <div className="font-bold text-indigo-300">
                {((results.total_tokens * results.duplication_percentage) / 100).toFixed(0)} tokens
              </div>
            </div>

            <div className="bg-slate-900/60 p-3 rounded-lg border border-slate-800 space-y-1">
              <span className="text-slate-500">Scan Duration</span>
              <div className="font-bold text-slate-200">{results.duration_ms} ms</div>
            </div>
          </div>
        </div>

        {/* Actionable Recommendations */}
        <div className="bg-slate-950/80 p-4 rounded-xl border border-slate-800 space-y-3">
          <h4 className="text-xs font-bold uppercase tracking-wider text-slate-300 font-mono flex items-center gap-2">
            <Copy className="w-4 h-4 text-indigo-400" />
            <span>Recommended Action Items</span>
          </h4>

          <div className="space-y-2 text-xs font-mono text-slate-300">
            <div className="p-3 bg-slate-900/40 border border-slate-800/80 rounded-lg flex items-start gap-2.5">
              <ArrowRight className="w-4 h-4 text-indigo-400 shrink-0 mt-0.5" />
              <div>
                <strong className="text-slate-100">Target Top Duplication Hotspots</strong>
                <p className="text-slate-400 text-[11px] mt-0.5">
                  Launch the Refactor Advisor on the top clone pairs to synthesize extract-function
                  patches.
                </p>
              </div>
            </div>

            <div className="p-3 bg-slate-900/40 border border-slate-800/80 rounded-lg flex items-start gap-2.5">
              <ArrowRight className="w-4 h-4 text-indigo-400 shrink-0 mt-0.5" />
              <div>
                <strong className="text-slate-100">
                  Integrate SARIF Export into GitHub Code Scanning
                </strong>
                <p className="text-slate-400 text-[11px] mt-0.5">
                  Export OASIS SARIF v2.1.0 diagnostics to enforce deduplication quality gates on
                  pull requests.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Win2xWindow>
  );
};
