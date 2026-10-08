import React, { useState } from "react";
import { useCDDMStore } from "../store/cddm-store";
import { Win2xWindow } from "./ui/win2x-manager";
import {
  GitBranch,
  ShieldCheck,
  CheckCircle2,
  XCircle,
  Copy,
  Check,
  Terminal,
  FileCode,
  RefreshCw,
  Sliders,
} from "lucide-react";

export interface HookManagerModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const GITEA_CI_SNIPPET = `name: CDDM Duplication Quality Gate
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  duplication-gate:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install CDDM CLI
        run: cargo install cddm-cli --locked

      - name: Run CDDM Duplication Scan
        run: cddm scan . --min-tokens 50 --fail-threshold 5.0 --enable-cache
`;

export const GITHUB_CI_SNIPPET = `name: CDDM Duplication Quality Gate
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  duplication-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Run CDDM Scan
        run: |
          cargo install cddm-cli --locked
          cddm scan . --min-tokens 50 --fail-threshold 5.0
`;

export const PRE_COMMIT_HOOK_SCRIPT = `#!/usr/bin/env sh
# CDDM Quality Gate Pre-commit Hook
# Prevents duplicate code from being committed to repository

if command -v cddm >/dev/null 2>&1; then
    echo "Running CDDM pre-commit duplication check..."
    cddm scan . --min-tokens 50 --fail-threshold 5.0 --enable-cache || {
        echo "CDDM Quality Gate failed: Duplication exceeds threshold!"
        exit 1
    }
fi
`;

export const HookManagerModal: React.FC<HookManagerModalProps> = ({ isOpen, onClose }) => {
  const { hookStatus, isTimelineLoading, fetchHookStatus, installHook, config } = useCDDMStore();

  const [activeTab, setActiveTab] = useState<"hooks" | "workflows">("hooks");
  const [workflowType, setWorkflowType] = useState<"gitea" | "github" | "script">("gitea");
  const [failThreshold, setFailThreshold] = useState<number>(config.fail_threshold ?? 5.0);
  const [minTokens, setMinTokens] = useState<number>(config.min_tokens ?? 50);
  const [installMessage, setInstallMessage] = useState<string | null>(null);
  const [isInstalling, setIsInstalling] = useState<boolean>(false);
  const [copied, setCopied] = useState<boolean>(false);

  if (!isOpen) return null;

  const handleInstall = async (hookType: "pre-commit" | "pre-push") => {
    setIsInstalling(true);
    setInstallMessage(null);
    try {
      const msg = await installHook(hookType, failThreshold, minTokens);
      setInstallMessage(msg);
      await fetchHookStatus();
    } catch (err) {
      setInstallMessage(err instanceof Error ? err.message : `Failed to install ${hookType} hook`);
    } finally {
      setIsInstalling(false);
    }
  };

  const getActiveSnippet = () => {
    switch (workflowType) {
      case "gitea":
        return GITEA_CI_SNIPPET;
      case "github":
        return GITHUB_CI_SNIPPET;
      case "script":
        return PRE_COMMIT_HOOK_SCRIPT;
    }
  };

  const handleCopySnippet = async () => {
    try {
      await navigator.clipboard.writeText(getActiveSnippet());
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // Fallback
    }
  };

  const isPreCommitActive = Boolean(hookStatus?.pre_commit_installed);
  const isPrePushActive = Boolean(hookStatus?.pre_push_installed);

  const footerContent = (
    <div className="flex items-center justify-between w-full">
      <button
        type="button"
        onClick={() => void fetchHookStatus()}
        disabled={isTimelineLoading}
        className="flex items-center gap-1.5 px-3 py-1.5 text-xs text-slate-400 hover:text-slate-200 transition-colors disabled:opacity-50"
      >
        <RefreshCw className={`w-3.5 h-3.5 ${isTimelineLoading ? "animate-spin" : ""}`} />
        <span>Refresh Status</span>
      </button>

      <button
        type="button"
        onClick={onClose}
        className="px-4 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold text-xs transition-colors"
      >
        Close
      </button>
    </div>
  );

  return (
    <Win2xWindow
      id="hook-manager-modal"
      windowType="hook-manager"
      isOpen={isOpen}
      onClose={onClose}
      title="Git Hook Manager & CI/CD Studio"
      subtitle="Configure local pre-commit/pre-push gates, view installation state, and generate turnkey CI pipelines"
      badge={isPreCommitActive ? "Protected" : "Unguarded"}
      icon={<ShieldCheck className="w-4 h-4 text-emerald-400" />}
      footer={footerContent}
      initialWidth={800}
      initialHeight={600}
    >
      <div className="space-y-6">
        {/* Navigation Tabs */}
        <div className="flex items-center gap-2 border-b border-slate-800 pb-2">
          <button
            type="button"
            onClick={() => setActiveTab("hooks")}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              activeTab === "hooks"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <GitBranch className="w-3.5 h-3.5" />
            <span>Local Git Hooks</span>
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("workflows")}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              activeTab === "workflows"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-600/30"
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/60"
            }`}
          >
            <FileCode className="w-3.5 h-3.5" />
            <span>Turnkey CI/CD Workflows</span>
          </button>
        </div>

        {installMessage && (
          <div className="p-3 bg-indigo-950/60 border border-indigo-500/40 rounded-lg text-xs text-indigo-200 flex items-center gap-2">
            <CheckCircle2 className="w-4 h-4 text-indigo-400 shrink-0" />
            <span>{installMessage}</span>
          </div>
        )}

        {activeTab === "hooks" ? (
          <div className="space-y-6">
            {/* Threshold & Parameters Tuning */}
            <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-4 space-y-4">
              <div className="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-slate-300">
                <Sliders className="w-4 h-4 text-indigo-400" />
                <span>Gate Parameters</span>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label
                    htmlFor="hook-fail-threshold"
                    className="block text-xs text-slate-400 mb-1"
                  >
                    Fail Threshold (% Duplication):
                  </label>
                  <input
                    id="hook-fail-threshold"
                    type="number"
                    step="0.5"
                    min="0.5"
                    max="50"
                    value={failThreshold}
                    onChange={(e) => setFailThreshold(Number(e.target.value))}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
                  />
                </div>
                <div>
                  <label htmlFor="hook-min-tokens" className="block text-xs text-slate-400 mb-1">
                    Minimum Tokens:
                  </label>
                  <input
                    id="hook-min-tokens"
                    type="number"
                    step="5"
                    min="10"
                    max="500"
                    value={minTokens}
                    onChange={(e) => setMinTokens(Number(e.target.value))}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-1.5 text-xs font-mono text-slate-200 focus:outline-none focus:border-indigo-500"
                  />
                </div>
              </div>
            </div>

            {/* Hook Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {[
                {
                  type: "pre-commit" as const,
                  title: "Pre-Commit Hook",
                  desc: "Executes quick delta scan prior to every commit. Blocks commits that violate the DRY duplication threshold.",
                  isActive: isPreCommitActive,
                  iconColor: "text-indigo-400",
                  btnColor: "bg-indigo-600 hover:bg-indigo-500",
                },
                {
                  type: "pre-push" as const,
                  title: "Pre-Push Hook",
                  desc: "Verifies whole-repository health score before pushing to remote origin. Prevents upstream CI gate failures.",
                  isActive: isPrePushActive,
                  iconColor: "text-purple-400",
                  btnColor: "bg-purple-600 hover:bg-purple-500",
                },
              ].map((hook) => (
                <div
                  key={hook.type}
                  className="bg-slate-900/70 border border-slate-800 rounded-xl p-4 flex flex-col justify-between"
                >
                  <div>
                    <div className="flex items-center justify-between mb-2">
                      <div className="flex items-center gap-2 font-bold text-sm text-slate-200">
                        <Terminal className={`w-4 h-4 ${hook.iconColor}`} />
                        <span>{hook.title}</span>
                      </div>
                      {hook.isActive ? (
                        <span className="flex items-center gap-1 text-[11px] font-semibold text-emerald-400 bg-emerald-950/60 border border-emerald-800/60 px-2 py-0.5 rounded-full">
                          <CheckCircle2 className="w-3 h-3" />
                          Installed
                        </span>
                      ) : (
                        <span className="flex items-center gap-1 text-[11px] font-semibold text-amber-400 bg-amber-950/60 border border-amber-800/60 px-2 py-0.5 rounded-full">
                          <XCircle className="w-3 h-3" />
                          Not Installed
                        </span>
                      )}
                    </div>
                    <p className="text-xs text-slate-400 mb-4">{hook.desc}</p>
                  </div>
                  <button
                    type="button"
                    onClick={() => void handleInstall(hook.type)}
                    disabled={isInstalling}
                    className={`w-full py-2 px-3 rounded-lg text-xs font-bold ${hook.btnColor} text-white transition-colors flex items-center justify-center gap-2 disabled:opacity-50`}
                  >
                    <ShieldCheck className="w-3.5 h-3.5" />
                    <span>
                      {hook.isActive ? `Reinstall ${hook.title}` : `Install ${hook.title}`}
                    </span>
                  </button>
                </div>
              ))}
            </div>

            {/* Target Directory info */}
            {hookStatus?.hooks_dir && (
              <div className="text-[11px] font-mono text-slate-500 flex items-center gap-1.5">
                <span>Hook directory:</span>
                <span className="text-slate-400">{hookStatus.hooks_dir}</span>
              </div>
            )}
          </div>
        ) : (
          <div className="space-y-4">
            {/* Snippet type tabs */}
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() => setWorkflowType("gitea")}
                  className={`px-3 py-1 rounded text-xs font-mono font-medium transition-colors ${
                    workflowType === "gitea"
                      ? "bg-slate-800 text-indigo-300 border border-indigo-500/50"
                      : "text-slate-400 hover:text-slate-200"
                  }`}
                >
                  Gitea Actions (.gitea/workflows/cddm.yaml)
                </button>
                <button
                  type="button"
                  onClick={() => setWorkflowType("github")}
                  className={`px-3 py-1 rounded text-xs font-mono font-medium transition-colors ${
                    workflowType === "github"
                      ? "bg-slate-800 text-indigo-300 border border-indigo-500/50"
                      : "text-slate-400 hover:text-slate-200"
                  }`}
                >
                  GitHub Actions (.github/workflows/cddm.yml)
                </button>
                <button
                  type="button"
                  onClick={() => setWorkflowType("script")}
                  className={`px-3 py-1 rounded text-xs font-mono font-medium transition-colors ${
                    workflowType === "script"
                      ? "bg-slate-800 text-indigo-300 border border-indigo-500/50"
                      : "text-slate-400 hover:text-slate-200"
                  }`}
                >
                  Shell Script (.git/hooks/pre-commit)
                </button>
              </div>

              <button
                type="button"
                onClick={() => void handleCopySnippet()}
                className="flex items-center gap-1.5 px-3 py-1 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded text-xs font-medium transition-colors"
              >
                {copied ? (
                  <>
                    <Check className="w-3.5 h-3.5 text-emerald-400" />
                    <span className="text-emerald-300">Copied</span>
                  </>
                ) : (
                  <>
                    <Copy className="w-3.5 h-3.5" />
                    <span>Copy Snippet</span>
                  </>
                )}
              </button>
            </div>

            {/* Code Block */}
            <div className="bg-slate-950 border border-slate-800 rounded-xl p-4 font-mono text-xs text-slate-200 overflow-x-auto max-h-[320px]">
              <pre>{getActiveSnippet()}</pre>
            </div>
          </div>
        )}
      </div>
    </Win2xWindow>
  );
};
