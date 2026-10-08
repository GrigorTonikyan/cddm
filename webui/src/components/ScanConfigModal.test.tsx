import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { ScanConfigModal } from "./ScanConfigModal";
import { renderWithWin2x } from "../test/test-helpers";
import { useCDDMStore } from "../store/cddm-store";

describe("ScanConfigModal Component", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      config: {
        directory: ".",
        min_tokens: 50,
        languages: [],
        ignore_patterns: [],
        detect_type2: true,
        scan_self: true,
        fail_threshold: 5.0,
        cache_dir: ".cddm-cache",
        enable_cache: true,
        in_tree_cache: false,
        ignore_tests: false,
        ignore_mocks: false,
      },
      isScanConfigCollapsed: false,
      isHookManagerModalOpen: false,
      isMonorepoModalOpen: false,
      isPolicyRulesModalOpen: false,
      isHubModalOpen: false,
    });
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(<ScanConfigModal isOpen={false} onClose={() => {}} />);
    expect(container.firstChild).toBeNull();
  });

  it("should render Centralized Configuration Studio with tabs when open", () => {
    const onClose = vi.fn();
    renderWithWin2x(<ScanConfigModal isOpen={true} onClose={onClose} />);

    expect(screen.getByText("Scan Parameters & Centralized Configuration Studio")).toBeDefined();
    expect(screen.getByText("Engine Parameters & Tuning")).toBeDefined();
    expect(screen.getByText("Persistent Cache Pack")).toBeDefined();
    expect(screen.getByText("CI/CD & Integrations Hub")).toBeDefined();
    expect(screen.getByText("Advanced Filtering & Baseline")).toBeDefined();

    // Close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should toggle advanced filtering checkboxes in tuning tab", () => {
    renderWithWin2x(<ScanConfigModal isOpen={true} onClose={() => {}} />);

    const ignoreTestsLabel = screen.getByText("Ignore Test Files");
    expect(ignoreTestsLabel).toBeDefined();

    const checkboxes = screen.getAllByRole("checkbox") as HTMLInputElement[];
    const ignoreTestsCb = checkboxes.find((cb) =>
      cb.parentElement?.textContent?.includes("Ignore Test Files"),
    );
    expect(ignoreTestsCb).toBeDefined();
    expect(ignoreTestsCb?.checked).toBe(false);

    if (ignoreTestsCb) {
      fireEvent.click(ignoreTestsCb);
      expect(useCDDMStore.getState().config.ignore_tests).toBe(true);
    }
  });

  it("should switch to Persistent Cache Pack tab and handle export & import", async () => {
    const mockExport = vi.fn().mockResolvedValue({
      success: true,
      entry_count: 10,
      pack_file: "cddm-cache.pack",
      checksum: "abc123456789",
      message: "Exported successfully",
    });
    const mockImport = vi.fn().mockResolvedValue({
      success: true,
      entry_count: 10,
      pack_file: "cddm-cache.pack",
      checksum: "abc123456789",
      message: "Imported successfully",
    });

    useCDDMStore.setState({
      exportCachePack: mockExport,
      importCachePack: mockImport,
    });

    renderWithWin2x(<ScanConfigModal isOpen={true} onClose={() => {}} />);

    const cacheTab = screen.getByText("Persistent Cache Pack");
    fireEvent.click(cacheTab);

    expect(screen.getAllByText("Export Cache Pack").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("Import Cache Pack").length).toBeGreaterThanOrEqual(1);

    const exportBtn = screen.getByRole("button", { name: /Export Cache Pack/i });
    fireEvent.click(exportBtn);
    expect(mockExport).toHaveBeenCalled();

    const importBtn = screen.getByRole("button", { name: /Import Cache Pack/i });
    fireEvent.click(importBtn);
    expect(mockImport).toHaveBeenCalled();
  });

  it("should switch to CI/CD & Integrations Hub and open sub-modals", () => {
    const onClose = vi.fn();
    renderWithWin2x(<ScanConfigModal isOpen={true} onClose={onClose} />);

    const integrationsTab = screen.getByText("CI/CD & Integrations Hub");
    fireEvent.click(integrationsTab);

    expect(screen.getByText("Git Hook Manager & CI/CD Studio")).toBeDefined();
    expect(screen.getByText("Monorepo Workspace Studio")).toBeDefined();

    const hookBtn = screen.getByText("Git Hook Manager & CI/CD Studio");
    fireEvent.click(hookBtn);
    expect(onClose).toHaveBeenCalled();
    expect(useCDDMStore.getState().isHookManagerModalOpen).toBe(true);
  });

  it("should render cleanly in uninitialized default store state", () => {
    useCDDMStore.setState({
      config: undefined as unknown as any,
    });

    renderWithWin2x(<ScanConfigModal isOpen={true} onClose={() => {}} />);
    expect(screen.getByText("Scan Parameters & Centralized Configuration Studio")).toBeDefined();
    expect(screen.getByText("Engine Parameters & Tuning")).toBeDefined();
  });
});
