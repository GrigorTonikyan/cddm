import { API_ROUTES } from "../../constants/cddm-constants";
import type {
  AiPromptResponse,
  AiRefactorPromptRequest,
  ExtractRequest,
  ExtractResult,
  RefactorSandboxRequest,
  VerifyRefactorRequest,
} from "../../types/cddm-types";
import type { GetStoreState, SetStoreState } from "./scan-slice";

export const createRefactorSlice = (set: SetStoreState, _get: GetStoreState) => ({
  openRefactorSandbox: async (req: RefactorSandboxRequest) => {
    const normalizedReq = {
      ...req,
      occurrences: (req.occurrences || []).map((occ) => ({
        ...occ,
        file: occ.file.replace(/\\/g, "/"),
      })),
    };
    set({
      isRefactorSandboxOpen: true,
      sandboxRequest: normalizedReq,
      sandboxResult: null,
      isSandboxLoading: true,
      sandboxError: null,
    });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_SANDBOX, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(normalizedReq),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Sandbox simulation failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({ sandboxResult: result, isSandboxLoading: false, sandboxError: null });
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to run sandbox simulation";
      set({ sandboxError: msg, isSandboxLoading: false });
    }
  },

  previewRefactorSandbox: async (req: RefactorSandboxRequest) => {
    const normalizedReq = {
      ...req,
      occurrences: (req.occurrences || []).map((occ) => ({
        ...occ,
        file: occ.file.replace(/\\/g, "/"),
      })),
    };
    set({ isSandboxLoading: true, sandboxError: null });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_SANDBOX, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(normalizedReq),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Sandbox simulation failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({
        sandboxRequest: normalizedReq,
        sandboxResult: result,
        isSandboxLoading: false,
        sandboxError: null,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to run sandbox simulation";
      set({ sandboxError: msg, isSandboxLoading: false });
      throw err;
    }
  },

  previewAstRefactor: async (req: RefactorSandboxRequest) => {
    const normalizedReq = {
      ...req,
      occurrences: (req.occurrences || []).map((occ) => ({
        ...occ,
        file: occ.file.replace(/\\/g, "/"),
      })),
    };
    set({ isAstLoading: true, astError: null });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_AST, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(normalizedReq),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`AST refactor simulation failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({
        astRewriteResult: result,
        isAstLoading: false,
        astError: null,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to synthesize AST refactoring";
      set({ astError: msg, isAstLoading: false });
      throw err;
    }
  },

  verifyRefactorTestSuite: async (req: VerifyRefactorRequest) => {
    set({ isVerifying: true, verifyError: null });
    try {
      const res = await fetch(API_ROUTES.REFACTOR_VERIFY, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(req),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Test verification execution failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({
        verifyResult: result,
        isVerifying: false,
        verifyError: null,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to execute test verification";
      set({ verifyError: msg, isVerifying: false });
      throw err;
    }
  },

  applyRefactorBranch: async (patch: string, branchName?: string, createBranch: boolean = true) => {
    try {
      const res = await fetch(API_ROUTES.REFACTOR_APPLY_BRANCH, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          patch,
          branch_name: branchName,
          create_branch: createBranch,
        }),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Branch application failed (${res.status}): ${errorText}`);
      }
      const result = await res.json();
      set({ patchStatusMessage: result.message });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to apply refactor branch";
      set({ patchStatusMessage: msg });
      throw err;
    }
  },

  generateAiPrompt: async (req: AiRefactorPromptRequest) => {
    const res = await fetch(API_ROUTES.REFACTOR_AI_PROMPT, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(req),
    });
    if (!res.ok) {
      const errorText = await res.text().catch(() => res.statusText);
      throw new Error(`AI prompt generation failed (${res.status}): ${errorText}`);
    }
    const data: AiPromptResponse = await res.json();
    return data.prompt;
  },

  previewExtractModule: async (req: ExtractRequest) => {
    set({ isExtractLoading: true, extractError: null });
    try {
      const normalizedOccurrences = (req.occurrences || []).map((occ) => ({
        ...occ,
        file: occ.file.replace(/\\/g, "/"),
      }));
      const res = await fetch(API_ROUTES.EXTRACT_PREVIEW, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...req, occurrences: normalizedOccurrences }),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Extract preview failed (${res.status}): ${errorText}`);
      }
      const result: ExtractResult = await res.json();
      set({
        extractResult: result,
        isExtractLoading: false,
        extractError: null,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to generate extraction preview";
      set({ extractError: msg, isExtractLoading: false });
      throw err;
    }
  },

  applyExtractModule: async (req: ExtractRequest) => {
    set({ isExtractLoading: true, extractError: null });
    try {
      const normalizedOccurrences = (req.occurrences || []).map((occ) => ({
        ...occ,
        file: occ.file.replace(/\\/g, "/"),
      }));
      const res = await fetch(API_ROUTES.EXTRACT_APPLY, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...req, occurrences: normalizedOccurrences, dry_run: false }),
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => res.statusText);
        throw new Error(`Extract application failed (${res.status}): ${errorText}`);
      }
      const result: ExtractResult = await res.json();
      set({
        extractResult: result,
        isExtractLoading: false,
        extractError: null,
        patchStatusMessage: result.message,
      });
      return result;
    } catch (err) {
      const msg = err instanceof Error ? err.message : "Failed to apply shared extraction";
      set({ extractError: msg, isExtractLoading: false });
      throw err;
    }
  },
});
