import React from "react";
import type { VerifyRefactorResult } from "../../types/cddm-types";
import { Play, RefreshCw } from "lucide-react";

export interface TestVerificationPanelProps {
  showVerifyOutput: boolean;
  isVerifying: boolean;
  verifyResult: VerifyRefactorResult | null;
  verifyError: string | null;
}

export const TestVerificationPanel: React.FC<TestVerificationPanelProps> = ({
  showVerifyOutput,
  isVerifying,
  verifyResult,
  verifyError,
}) => {
  if (!showVerifyOutput) return null;

  return (
    <div className="p-3.5 bg-slate-900/90 border border-slate-800 rounded-xl space-y-2 font-mono text-xs">
      <div className="flex items-center justify-between">
        <span className="text-slate-200 font-semibold flex items-center gap-2">
          <Play className="w-3.5 h-3.5 text-emerald-400" />
          Test Suite Verification
        </span>
        {isVerifying ? (
          <span className="text-slate-400 flex items-center gap-1.5">
            <RefreshCw className="w-3 h-3 animate-spin text-emerald-400" />
            Running test suite...
          </span>
        ) : verifyResult ? (
          <span
            className={`font-bold px-2 py-0.5 rounded text-[11px] border ${
              verifyResult.success
                ? "bg-emerald-950/80 text-emerald-400 border-emerald-800/60"
                : "bg-rose-950/80 text-rose-400 border-rose-800/60"
            }`}
          >
            {verifyResult.success ? "[PASS]" : "[FAIL]"} (exit {verifyResult.exit_code} in{" "}
            {verifyResult.duration_ms}ms)
          </span>
        ) : null}
      </div>

      {verifyError && (
        <div className="p-2 bg-rose-950/40 border border-rose-900/50 rounded text-rose-300 text-[11px]">
          {verifyError}
        </div>
      )}

      {verifyResult && (
        <div className="space-y-1.5">
          <div className="text-slate-400 text-[11px]">
            Command:{" "}
            <span className="text-slate-200 font-semibold">{verifyResult.command_executed}</span>
          </div>
          <div className="p-2.5 bg-slate-950 border border-slate-800 rounded-lg max-h-[140px] overflow-y-auto font-mono text-[11px] text-slate-300 leading-normal">
            <pre>
              {verifyResult.stdout_snippet || verifyResult.stderr_snippet || verifyResult.message}
            </pre>
          </div>
        </div>
      )}
    </div>
  );
};
