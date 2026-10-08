import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { HookManagerModal } from "./HookManagerModal";
import { renderWithWin2x } from "../test/test-helpers";
import { useCDDMStore } from "../store/cddm-store";

describe("HookManagerModal Component", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      hookStatus: {
        pre_commit_installed: false,
        pre_push_installed: true,
        hooks_dir: ".git/hooks",
      },
      isTimelineLoading: false,
    });
  });

  it("should return null when not open", () => {
    const { container } = renderWithWin2x(<HookManagerModal isOpen={false} onClose={() => {}} />);
    expect(container.firstChild).toBeNull();
  });

  it("should render git hook statuses and controls when open", () => {
    const onClose = vi.fn();
    renderWithWin2x(<HookManagerModal isOpen={true} onClose={onClose} />);

    expect(screen.getByText("Git Hook Manager & CI/CD Studio")).toBeDefined();
    expect(screen.getByText("Pre-Commit Hook")).toBeDefined();
    expect(screen.getByText("Pre-Push Hook")).toBeDefined();
    expect(screen.getByText("Install Pre-Commit Hook")).toBeDefined();
    expect(screen.getByText("Reinstall Pre-Push Hook")).toBeDefined();
    expect(screen.getByText(".git/hooks")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should handle hook installation trigger", async () => {
    const mockInstall = vi.fn().mockResolvedValue("Hook installed successfully");
    const mockFetchHooks = vi.fn().mockResolvedValue(undefined);
    useCDDMStore.setState({
      installHook: mockInstall,
      fetchHookStatus: mockFetchHooks,
    });

    renderWithWin2x(<HookManagerModal isOpen={true} onClose={() => {}} />);

    const installPreCommitBtn = screen.getByText("Install Pre-Commit Hook");
    fireEvent.click(installPreCommitBtn);

    expect(mockInstall).toHaveBeenCalledWith("pre-commit", 5, 50);
  });

  it("should switch tabs to turnkey workflows and display CI/CD YAML", () => {
    renderWithWin2x(<HookManagerModal isOpen={true} onClose={() => {}} />);

    const workflowsTab = screen.getByText("Turnkey CI/CD Workflows");
    fireEvent.click(workflowsTab);

    expect(screen.getByText(/cddm scan \. --min-tokens 50/)).toBeDefined();
    expect(screen.getByText("Copy Snippet")).toBeDefined();

    const githubBtn = screen.getByText("GitHub Actions (.github/workflows/cddm.yml)");
    fireEvent.click(githubBtn);
    expect(screen.getByText(/dtolnay\/rust-toolchain/)).toBeDefined();
  });

  it("should handle uninitialized hookStatus gracefully", () => {
    useCDDMStore.setState({ hookStatus: null });
    renderWithWin2x(<HookManagerModal isOpen={true} onClose={() => {}} />);

    expect(screen.getByText("Git Hook Manager & CI/CD Studio")).toBeDefined();
    expect(screen.getAllByText("Not Installed").length).toBe(2);
  });
});
