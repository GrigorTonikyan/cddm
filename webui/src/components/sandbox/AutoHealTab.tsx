import React, { useState } from "react";
import { API_ROUTES, DEFAULT_HEAL_CONFIG } from "../../constants/cddm-constants";
import { useFormState } from "../../hooks/use-form-state";
import type {
  AiProviderKind,
  CloneLocation,
  HealIterationLog,
  HealRefactorRequest,
  HealRefactorResult,
} from "../../types/cddm-types";
import {
  AlertCircle,
  Bot,
  Check,
  CheckCircle2,
  GitBranch,
  Play,
  RefreshCw,
  XCircle,
} from "lucide-react";

export interface AutoHealTabProps {
  occurrences: CloneLocation[];
  clusterId?: number;
  customFunctionName?: string;
  targetModulePath?: string;
}

interface AutoHealForm {
  provider: AiProviderKind;
  model: string;
  apiKey: string;
  endpoint: string;
  maxIterations: number;
  verify: boolean;
  testCmd: string;
  branch: string;
  customInstructions: string;
}

export const AutoHealTab: React.FC<AutoHealTabProps> = ({
  occurrences,
  clusterId,
  customFunctionName,
  targetModulePath,
}) => {
  const { form, updateField } = useFormState<AutoHealForm>({
    provider: DEFAULT_HEAL_CONFIG.default_provider as AiProviderKind,
    model: "",
    apiKey: "",
    endpoint: "",
    maxIterations: DEFAULT_HEAL_CONFIG.max_iterations,
    verify: DEFAULT_HEAL_CONFIG.verify,
    testCmd: "",
    branch: `cddm/heal-cluster-${clusterId || 1}`,
    customInstructions: "",
  });
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [healResult, setHealResult] = useState<HealRefactorResult | null>(null);
  const [healError, setHealError] = useState<string | null>(null);

  const handleStartHealing = async () => {
    setIsRunning(true);
    setHealError(null);
    setHealResult(null);

    const payload: HealRefactorRequest = {
      cluster_id: clusterId,
      occurrences,
      function_name: customFunctionName?.trim() || undefined,
      target_module: targetModulePath?.trim() || undefined,
      custom_instructions: form.customInstructions.trim() || undefined,
      provider_config: {
        provider: form.provider,
        model: form.model.trim() || undefined,
        api_key: form.apiKey.trim() || undefined,
        endpoint: form.endpoint.trim() || undefined,
      },
      max_iterations: form.maxIterations,
      apply_branch: form.branch.trim() || undefined,
      verify: form.verify,
      test_cmd: form.testCmd.trim() || undefined,
    };

    try {
      const res = await fetch(API_ROUTES.REFACTOR_HEAL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      if (!res.ok) {
        const txt = await res.text();
        throw new Error(txt || `HTTP ${res.status}`);
      }
      const data: HealRefactorResult = await res.json();
      setHealResult(data);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : String(e);
      setHealError(msg);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <div className="space-y-4 text-xs font-mono">
      <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-lg space-y-3">
        <div className="flex items-center gap-2 text-sm font-semibold text-emerald-400">
          <Bot className="w-4 h-4" />
          <span>AI Code Surgeon Closed-Loop Autonomous Refactoring</span>
        </div>
        <p className="text-zinc-400 text-xs">
          CDDM autonomously prompts the LLM, extracts AST patches, applies them to a sandbox branch,
          and continuously heals test failures in a closed repair loop.
        </p>

        <div className="grid grid-cols-2 sm:grid-cols-4 gap-2 pt-2">
          <div>
            <label className="text-zinc-400 block mb-1">Provider</label>
            <select
              value={form.provider}
              onChange={(e) => updateField("provider", e.target.value as AiProviderKind)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            >
              <option value="Mock">Mock / Deterministic</option>
              <option value="Gemini">Google Gemini (gemini-2.5-pro)</option>
              <option value="Claude">Anthropic Claude (claude-3-7-sonnet)</option>
              <option value="OpenAi">OpenAI (gpt-4.5-preview)</option>
              <option value="Ollama">Ollama Local (qwen2.5-coder)</option>
            </select>
          </div>

          <div>
            <label className="text-zinc-400 block mb-1">Model ID</label>
            <input
              type="text"
              placeholder={
                form.provider === "Ollama"
                  ? DEFAULT_HEAL_CONFIG.default_ollama_model
                  : form.provider === "Claude"
                    ? DEFAULT_HEAL_CONFIG.default_claude_model
                    : form.provider === "OpenAi"
                      ? DEFAULT_HEAL_CONFIG.default_openai_model
                      : DEFAULT_HEAL_CONFIG.default_gemini_model
              }
              value={form.model}
              onChange={(e) => updateField("model", e.target.value)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>

          <div>
            <label className="text-zinc-400 block mb-1">API Key / Token</label>
            <input
              type="password"
              placeholder="env var or key"
              value={form.apiKey}
              onChange={(e) => updateField("apiKey", e.target.value)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>

          <div>
            <label className="text-zinc-400 block mb-1">Max Iterations</label>
            <input
              type="number"
              min={1}
              max={10}
              value={form.maxIterations}
              onChange={(e) => updateField("maxIterations", Number(e.target.value))}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
          <div>
            <label className="text-zinc-400 block mb-1">Custom Endpoint URL</label>
            <input
              type="text"
              placeholder={`e.g. ${DEFAULT_HEAL_CONFIG.default_ollama_endpoint}`}
              value={form.endpoint}
              onChange={(e) => updateField("endpoint", e.target.value)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>
          <div>
            <label className="text-zinc-400 block mb-1">Test Command</label>
            <input
              type="text"
              placeholder="e.g. cargo test, bun test"
              value={form.testCmd}
              onChange={(e) => updateField("testCmd", e.target.value)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>
          <div>
            <label className="text-zinc-400 block mb-1">Target Branch</label>
            <input
              type="text"
              value={form.branch}
              onChange={(e) => updateField("branch", e.target.value)}
              className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
            />
          </div>
        </div>

        <div className="flex items-center gap-2">
          <label className="flex items-center gap-2 text-zinc-300 text-xs cursor-pointer">
            <input
              type="checkbox"
              checked={form.verify}
              onChange={(e) => updateField("verify", e.target.checked)}
              className="rounded bg-zinc-950 border-zinc-800 text-emerald-500 focus:ring-0"
            />
            Run closed-loop test suite verification on each iteration
          </label>
        </div>

        <div>
          <label className="text-zinc-400 block mb-1">Custom Architectural Instructions</label>
          <input
            type="text"
            placeholder="e.g. Use async functions, prefer immutable data structures"
            value={form.customInstructions}
            onChange={(e) => updateField("customInstructions", e.target.value)}
            className="w-full bg-zinc-950 border border-zinc-800 rounded px-2 py-1 text-zinc-200"
          />
        </div>

        <button
          onClick={handleStartHealing}
          disabled={isRunning}
          className="flex items-center justify-center gap-2 w-full py-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-white font-medium rounded-lg transition-colors"
        >
          {isRunning ? (
            <>
              <RefreshCw className="w-4 h-4 animate-spin" />
              <span>Surgeon Active: Healing Refactoring Loop...</span>
            </>
          ) : (
            <>
              <Play className="w-4 h-4" />
              <span>Start Autonomous Healing Refactor</span>
            </>
          )}
        </button>
      </div>

      {healError && (
        <div className="p-3 bg-red-950/40 border border-red-800/60 rounded-lg text-red-300 flex items-center gap-2">
          <AlertCircle className="w-4 h-4 text-red-400 shrink-0" />
          <span>{healError}</span>
        </div>
      )}

      {healResult && (
        <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-lg space-y-3">
          <div className="flex items-center justify-between">
            <span className="font-semibold text-zinc-200">Healing Session Results</span>
            <span
              className={`px-2 py-0.5 rounded text-xs ${
                healResult.success
                  ? "bg-emerald-950 text-emerald-400 border border-emerald-800"
                  : "bg-amber-950 text-amber-400 border border-amber-800"
              }`}
            >
              {healResult.success ? "SUCCESS" : "INCOMPLETE"}
            </span>
          </div>

          <p className="text-zinc-300">{healResult.message}</p>

          {healResult.branch_created && (
            <div className="flex items-center gap-2 text-emerald-400 text-xs">
              <GitBranch className="w-3.5 h-3.5" />
              <span>Committed to branch: {healResult.branch_created}</span>
            </div>
          )}

          <div className="space-y-2 pt-2">
            <span className="text-zinc-400 text-xs font-semibold block">Iteration Timeline</span>
            {healResult.iterations.map((it: HealIterationLog) => (
              <div
                key={it.iteration}
                className="p-2.5 bg-zinc-950 border border-zinc-800/80 rounded space-y-1"
              >
                <div className="flex items-center justify-between">
                  <span className="font-medium text-zinc-300">Iteration #{it.iteration}</span>
                  <div className="flex items-center gap-2">
                    {it.patch_applied ? (
                      <span className="text-emerald-400 flex items-center gap-1 text-[11px]">
                        <Check className="w-3 h-3" /> Patch Applied
                      </span>
                    ) : (
                      <span className="text-red-400 flex items-center gap-1 text-[11px]">
                        <XCircle className="w-3 h-3" /> Patch Failed
                      </span>
                    )}
                    {it.test_passed ? (
                      <span className="text-emerald-400 flex items-center gap-1 text-[11px]">
                        <CheckCircle2 className="w-3 h-3" /> Tests Passed
                      </span>
                    ) : (
                      <span className="text-amber-400 flex items-center gap-1 text-[11px]">
                        <XCircle className="w-3 h-3" /> Tests Failed
                      </span>
                    )}
                  </div>
                </div>

                {it.error_feedback && (
                  <div className="p-2 bg-red-950/30 border border-red-900/40 rounded text-red-300 text-[11px] overflow-x-auto">
                    {it.error_feedback}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};
