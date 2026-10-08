import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { CodeEditorModal } from "./CodeEditorModal";
import { Win2xManagerProvider } from "./ui/win2x-manager/context/win2x-manager-context";
import { useCDDMStore } from "../store/cddm-store";

describe("CodeEditorModal Component", () => {
  beforeEach(() => {
    act(() => {
      useCDDMStore.setState({
        isCodeEditorOpen: false,
        activeEditorFile: "crates/cddm-core/src/lib.rs",
        activeEditorContent: "pub fn test_initial() {}\n",
        originalEditorContent: "pub fn test_initial() {}\n",
        editorLanguage: "rust",
        editorIsDirty: false,
        editorIsLoading: false,
        editorError: null,
        editorSaveSuccess: false,
        editorSplitDiffMode: false,
        editorDiffFileA: "crates/cddm-core/src/lib.rs",
        editorDiffFileB: "crates/cddm-core/src/lib.rs",
        editorDiffContentA: "pub fn test_original() {}\n",
        editorDiffContentB: "pub fn test_initial() {}\n",
        editorFilesList: [
          {
            path: "crates/cddm-core/src/lib.rs",
            name: "lib.rs",
            is_dir: false,
            language: "rust",
            size_bytes: 25,
          },
        ],
      });
    });
  });

  it("should return null when not open", () => {
    const { container } = render(
      <Win2xManagerProvider>
        <CodeEditorModal isOpen={false} onClose={() => {}} />
      </Win2xManagerProvider>,
    );
    expect(container.firstChild).toBeNull();
  });

  it("should render Integrated Code Editor Studio window when open", () => {
    const onClose = vi.fn();
    render(
      <Win2xManagerProvider>
        <CodeEditorModal isOpen={true} onClose={onClose} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Integrated Code Editor & Split Diff Studio")).toBeDefined();
    expect(screen.getAllByText("crates/cddm-core/src/lib.rs").length).toBeGreaterThan(0);
    expect(screen.getByRole("textbox", { name: "Code Editor Content" })).toBeDefined();

    // Close button
    const closeButtons = screen.getAllByRole("button", { name: "Close" });
    expect(closeButtons.length).toBeGreaterThan(0);
    const closeBtn = closeButtons[0];
    if (closeBtn) {
      fireEvent.click(closeBtn);
      expect(onClose).toHaveBeenCalled();
    }
  });

  it("should allow editing content and update dirty state", () => {
    render(
      <Win2xManagerProvider>
        <CodeEditorModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const textarea = screen.getByRole("textbox", {
      name: "Code Editor Content",
    }) as HTMLTextAreaElement;
    expect(textarea.value).toBe("pub fn test_initial() {}\n");

    fireEvent.change(textarea, { target: { value: "pub fn modified() {}\n" } });
    expect(useCDDMStore.getState().editorIsDirty).toBe(true);
    expect(useCDDMStore.getState().activeEditorContent).toBe("pub fn modified() {}\n");
  });

  it("should toggle Split Diff mode", () => {
    render(
      <Win2xManagerProvider>
        <CodeEditorModal isOpen={true} onClose={() => {}} />
      </Win2xManagerProvider>,
    );

    const toggleBtn = screen.getByTitle("Toggle Split Diff Mode");
    expect(toggleBtn).toBeDefined();

    fireEvent.click(toggleBtn);
    expect(useCDDMStore.getState().editorSplitDiffMode).toBe(true);

    // Should now render side-by-side split diff surfaces
    expect(screen.getByTestId("monaco-editor-diff")).toBeDefined();
    expect(screen.getByRole("textbox", { name: "Original file content" })).toBeDefined();
    expect(screen.getByRole("textbox", { name: "Modified file content" })).toBeDefined();
  });

  it("should render safely in uninitialized default store state without throwing", () => {
    act(() => {
      useCDDMStore.getState().resetScan();
      useCDDMStore.setState({
        activeEditorFile: null,
        activeEditorContent: "",
        originalEditorContent: "",
        editorFilesList: [],
      });
    });

    render(
      <Win2xManagerProvider>
        <CodeEditorModal isOpen={true} onClose={vi.fn()} />
      </Win2xManagerProvider>,
    );

    expect(screen.getByText("Integrated Code Editor & Split Diff Studio")).toBeDefined();
    expect(screen.getByText(/Ready — choose a file to begin/)).toBeDefined();
  });
});
