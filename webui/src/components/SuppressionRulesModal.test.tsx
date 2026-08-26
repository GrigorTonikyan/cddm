import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { SuppressionRulesModal } from "./SuppressionRulesModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "./../store/cddm-store";
import { SuppressionConfig } from "./../types/cddm-types";

describe("SuppressionRulesModal Component", () => {
  const mockConfig: SuppressionConfig = {
    rules: [
      {
        pattern: "**/tests/**",
        comment: "Ignore test fixtures",
        min_tokens_override: undefined,
        ignored_clone_types: ["Exact"],
      },
      {
        pattern: "legacy/services/**",
        comment: "Legacy service threshold",
        min_tokens_override: 120,
        ignored_clone_types: [],
      },
    ],
    ignore_tests: true,
    ignore_mocks: false,
    ignore_generated: true,
    raw_cddmignore: "**/tests/**\nlegacy/services/**\n",
  };

  beforeEach(() => {
    useCDDMStore.setState({
      suppressionConfig: mockConfig,
      isSuppressionLoading: false,
      suppressionError: null,
    });
  });

  it("should return null when closed", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <SuppressionRulesModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render tabs, active rules table, and category filters when open", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <SuppressionRulesModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Intelligent AST Suppression & .cddmignore Engine")).toBeDefined();
    expect(screen.getByText("Category & Path Rules")).toBeDefined();
    expect(screen.getByText(".cddmignore Editor")).toBeDefined();
    expect(screen.getByText("Inline Directives Guide")).toBeDefined();

    expect(screen.getByText("Ignore Tests")).toBeDefined();
    expect(screen.getByText("Ignore Mocks")).toBeDefined();
    expect(screen.getByText("Ignore Generated")).toBeDefined();

    expect(screen.getByText("**/tests/**")).toBeDefined();
    expect(screen.getByText("legacy/services/**")).toBeDefined();
    expect(screen.getByText("120 tokens")).toBeDefined();

    // Test close button
    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });

  it("should switch between editor and directives tabs", () => {
    render(
      <Win2xManagerProvider>
        <SuppressionRulesModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

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
