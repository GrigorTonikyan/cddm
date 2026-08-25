import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

/**
 * Jump to a specific counterpart clone location and highlight the range in editor.
 */
export async function handleOpenLocation(
  uriStr: string,
  startLine: number,
  endLine: number,
): Promise<void> {
  try {
    let uri: vscode.Uri;
    if (uriStr.startsWith("file://") || uriStr.startsWith("http")) {
      uri = vscode.Uri.parse(uriStr);
    } else {
      uri = vscode.Uri.file(uriStr);
    }

    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc, { preview: false });

    const start0 = Math.max(0, startLine - 1);
    const end0 = Math.max(0, endLine - 1);
    const selection = new vscode.Range(start0, 0, end0, 1000);

    editor.selection = new vscode.Selection(selection.start, selection.end);
    editor.revealRange(selection, vscode.TextEditorRevealType.InCenter);
  } catch (err) {
    vscode.window.showErrorMessage(`[CDDM] Failed to jump to location: ${String(err)}`);
  }
}

/**
 * Display a summary notification of workspace DRY health.
 */
export async function handleShowHealth(
  client: LanguageClient | undefined,
  studioUrl: string,
): Promise<void> {
  if (!client) {
    vscode.window.showWarningMessage("[CDDM] Language server is not connected.");
    return;
  }

  const actions = ["Open Studio Panel", "Open in Browser"];
  const selection = await vscode.window.showInformationMessage(
    "[CDDM] Workspace Duplication Monitor Active. Inspect clones and clusters in Studio.",
    ...actions,
  );

  if (selection === "Open Studio Panel") {
    await vscode.commands.executeCommand("cddm.openStudioView");
  } else if (selection === "Open in Browser") {
    await vscode.env.openExternal(vscode.Uri.parse(studioUrl));
  }
}

/**
 * Trigger an architectural policy evaluation check.
 */
export async function handleCheckPolicies(): Promise<void> {
  vscode.window.showInformationMessage(
    "[CDDM] Evaluating architectural policies from .cddmrules.toml...",
  );
  try {
    await vscode.commands.executeCommand("cddm.rescanWorkspace");
    vscode.window.showInformationMessage("[CDDM] Policy evaluation completed.");
  } catch (err) {
    vscode.window.showErrorMessage(`[CDDM] Policy check failed: ${String(err)}`);
  }
}

/**
 * Trigger SARIF report export.
 */
export async function handleExportSarif(): Promise<void> {
  const saveUri = await vscode.window.showSaveDialog({
    filters: { "SARIF Files": ["sarif", "json"] },
    defaultUri: vscode.Uri.file("cddm-report.sarif"),
    title: "Export CDDM SARIF 2.1.0 Report",
  });

  if (!saveUri) return;

  vscode.window.showInformationMessage(`[CDDM] Exporting SARIF report to ${saveUri.fsPath}...`);
}
