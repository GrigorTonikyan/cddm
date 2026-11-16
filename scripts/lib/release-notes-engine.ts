/**
 * Rich Release Notes Engine for CDDM
 *
 * Generates and validates comprehensive, production-grade release notes
 * combining the version changelog, 4-pillar interface highlights (including MCP),
 * standalone binary download tables, and live-verified ecosystem package manager commands.
 * Strictly forbids empty bodies, "See CHANGELOG.md" stubs, or unverified 404 channels.
 */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { buildCargoRegistryIndexUrl, type CargoChannelAvailability } from "./cargo-registry";
import { buildGiteaNpmRegistryUrl, type NpmChannelAvailability } from "./npm-registry";
import { resolvePublisherConfig } from "./publisher-config";

export interface ReleaseNotesOptions {
  milestoneTitle?: string;
  rootDir?: string;
  includeMcpHighlight?: boolean;
  includeInstallGuides?: boolean;
  cargoChannels?: CargoChannelAvailability;
  npmChannels?: NpmChannelAvailability;
  wingetUpstream?: boolean;
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
export function buildMcpServerSection(npmChannels?: NpmChannelAvailability): string {
  const showNpx = !npmChannels || npmChannels.npmjs || npmChannels.gitea;
  const npxLine = showNpx
    ? "\n\n- **NPM & NPX Ready**: Run instantly without local installation via `npx -p cddm cddm-mcp`."
    : "";

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
\`\`\`${npxLine}`;
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

function buildCargoInstallSnippet(
  pubConfig: ReturnType<typeof resolvePublisherConfig>,
  cargoChannels?: CargoChannelAvailability,
): string {
  const giteaIndex = buildCargoRegistryIndexUrl(pubConfig.vcsHost, pubConfig.publisherId);
  const giteaBlock = `# Cargo (Gitea Private Cargo Registry)\nCARGO_REGISTRIES_GITEA_INDEX=${giteaIndex} cargo install cddm-cli cddm-mcp --registry gitea`;
  const cratesIoBlock = `# Cargo (crates.io)\ncargo install cddm-cli cddm-mcp`;

  if (!cargoChannels) {
    return `${giteaBlock}\n\n`;
  }

  const sections: string[] = [];
  if (cargoChannels.cratesIo) {
    sections.push(cratesIoBlock);
  }
  if (cargoChannels.gitea) {
    sections.push(giteaBlock);
  }
  return sections.length > 0 ? `${sections.join("\n\n")}\n\n` : "";
}

function buildNpmInstallSnippet(
  pubConfig: ReturnType<typeof resolvePublisherConfig>,
  npmChannels?: NpmChannelAvailability,
): string {
  const giteaRegistry = buildGiteaNpmRegistryUrl(pubConfig.vcsHost, pubConfig.publisherId);
  const giteaBlock = `# NPM / NPX (Gitea Private NPM Registry)\nnpm install -g cddm --registry ${giteaRegistry}\nnpx --registry ${giteaRegistry} -p cddm cddm-mcp`;
  const publicBlock = `# NPM / NPX (global CLI & MCP server)\nnpm install -g cddm\nnpx -p cddm cddm-mcp`;

  if (!npmChannels) {
    return `${giteaBlock}\n\n`;
  }

  const sections: string[] = [];
  if (npmChannels.npmjs) {
    sections.push(publicBlock);
  }
  if (npmChannels.gitea) {
    sections.push(giteaBlock);
  }
  return sections.length > 0 ? `${sections.join("\n\n")}\n\n` : "";
}

/**
 * Builds the download and installation guides section.
 */
export function buildInstallationSection(
  version: string,
  options: Pick<
    ReleaseNotesOptions,
    "rootDir" | "cargoChannels" | "npmChannels" | "wingetUpstream"
  > = {},
): string {
  const cleanVersion = version.replace(/^v/, "").trim();
  const tag = `v${cleanVersion}`;
  const pubConfig = resolvePublisherConfig(options.rootDir);
  const repoUrl = `https://${pubConfig.vcsHost}/${pubConfig.vcsRepo}`;
  const cargoSnippet = buildCargoInstallSnippet(pubConfig, options.cargoChannels);
  const npmSnippet = buildNpmInstallSnippet(pubConfig, options.npmChannels);
  const firstChar = pubConfig.publisherId.charAt(0).toLowerCase();
  const wingetManifestUrl = `https://${pubConfig.vcsHost}/${pubConfig.wingetRepo}/raw/branch/main/manifests/${firstChar}/${pubConfig.publisherId}/cddm/${cleanVersion}/${pubConfig.publisherId}.cddm.installer.yaml`;
  const wingetSnippet = options.wingetUpstream
    ? `# WinGet (Windows Package Manager)\nwinget install ${pubConfig.publisherId}.cddm`
    : `# WinGet (Windows Package Manager — Self-Hosted Manifest)\nwinget install --manifest ${wingetManifestUrl}`;

  return `### Installation & Distribution Ecosystem

#### Standalone Prebuilt Binaries
Download standalone binaries containing \`cddm\` (CLI), \`cddm-mcp\` (MCP Server), and \`cddm-lsp\` (Language Server):

- **Linux (x86_64)**: [\`cddm-${tag}-x86_64-unknown-linux-gnu.tar.gz\`](${repoUrl}/releases/download/${tag}/cddm-${tag}-x86_64-unknown-linux-gnu.tar.gz)
- **Windows (x86_64)**: [\`cddm-${tag}-x86_64-pc-windows-msvc.zip\`](${repoUrl}/releases/download/${tag}/cddm-${tag}-x86_64-pc-windows-msvc.zip)
- **VS Code Extension**: [\`cddm-${cleanVersion}.vsix\`](${repoUrl}/releases/download/${tag}/cddm-${cleanVersion}.vsix)
- **SHA-256 Checksums**: Verify all package downloads with \`SHA256SUMS.txt\`.

#### Ecosystem Package Managers
\`\`\`bash
${cargoSnippet}${npmSnippet}# Homebrew (macOS / Linux — Central Tap)
brew tap ${pubConfig.publisherId}/tap https://${pubConfig.vcsHost}/${pubConfig.homebrewTapRepo}
brew install ${pubConfig.publisherId}/tap/cddm

# Scoop (Windows — Central Bucket)
scoop bucket add ${pubConfig.publisherId} https://${pubConfig.vcsHost}/${pubConfig.scoopBucketRepo}
scoop install cddm

${wingetSnippet}

# Turnkey Install Scripts
curl -fsSL ${repoUrl}/raw/branch/main/packaging/install.sh | bash
irm ${repoUrl}/raw/branch/main/packaging/install.ps1 | iex
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
  const pubConfig = resolvePublisherConfig(rootDir);

  const changelog = extractChangelogSection(cleanVersion, rootDir);
  const mcpSection =
    options.includeMcpHighlight !== false
      ? `\n\n${buildMcpServerSection(options.npmChannels)}`
      : "";
  const paritySection = `\n\n${buildInterfaceParitySection()}`;
  const installSection =
    options.includeInstallGuides !== false
      ? `\n\n${buildInstallationSection(cleanVersion, {
          rootDir,
          cargoChannels: options.cargoChannels,
          npmChannels: options.npmChannels,
          wingetUpstream: options.wingetUpstream,
        })}`
      : "";

  return `## CDDM ${tag}${title}

${changelog}${mcpSection}${paritySection}${installSection}

---
*Authoritative Single Source of Truth: [Gitea Portal](https://${pubConfig.vcsHost}/${pubConfig.vcsRepo})*`;
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

  if (/(?<!-p cddm\s)npx\s+cddm-mcp\b/.test(trimmed)) {
    errors.push("Release notes must use 'npx -p cddm cddm-mcp' instead of bare 'npx cddm-mcp'.");
  }

  if (/^brew\s+install\s+cddm\s*$/m.test(trimmed)) {
    errors.push(
      "Release notes must use fully-qualified 'brew install <publisher>/tap/cddm' instead of bare 'brew install cddm'.",
    );
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
