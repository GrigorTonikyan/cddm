/**
 * Gitea & Git Wiki Content Synchronizer for CDDM.
 * Dynamically binds wiki pages to canonical Markdown docs in docs/ and workspace root.
 */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

export interface WikiPageDef {
  title: string;
  content: string;
}

function readDocOrFallback(relPath: string, fallbackContent: string): string {
  const fullPath = join(process.cwd(), relPath);
  if (existsSync(fullPath)) {
    return readFileSync(fullPath, "utf-8");
  }
  return fallbackContent;
}

export function getDynamicallyAssembledWikiPages(): WikiPageDef[] {
  const homeContent = `# CDDM — Code De-Duplication Meister Wiki

Welcome to the official documentation and technical wiki for **CDDM** (*Code De-Duplication Meister*).

CDDM is an open-source, ultra-fast, multi-threaded polyglot code clone detection, AST refactoring, and architectural governance engine built natively in Rust.

---

## The 4 Interaction Pillars

CDDM enforces strict **100% Feature Parity** across all four primary interaction surfaces:

\`\`\`text
+-----------------------------------------------------------------------------+
|                          CDDM Core AST & SIMD Engine                         |
+------┬----------------------┬----------------------┬-----------------┬------+
       |                      |                      |                 |
       v                      v                      v                 v
+---------------+     +---------------+      +---------------+ +---------------+
| 1. CLI Engine |     | 2. WebUI      |      | 3. MCP Server | | 4. TUI Studio |
| Terminal Sub- |     | React 19 FSD  |      | JSON-RPC 2.0  | | Ratatui Term  |
| commands, CI  |     | Monaco Diffs  |      | AI Coding     | | Interactive   |
| & Dogfooding  |     | SSE Live Sync |      | Agent Tools   | | Dashboard     |
+---------------+     +---------------+      +---------------+ +---------------+
\`\`\`

---

## Wiki Table of Contents

- [[Getting-Started|Getting Started]]: Installation, package managers, and quick start.
- [[CLI-Reference|CLI Command Reference]]: Complete manual for all 22 CLI subcommands.
- [[WebUI-Studio|WebUI Studio]]: Feature-Sliced React 19 Studio, Monaco diffs, and 19 modals.
- [[MCP-Server-Protocol|MCP Server Protocol]]: 30 Model Context Protocol tools & resources for AI agents.
- [[TUI-Studio|TUI Terminal Studio]]: 12-tab Ratatui terminal dashboard.
- [[AST-Engine-and-Deduplication|System Architecture & AST]]: System pipeline phases, hashing, and crate breakdown.
- [[4-Pillar-Feature-Parity|Feature Parity Matrix]]: 21 core capabilities across all 4 interaction pillars.
- [[CI-CD-and-Releases|CI/CD, Releases & LSP]]: Language Server Protocol 3.17, Gitea Actions, and VS Code extension.
`;

  return [
    {
      title: "Home",
      content: homeContent,
    },
    {
      title: "Getting-Started",
      content: readDocOrFallback("README.md", "# Getting Started\n\nSee README.md."),
    },
    {
      title: "CLI-Reference",
      content: readDocOrFallback("docs/CLI.md", "# CLI Reference\n\nSee docs/CLI.md."),
    },
    {
      title: "WebUI-Studio",
      content: readDocOrFallback("docs/WEBUI.md", "# WebUI Studio\n\nSee docs/WEBUI.md."),
    },
    {
      title: "MCP-Server-Protocol",
      content: readDocOrFallback("docs/MCP.md", "# MCP Server Protocol\n\nSee docs/MCP.md."),
    },
    {
      title: "TUI-Studio",
      content: readDocOrFallback("docs/TUI.md", "# TUI Studio\n\nSee docs/TUI.md."),
    },
    {
      title: "AST-Engine-and-Deduplication",
      content: readDocOrFallback(
        "docs/ARCHITECTURE.md",
        "# Architecture\n\nSee docs/ARCHITECTURE.md.",
      ),
    },
    {
      title: "4-Pillar-Feature-Parity",
      content: readDocOrFallback(
        "docs/FEATURE_PARITY.md",
        "# Feature Parity\n\nSee docs/FEATURE_PARITY.md.",
      ),
    },
    {
      title: "CI-CD-and-Releases",
      content: readDocOrFallback(
        "docs/LSP_SETUP.md",
        "# LSP & CI/CD Setup\n\nSee docs/LSP_SETUP.md.",
      ),
    },
  ];
}

export const WIKI_PAGES: WikiPageDef[] = getDynamicallyAssembledWikiPages();
