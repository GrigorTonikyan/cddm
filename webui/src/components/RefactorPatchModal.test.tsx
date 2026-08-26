import { render, screen, waitFor, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { RefactorPatchModal } from "./RefactorPatchModal";
import { RefactorSuggestion } from "./../types/cddm-types";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";

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

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <RefactorPatchModal
          isOpen={false}
          onClose={() => {}}
          fileA="src/a.ts"
          startLineA={10}
          endLineA={12}
          fileB="src/b.ts"
          startLineB={20}
          endLineB={22}
        />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should fetch refactoring suggestions and render patch preview", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockSuggestion),
    } as Response);

    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <RefactorPatchModal
          isOpen={true}
          onClose={onClose}
          fileA="src/a.ts"
          startLineA={10}
          endLineA={12}
          fileB="src/b.ts"
          startLineB={20}
          endLineB={22}
        />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Automated Refactoring Advisor")).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText("Extract Function")).toBeDefined();
    });

    expect(screen.getByText("~5 lines eliminated")).toBeDefined();
    expect(screen.getByText("extracted_shared_helper()")).toBeDefined();
    expect(screen.getByText("Shared utility module")).toBeDefined();
    expect(screen.getByText("Copy Patch")).toBeDefined();
    expect(screen.getByText("Download .patch")).toBeDefined();
  });

  it("should close on Close button click", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockSuggestion),
    } as Response);

    const onClose = vi.fn();

    render(
      <Win2xManagerProvider>
        <RefactorPatchModal
          isOpen={true}
          onClose={onClose}
          fileA="src/a.ts"
          startLineA={10}
          endLineA={12}
          fileB="src/b.ts"
          startLineB={20}
          endLineB={22}
        />
      </Win2xManagerProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("Extract Function")).toBeDefined();
    });

    const closeBtn = screen.getByText("Close");
    fireEvent.click(closeBtn);
    expect(onClose).toHaveBeenCalled();
  });
});
