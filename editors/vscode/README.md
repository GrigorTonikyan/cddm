# CDDM for Visual Studio Code, Cursor & Antigravity

Official Visual Studio Code and Cursor extension for **CDDM (Code De-Duplication Meister)**.

Provides real-time duplicate code diagnostics, inline DRY health analytics, embedded WebUI Studio, Activity Bar dashboard, counterpart jump navigation, and automated one-click deduplication refactorings.

---

## Features

- **Embedded WebUI Studio (`cddm.openStudioView`)**: Full-featured interactive CDDM Studio running directly inside an editor tab, complete with side-by-side diffs, N-Way cluster consensus refactoring, and AST rewrite sandboxes.
- **Activity Bar Dashboard (`cddm.sidebarView`)**: Quick-glance DRY health score gauge, cluster metrics, and one-click actions right in your editor sidebar.
- **Inline Duplicate Diagnostics**: Highlights duplicate code blocks with severity based on duplication size and classification (Type-1 Exact, Type-2 Renamed, Type-3 Near-Miss, Type-4 Semantic).
- **One-Click Quick Fixes (`textDocument/codeAction`)**: Deduplicate repeated snippets into shared helper functions directly from the editor lightbulb menu.
- **Rich Hover Tooltips**: Hover over duplicate blocks to inspect match percentage, token volume, line span, and counterpart file links.
- **Jump to Counterpart**: Navigate directly from clone site A to clone site B with a single click.
- **Polyglot AST Support**: Works across 24 languages: Rust, TypeScript, JavaScript, Python, Go, C, C++, Java, C#, Ruby, PHP, Swift, Shell/Bash, Lua, JSON, HTML, Kotlin, Zig, Scala, Elixir, SQL, and Dockerfile.

---

## Requirements

Ensure `cddm` or `cddm-lsp` is installed and available on your system `PATH`:

```bash
cargo install --path crates/cddm-cli
# or
cargo install --path crates/cddm-lsp
```

---

## Extension Settings

| Setting                  | Default                   | Description                                                   |
| :----------------------- | :------------------------ | :------------------------------------------------------------ |
| `cddm.executablePath`    | `"cddm"`                  | Path to the `cddm` or `cddm-lsp` executable                   |
| `cddm.minTokens`         | `50`                      | Minimum token count threshold to classify duplicate fragments |
| `cddm.enableHover`       | `true`                    | Enable rich Markdown hover tooltips                           |
| `cddm.enableCodeActions` | `true`                    | Enable quick-fix refactoring and function extraction          |
| `cddm.studioUrl`         | `"http://127.0.0.1:3000"` | Local CDDM WebUI Studio daemon URL                            |

---

## Commands

| Command Title                           | Identifier             | Description                                     |
| :-------------------------------------- | :--------------------- | :---------------------------------------------- |
| `CDDM: Open Embedded Studio Panel`      | `cddm.openStudioView`  | Open CDDM Studio in a dedicated VS Code tab     |
| `CDDM: Rescan Workspace Duplication`    | `cddm.rescanWorkspace` | Trigger an immediate LSP workspace rescan       |
| `CDDM: Show DRY Health Summary`         | `cddm.showHealth`      | Display workspace DRY health score notification |
| `CDDM: Evaluate Architectural Policies` | `cddm.checkPolicies`   | Check boundary rules from `.cddmrules.toml`     |
| `CDDM: Export SARIF 2.1.0 Report`       | `cddm.exportSarif`     | Export standard SARIF report for CodeQL/CI      |
| `CDDM: Open WebUI Studio (Browser)`     | `cddm.openStudio`      | Open Studio in default external browser         |
| `CDDM: Jump to Counterpart Location`    | `cddm.openLocation`    | Navigate to duplicate counterpart occurrence    |

---

## Building and Packaging VSIX

To build and package the extension into a standalone `.vsix` installer:

```bash
bun run package:vscode
# or
bun scripts/package-vscode.ts
```

The output package will be generated at `packaging/vscode/cddm-1.7.0.vsix`.

To install into VS Code or Cursor:

```bash
code --install-extension packaging/vscode/cddm-1.7.0.vsix
```

---

## License

MIT OR Apache-2.0
