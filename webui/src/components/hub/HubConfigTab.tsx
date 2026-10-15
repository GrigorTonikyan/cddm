import React, { useState, useEffect } from "react";
import type { HubConfig, HubRepoConfig } from "../../types/hub-types";
import { Plus, Trash2, Save, FileCode2, CheckCircle2 } from "lucide-react";

export interface HubConfigTabProps {
  config: HubConfig | null;
  onSave: (config: HubConfig) => Promise<void>;
  isLoading: boolean;
}

export const HubConfigTab: React.FC<HubConfigTabProps> = ({ config, onSave, isLoading }) => {
  const [name, setName] = useState<string>("");
  const [minTokens, setMinTokens] = useState<number>(50);
  const [failThreshold, setFailThreshold] = useState<number>(5.0);
  const [repositories, setRepositories] = useState<HubRepoConfig[]>([]);
  const [ignorePatterns, setIgnorePatterns] = useState<string>("");
  const [savedSuccess, setSavedSuccess] = useState<boolean>(false);

  useEffect(() => {
    if (config) {
      setName(config.name || "Federation Hub");
      setMinTokens(config.min_tokens ?? 50);
      setFailThreshold(config.fail_threshold ?? 5.0);
      setRepositories(config.repositories || []);
      setIgnorePatterns((config.ignore_patterns || []).join("\n"));
    }
  }, [config]);

  const handleAddRepo = () => {
    setRepositories([
      ...repositories,
      { name: `repo-${repositories.length + 1}`, path: `./repo-${repositories.length + 1}` },
    ]);
  };

  const handleRemoveRepo = (index: number) => {
    setRepositories(repositories.filter((_, i) => i !== index));
  };

  const handleUpdateRepo = (index: number, field: keyof HubRepoConfig, value: string | number) => {
    const updated = [...repositories];
    updated[index] = { ...updated[index], [field]: value } as HubRepoConfig;
    setRepositories(updated);
  };

  const handleSave = async () => {
    const updatedConfig: HubConfig = {
      name,
      min_tokens: Number(minTokens),
      fail_threshold: Number(failThreshold),
      repositories,
      ignore_patterns: ignorePatterns
        .split("\n")
        .map((p) => p.trim())
        .filter(Boolean),
    };
    await onSave(updatedConfig);
    setSavedSuccess(true);
    setTimeout(() => setSavedSuccess(false), 2500);
  };

  return (
    <div className="space-y-4 p-4 font-mono text-xs text-slate-200">
      <div className="flex items-center justify-between pb-3 border-b border-slate-800">
        <div className="flex items-center gap-2">
          <FileCode2 className="w-4 h-4 text-indigo-400" />
          <span className="font-semibold text-slate-100 text-sm">
            Hub Federation Configuration (.cddmhub.toml)
          </span>
        </div>
        <button
          type="button"
          onClick={handleSave}
          disabled={isLoading}
          className="px-3 py-1.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-semibold flex items-center gap-1.5 transition-colors shadow-sm"
        >
          {savedSuccess ? (
            <>
              <CheckCircle2 className="w-3.5 h-3.5 text-emerald-300" />
              <span>Saved!</span>
            </>
          ) : (
            <>
              <Save className="w-3.5 h-3.5" />
              <span>Save Configuration</span>
            </>
          )}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
        <div className="space-y-1">
          <label className="text-[11px] text-slate-400">Hub Organization Name</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-2.5 py-1.5 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-indigo-500"
          />
        </div>
        <div className="space-y-1">
          <label className="text-[11px] text-slate-400">Min Tokens Threshold</label>
          <input
            type="number"
            value={minTokens}
            onChange={(e) => setMinTokens(parseInt(e.target.value, 10) || 50)}
            className="w-full px-2.5 py-1.5 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-indigo-500"
          />
        </div>
        <div className="space-y-1">
          <label className="text-[11px] text-slate-400">Quality Gate Threshold (%)</label>
          <input
            type="number"
            step="0.1"
            value={failThreshold}
            onChange={(e) => setFailThreshold(parseFloat(e.target.value) || 5.0)}
            className="w-full px-2.5 py-1.5 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-indigo-500"
          />
        </div>
      </div>

      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <span className="text-slate-300 font-semibold text-xs">
            Federated Repositories ({repositories.length})
          </span>
          <button
            type="button"
            onClick={handleAddRepo}
            className="px-2 py-1 rounded bg-slate-800 hover:bg-slate-700 text-slate-300 flex items-center gap-1 text-[11px]"
          >
            <Plus className="w-3 h-3 text-indigo-400" />
            <span>Add Repository</span>
          </button>
        </div>

        <div className="space-y-2 max-h-56 overflow-y-auto pr-1">
          {repositories.map((repo, idx) => (
            <div
              key={idx}
              className="flex items-center gap-2 p-2 bg-slate-900/60 border border-slate-800 rounded-lg"
            >
              <input
                type="text"
                placeholder="Repo Name"
                value={repo.name}
                onChange={(e) => handleUpdateRepo(idx, "name", e.target.value)}
                className="flex-1 px-2 py-1 bg-slate-950 border border-slate-800 rounded text-slate-200 text-xs focus:outline-none focus:border-indigo-500"
              />
              <input
                type="text"
                placeholder="Relative Path (e.g. ./apps/web)"
                value={repo.path}
                onChange={(e) => handleUpdateRepo(idx, "path", e.target.value)}
                className="flex-1 px-2 py-1 bg-slate-950 border border-slate-800 rounded text-slate-200 text-xs focus:outline-none focus:border-indigo-500"
              />
              <button
                type="button"
                onClick={() => handleRemoveRepo(idx)}
                className="p-1.5 text-slate-400 hover:text-rose-400 rounded hover:bg-slate-800 transition-colors"
                title="Remove Repository"
              >
                <Trash2 className="w-3.5 h-3.5" />
              </button>
            </div>
          ))}
        </div>
      </div>

      <div className="space-y-1">
        <label className="text-[11px] text-slate-400">Ignore Patterns (one pattern per line)</label>
        <textarea
          rows={3}
          value={ignorePatterns}
          onChange={(e) => setIgnorePatterns(e.target.value)}
          placeholder="node_modules/**&#10;target/**&#10;dist/**"
          className="w-full px-2.5 py-1.5 bg-slate-900 border border-slate-800 rounded-lg text-slate-200 focus:outline-none focus:border-indigo-500 resize-none font-mono text-xs"
        />
      </div>
    </div>
  );
};
