import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { RefactorSandboxModal } from "../RefactorSandboxModal";
import { Win2xManagerProvider } from "../ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "../../store/cddm-store";
import { RefactorSandboxRequest, RefactorSandboxResult } from "../../types/cddm-types";

describe("RefactorSandboxModal Component", () => {
  const mockReq: RefactorSandboxRequest = {
    cluster_id: 1,
    occurrences: [
      {
        file: "src/a.ts",
        start_line: 10,
        end_line: 25,
      },
      {
        file: "src/b.ts",
        start_line: 30,
        end_line: 45,
      },
    ],
    custom_function_name: "custom_shared_helper",
    target_module_path: "src/shared.ts",
  };

  const mockResult: RefactorSandboxResult = {
    function_name: "custom_shared_helper",
    target_module_path: "src/shared.ts",
    parameter_names: ["arg0", "arg1"],
    unified_patch:
      "--- a/src/a.ts\n+++ b/src/a.ts\n@@ -10,15 +10,3 @@\n-    let x = 1;\n+    custom_shared_helper();\n",
    total_lines_saved: 24,
    affected_files: ["src/a.ts", "src/b.ts"],
    preview_diff_hunks: ["@@ -10,15 +10,3 @@"],
  };

  beforeEach(() => {
    useCDDMStore.setState({
      sandboxRequest: mockReq,
      sandboxResult: mockResult,
      isSandboxLoading: false,
      sandboxError: null,
    });
  });

  it("should return null when closed", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render sandbox controls, metrics, diff preview, and actions when open", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Interactive Auto-Refactor Sandbox & Visual Studio")).toBeDefined();
    expect(screen.getByText("Parameterized Refactoring Studio Controls")).toBeDefined();
    expect(screen.getByText("+24 lines")).toBeDefined();
    expect(screen.getByText("2 files")).toBeDefined();
    expect(screen.getByText("Live Synthesized Unified Diff Patch")).toBeDefined();
    expect(screen.getByText("Apply to Git Branch")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should allow changing custom function name and branch inputs", () => {
    render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const funcInput = screen.getByPlaceholderText("extracted_shared_helper") as HTMLInputElement;
    fireEvent.change(funcInput, { target: { value: "my_new_helper" } });
    expect(funcInput.value).toBe("my_new_helper");

    const branchInput = screen.getByPlaceholderText("cddm/refactor-cluster-1") as HTMLInputElement;
    fireEvent.change(branchInput, { target: { value: "cddm/custom-refactor-branch" } });
    expect(branchInput.value).toBe("cddm/custom-refactor-branch");
  });

  it("should render and handle Copy AI Prompt action", async () => {
    const mockGenerateAiPrompt = vi.fn().mockResolvedValue("AI Refactor Prompt markdown");
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    useCDDMStore.setState({
      generateAiPrompt: mockGenerateAiPrompt,
    });

    render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const copyPromptBtn = screen.getByText("Copy AI Prompt");
    expect(copyPromptBtn).toBeDefined();
    fireEvent.click(copyPromptBtn);

    expect(mockGenerateAiPrompt).toHaveBeenCalled();
  });
});
