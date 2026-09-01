#!/usr/bin/env bun
/**
 * CDDM Git & Gitea Wiki Synchronization CLI.
 * Tightly couples and pushes canonical Markdown documentation to Gitea Wiki.
 */

import { GITEA_REPO, GITEA_TOKEN, giteaFetch } from "./lib/gitea-client";
import { getDynamicallyAssembledWikiPages } from "./lib/gitea-wiki-data";

export async function syncWikiPages(dryRun: boolean = false): Promise<void> {
  const pages = getDynamicallyAssembledWikiPages();
  console.log(`\x1b[36m--> Synchronizing ${pages.length} Wiki pages with Gitea Portal...\x1b[0m`);

  if (!GITEA_TOKEN) {
    console.log(
      "\x1b[33m[WARN] GITEA_TOKEN not found in environment; performing dry-run wiki assembly validation.\x1b[0m",
    );
    for (const p of pages) {
      console.log(`  [VALID] Wiki Page '${p.title}' assembled (${p.content.length} bytes)`);
    }
    return;
  }

  if (dryRun) {
    console.log("\x1b[33m[DRY RUN] Skipping network calls.\x1b[0m");
    for (const p of pages) {
      console.log(
        `  [DRY RUN] Wiki Page '${p.title}' ready for upload (${p.content.length} bytes)`,
      );
    }
    return;
  }

  const { data: existingPages } = await giteaFetch<{ title: string; sub_url: string }[]>(
    `/repos/${GITEA_REPO}/wiki/pages`,
  );
  const pageList = Array.isArray(existingPages) ? existingPages : [];
  const existingMap = new Map<string, string>();
  for (const p of pageList) {
    existingMap.set(p.title.toLowerCase(), p.sub_url);
  }

  for (const page of pages) {
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
      console.log(`  \x1b[32m[UPDATED]\x1b[0m Wiki: ${page.title} -> status: ${patchRes.status}`);
    } else {
      const postRes = await giteaFetch(`/repos/${GITEA_REPO}/wiki/new`, {
        method: "POST",
        body: JSON.stringify({ title: page.title, content_base64: contentBase64 }),
      });
      console.log(`  \x1b[32m[CREATED]\x1b[0m Wiki: ${page.title} -> status: ${postRes.status}`);
    }
  }
}

async function main() {
  const dryRun = process.argv.includes("--dry-run");
  await syncWikiPages(dryRun);
  console.log("\x1b[32m[SUCCESS] Git Wiki synchronization complete!\x1b[0m\n");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal wiki sync error:", err);
    process.exit(1);
  });
}
