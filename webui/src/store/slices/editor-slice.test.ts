import { describe, expect, it, vi, beforeEach } from "vite-plus/test";
import { createEditorSlice, initialEditorState } from "./editor-slice";

describe("EditorSlice Store", () => {
  let state: any;
  const set = (updater: any) => {
    if (typeof updater === "function") {
      state = { ...state, ...updater(state) };
    } else {
      state = { ...state, ...updater };
    }
  };
  const get = () => state;

  beforeEach(() => {
    state = {
      ...initialEditorState,
      ...createEditorSlice(set, get),
    };
    vi.restoreAllMocks();
  });

  it("should initialize with default closed editor state", () => {
    expect(state.isCodeEditorOpen).toBe(false);
    expect(state.activeEditorFile).toBeNull();
    expect(state.activeEditorContent).toBe("");
    expect(state.editorIsDirty).toBe(false);
    expect(state.editorSplitDiffMode).toBe(false);
  });

  it("should toggle isCodeEditorOpen and trigger file fetch if list is empty", () => {
    const fetchSpy = vi.fn().mockResolvedValue(undefined);
    state.fetchEditorFiles = fetchSpy;

    state.setIsCodeEditorOpen(true);
    expect(state.isCodeEditorOpen).toBe(true);
    expect(fetchSpy).toHaveBeenCalled();
  });

  it("should open file in editor and populate content", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        path: "src/main.rs",
        content: "fn main() {}",
        language: "Rust",
        size_bytes: 12,
        total_lines: 1,
        modified_timestamp_ms: 12345,
      }),
    });

    await state.openFileInEditor("src/main.rs");

    expect(state.isCodeEditorOpen).toBe(true);
    expect(state.activeEditorFile).toBe("src/main.rs");
    expect(state.activeEditorContent).toBe("fn main() {}");
    expect(state.editorLanguage).toBe("rust");
    expect(state.editorIsDirty).toBe(false);
    expect(state.editorIsLoading).toBe(false);
  });

  it("should mark editor as dirty when content changes from original", () => {
    state.originalEditorContent = "let x = 1;";
    state.activeEditorContent = "let x = 1;";
    state.editorIsDirty = false;

    state.setActiveEditorContent("let x = 2;");
    expect(state.activeEditorContent).toBe("let x = 2;");
    expect(state.editorIsDirty).toBe(true);

    state.setActiveEditorContent("let x = 1;");
    expect(state.editorIsDirty).toBe(false);
  });

  it("should save active editor file via POST", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        path: "src/main.rs",
        bytes_written: 20,
        success: true,
        modified_timestamp_ms: 54321,
      }),
    });

    state.activeEditorFile = "src/main.rs";
    state.activeEditorContent = "fn updated() {}";
    state.editorIsDirty = true;

    const saved = await state.saveActiveEditorFile();
    expect(saved).toBe(true);
    expect(state.editorIsDirty).toBe(false);
    expect(state.editorSaveSuccess).toBe(true);
    expect(state.originalEditorContent).toBe("fn updated() {}");
  });

  it("should open diff in editor with two files", async () => {
    globalThis.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("fileA.ts")) {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            path: "fileA.ts",
            content: "const a = 1;",
            language: "typescript",
            size_bytes: 12,
            total_lines: 1,
            modified_timestamp_ms: 100,
          }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({
          path: "fileB.ts",
          content: "const b = 2;",
          language: "typescript",
          size_bytes: 12,
          total_lines: 1,
          modified_timestamp_ms: 200,
        }),
      });
    });

    await state.openDiffInEditor("fileA.ts", "fileB.ts");

    expect(state.editorSplitDiffMode).toBe(true);
    expect(state.editorDiffFileA).toBe("fileA.ts");
    expect(state.editorDiffFileB).toBe("fileB.ts");
    expect(state.editorDiffContentA).toBe("const a = 1;");
    expect(state.editorDiffContentB).toBe("const b = 2;");
  });

  it("should close editor cleanly", () => {
    state.isCodeEditorOpen = true;
    state.editorError = "some error";
    state.editorSaveSuccess = true;

    state.closeCodeEditor();

    expect(state.isCodeEditorOpen).toBe(false);
    expect(state.editorError).toBeNull();
    expect(state.editorSaveSuccess).toBe(false);
  });
});
