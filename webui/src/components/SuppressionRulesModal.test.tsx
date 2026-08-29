import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { SuppressionRulesModal } from "./SuppressionRulesModal";
import {
  createMockSuppressionConfig,
  expectDefinedTexts,
  expectNullWhenClosed,
  renderWithWin2x,
} from "../test/test-helpers";
import { useCDDMStore } from "./../store/cddm-store";

describe("SuppressionRulesModal Component", () => {
  beforeEach(() => {
    useCDDMStore.setState({
      suppressionConfig: createMockSuppressionConfig(),
      isSuppressionLoading: false,
      suppressionError: null,
    });
  });

  it("should return null when closed", () => {
    expectNullWhenClosed(<SuppressionRulesModal isOpen={false} onClose={() => {}} />);
  });

  it("should render tabs, active rules table, and category filters when open", () => {
    const onClose = vi.fn();
    renderWithWin2x(<SuppressionRulesModal isOpen={true} onClose={onClose} />);

    expectDefinedTexts([
      "Intelligent AST Suppression & .cddmignore Engine",
      "Category & Path Rules",
      ".cddmignore Editor",
      "Inline Directives Guide",
      "Ignore Tests",
      "Ignore Mocks",
      "Ignore Generated",
      "**/tests/**",
      "legacy/services/**",
      "120 tokens",
    ]);

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should switch between editor and directives tabs", () => {
    renderWithWin2x(<SuppressionRulesModal isOpen={true} onClose={() => {}} />);

    // Switch to Editor tab
    const editorTab = screen.getByText(".cddmignore Editor");
    fireEvent.click(editorTab);
    expect(screen.getByPlaceholderText("# Enter glob patterns to suppress...")).toBeDefined();

    // Switch to Directives tab
    const directivesTab = screen.getByText("Inline Directives Guide");
    fireEvent.click(directivesTab);
    expect(screen.getByText("Single-Line Suppression Directive")).toBeDefined();
    expect(screen.getByText("Block-Level Suppression Directives")).toBeDefined();
  });
});
