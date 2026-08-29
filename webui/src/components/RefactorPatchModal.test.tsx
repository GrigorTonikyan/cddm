import { screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { RefactorPatchModal } from "./RefactorPatchModal";
import { RefactorSuggestion } from "./../types/cddm-types";
import {
  DEFAULT_TEST_CLONE_PAIR_PROPS,
  expectDefinedTexts,
  renderWithWin2x,
} from "../test/test-helpers";

describe("RefactorPatchModal Component", () => {
  const mockSuggestion: RefactorSuggestion = {
    suggested_function_name: "extracted_shared_helper",
    strategy: "extract_function",
    common_body_lines: ["let x = 1;", "let y = 2;"],
    parameter_differences: [],
    target_module_hint: "Shared utility module",
    unified_patch:
      "--- a/src/a.ts\n+++ b/src/a.ts\n@@ -10,2 +10,1 @@\n-let x = 1;\n+extracted_shared_helper();",
    lines_saved: 5,
  };

  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("should not render when isOpen is false", () => {
    const { container } = renderWithWin2x(
      <RefactorPatchModal isOpen={false} onClose={() => {}} {...DEFAULT_TEST_CLONE_PAIR_PROPS} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render refactoring suggestion details when open", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockSuggestion),
    } as Response);

    const onClose = vi.fn();

    renderWithWin2x(
      <RefactorPatchModal isOpen={true} onClose={onClose} {...DEFAULT_TEST_CLONE_PAIR_PROPS} />,
    );

    expect(screen.getByText("Automated Refactoring Advisor")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Extract Function")).toBeDefined();
    });

    expectDefinedTexts([
      "~5 lines eliminated",
      "extracted_shared_helper()",
      "Shared utility module",
      "Copy Patch",
      "Download .patch",
    ]);
  });

  it("should close on Close button click", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockSuggestion),
    } as Response);

    const onClose = vi.fn();

    renderWithWin2x(
      <RefactorPatchModal isOpen={true} onClose={onClose} {...DEFAULT_TEST_CLONE_PAIR_PROPS} />,
    );

    await waitFor(() => {
      expect(screen.getByText("Extract Function")).toBeDefined();
    });

    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });
});
