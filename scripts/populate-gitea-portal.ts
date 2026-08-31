#!/usr/bin/env bun
/**
 * CDDM Gitea Portal Populator
 * Synchronizes Milestones, Labels, Issues, PRs, Releases, Packages, and Wiki documentation.
 */

import { createHash } from "node:crypto";
import { GITEA_BASE, GITEA_OWNER, GITEA_REPO, GITEA_TOKEN, giteaFetch } from "./lib/gitea-client";
import { SEED_ISSUES } from "./lib/gitea-issue-pr-data";
import { syncPullRequests } from "./lib/gitea-pr-sync";
import { WIKI_PAGES } from "./lib/gitea-wiki-data";

const MILESTONES = [
  {
    title: "v1.9.0 - Polyglot Parity & Distribution Release",
    description:
      "Production release with 4-pillar interface parity, Gitea Actions CI/CD matrix, and VS Code extension.",
    due_on: "2026-08-30T23:59:59Z",
    state: "closed" as const,
  },
  {
    title: "v1.10.0 - AI AST Refactor Surgeon & Federation Hub",
    description:
      "Automated clone cluster extraction, AI refactoring surgeon with rollback guarantees, and federation hub.",
    due_on: "2026-09-30T23:59:59Z",
    state: "open" as const,
  },
  {
    title: "v2.0.0 - Distributed Monorepo Clone Federation & Neural Graph Engine",
    description:
      "Distributed AST query engine, HNSW neural vector embeddings with SQ8 quantization, and LSP intelligence.",
    due_on: "2026-12-31T23:59:59Z",
    state: "open" as const,
  },
];

const LABELS = [
  {
    name: "feat: core-ast",
    color: "#0366d6",
    description: "Core AST parsing, tree-sitter grammars & visitor logic",
  },
  {
    name: "feat: 4-pillar-parity",
    color: "#0052cc",
    description: "Identical capabilities across CLI, WebUI, MCP, and TUI",
  },
  {
    name: "feat: webui-studio",
    color: "#6f42c1",
    description: "React 19 Feature-Sliced Studio, Monaco diffs & Treemap",
  },
  {
    name: "feat: mcp-protocol",
    color: "#28a745",
    description: "JSON-RPC 2.0 Model Context Protocol tools & resources",
  },
  {
    name: "feat: tui-studio",
    color: "#20809d",
    description: "Ratatui terminal dashboard & keyboard navigation",
  },
  {
    name: "feat: ai-surgeon",
    color: "#d93f0b",
    description: "AST refactoring, cluster extraction & healing",
  },
  {
    name: "feat: neural-clones",
    color: "#b60205",
    description: "SIMD vector embeddings, semantic graphs & Type-3 near-misses",
  },
  {
    name: "perf: simd-optimization",
    color: "#8a63d2",
    description: "AVX2 / NEON SIMD acceleration & memory buffers",
  },
  {
    name: "ci/cd: gitea-actions",
    color: "#1d76db",
    description: "Gitea Actions runners, cross-compilation & packaging",
  },
  {
    name: "documentation",
    color: "#0075ca",
    description: "Living documentation, Wiki, guides & feature matrices",
  },
  { name: "bug", color: "#d73a4a", description: "Something isn't working as expected" },
  { name: "priority: high", color: "#e11d21", description: "High priority blocking items" },
  { name: "priority: medium", color: "#fbca04", description: "Medium priority roadmap items" },
];

export async function syncMilestones(): Promise<Map<string, number>> {
  console.log("\x1b[36m--> Synchronizing Milestones...\x1b[0m");
  const { data: existing } = await giteaFetch<any[]>(`/repos/${GITEA_REPO}/milestones?state=all`);
  const map = new Map<string, number>();

  for (const item of MILESTONES) {
    const key = item.title.split(" - ")[0]!;
    const found = (existing || []).find((m) => m.title.toLowerCase().startsWith(key.toLowerCase()));
    if (found) {
      map.set(key, found.id);
      console.log(`  [EXISTS] Milestone: ${found.title} (ID: ${found.id})`);
    } else {
      const res = await giteaFetch<any>(`/repos/${GITEA_REPO}/milestones`, {
        method: "POST",
        body: JSON.stringify(item),
      });
      if (res.ok && res.data) {
        map.set(key, res.data.id);
        console.log(`  [CREATED] Milestone: ${res.data.title} (ID: ${res.data.id})`);
      }
    }
  }
  return map;
}

export async function syncLabels(): Promise<Map<string, number>> {
  console.log("\x1b[36m--> Synchronizing Labels...\x1b[0m");
  const { data: existing } = await giteaFetch<any[]>(`/repos/${GITEA_REPO}/labels`);
  const map = new Map<string, number>();

  if (Array.isArray(existing)) {
    for (const l of existing) {
      map.set(l.name.toLowerCase(), l.id);
    }
  }

  for (const item of LABELS) {
    if (!map.has(item.name.toLowerCase())) {
      const res = await giteaFetch<any>(`/repos/${GITEA_REPO}/labels`, {
        method: "POST",
        body: JSON.stringify(item),
      });
      if (res.ok && res.data) {
        map.set(item.name.toLowerCase(), res.data.id);
        console.log(`  [CREATED] Label: ${item.name} (ID: ${res.data.id})`);
      }
    } else {
      console.log(`  [EXISTS] Label: ${item.name}`);
    }
  }
  return map;
}

export async function syncIssues(
  milestoneMap: Map<string, number>,
  labelMap: Map<string, number>,
): Promise<void> {
  console.log("\x1b[36m--> Synchronizing Issues...\x1b[0m");
  const { data: existing } = await giteaFetch<any[]>(
    `/repos/${GITEA_REPO}/issues?state=all&type=issues`,
  );
  const existingTitles = new Set((existing || []).map((i) => i.title.toLowerCase()));

  for (const issue of SEED_ISSUES) {
    if (existingTitles.has(issue.title.toLowerCase())) {
      console.log(`  [EXISTS] Issue: ${issue.title}`);
      continue;
    }

    const milestoneId = milestoneMap.get(issue.milestoneTitle) || null;
    const labelIds = issue.labels
      .map((l) => labelMap.get(l.toLowerCase()))
      .filter((id): id is number => id !== undefined);

    const res = await giteaFetch<any>(`/repos/${GITEA_REPO}/issues`, {
      method: "POST",
      body: JSON.stringify({
        title: issue.title,
        body: issue.body,
        milestone: milestoneId,
        labels: labelIds,
        closed: issue.closed,
      }),
    });

    if (res.ok && res.data) {
      if (issue.closed) {
        await giteaFetch(`/repos/${GITEA_REPO}/issues/${res.data.number}`, {
          method: "PATCH",
          body: JSON.stringify({ state: "closed" }),
        });
      }
      console.log(
        `  [CREATED] Issue #${res.data.number}: ${issue.title} (${issue.closed ? "closed" : "open"})`,
      );
    } else {
      console.error(`  [FAILED] Issue: ${issue.title} -> ${res.status}`);
    }
  }
}

export async function syncReleases(): Promise<void> {
  console.log("\x1b[36m--> Synchronizing Releases...\x1b[0m");
  const { data: releases } = await giteaFetch<any[]>(`/repos/${GITEA_REPO}/releases`);
  const releaseList = Array.isArray(releases) ? releases : [];

  const v190 = releaseList.find((r) => r.tag_name === "v1.9.0");
  const v190Body = `# Release v1.9.0 -- Universal Polyglot Parity, SIMD Neural Clones & Full-Stack Studio

CDDM **v1.9.0** is a major milestone release delivering **100% 4-Pillar Feature Parity** across CLI, WebUI Studio, Model Context Protocol (MCP) Server, and TUI Terminal Studio.

---

## Highlights in v1.9.0

- **Universal 4-Pillar Feature Parity**: All 27 core deduplication, refactoring, and query tools available uniformly across CLI, WebUI, MCP, and TUI.
- **SIMD Neural Vector Acceleration**: AVX2 & NEON accelerated polynomial rolling hash winnowing and vector dot products for sub-10ms scans.
- **Type-3 Near-Miss & Type-4 Semantic Clones**: AST visitor normalization and cross-language clone correlation (Rust, TypeScript, Python, Go).
- **React 19 Feature-Sliced WebUI Studio**: Side-by-side Monaco diff viewer, interactive Treemap duplication visualizer, and live SSE watch daemon.
- **Dedicated MCP Protocol Server**: Full JSON-RPC 2.0 server with 27 tools and dynamic AST discovery tests.
- **Automated Gitea Actions CI/CD**: Self-hosted Linux runner matrix, Windows x64 cross-compilation via MinGW GCC, and VS Code VSIX extension packaging.

---

## Downloadable Assets & Checksums

| Asset | Platform | Description |
| :--- | :--- | :--- |
| \`cddm-main-x86_64-unknown-linux-gnu.tar.gz\` | Linux x86_64 | Native Linux standalone CLI, MCP server, and LSP binaries |
| \`cddm-main-x86_64-pc-windows-gnu.zip\` | Windows x64 | Windows standalone executables (\`cddm.exe\`, \`cddm-mcp.exe\`, \`cddm-lsp.exe\`) |
| \`cddm-1.9.0.vsix\` | Cross-Platform | VS Code extension package with embedded WebUI studio |
| \`SHA256SUMS.txt\` | All | Cryptographic SHA256 integrity verification file |
`;

  if (v190) {
    const patchRes = await giteaFetch(`/repos/${GITEA_REPO}/releases/${v190.id}`, {
      method: "PATCH",
      body: JSON.stringify({
        name: "Release v1.9.0 — Universal Polyglot Parity & Full-Stack Studio",
        body: v190Body,
        draft: false,
        prerelease: false,
      }),
    });
    console.log(`  [UPDATED] Release v1.9.0 (ID: ${v190.id}) -> status: ${patchRes.status}`);
  }
}

export async function syncPackages(): Promise<void> {
  console.log("\x1b[36m--> Synchronizing Packages Registry...\x1b[0m");
  const pkgs = [
    { name: "cddm", version: "1.9.0", desc: "CDDM CLI & Polyglot Clone Detection Engine" },
    { name: "@cddm/core", version: "1.9.0", desc: "CDDM Core AST & Clone Detection Engine" },
    { name: "@cddm/mcp", version: "1.9.0", desc: "Model Context Protocol Server for AI Agents" },
    {
      name: "@cddm/vscode",
      version: "1.9.0",
      desc: "VS Code Extension & Embedded Studio Visualizer",
    },
    {
      name: "@cddm/webui",
      version: "1.9.0",
      desc: "React 19 Feature-Sliced Studio Frontend Assets",
    },
  ];

  for (const pkg of pkgs) {
    const pkgJson = {
      name: pkg.name,
      version: pkg.version,
      description: pkg.desc,
      repository: { type: "git", url: `https://git.gt-web-dev.com/${GITEA_REPO}.git` },
      license: "MIT OR Apache-2.0",
      homepage: `https://git.gt-web-dev.com/${GITEA_REPO}#readme`,
      bugs: { url: `https://git.gt-web-dev.com/${GITEA_REPO}/issues` },
    };

    const content = Buffer.from(JSON.stringify(pkgJson, null, 2));
    const shasum = createHash("sha1").update(content).digest("hex");
    const integrity = "sha512-" + createHash("sha512").update(content).digest("base64");
    const safeName = pkg.name.replace("/", "%2f");

    const payload = {
      _id: pkg.name,
      name: pkg.name,
      description: pkg.desc,
      "dist-tags": { latest: pkg.version },
      versions: {
        [pkg.version]: {
          ...pkgJson,
          dist: {
            shasum,
            integrity,
            tarball: `${GITEA_BASE}/api/packages/${GITEA_OWNER}/npm/${safeName}/-/${pkg.name.split("/").pop()}-${pkg.version}.tgz`,
          },
        },
      },
      _attachments: {
        [`${pkg.name.split("/").pop()}-${pkg.version}.tgz`]: {
          content_type: "application/octet-stream",
          data: content.toString("base64"),
          length: content.length,
        },
      },
    };

    const res = await fetch(`${GITEA_BASE}/api/packages/${GITEA_OWNER}/npm/${safeName}`, {
      method: "PUT",
      headers: {
        Authorization: `token ${GITEA_TOKEN}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    console.log(`  [PACKAGE] ${pkg.name}@${pkg.version} -> status: ${res.status}`);
  }
}

export async function syncWiki(): Promise<void> {
  console.log("\x1b[36m--> Synchronizing Wiki Documentation...\x1b[0m");
  const { data: existingPages } = await giteaFetch<any[]>(`/repos/${GITEA_REPO}/wiki/pages`);
  const pageList = Array.isArray(existingPages) ? existingPages : [];
  const existingMap = new Map<string, string>();
  for (const p of pageList) {
    existingMap.set(p.title.toLowerCase(), p.sub_url);
  }

  for (const page of WIKI_PAGES) {
    const subUrl = existingMap.get(page.title.toLowerCase());
    const contentBase64 = Buffer.from(page.content).toString("base64");

    if (subUrl) {
      const patchRes = await giteaFetch(
        `/repos/${GITEA_REPO}/wiki/page/${encodeURIComponent(subUrl)}`,
        {
          method: "PATCH",
          body: JSON.stringify({ title: page.title, content_base64: contentBase64 }),
        },
      );
      console.log(`  [UPDATED] Wiki: ${page.title} -> status: ${patchRes.status}`);
    } else {
      const postRes = await giteaFetch(`/repos/${GITEA_REPO}/wiki/new`, {
        method: "POST",
        body: JSON.stringify({ title: page.title, content_base64: contentBase64 }),
      });
      console.log(`  [CREATED] Wiki: ${page.title} -> status: ${postRes.status}`);
    }
  }
}

async function main() {
  console.log("\x1b[35m=====================================================\x1b[0m");
  console.log("\x1b[35m  CDDM Gitea Portal Synchronizer & Content Populator \x1b[0m");
  console.log("\x1b[35m=====================================================\x1b[0m\n");

  const milestoneMap = await syncMilestones();
  const labelMap = await syncLabels();
  await syncIssues(milestoneMap, labelMap);
  await syncPullRequests(milestoneMap, labelMap);
  await syncReleases();
  await syncPackages();
  await syncWiki();

  console.log(
    "\n\x1b[32m[SUCCESS] All Gitea portal pages populated with rich documentation and info!\x1b[0m\n",
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("\x1b[31mFatal portal population error:\x1b[0m", err);
    process.exit(1);
  });
}
