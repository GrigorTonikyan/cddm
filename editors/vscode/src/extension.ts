import * as vscode from "vscode";
import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;
let statusBarItem: vscode.StatusBarItem | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const config = vscode.workspace.getConfiguration("cddm");
  const executablePath = config.get<string>("executablePath", "cddm");
  const minTokens = config.get<number>("minTokens", 50);
  const studioUrl = config.get<string>("studioUrl", "http://127.0.0.1:3000");

  const isStandaloneLsp =
    executablePath.endsWith("cddm-lsp") || executablePath.endsWith("cddm-lsp.exe");
  const args = isStandaloneLsp ? [] : ["lsp", "--min-tokens", String(minTokens)];

  const serverExecutable: Executable = {
    command: executablePath,
    args,
    options: {
      env: {
        ...process.env,
        RUST_LOG: "info",
      },
    },
  };

  const serverOptions: ServerOptions = {
    run: serverExecutable,
    debug: serverExecutable,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "rust" },
      { scheme: "file", language: "typescript" },
      { scheme: "file", language: "typescriptreact" },
      { scheme: "file", language: "javascript" },
      { scheme: "file", language: "javascriptreact" },
      { scheme: "file", language: "python" },
      { scheme: "file", language: "go" },
      { scheme: "file", language: "c" },
      { scheme: "file", language: "cpp" },
      { scheme: "file", language: "java" },
      { scheme: "file", language: "csharp" },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*"),
    },
  };

  client = new LanguageClient("cddm-lsp", "CDDM Language Server", serverOptions, clientOptions);

  // Status Bar Indicator
  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBarItem.text = "[CDDM] Active";
  statusBarItem.tooltip = "CDDM (Code De-Duplication Meister) LSP Active. Click to rescan.";
  statusBarItem.command = "cddm.rescanWorkspace";
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("cddm.rescanWorkspace", async () => {
      if (!client) return;
      try {
        await client.sendRequest("workspace/executeCommand", {
          command: "cddm.rescanWorkspace",
        });
        vscode.window.showInformationMessage("[CDDM] Workspace rescan completed.");
      } catch (err) {
        vscode.window.showErrorMessage(`[CDDM] Rescan failed: ${String(err)}`);
      }
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("cddm.openStudio", async () => {
      try {
        const uri = vscode.Uri.parse(studioUrl);
        await vscode.env.openExternal(uri);
      } catch (err) {
        vscode.window.showErrorMessage(`[CDDM] Failed to open WebUI Studio: ${String(err)}`);
      }
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "cddm.openLocation",
      async (uriStr: string, startLine: number, endLine: number) => {
        try {
          const uri = vscode.Uri.parse(uriStr);
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
      },
    ),
  );

  await client.start();
}

export async function deactivate(): Promise<void> {
  if (statusBarItem) {
    statusBarItem.dispose();
  }
  if (client) {
    await client.stop();
  }
}
