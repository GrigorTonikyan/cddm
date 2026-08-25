import * as vscode from "vscode";
import { handleOpenLocation } from "../commands/actions";
import { WebviewIncomingMessage } from "../types";

/**
 * Manages the full-tab CDDM Studio Webview panel inside Visual Studio Code.
 */
export class StudioPanel {
  public static currentPanel: StudioPanel | undefined;
  public static readonly viewType = "cddmStudio";

  private readonly _panel: vscode.WebviewPanel;
  private readonly _extensionUri: vscode.Uri;
  private _disposables: vscode.Disposable[] = [];
  private _studioUrl: string;

  public static createOrShow(extensionUri: vscode.Uri, studioUrl: string): StudioPanel {
    const column = vscode.window.activeTextEditor
      ? vscode.window.activeTextEditor.viewColumn
      : undefined;

    if (StudioPanel.currentPanel) {
      StudioPanel.currentPanel._studioUrl = studioUrl;
      StudioPanel.currentPanel._panel.reveal(column);
      StudioPanel.currentPanel._update();
      return StudioPanel.currentPanel;
    }

    const panel = vscode.window.createWebviewPanel(
      StudioPanel.viewType,
      "CDDM Studio",
      column || vscode.ViewColumn.One,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
        localResourceRoots: [vscode.Uri.joinPath(extensionUri, "resources")],
      },
    );

    StudioPanel.currentPanel = new StudioPanel(panel, extensionUri, studioUrl);
    return StudioPanel.currentPanel;
  }

  private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, studioUrl: string) {
    this._panel = panel;
    this._extensionUri = extensionUri;
    this._studioUrl = studioUrl;

    this._update();

    this._panel.onDidDispose(() => this.dispose(), null, this._disposables);

    this._panel.webview.onDidReceiveMessage(
      async (message: WebviewIncomingMessage) => {
        switch (message.type) {
          case "openLocation":
            await handleOpenLocation(message.file, message.startLine, message.endLine);
            break;
          case "rescanWorkspace":
            await vscode.commands.executeCommand("cddm.rescanWorkspace");
            break;
          case "checkPolicies":
            await vscode.commands.executeCommand("cddm.checkPolicies");
            break;
          case "openExternalStudio":
            await vscode.env.openExternal(vscode.Uri.parse(this._studioUrl));
            break;
          case "copyText":
            await vscode.env.clipboard.writeText(message.text);
            vscode.window.showInformationMessage("[CDDM] Copied to clipboard.");
            break;
        }
      },
      null,
      this._disposables,
    );
  }

  public get extensionUri(): vscode.Uri {
    return this._extensionUri;
  }

  public dispose(): void {
    StudioPanel.currentPanel = undefined;
    this._panel.dispose();
    while (this._disposables.length) {
      const d = this._disposables.pop();
      if (d) d.dispose();
    }
  }

  private _update(): void {
    this._panel.title = "CDDM Studio";
    this._panel.webview.html = this._getHtmlForWebview(this._panel.webview);
  }

  private _getHtmlForWebview(webview: vscode.Webview): string {
    const nonce = getNonce();

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; frame-src ${this._studioUrl} http://127.0.0.1:* http://localhost:*; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
  <title>CDDM Studio</title>
  <style>
    body, html {
      margin: 0;
      padding: 0;
      width: 100%;
      height: 100%;
      overflow: hidden;
      background-color: var(--vscode-editor-background);
      color: var(--vscode-editor-foreground);
      font-family: var(--vscode-font-family);
    }
    .header-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      height: 36px;
      padding: 0 12px;
      background: var(--vscode-titleBar-activeBackground, #1e1e1e);
      border-bottom: 1px solid var(--vscode-panel-border, #333);
      box-sizing: border-box;
    }
    .title-group {
      display: flex;
      align-items: center;
      gap: 8px;
      font-weight: 600;
      font-size: 12px;
    }
    .status-badge {
      display: inline-block;
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: #22c55e;
    }
    .actions-group {
      display: flex;
      align-items: center;
      gap: 6px;
    }
    .btn {
      background: var(--vscode-button-secondaryBackground, #3a3d41);
      color: var(--vscode-button-secondaryForeground, #ffffff);
      border: 1px solid var(--vscode-button-border, transparent);
      padding: 3px 8px;
      font-size: 11px;
      border-radius: 3px;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      gap: 4px;
    }
    .btn:hover {
      background: var(--vscode-button-secondaryHoverBackground, #45494e);
    }
    .btn-primary {
      background: var(--vscode-button-background, #0e639c);
      color: var(--vscode-button-foreground, #ffffff);
    }
    .btn-primary:hover {
      background: var(--vscode-button-hoverBackground, #1177bb);
    }
    .frame-container {
      width: 100%;
      height: calc(100% - 36px);
      position: relative;
    }
    iframe {
      width: 100%;
      height: 100%;
      border: none;
    }
  </style>
</head>
<body>
  <div class="header-bar">
    <div class="title-group">
      <span class="status-badge" title="CDDM Studio Connected"></span>
      <span>CDDM Studio WebUI</span>
    </div>
    <div class="actions-group">
      <button class="btn" id="btn-rescan">Rescan</button>
      <button class="btn" id="btn-policies">Policies</button>
      <button class="btn" id="btn-refresh">Reload Frame</button>
      <button class="btn btn-primary" id="btn-browser">Open in Browser</button>
    </div>
  </div>
  <div class="frame-container">
    <iframe id="studio-frame" src="${this._studioUrl}"></iframe>
  </div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const frame = document.getElementById('studio-frame');

    document.getElementById('btn-rescan').addEventListener('click', () => {
      vscode.postMessage({ type: 'rescanWorkspace' });
    });

    document.getElementById('btn-policies').addEventListener('click', () => {
      vscode.postMessage({ type: 'checkPolicies' });
    });

    document.getElementById('btn-refresh').addEventListener('click', () => {
      frame.src = frame.src;
    });

    document.getElementById('btn-browser').addEventListener('click', () => {
      vscode.postMessage({ type: 'openExternalStudio' });
    });

    // Listen for messages from iframe
    window.addEventListener('message', (event) => {
      if (event.data && event.data.type === 'cddm-open-location') {
        vscode.postMessage({
          type: 'openLocation',
          file: event.data.file,
          startLine: event.data.startLine,
          endLine: event.data.endLine
        });
      }
    });
  </script>
</body>
</html>`;
  }
}

function getNonce(): string {
  let text = "";
  const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
