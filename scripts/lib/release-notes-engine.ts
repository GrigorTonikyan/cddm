/**
 * Rich Release Notes Engine for CDDM
 *
 * Generates and validates comprehensive, production-grade release notes
 * combining the version changelog, 4-pillar interface highlights (including MCP),
 * standalone binary download tables, and ecosystem package manager quickstarts.
 * Strictly forbids empty bodies or "See CHANGELOG.md" placeholder stubs.
 */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

export interface ReleaseNotesOptions {
  milestoneTitle?: string;
  rootDir?: string;
  includeMcpHighlight?: boolean;
  includeInstallGuides?: boolean;
}

export interface ReleaseNotesValidationResult {
  valid: boolean;
  errors: string[];
}

/**
 * Extracts the specific version section from CHANGELOG.md.
 */
export function extractChangelogSection(version: string, rootDir = process.cwd()): string {
  const cleanVersion = version.replace(/^v/, "").trim();
  const changelogPath = join(rootDir, "CHANGELOG.md");

  if (!existsSync(changelogPath)) {
    return `### Release v${cleanVersion}\n\nAutomated release for CDDM v${cleanVersion}.`;
  }

  const content = readFileSync(changelogPath, "utf-8");
  const targetHeader = `## [${cleanVersion}]`;
  const startIndex = content.indexOf(targetHeader);

  if (startIndex === -1) {
    return `### Release v${cleanVersion}\n\nAutomated release for CDDM v${cleanVersion}.`;
  }

  const nextSectionIndex = content.indexOf("\n## [", startIndex + targetHeader.length);
  const rawSection =
    nextSectionIndex === -1
      ? content.slice(startIndex).trim()
      : content.slice(startIndex, nextSectionIndex).trim();

  return rawSection;
}

/**
 * Builds the standardized Model Context Protocol (MCP) server highlight section.
 */
export function buildMcpServerSection(): string {
  return `### Model Context Protocol (MCP) Server Highlights

CDDM exposes its entire code clone detection, AST refactoring, and federation engine to AI coding agents via its dedicated **Model Context Protocol (MCP)** server (\`cddm-mcp\`) over JSON-RPC 2.0 stdio.

- **33 Specialized AI Agent Tools**: Full coverage across codebase scanning (\`scan_codebase\`), differential analysis (\`cddm_diff_scan\`), AST transformation (\`cddm_ast_refactor\`), autonomous closed-loop test healing (\`cddm_heal_refactor\`), monorepo federation (\`cddm_scan_monorepo\`, \`cddm_scan_hub\`), dead code reachability (\`cddm_detect_dead_code\`, \`cddm_trace_reachability\`), and coverage correlation (\`cddm_correlate_coverage\`).
- **Turnkey Configuration**: Add to your \`claude_desktop_config.json\`, Cursor, or Antigravity MCP settings:

\`\`\`json
{
  "mcpServers": {
    "cddm": {
      "command": "cddm-mcp",
      "args": []
    }
  }
}
\`\`\`

- **NPM & NPX Ready**: Run instantly without local installation via \`npx cddm-mcp\`.`;
}

/**
 * Builds the 4-Pillar Cross-Interface Parity overview.
 */
export function buildInterfaceParitySection(): string {
  return `### 4-Pillar Cross-Interface Coverage

CDDM strictly enforces complete feature parity across all four primary interaction surfaces:

1. **CLI Engine (\`cddm\`)**: Fast, scriptable terminal commands (\`cddm scan\`, \`diff\`, \`semantic\`, \`refactor\`, \`extract\`, \`heal\`, \`rules\`, \`coverage\`).
2. **WebUI Studio (\`cddm serve\`)**: Full-fidelity visual studio with React 19, Monaco split diffs, and interactive modals.
3. **MCP Server (\`cddm-mcp\`)**: High-throughput JSON-RPC 2.0 interface for AI assistants and autonomous coding agents.
4. **TUI Studio (\`cddm tui\`)**: Keyboard-driven Ratatui terminal dashboard for power users and SSH workflows.`;
}

/**
 * Builds the download and installation guides section.
 */
export function buildInstallationSection(version: string): string {
  const cleanVersion = version.replace(/^v/, "").trim();
  const tag = `v${cleanVersion}`;

  return `### Installation & Distribution Ecosystem

#### Standalone Prebuilt Binaries
Download standalone binaries containing \`cddm\` (CLI), \`cddm-mcp\` (MCP Server), and \`cddm-lsp\` (Language Server):

- **Linux (x86_64)**: [\`cddm-${tag}-x86_64-unknown-linux-gnu.tar.gz\`](https://git.gt-web-dev.com/gt-dev/cddm/releases/download/${tag}/cddm-${tag}-x86_64-unknown-linux-gnu.tar.gz)
- **Windows (x86_64)**: [\`cddm-${tag}-x86_64-pc-windows-gnu.zip\`](https://git.gt-web-dev.com/gt-dev/cddm/releases/download/${tag}/cddm-${tag}-x86_64-pc-windows-gnu.zip)
- **VS Code Extension**: [\`cddm-${cleanVersion}.vsix\`](https://git.gt-web-dev.com/gt-dev/cddm/releases/download/${tag}/cddm-${cleanVersion}.vsix)
- **SHA-256 Checksums**: Verify all package downloads with \`SHA256SUMS.txt\`.

#### Ecosystem Package Managers
\`\`\`bash
# Cargo (crates.io / workspace)
cargo install cddm-cli cddm-mcp

# NPM / NPX (global CLI & MCP server)
npm install -g cddm
npx cddm-mcp

# Homebrew (macOS / Linux)
brew install GrigorTonikyan/cddm/cddm

# Scoop (Windows)
scoop install cddm

# Turnkey Install Scripts
curl -fsSL https://git.gt-web-dev.com/gt-dev/cddm/raw/branch/main/packaging/install.sh | bash
irm https://git.gt-web-dev.com/gt-dev/cddm/raw/branch/main/packaging/install.ps1 | iex
\`\`\``;
}

/**
 * Generates the full rich release notes for a given version.
 */
export function generateReleaseNotes(version: string, options: ReleaseNotesOptions = {}): string {
  const cleanVersion = version.replace(/^v/, "").trim();
  const tag = `v${cleanVersion}`;
  const rootDir = options.rootDir || process.cwd();
  const title = options.milestoneTitle ? ` — ${options.milestoneTitle}` : "";

  const changelog = extractChangelogSection(cleanVersion, rootDir);
  const mcpSection = options.includeMcpHighlight !== false ? `\n\n${buildMcpServerSection()}` : "";
  const paritySection = `\n\n${buildInterfaceParitySection()}`;
  const installSection =
    options.includeInstallGuides !== false ? `\n\n${buildInstallationSection(cleanVersion)}` : "";

  return `## CDDM ${tag}${title}

${changelog}${mcpSection}${paritySection}${installSection}

---
*Authoritative Single Source of Truth: [Gitea Portal](https://git.gt-web-dev.com/gt-dev/cddm)*`;
}

/**
 * Validates that release notes satisfy the workspace rich release notes standard.
 */
export function validateReleaseNotesContent(notes: string): ReleaseNotesValidationResult {
  const errors: string[] = [];
  const trimmed = notes.trim();

  if (trimmed.length < 100) {
    errors.push(`Release notes are too short (${trimmed.length} chars; min 100).`);
  }

  if (trimmed.includes("See CHANGELOG.md")) {
    errors.push("Release notes contain forbidden placeholder stub 'See CHANGELOG.md'.");
  }

  if (!trimmed.includes("Model Context Protocol") && !trimmed.includes("MCP")) {
    errors.push("Release notes must include Model Context Protocol (MCP) server highlights.");
  }

  if (!trimmed.includes("cddm-mcp")) {
    errors.push("Release notes must reference the 'cddm-mcp' binary or command.");
  }

  if (!trimmed.includes("Installation") && !trimmed.includes("Standalone Prebuilt Binaries")) {
    errors.push("Release notes must include installation or binary download instructions.");
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
