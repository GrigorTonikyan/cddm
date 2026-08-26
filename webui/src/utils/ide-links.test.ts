import { describe, it, expect } from "vite-plus/test";
import {
  getIdeDeeplink,
  getEditorDisplayName,
  normalizePathForDeeplink,
  SUPPORTED_EDITORS,
} from "./ide-links";

describe("ide-links utility", () => {
  it("normalizes backslashes to forward slashes", () => {
    expect(normalizePathForDeeplink("crates\\cddm-core\\src\\main.rs")).toBe(
      "crates/cddm-core/src/main.rs",
    );
  });

  it("generates correct VS Code deeplinks", () => {
    const link = getIdeDeeplink("src/lib.rs", 42, "vscode");
    expect(link).toBe("vscode://file/src/lib.rs:42");
  });

  it("generates correct Cursor deeplinks", () => {
    const link = getIdeDeeplink("src/detector.rs", 15, "cursor");
    expect(link).toBe("cursor://file/src/detector.rs:15");
  });

  it("generates correct Zed deeplinks", () => {
    const link = getIdeDeeplink("src/refactor.rs", 88, "zed");
    expect(link).toBe("zed://file/src/refactor.rs:88");
  });

  it("generates correct Windsurf deeplinks", () => {
    const link = getIdeDeeplink("webui/src/App.tsx", 10, "windsurf");
    expect(link).toBe("windsurf://file/webui/src/App.tsx:10");
  });

  it("handles line numbers below 1 by clamping to 1", () => {
    const link = getIdeDeeplink("src/main.rs", 0, "vscode");
    expect(link).toBe("vscode://file/src/main.rs:1");
  });

  it("returns human-readable display names", () => {
    expect(getEditorDisplayName("vscode")).toBe("Visual Studio Code");
    expect(getEditorDisplayName("cursor")).toBe("Cursor");
    expect(getEditorDisplayName("zed")).toBe("Zed");
    expect(getEditorDisplayName("windsurf")).toBe("Windsurf");
  });

  it("exposes 4 supported editor options", () => {
    expect(SUPPORTED_EDITORS).toHaveLength(4);
  });
});
