/**
 * Interface Documentation Generator for CDDM.
 * Discovers and synchronizes:
 * 1. CLI Commands & Arguments (crates/cddm-cli) -> docs/CLI.md
 * 2. MCP Tools, Resources & Prompts (crates/cddm-mcp) -> docs/MCP.md
 * 3. WebUI Modals & REST/SSE Endpoints (webui/ & serve/) -> docs/WEBUI.md
 * 4. TUI Tabs & Keybindings (crates/cddm-cli/src/tui) -> docs/TUI.md
 */

export interface CliCommandMetadata {
  name: string;
  alias?: string;
  summary: string;
  usage: string;
  keyFlags: string[];
}

export interface McpToolMetadata {
  name: string;
  description: string;
  parameters: string[];
}

export interface McpResourceMetadata {
  uri: string;
  name: string;
  description: string;
  mimeType: string;
}

export interface TuiTabMetadata {
  index: number;
  title: string;
  key: string;
  description: string;
}

export interface WebUiEndpointMetadata {
  method: string;
  path: string;
  summary: string;
}

export interface WebUiModalMetadata {
  component: string;
  description: string;
}

export const CLI_COMMANDS_CATALOG: CliCommandMetadata[] = [
  {
    name: "scan",
    summary: "Scan target directory for code duplication, clone pairs, and DRY health score",
    usage: "cddm scan [OPTIONS] [DIRECTORY]",
    keyFlags: [
      "--min-tokens",
      "--format",
      "--fail-threshold",
      "--languages",
      "--ignore",
      "--git-blame",
      "--cross-language",
      "--rules",
      "--enforce-policies",
      "--threads",
    ],
  },
  {
    name: "dead-code",
    alias: "dead",
    summary: "Detect unreferenced functions, unreachable code blocks, and dead duplicate clones",
    usage: "cddm dead-code [OPTIONS] [DIRECTORY]",
    keyFlags: ["--min-tokens", "--format", "--coverage", "--static-only", "--languages"],
  },
  {
    name: "diff",
    summary: "Differential clone scan comparing working tree against Git base revisions",
    usage: "cddm diff [OPTIONS] <BASE_REF> [TARGET_REF]",
    keyFlags: ["--matrix", "--cross-language", "--fail-threshold", "--git-blame", "--rules"],
  },
  {
    name: "semantic",
    summary: "Analyze cross-language semantic clones and dense neural algorithmic equivalences",
    usage: "cddm semantic [OPTIONS] [DIRECTORY]",
    keyFlags: ["--threshold", "--neural", "--neural-threshold", "--min-tokens", "--threads"],
  },
  {
    name: "refactor",
    summary: "Synthesize deduplication refactoring patches, AST rewrites, and AI prompts",
    usage: "cddm refactor [OPTIONS]",
    keyFlags: [
      "--pair",
      "--cluster",
      "--ast",
      "--output",
      "--prompt",
      "--verify",
      "--test-cmd",
      "--apply-branch",
    ],
  },
  {
    name: "extract",
    summary: "Extract duplicate clone clusters into standalone shared packages or crates",
    usage: "cddm extract [OPTIONS]",
    keyFlags: ["--cluster", "--pkg-name", "--pkg-type", "--target-dir", "--dry-run"],
  },
  {
    name: "serve",
    summary: "Launch the embedded React 19 Studio WebUI dashboard in browser",
    usage: "cddm serve [OPTIONS]",
    keyFlags: ["--port", "--open"],
  },
  {
    name: "watch",
    summary: "Continuous file watcher with real-time incremental rescanning on save",
    usage: "cddm watch [OPTIONS] [DIRECTORY]",
    keyFlags: ["--min-tokens", "--debounce-ms", "--serve", "--open", "--fail-threshold"],
  },
  {
    name: "lsp",
    summary: "Start Language Server Protocol (LSP 3.17) daemon for real-time IDE diagnostics",
    usage: "cddm lsp [OPTIONS] [DIRECTORY]",
    keyFlags: ["--min-tokens"],
  },
  {
    name: "trend",
    summary: "Analyze historical duplication trajectories and DRY score across Git commits",
    usage: "cddm trend [OPTIONS] [DIRECTORY]",
    keyFlags: ["--max-samples", "--min-tokens", "--format"],
  },
  {
    name: "hook",
    summary: "Manage automated Git pre-commit and pre-push duplication gate enforcement hooks",
    usage: "cddm hook <install|uninstall|status> [OPTIONS]",
    keyFlags: ["--type", "--fail-threshold", "--min-tokens"],
  },
  {
    name: "ignore",
    summary: "Manage .cddmignore suppression rules and inspect file/line suppression status",
    usage: "cddm ignore <init|check> [OPTIONS]",
    keyFlags: ["--force", "--line", "--ignore-tests", "--ignore-mocks", "--ignore-generated"],
  },
  {
    name: "rules",
    summary: "Manage architectural boundary policies and zero-duplication zones (.cddmrules.toml)",
    usage: "cddm rules <init|check> [OPTIONS]",
    keyFlags: ["--rules", "--enforce-policies", "--format", "--force"],
  },
  {
    name: "init",
    summary: "Generate turnkey CI/CD workflows for Gitea Actions, GitHub, GitLab, and Azure",
    usage: "cddm init <gitea|github|gitlab|azure> [OPTIONS]",
    keyFlags: ["--fail-threshold", "--min-tokens", "--output", "--write"],
  },
  {
    name: "comment",
    summary: "Generate formatted Markdown DRY health tables for Pull / Merge Request comments",
    usage: "cddm comment [OPTIONS] [DIRECTORY]",
    keyFlags: ["--platform", "--fail-threshold", "--min-tokens", "--output"],
  },
  {
    name: "heal",
    summary: "Autonomous AI Code Surgeon refactoring with closed-loop test repair loop",
    usage: "cddm heal [OPTIONS]",
    keyFlags: [
      "--cluster",
      "--pair",
      "--provider",
      "--model",
      "--api-key",
      "--verify",
      "--test-cmd",
      "--branch",
      "--max-iterations",
    ],
  },
  {
    name: "cache",
    summary: "Manage persistent fingerprint cache and export/import portable .cddmpack bundles",
    usage: "cddm cache <export|import> [OPTIONS]",
    keyFlags: ["--cache-dir", "--output", "--pack-file", "--target-dir"],
  },
  {
    name: "monorepo",
    summary: "Discover and scan multi-package monorepos for cross-package duplicates",
    usage: "cddm monorepo [OPTIONS] [DIRECTORY]",
    keyFlags: ["--min-tokens"],
  },
  {
    name: "tui",
    summary: "Launch the interactive 12-tab Terminal UI (TUI) Studio dashboard",
    usage: "cddm tui [OPTIONS] [DIRECTORY]",
    keyFlags: ["--watch", "--fail-threshold", "--min-tokens", "--languages", "--ignore"],
  },
  {
    name: "overlap",
    summary: "Detect reimplemented standard library and ecosystem package algorithms",
    usage: "cddm overlap [OPTIONS] [DIRECTORY]",
    keyFlags: ["--threshold", "--format"],
  },
  {
    name: "hub",
    summary: "Manage and scan multi-repository Organization Federation Hub (.cddmhub.toml)",
    usage: "cddm hub <init|scan|extract> [OPTIONS]",
    keyFlags: ["--config", "--targets", "--cluster", "--pkg-name", "--pkg-type", "--target-dir"],
  },
  {
    name: "coverage",
    summary: "Correlate runtime execution coverage reports with duplicate code clones",
    usage: "cddm coverage [OPTIONS]",
    keyFlags: ["--report", "--dead-code-only", "--min-hits", "--risk-threshold", "--format"],
  },
];

export const TUI_TABS_CATALOG: TuiTabMetadata[] = [
  {
    index: 1,
    title: "Overview",
    key: "1 or s",
    description: "Workspace summary metrics, DRY health gauge, language breakdown & scan trigger",
  },
  {
    index: 2,
    title: "Clones & Diffs",
    key: "2 or c / d",
    description: "Clone pairs, N-way cluster trees, and split/unified Monaco-style diff viewer",
  },
  {
    index: 3,
    title: "Semantic",
    key: "3",
    description: "Cross-language Weisfeiler-Lehman AST graph isomorphisms and neural embeddings",
  },
  {
    index: 4,
    title: "Refactor",
    key: "4 or r / a / p",
    description: "AST-native refactoring sandbox, AI Prompt generator, and AI Code Surgeon",
  },
  {
    index: 5,
    title: "Extract",
    key: "5 or e",
    description:
      "Standalone shared crate/package synthesizer with multi-ecosystem manifest updates",
  },
  {
    index: 6,
    title: "Policies",
    key: "6",
    description:
      "Architectural rules checker (.cddmrules.toml) and .cddmignore suppression manager",
  },
  {
    index: 7,
    title: "Timeline",
    key: "7",
    description: "Git commit history time-series trajectories and multi-branch clone drift matrix",
  },
  {
    index: 8,
    title: "CI/CD & Hooks",
    key: "8",
    description: "Turnkey workflow generator (Gitea/GitHub/GitLab/Azure) and Git pre-commit hooks",
  },
  {
    index: 9,
    title: "Overlap",
    key: "9",
    description: "Ecosystem library duplication detector for reimplemented utility functions",
  },
  {
    index: 10,
    title: "Hub",
    key: "0",
    description: "Multi-repository Organization Federation Hub viewer and cross-repo extractor",
  },
  {
    index: 11,
    title: "Coverage",
    key: "C or v",
    description: "Runtime execution trace correlation, hot-path analysis, and risk scoring",
  },
  {
    index: 12,
    title: "Dead Code",
    key: "D",
    description: "Unreferenced functions, unreachable code blocks, and 0-hit duplicate clones",
  },
];

export function generateCliMarkdownTable(): string {
  let table = "| Command | Usage | Description | Key Options |\n";
  table += "| :--- | :--- | :--- | :--- |\n";
  for (const cmd of CLI_COMMANDS_CATALOG) {
    const aliasStr = cmd.alias ? ` (alias \`${cmd.alias}\`)` : "";
    const flagsStr = cmd.keyFlags.map((f) => `\`${f}\``).join(", ");
    table += `| **\`cddm ${cmd.name}\`**${aliasStr} | \`${cmd.usage}\` | ${cmd.summary} | ${flagsStr} |\n`;
  }
  return table;
}

export function generateTuiMarkdownTable(): string {
  let table = "| Tab # | Tab Title | Hotkey | Description |\n";
  table += "| :---: | :--- | :---: | :--- |\n";
  for (const tab of TUI_TABS_CATALOG) {
    table += `| **${tab.index}** | **${tab.title}** | \`${tab.key}\` | ${tab.description} |\n`;
  }
  return table;
}

export function replaceAutogeneratedSection(
  content: string,
  markerKey: string,
  newTable: string,
): string {
  const startMarker = `<!-- AUTOGEN:${markerKey}:START -->`;
  const endMarker = `<!-- AUTOGEN:${markerKey}:END -->`;

  const startIndex = content.indexOf(startMarker);
  const endIndex = content.indexOf(endMarker);

  if (startIndex === -1 || endIndex === -1) {
    return content;
  }

  return (
    content.slice(0, startIndex + startMarker.length) +
    "\n\n" +
    newTable.trim() +
    "\n\n" +
    content.slice(endIndex)
  );
}

export function normalizeMarkdown(content: string): string {
  return content
    .split("\n")
    .map((line) => {
      const trimmed = line.trim();
      if (trimmed.startsWith("|")) {
        if (/^\|(?:\s*:?-+:?\s*\|)+$/.test(trimmed)) {
          const cols = trimmed.split("|").filter(Boolean).length;
          return `|${Array(cols).fill("---").join("|")}|`;
        }
        return trimmed
          .split("|")
          .map((cell) => cell.trim())
          .join("|");
      }
      return trimmed;
    })
    .filter((line) => line.length > 0)
    .join("\n");
}
