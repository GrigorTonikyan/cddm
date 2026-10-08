import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { EditorToolbar } from "./EditorToolbar";
import { useCDDMStore } from "../../store/cddm-store";

describe("EditorToolbar Component", () => {
  beforeEach(() => {
    act(() => {
      useCDDMStore.setState({
        activeEditorFile: "crates/cddm-core/src/lib.rs",
        editorLanguage: "rust",
        editorIsDirty: true,
        editorIsLoading: false,
        editorError: null,
        editorSaveSuccess: false,
        editorSplitDiffMode: false,
        editorFilesList: [
          {
            path: "crates/cddm-core/src/lib.rs",
            name: "lib.rs",
            is_dir: false,
            language: "rust",
            size_bytes: 120,
          },
          {
            path: "webui/src/App.tsx",
            name: "App.tsx",
            is_dir: false,
            language: "typescript",
            size_bytes: 500,
          },
        ],
      });
    });
  });

  it("should render file selector and language picker", () => {
    render(<EditorToolbar onSave={() => {}} onReload={() => {}} />);

    const fileSelect = screen.getByRole("combobox", {
      name: "Select file to edit",
    }) as HTMLSelectElement;
    expect(fileSelect.value).toBe("crates/cddm-core/src/lib.rs");

    const langSelect = screen.getByRole("combobox", {
      name: "Select syntax language",
    }) as HTMLSelectElement;
    expect(langSelect.value).toBe("rust");

    expect(screen.getByText("Unsaved Changes")).toBeDefined();
  });

  it("should invoke onSave when Save button clicked", () => {
    const onSave = vi.fn();
    render(<EditorToolbar onSave={onSave} onReload={() => {}} />);

    const saveBtn = screen.getByTitle("Save file to workspace (Ctrl+S)");
    fireEvent.click(saveBtn);
    expect(onSave).toHaveBeenCalled();
  });

  it("should invoke onReload when Reload button clicked", () => {
    const onReload = vi.fn();
    render(<EditorToolbar onSave={() => {}} onReload={onReload} />);

    const reloadBtn = screen.getByTitle("Discard changes and reload from disk");
    fireEvent.click(reloadBtn);
    expect(onReload).toHaveBeenCalled();
  });

  it("should toggle Split Diff mode", () => {
    render(<EditorToolbar onSave={() => {}} onReload={() => {}} />);

    const toggleBtn = screen.getByTitle("Toggle Split Diff Mode");
    expect(screen.getByText("Split Diff: OFF")).toBeDefined();

    fireEvent.click(toggleBtn);
    expect(useCDDMStore.getState().editorSplitDiffMode).toBe(true);
  });
});
