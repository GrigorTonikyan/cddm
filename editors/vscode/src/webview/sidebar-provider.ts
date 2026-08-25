import * as vscode from "vscode";
import { handleOpenLocation } from "../commands/actions";
import { WebviewIncomingMessage } from "../types";

/**
 * Provides the CDDM DRY Health & Studio dashboard inside the VS Code Activity Bar sidebar.
 */
export class CDDMSidebarViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = "cddm.sidebarView";
  private _view?: vscode.WebviewView;

  constructor(
    private readonly _extensionUri: vscode.Uri,
    private readonly _studioUrl: string,
  ) {}

  public get view(): vscode.WebviewView | undefined {
    return this._view;
  }

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ): void {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [vscode.Uri.joinPath(this._extensionUri, "resources")],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage(async (message: WebviewIncomingMessage) => {
      switch (message.type) {
        case "openEmbeddedStudio":
          await vscode.commands.executeCommand("cddm.openStudioView");
          break;
        case "openExternalStudio":
          await vscode.env.openExternal(vscode.Uri.parse(this._studioUrl));
          break;
        case "rescanWorkspace":
          await vscode.commands.executeCommand("cddm.rescanWorkspace");
          break;
        case "checkPolicies":
          await vscode.commands.executeCommand("cddm.checkPolicies");
          break;
        case "exportSarif":
          await vscode.commands.executeCommand("cddm.exportSarif");
          break;
        case "openLocation":
          await handleOpenLocation(message.file, message.startLine, message.endLine);
          break;
      }
    });
  }

  private _getHtmlForWebview(webview: vscode.Webview): string {
    const nonce = getNonce();

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
  <title>CDDM Duplication Dashboard</title>
  <style>
    body {
      padding: 12px;
      color: var(--vscode-foreground);
      font-family: var(--vscode-font-family);
      font-size: var(--vscode-font-size);
      background: transparent;
      box-sizing: border-box;
    }
    .hero-card {
      background: var(--vscode-editorWidget-background, #252526);
      border: 1px solid var(--vscode-widget-border, #3c3c3c);
      border-radius: 6px;
      padding: 12px;
      margin-bottom: 12px;
      text-align: center;
    }
    .score-title {
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      color: var(--vscode-descriptionForeground, #888);
      margin-bottom: 4px;
    }
    .score-value {
      font-size: 24px;
      font-weight: 700;
      color: #22c55e;
    }
    .score-label {
      font-size: 11px;
      color: var(--vscode-descriptionForeground, #aaa);
    }
    .section-title {
      font-size: 11px;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      color: var(--vscode-sideBarSectionHeader-foreground, #aaa);
      margin: 14px 0 8px 0;
    }
    .action-btn {
      width: 100%;
      background: var(--vscode-button-background, #0e639c);
      color: var(--vscode-button-foreground, #ffffff);
      border: none;
      border-radius: 4px;
      padding: 7px 10px;
      font-size: 12px;
      font-weight: 500;
      cursor: pointer;
      margin-bottom: 6px;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 6px;
      box-sizing: border-box;
    }
    .action-btn:hover {
      background: var(--vscode-button-hoverBackground, #1177bb);
    }
    .action-btn-secondary {
      background: var(--vscode-button-secondaryBackground, #3a3d41);
      color: var(--vscode-button-secondaryForeground, #ffffff);
    }
    .action-btn-secondary:hover {
      background: var(--vscode-button-secondaryHoverBackground, #45494e);
    }
    .features-list {
      display: flex;
      flex-direction: column;
      gap: 6px;
    }
    .feature-card {
      background: var(--vscode-editorWidget-background, #252526);
      border: 1px solid var(--vscode-widget-border, #333);
      border-radius: 4px;
      padding: 8px 10px;
      cursor: pointer;
      transition: background 0.15s ease;
    }
    .feature-card:hover {
      background: var(--vscode-list-hoverBackground, #2a2d2e);
    }
    .feature-name {
      font-weight: 600;
      font-size: 12px;
      margin-bottom: 2px;
    }
    .feature-desc {
      font-size: 11px;
      color: var(--vscode-descriptionForeground, #888);
    }
  </style>
</head>
<body>
  <div class="hero-card">
    <div class="score-title">DRY Health Status</div>
    <div class="score-value">Active</div>
    <div class="score-label">LSP Real-Time Clone Diagnostics</div>
  </div>

  <button class="action-btn" id="btn-open-panel">
    Open Embedded Studio Panel
  </button>
  <button class="action-btn action-btn-secondary" id="btn-rescan">
    Rescan Workspace
  </button>

  <div class="section-title">Studio Features</div>
  <div class="features-list">
    <div class="feature-card" id="card-clusters">
      <div class="feature-name">N-Way Graph Clusters</div>
      <div class="feature-desc">Transitive multi-site deduplication</div>
    </div>
    <div class="feature-card" id="card-sandbox">
      <div class="feature-name">Refactor Sandbox</div>
      <div class="feature-desc">Live AST patches and test runner</div>
    </div>
    <div class="feature-card" id="card-policies">
      <div class="feature-name">Architectural Policies</div>
      <div class="feature-desc">Cross-layer boundary isolation</div>
    </div>
    <div class="feature-card" id="card-sarif">
      <div class="feature-name">Export SARIF 2.1.0</div>
      <div class="feature-desc">CI/CD & CodeQL reporting format</div>
    </div>
  </div>

  <div class="section-title">External</div>
  <button class="action-btn action-btn-secondary" id="btn-external">
    Open in Browser (Port 3000)
  </button>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();

    document.getElementById('btn-open-panel').addEventListener('click', () => {
      vscode.postMessage({ type: 'openEmbeddedStudio' });
    });

    document.getElementById('btn-rescan').addEventListener('click', () => {
      vscode.postMessage({ type: 'rescanWorkspace' });
    });

    document.getElementById('card-clusters').addEventListener('click', () => {
      vscode.postMessage({ type: 'openEmbeddedStudio' });
    });

    document.getElementById('card-sandbox').addEventListener('click', () => {
      vscode.postMessage({ type: 'openEmbeddedStudio' });
    });

    document.getElementById('card-policies').addEventListener('click', () => {
      vscode.postMessage({ type: 'checkPolicies' });
    });

    document.getElementById('card-sarif').addEventListener('click', () => {
      vscode.postMessage({ type: 'exportSarif' });
    });

    document.getElementById('btn-external').addEventListener('click', () => {
      vscode.postMessage({ type: 'openExternalStudio' });
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
