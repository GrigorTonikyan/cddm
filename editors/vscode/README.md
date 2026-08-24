# CDDM for Visual Studio Code & Cursor

Official Visual Studio Code and Cursor extension for **CDDM (Code De-Duplication Meister)**.

Provides real-time duplicate code diagnostics, inline DRY health analytics, counterpart jump navigation, and automated one-click deduplication refactorings.

---

## Features

- **Inline Duplicate Diagnostics**: Highlights duplicate code blocks with severity based on duplication size and classification (Type-1 Exact, Type-2 Renamed, Type-3 Near-Miss, Type-4 Semantic).
- **One-Click Quick Fixes (`textDocument/codeAction`)**: Deduplicate repeated snippets into shared helper functions directly from the editor lightbulb menu.
- **Rich Hover Tooltips**: Hover over duplicate blocks to inspect match percentage, token volume, line span, and counterpart file links.
- **Jump to Counterpart**: Navigate directly from clone site A to clone site B with a single click.
- **Polyglot Support**: Works across Rust, TypeScript, JavaScript, Python, Go, C, C++, Java, and C#.

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

- `CDDM: Rescan Workspace Duplication` (`cddm.rescanWorkspace`): Manually trigger a fresh workspace scan.
- `CDDM: Open WebUI Studio` (`cddm.openStudio`): Launch the interactive CDDM Studio in your default browser.
- `CDDM: Jump to Counterpart Location` (`cddm.openLocation`): Navigate to a matched duplicate occurrence.

---

## License

MIT OR Apache-2.0
