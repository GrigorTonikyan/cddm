import { render, screen, fireEvent, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vite-plus/test";
import { EditorSurface } from "./EditorSurface";
import { useCDDMStore } from "../../store/cddm-store";

describe("EditorSurface Component", () => {
  beforeEach(() => {
    act(() => {
      useCDDMStore.setState({
        activeEditorFile: "crates/cddm-core/src/lib.rs",
        activeEditorContent: "fn main() {}\n",
        originalEditorContent: "fn main() {}\n",
        editorLanguage: "rust",
        editorIsDirty: false,
        editorSplitDiffMode: false,
        editorDiffFileA: "crates/cddm-core/src/base.rs",
        editorDiffContentA: "fn base() {}\n",
      });
    });
  });

  it("should render fallback editor in test environment", () => {
    render(<EditorSurface onSave={() => {}} />);
    const textarea = screen.getByRole("textbox", {
      name: "Code Editor Content",
    }) as HTMLTextAreaElement;
    expect(textarea).toBeDefined();
    expect(textarea.value).toBe("fn main() {}\n");
  });

  it("should trigger onSave when Ctrl+S pressed on textarea", () => {
    const onSave = vi.fn();
    render(<EditorSurface onSave={onSave} />);
    const textarea = screen.getByRole("textbox", { name: "Code Editor Content" });

    fireEvent.keyDown(textarea, { key: "s", ctrlKey: true });
    expect(onSave).toHaveBeenCalled();
  });

  it("should render side-by-side diff when split diff mode is active", () => {
    act(() => {
      useCDDMStore.setState({ editorSplitDiffMode: true });
    });

    render(<EditorSurface onSave={() => {}} />);
    expect(screen.getByTestId("monaco-editor-diff")).toBeDefined();
    expect(screen.getByRole("textbox", { name: "Original file content" })).toBeDefined();
    expect(screen.getByRole("textbox", { name: "Modified file content" })).toBeDefined();
  });
});
