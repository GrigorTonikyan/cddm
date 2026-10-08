import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { MonorepoWorkspaceModal } from "./MonorepoWorkspaceModal";
import { renderWithWin2x, createMockScanResult } from "../test/test-helpers";
import { useCDDMStore } from "../store/cddm-store";
import type { MonorepoScanSummary } from "../types/scan-types";

describe("MonorepoWorkspaceModal Component", () => {
  const mockMonorepoSummary: MonorepoScanSummary = {
    workspaces: [
      {
        name: "cddm-cli",
        path: "crates/cddm-cli",
        manifest_file: "crates/cddm-cli/Cargo.toml",
        package_type: "cargo",
      },
      {
        name: "webui",
        path: "webui",
        manifest_file: "webui/package.json",
        package_type: "npm",
      },
    ],
    total_workspaces: 2,
    total_files: 104,
    total_tokens: 23500,
    total_clones: 7,
    cross_workspace_clones: 1,
    average_dry_score: 94.2,
    scan_result: createMockScanResult(),
  };

  beforeEach(() => {
    useCDDMStore.setState({
      monorepoData: mockMonorepoSummary,
      isMonorepoLoading: false,
      monorepoError: null,
      config: {
        directory: ".",
        min_tokens: 50,
        languages: [],
        ignore_patterns: [],
        detect_type2: true,
        scan_self: true,
      },
    });
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(
      <MonorepoWorkspaceModal isOpen={false} onClose={() => {}} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render monorepo summary metrics and package cards when open", () => {
    const onClose = vi.fn();
    renderWithWin2x(<MonorepoWorkspaceModal isOpen={true} onClose={onClose} />);

    expect(screen.getByText("Monorepo Workspace & Multi-Package Architecture")).toBeDefined();
    expect(screen.getByText("2 Packages")).toBeDefined();
    expect(screen.getByText("cddm-cli")).toBeDefined();
    expect(screen.getByText("crates/cddm-cli")).toBeDefined();
    expect(screen.getAllByText("webui").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("94.2%")).toBeDefined();
    expect(screen.getByText("1 cross-package clone detected")).toBeDefined();

    // Close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should trigger runMonorepoScan when clicking scan button", async () => {
    const mockScan = vi.fn().mockResolvedValue(mockMonorepoSummary);
    useCDDMStore.setState({ runMonorepoScan: mockScan });

    renderWithWin2x(<MonorepoWorkspaceModal isOpen={true} onClose={() => {}} />);

    const scanBtn = screen.getByText("Scan Workspace");
    fireEvent.click(scanBtn);

    expect(mockScan).toHaveBeenCalledWith(".", 50);
  });

  it("should render empty state when no workspaces exist", () => {
    useCDDMStore.setState({
      monorepoData: {
        workspaces: [],
        total_workspaces: 0,
        total_files: 0,
        total_tokens: 0,
        total_clones: 0,
        cross_workspace_clones: 0,
        average_dry_score: 100.0,
        scan_result: createMockScanResult(),
      },
    });

    renderWithWin2x(<MonorepoWorkspaceModal isOpen={true} onClose={() => {}} />);

    expect(screen.getByText("No workspace packages detected")).toBeDefined();
  });

  it("should display error message when monorepoError is present", () => {
    useCDDMStore.setState({
      monorepoError: "Failed to read root workspace Cargo.toml",
    });

    renderWithWin2x(<MonorepoWorkspaceModal isOpen={true} onClose={() => {}} />);

    expect(screen.getByText("Failed to read root workspace Cargo.toml")).toBeDefined();
  });

  it("should render cleanly in uninitialized default store state", () => {
    useCDDMStore.setState({
      monorepoData: null,
      isMonorepoLoading: false,
      monorepoError: null,
      config: undefined as unknown as any,
    });

    renderWithWin2x(<MonorepoWorkspaceModal isOpen={true} onClose={() => {}} />);
    expect(screen.getByText("Monorepo Workspace & Multi-Package Architecture")).toBeDefined();
    expect(screen.getByText("0 Packages")).toBeDefined();
    expect(screen.getByText("No workspace packages detected")).toBeDefined();
  });
});
