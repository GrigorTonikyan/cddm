export type SupportedEditor = "vscode" | "cursor" | "zed" | "windsurf";

export interface EditorOption {
  id: SupportedEditor;
  name: string;
  scheme: string;
}

export const SUPPORTED_EDITORS: readonly EditorOption[] = [
  { id: "vscode", name: "Visual Studio Code", scheme: "vscode" },
  { id: "cursor", name: "Cursor", scheme: "cursor" },
  { id: "zed", name: "Zed", scheme: "zed" },
  { id: "windsurf", name: "Windsurf", scheme: "windsurf" },
] as const;

export const DEFAULT_EDITOR: SupportedEditor = "vscode";

/**
 * Normalizes a file path to forward slashes.
 */
export function normalizePathForDeeplink(filePath: string): string {
  return filePath.replace(/\\/g, "/");
}

/**
 * Constructs an editor URI protocol deeplink for a given file path and 1-based line number.
 */
export function getIdeDeeplink(
  filePath: string,
  line: number = 1,
  editor: SupportedEditor = DEFAULT_EDITOR,
): string {
  const normalized = normalizePathForDeeplink(filePath);
  const cleanLine = Math.max(1, line);

  switch (editor) {
    case "cursor":
      return `cursor://file/${normalized}:${cleanLine}`;
    case "zed":
      return `zed://file/${normalized}:${cleanLine}`;
    case "windsurf":
      return `windsurf://file/${normalized}:${cleanLine}`;
    case "vscode":
    default:
      return `vscode://file/${normalized}:${cleanLine}`;
  }
}

/**
 * Returns human-readable display name for an editor.
 */
export function getEditorDisplayName(editor: SupportedEditor): string {
  const match = SUPPORTED_EDITORS.find((e) => e.id === editor);
  return match ? match.name : "Visual Studio Code";
}
