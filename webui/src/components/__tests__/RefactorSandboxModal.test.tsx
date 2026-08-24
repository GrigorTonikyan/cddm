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

  it("should switch to AST-Native Rewrite tab and render AST synthesis results", async () => {
    const mockPreviewAst = vi.fn().mockResolvedValue({
      function_name: "custom_shared_helper",
      target_module_path: "src/shared.ts",
      helper_signature: "export function custom_shared_helper(x: number): void",
      helper_function_code:
        "export function custom_shared_helper(x: number): void {\n  console.log(x);\n}",
      inferred_parameters: [
        {
          name: "x",
          inferred_type: "number",
          original_values: ["1"],
        },
      ],
      rewritten_files: [
        {
          file_path: "src/a.ts",
          original_line_count: 50,
          new_line_count: 30,
          call_sites_count: 1,
          rewritten_source: "",
          imports_added: ["import { custom_shared_helper } from './shared';"],
        },
      ],
      unified_patch: "",
      total_lines_saved: 20,
      syntax_valid: true,
    });

    useCDDMStore.setState({
      previewAstRefactor: mockPreviewAst,
      astRewriteResult: {
        function_name: "custom_shared_helper",
        target_module_path: "src/shared.ts",
        helper_signature: "export function custom_shared_helper(x: number): void",
        helper_function_code:
          "export function custom_shared_helper(x: number): void {\n  console.log(x);\n}",
        inferred_parameters: [
          {
            name: "x",
            inferred_type: "number",
            original_values: ["1"],
          },
        ],
        rewritten_files: [
          {
            file_path: "src/a.ts",
            original_line_count: 50,
            new_line_count: 30,
            call_sites_count: 1,
            rewritten_source: "",
            imports_added: ["import { custom_shared_helper } from './shared';"],
          },
        ],
        unified_patch: "",
        total_lines_saved: 20,
        syntax_valid: true,
      },
    });

    render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const astTabBtn = screen.getByText(/AST-Native Rewrite/);
    expect(astTabBtn).toBeDefined();
    fireEvent.click(astTabBtn);

    expect(screen.getByText("Synthesized Function Implementation")).toBeDefined();
    expect(screen.getByText("Transformed Source Files (1)")).toBeDefined();
    expect(screen.getByText("x:")).toBeDefined();
    expect(screen.getByText("number")).toBeDefined();
  });

  it("should trigger test suite verification when Run Test Verification is clicked", async () => {
    const mockVerify = vi.fn().mockResolvedValue({
      success: true,
      exit_code: 0,
      duration_ms: 120,
      command_executed: "cargo test",
      stdout_snippet: "test result: ok. 10 passed",
      stderr_snippet: "",
      message: "Suite passed",
    });

    useCDDMStore.setState({
      verifyRefactorTestSuite: mockVerify,
      verifyResult: {
        success: true,
        exit_code: 0,
        duration_ms: 120,
        command_executed: "cargo test",
        stdout_snippet: "test result: ok. 10 passed",
        stderr_snippet: "",
        message: "Suite passed",
      },
    });

    render(
      <Win2xManagerProvider>
        <RefactorSandboxModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const verifyBtn = screen.getByText("Run Test Verification");
    expect(verifyBtn).toBeDefined();
    fireEvent.click(verifyBtn);

    expect(mockVerify).toHaveBeenCalledWith(
      expect.objectContaining({
        directory: ".",
      }),
    );
  });
});
