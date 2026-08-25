import * as vscode from "vscode";
import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";
import {
  handleCheckPolicies,
  handleExportSarif,
  handleOpenLocation,
  handleShowHealth,
} from "./commands/actions";
import { DEFAULT_MIN_TOKENS, DEFAULT_STUDIO_URL, SUPPORTED_LANGUAGES } from "./constants";
import { CDDMSidebarViewProvider } from "./webview/sidebar-provider";
import { StudioPanel } from "./webview/studio-panel";

export { SUPPORTED_LANGUAGES };

let client: LanguageClient | undefined;
let statusBarItem: vscode.StatusBarItem | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const config = vscode.workspace.getConfiguration("cddm");
  const executablePath = config.get<string>("executablePath", "cddm");
  const minTokens = config.get<number>("minTokens", DEFAULT_MIN_TOKENS);
  const studioUrl = config.get<string>("studioUrl", DEFAULT_STUDIO_URL);

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
    documentSelector: SUPPORTED_LANGUAGES.map((lang) => ({ scheme: "file", language: lang })),
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

  // Sidebar Dashboard Webview Provider
  const sidebarProvider = new CDDMSidebarViewProvider(context.extensionUri, studioUrl);
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(CDDMSidebarViewProvider.viewType, sidebarProvider),
  );

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("cddm.openStudioView", () => {
      StudioPanel.createOrShow(context.extensionUri, studioUrl);
    }),
  );

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
    vscode.commands.registerCommand("cddm.showHealth", async () => {
      await handleShowHealth(client, studioUrl);
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("cddm.checkPolicies", async () => {
      await handleCheckPolicies();
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("cddm.exportSarif", async () => {
      await handleExportSarif();
    }),
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "cddm.openLocation",
      async (uriStr: string, startLine: number, endLine: number) => {
        await handleOpenLocation(uriStr, startLine, endLine);
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
