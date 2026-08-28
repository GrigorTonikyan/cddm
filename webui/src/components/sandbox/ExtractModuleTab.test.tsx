import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi } from "vite-plus/test";
import { ExtractModuleTab } from "./ExtractModuleTab";
import type { ExtractResult, RefactorSandboxRequest } from "../../types/cddm-types";

describe("ExtractModuleTab Component", () => {
  const mockReq: RefactorSandboxRequest = {
    cluster_id: 1,
    occurrences: [
      {
        file: "crates/app_a/src/main.rs",
        start_line: 10,
        end_line: 25,
      },
      {
        file: "crates/app_b/src/main.rs",
        start_line: 15,
        end_line: 30,
      },
    ],
    custom_function_name: "calculate_score",
    target_module_path: "crates/shared_utils",
  };

  const mockExtractResult: ExtractResult = {
    function_name: "calculate_score",
    target_path: "crates/shared_utils",
    target_kind: "new_crate",
    helper_signature: "pub fn calculate_score() -> i32",
    inferred_parameters: [],
    generated_files: [
      {
        file_path: "crates/shared_utils/Cargo.toml",
        content: '[package]\nname = "shared_utils"\nversion = "0.1.0"\n',
        is_new: true,
      },
      {
        file_path: "crates/shared_utils/src/lib.rs",
        content: "pub fn calculate_score() -> i32 {\n    42\n}\n",
        is_new: true,
      },
    ],
    test_files: [
      {
        file_path: "crates/shared_utils/tests/calculate_score_test.rs",
        content: "#[test]\nfn test_calculate_score_execution() { calculate_score(); }",
        is_new: true,
      },
    ],
    benchmark_files: [
      {
        file_path: "crates/shared_utils/benches/calculate_score_bench.rs",
        content: "use criterion::Criterion;\nfn bench_throughput(c: &mut Criterion) {}",
        is_new: true,
      },
    ],
    manifest_updates: [
      {
        manifest_path: "Cargo.toml",
        dependency_name: "crates/shared_utils",
        diff_preview: '+    "crates/shared_utils",',
        updated_content: '[workspace]\nmembers = ["crates/shared_utils"]\n',
      },
    ],
    caller_rewrites: [
      {
        file_path: "crates/app_a/src/main.rs",
        injected_import: "use shared_utils::calculate_score;",
        rewritten_content: "use shared_utils::calculate_score;\nfn main() { calculate_score(); }",
        diff_patch:
          "--- a/crates/app_a/src/main.rs\n+++ b/crates/app_a/src/main.rs\n@@ -10,15 +10,1 @@\n+    calculate_score();\n",
      },
    ],
    total_lines_saved: 28,
    syntax_valid: true,
    message: "Successfully planned extraction",
  };

  it("should render configuration controls and empty state", () => {
    const onPreview = vi.fn();
    const onApply = vi.fn();

    render(
      <ExtractModuleTab
        sandboxRequest={mockReq}
        extractResult={null}
        isExtractLoading={false}
        extractError={null}
        onPreview={onPreview}
        onApply={onApply}
      />,
    );

    expect(screen.getByText("Automated Shared Crate & Module Extraction")).toBeDefined();
    expect(screen.getByText("Occurrences: 2")).toBeDefined();
    expect(screen.getByText("Preview Extraction Plan")).toBeDefined();
    expect(screen.getByLabelText("Generate Unit Tests")).toBeDefined();
    expect(screen.getByLabelText("Generate Micro-Benchmarks")).toBeDefined();
    expect(
      screen.getByText(
        'Click "Preview Extraction Plan" to synthesize shared crate and manifest updates.',
      ),
    ).toBeDefined();
  });

  it("should trigger onPreview when Preview button is clicked", async () => {
    const onPreview = vi.fn().mockResolvedValue(mockExtractResult);
    const onApply = vi.fn();

    render(
      <ExtractModuleTab
        sandboxRequest={mockReq}
        extractResult={null}
        isExtractLoading={false}
        extractError={null}
        onPreview={onPreview}
        onApply={onApply}
      />,
    );

    const targetInput = screen.getByLabelText("Target Path") as HTMLInputElement;
    await act(async () => {
      fireEvent.change(targetInput, { target: { value: "crates/my_custom_crate" } });
    });
    expect(targetInput.value).toBe("crates/my_custom_crate");

    const fnInput = screen.getByLabelText("Function Name") as HTMLInputElement;
    await act(async () => {
      fireEvent.change(fnInput, { target: { value: "do_computation" } });
    });
    expect(fnInput.value).toBe("do_computation");

    const previewBtn = screen.getByText("Preview Extraction Plan");
    await act(async () => {
      fireEvent.click(previewBtn);
    });

    expect(onPreview).toHaveBeenCalledWith({
      occurrences: mockReq.occurrences,
      target_path: "crates/my_custom_crate",
      custom_function_name: "do_computation",
      target_kind: "auto",
      generate_tests: true,
      generate_benchmarks: true,
      dry_run: true,
    });
  });

  it("should render generated files, test files, benchmark files, manifest updates, and apply button when result is present", async () => {
    const onPreview = vi.fn();
    const onApply = vi.fn().mockResolvedValue(mockExtractResult);

    render(
      <ExtractModuleTab
        sandboxRequest={mockReq}
        extractResult={mockExtractResult}
        isExtractLoading={false}
        extractError={null}
        onPreview={onPreview}
        onApply={onApply}
      />,
    );

    expect(screen.getByText("~28 lines saved")).toBeDefined();
    expect(screen.getByText("Strategy: new_crate")).toBeDefined();
    expect(screen.getByText("Generated Files (2):")).toBeDefined();
    expect(screen.getByText("crates/shared_utils/Cargo.toml")).toBeDefined();
    expect(screen.getByText("Synthesized Unit Tests (1):")).toBeDefined();
    expect(screen.getByText("crates/shared_utils/tests/calculate_score_test.rs")).toBeDefined();
    expect(screen.getByText("Synthesized Micro-Benchmarks (1):")).toBeDefined();
    expect(screen.getByText("crates/shared_utils/benches/calculate_score_bench.rs")).toBeDefined();
    expect(screen.getByText("Manifest Updates (1)")).toBeDefined();
    expect(screen.getByText("Cargo.toml")).toBeDefined();
    expect(screen.getByText("Occurrence Caller Rewrites (1)")).toBeDefined();
    expect(screen.getByText("crates/app_a/src/main.rs")).toBeDefined();

    const applyBtn = screen.getByText("Apply to Workspace");
    await act(async () => {
      fireEvent.click(applyBtn);
    });
    expect(onApply).toHaveBeenCalled();
  });
});
