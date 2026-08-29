import { describe, it, expect, beforeEach, vi } from "vite-plus/test";
import { useCDDMStore } from "./cddm-store";
import type { HubConfig, HubExtractResult, HubScanSummary } from "./../types/cddm-types";

describe("useCDDMStore - Hub Slice", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      isHubModalOpen: false,
      hubConfig: null,
      hubSummary: null,
      isHubLoading: false,
      hubError: null,
    });
  });

  it("should initialize with default hub state", () => {
    const state = useCDDMStore.getState();
    expect(state.isHubModalOpen).toBe(false);
    expect(state.hubConfig).toBeNull();
    expect(state.hubSummary).toBeNull();
    expect(state.isHubLoading).toBe(false);
    expect(state.hubError).toBeNull();
  });

  it("should toggle modal open state", () => {
    useCDDMStore.getState().setIsHubModalOpen(true);
    expect(useCDDMStore.getState().isHubModalOpen).toBe(true);

    useCDDMStore.getState().setIsHubModalOpen(false);
    expect(useCDDMStore.getState().isHubModalOpen).toBe(false);
  });

  it("should fetch hub configuration", async () => {
    const mockConfig: HubConfig = {
      name: "acme-corp",
      repositories: [{ name: "backend", path: "./backend" }],
      min_tokens: 50,
      fail_threshold: 15.0,
      ignore_patterns: [],
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockConfig),
    } as Response);

    await useCDDMStore.getState().fetchHubConfig();

    const state = useCDDMStore.getState();
    expect(state.hubConfig).toEqual(mockConfig);
    expect(state.isHubLoading).toBe(false);
    expect(state.hubError).toBeNull();
  });

  it("should run hub scan and store summary", async () => {
    const mockSummary: HubScanSummary = {
      hub_name: "acme-corp",
      total_repos: 2,
      total_files: 100,
      total_tokens: 20000,
      organization_dry_score: 96.5,
      repos: [],
      duplication_matrix: [],
      clusters: [],
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockSummary),
    } as Response);

    const summary = await useCDDMStore.getState().runHubScan();

    expect(summary).toEqual(mockSummary);
    expect(useCDDMStore.getState().hubSummary).toEqual(mockSummary);
    expect(useCDDMStore.getState().isHubLoading).toBe(false);
  });

  it("should extract shared package from hub cluster", async () => {
    const mockExtract: HubExtractResult = {
      package_name: "@acme/shared-utils",
      package_type: "npm",
      package_dir: "packages/shared-utils",
      generated_files: [{ file_path: "package.json", content: "{}" }],
      repo_updates: [],
      lines_saved: 120,
      summary: "Extraction complete",
    };

    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockExtract),
    } as Response);

    const result = await useCDDMStore.getState().extractHubPackage({
      cluster_id: 1,
      target_package_name: "@acme/shared-utils",
      package_type: "npm",
      target_dir: "packages/shared-utils",
      dry_run: true,
    });

    expect(result).toEqual(mockExtract);
    expect(useCDDMStore.getState().isHubLoading).toBe(false);
  });
});
