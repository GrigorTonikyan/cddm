import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  FileReadResponse,
  FileTreeResponse,
  FileWriteRequest,
  FileWriteResponse,
  WorkspaceFileEntry,
} from "../../types/editor-types";
import { getJson, postJson } from "../../utils/api-client";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const initialEditorState = {
  isCodeEditorOpen: false,
  activeEditorFile: null as string | null,
  activeEditorContent: "",
  originalEditorContent: "",
  editorLanguage: "typescript",
  editorIsDirty: false,
  editorIsLoading: false,
  editorError: null as string | null,
  editorSaveSuccess: false,
  editorSplitDiffMode: false,
  editorDiffFileA: null as string | null,
  editorDiffFileB: null as string | null,
  editorDiffContentA: "",
  editorDiffContentB: "",
  editorFilesList: [] as WorkspaceFileEntry[],
};

export const createEditorSlice = (set: SetStoreState, get: GetStoreState) => ({
  ...initialEditorState,

  setIsCodeEditorOpen: (isCodeEditorOpen: boolean) => {
    set({ isCodeEditorOpen });
    if (isCodeEditorOpen && get().editorFilesList.length === 0) {
      void get().fetchEditorFiles();
    }
  },

  openFileInEditor: async (path: string, _line?: number) => {
    set({
      isCodeEditorOpen: true,
      editorIsLoading: true,
      editorError: null,
      editorSaveSuccess: false,
      editorSplitDiffMode: false,
      activeEditorFile: path,
    });

    try {
      const url = `${API_ROUTES.FILE}?path=${encodeURIComponent(path)}`;
      const data = await getJson<FileReadResponse>(url, `Failed to load file '${path}'`);
      set({
        activeEditorFile: data.path,
        activeEditorContent: data.content,
        originalEditorContent: data.content,
        editorLanguage: data.language.toLowerCase(),
        editorIsDirty: false,
        editorIsLoading: false,
        editorError: null,
      });
    } catch (err) {
      set({
        editorError: err instanceof Error ? err.message : `Failed to load file '${path}'`,
        editorIsLoading: false,
      });
    }
  },

  openDiffInEditor: async (
    fileA: string,
    fileB: string,
    _rangeA?: [number, number],
    _rangeB?: [number, number],
  ) => {
    set({
      isCodeEditorOpen: true,
      editorIsLoading: true,
      editorError: null,
      editorSaveSuccess: false,
      editorSplitDiffMode: true,
      editorDiffFileA: fileA,
      editorDiffFileB: fileB,
      activeEditorFile: fileB,
    });

    try {
      const [resA, resB] = await Promise.all([
        getJson<FileReadResponse>(
          `${API_ROUTES.FILE}?path=${encodeURIComponent(fileA)}`,
          `Failed to load file A '${fileA}'`,
        ),
        getJson<FileReadResponse>(
          `${API_ROUTES.FILE}?path=${encodeURIComponent(fileB)}`,
          `Failed to load file B '${fileB}'`,
        ),
      ]);

      set({
        editorDiffFileA: resA.path,
        editorDiffFileB: resB.path,
        editorDiffContentA: resA.content,
        editorDiffContentB: resB.content,
        activeEditorFile: resB.path,
        activeEditorContent: resB.content,
        originalEditorContent: resB.content,
        editorLanguage: resB.language.toLowerCase() || resA.language.toLowerCase(),
        editorIsDirty: false,
        editorIsLoading: false,
        editorError: null,
      });
    } catch (err) {
      set({
        editorError: err instanceof Error ? err.message : "Failed to load files for diff",
        editorIsLoading: false,
      });
    }
  },

  saveActiveEditorFile: async (): Promise<boolean> => {
    const { activeEditorFile, activeEditorContent } = get();
    if (!activeEditorFile) return false;

    set({ editorIsLoading: true, editorError: null, editorSaveSuccess: false });
    try {
      const payload: FileWriteRequest = {
        path: activeEditorFile,
        content: activeEditorContent,
      };

      await postJson<FileWriteResponse>(
        API_ROUTES.FILE,
        payload,
        `Failed to save file '${activeEditorFile}'`,
      );

      set({
        originalEditorContent: activeEditorContent,
        editorIsDirty: false,
        editorIsLoading: false,
        editorSaveSuccess: true,
        editorError: null,
      });

      return true;
    } catch (err) {
      set({
        editorError:
          err instanceof Error ? err.message : `Failed to save file '${activeEditorFile}'`,
        editorIsLoading: false,
        editorSaveSuccess: false,
      });
      return false;
    }
  },

  setActiveEditorContent: (content: string) => {
    const { originalEditorContent } = get();
    set({
      activeEditorContent: content,
      editorIsDirty: content !== originalEditorContent,
      editorSaveSuccess: false,
    });
  },

  setEditorLanguage: (editorLanguage: string) => {
    set({ editorLanguage });
  },

  setEditorSplitDiffMode: (editorSplitDiffMode: boolean) => {
    set({ editorSplitDiffMode });
  },

  fetchEditorFiles: async () => {
    try {
      const data = await getJson<FileTreeResponse>(
        API_ROUTES.FILE_TREE,
        "Failed to load workspace file tree",
      );
      set({ editorFilesList: data.files });
    } catch {
      // Gracefully ignore tree errors
    }
  },

  closeCodeEditor: () => {
    set({
      isCodeEditorOpen: false,
      editorError: null,
      editorSaveSuccess: false,
    });
  },
});
