# CDDM Language Server Protocol (LSP) Setup & Configuration Guide

> **Document Status**: Living Setup Reference  
> **Repository**: [GrigorTonikyan/cddm](https://github.com/GrigorTonikyan/cddm)  
> **Protocol Specification**: LSP 3.17 over JSON-RPC 2.0 (Stdio)

---

## 1. Overview

The **CDDM LSP Server** (`crates/cddm-lsp` / `cddm lsp`) brings real-time duplicate code detection, DRY health analytics, counterpart jump navigation, and automated deduplication refactoring into your favorite IDE or text editor.

### Supported Capabilities

- **Diagnostics (`textDocument/publishDiagnostics`)**: Real-time warnings with duplicate line counts, token volumes, similarity percentages, and `DiagnosticRelatedInformation` links.
- **Code Actions (`textDocument/codeAction`)**: One-click quick-fixes to extract duplicate code into parameterized functions.
- **Hover (`textDocument/hover`)**: Rich Markdown documentation showing clone classification (Type-1 to Type-4), similarity percentages, and counterpart links.
- **Navigation (`textDocument/definition`, `textDocument/references`)**: Jump directly to counterpart duplicate sites.
- **Commands**: `cddm.rescanWorkspace`, `cddm.openStudio`.

---

## 2. Editor Setup Instructions

### A. Visual Studio Code / Cursor / Antigravity / Windsurf

The easiest approach is using the official **CDDM VS Code Extension** located in `editors/vscode/`:

1. Build or install the extension:
   ```bash
   cd editors/vscode
   bun install
   bun run build
   ```
2. In VS Code / Cursor, configure settings in `.vscode/settings.json` (optional):
   ```json
   {
     "cddm.executablePath": "cddm",
     "cddm.minTokens": 50,
     "cddm.enableHover": true,
     "cddm.enableCodeActions": true
   }
   ```

---

### B. Neovim (Native LSP or `nvim-lspconfig`)

Add the following configuration to your `init.lua` or `after/plugin/lsp.lua`:

#### Using Native Neovim 0.10+ `vim.lsp.start`

```lua
local function start_cddm_lsp()
  local root = vim.fs.root(0, { ".git", "Cargo.toml", "package.json" }) or vim.fn.getcwd()
  vim.lsp.start({
    name = "cddm",
    cmd = { "cddm", "lsp" },
    root_dir = root,
    filetypes = {
      "rust", "typescript", "typescriptreact", "javascript",
      "javascriptreact", "python", "go", "c", "cpp", "java", "cs"
    },
    settings = {
      min_tokens = 50,
    },
  })
end

vim.api.nvim_create_autocmd("FileType", {
  pattern = { "rust", "typescript", "typescriptreact", "javascript", "python", "go", "c", "cpp", "java", "cs" },
  callback = start_cddm_lsp,
})
```

---

### C. Zed Editor

Add CDDM as an external language server in `~/.config/zed/settings.json` or `.zed/settings.json`:

```json
{
  "lsp": {
    "cddm": {
      "binary": {
        "path": "cddm",
        "arguments": ["lsp"]
      }
    }
  },
  "languages": {
    "Rust": {
      "language_servers": ["rust-analyzer", "cddm"]
    },
    "TypeScript": {
      "language_servers": ["vtsls", "cddm"]
    },
    "Python": {
      "language_servers": ["pyright", "cddm"]
    },
    "Go": {
      "language_servers": ["gopls", "cddm"]
    }
  }
}
```

---

### D. Helix Editor

Add CDDM to `~/.config/helix/languages.toml`:

```toml
[language-server.cddm]
command = "cddm"
args = ["lsp"]

[[language]]
name = "rust"
language-servers = ["rust-analyzer", "cddm"]

[[language]]
name = "typescript"
language-servers = ["typescript-language-server", "cddm"]

[[language]]
name = "python"
language-servers = ["pyright", "cddm"]

[[language]]
name = "go"
language-servers = ["gopls", "cddm"]
```

---

### E. Sublime Text 4 (`LSP` Package)

Create `Packages/User/LSP-cddm.sublime-settings`:

```json
{
  "command": ["cddm", "lsp"],
  "selector": "source.rust | source.ts | source.tsx | source.js | source.jsx | source.python | source.go | source.c | source.c++ | source.java | source.cs",
  "enabled": true
}
```

---

### F. Emacs (`eglot` or `lsp-mode`)

#### Using Built-in `eglot` (Emacs 29+)

```elisp
(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs
               '(((rust-mode rust-ts-mode)
                  (typescript-mode typescript-ts-mode)
                  (python-mode python-ts-mode)
                  (go-mode go-ts-mode))
                 . ("cddm" "lsp"))))
```

---

## 3. CLI Command Reference

You can also run the LSP server directly from the command line:

```bash
# Run over standard I/O (Stdio transport)
cddm lsp

# Custom minimum token threshold
cddm lsp --min-tokens 60

# Run using the standalone binary
cddm-lsp
```
