# JetBrains IDE Integration Guide (IntelliJ IDEA, PyCharm, WebStorm, RustRover, GoLand)

CDDM seamlessly integrates into any JetBrains IDE via the standard **Language Server Protocol (LSP)** and **External Tools**.

---

## 1. Native LSP Integration (Recommended)

JetBrains IDEs (Ultimate Edition & 2024+ releases) include built-in Language Server Protocol support.

### Step-by-Step Configuration:

1. Open **Settings / Preferences** (`Ctrl + Alt + S` on Windows/Linux, `Cmd + ,` on macOS).
2. Navigate to **Languages & Frameworks > Language Servers**.
3. Click **+ (Add Server)**:
   - **Name**: `CDDM Duplication Meister`
   - **Server Type**: `Executable`
   - **Command**: `cddm lsp`
   - **File Patterns**: `*.rs, *.ts, *.tsx, *.js, *.jsx, *.py, *.go, *.java, *.cpp, *.c, *.cs, *.rb, *.php, *.swift, *.kt, *.scala, *.zig, *.lua, *.sql`
4. Apply and restart the IDE.

### LSP Features Available:

- **Code Diagnostics**: Real-time squiggly warnings on duplicated code blocks.
- **Code Actions (Quick Fixes)**: `Alt + Enter` on any clone to inspect refactoring recommendations and extract shared helper functions.
- **Hover Information**: In-editor clone similarity percentages, token metrics, and author blame attribution.

---

## 2. External Tools & Keybindings

You can bind CDDM commands to quick IDE keyboard shortcuts.

### Configure External Tool:

1. Navigate to **Settings > Tools > External Tools**.
2. Click **+**:
   - **Name**: `CDDM Scan Current File`
   - **Program**: `cddm`
   - **Arguments**: `scan $FileDir$ --min-tokens 40`
   - **Working directory**: `$ProjectFileDir$`
3. Add a second tool:
   - **Name**: `CDDM Open WebUI Studio`
   - **Program**: `cddm`
   - **Arguments**: `serve --open`
   - **Working directory**: `$ProjectFileDir$`

---

## 3. Git Pre-Commit Hook Integration

Enforce duplication quality gates natively in JetBrains Git tool window:

```bash
cddm hook install --hook-type pre-commit --fail-threshold 15.0
```

When committing via JetBrains IDE (`Ctrl + K`), CDDM will automatically prevent duplicate code commits.
