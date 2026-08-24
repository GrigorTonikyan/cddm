#!/usr/bin/env bun
/**
 * Cross-platform No-Emoji Policy Enforcement Scanner for CDDM.
 * Scans the entire codebase (code, docs, tests, scripts, configurations) to ensure
 * NO emojis or pictographic glyphs are present anywhere.
 */

import { readdirSync, statSync, readFileSync } from "node:fs";
import { join } from "node:path";

export interface EmojiMatch {
  file: string;
  line: number;
  column: number;
  char: string;
  codePoint: string;
  context: string;
}

// Regex matching Extended Pictographic characters, Emoji presentations, and miscellaneous symbols
export const EMOJI_REGEX =
  /[\p{Extended_Pictographic}\u{1F300}-\u{1FAFF}\u{1F000}-\u{1F02F}\u{1F0A0}-\u{1F0FF}\u{1F100}-\u{1F64F}\u{1F680}-\u{1F6FF}\u{2600}-\u{27BF}\u{2300}-\u{23FF}\u{2B50}\u{2B55}]/gu;

export function hasEmoji(text: string): boolean {
  EMOJI_REGEX.lastIndex = 0;
  return EMOJI_REGEX.test(text);
}

export const DEFAULT_IGNORED_DIRS = new Set([
  ".git",
  "node_modules",
  "target",
  "dist",
  "coverage",
  ".vscode",
  ".cddm",
]);

export const DEFAULT_IGNORED_EXTENSIONS = new Set([
  ".lock",
  ".png",
  ".jpg",
  ".jpeg",
  ".gif",
  ".webp",
  ".ico",
  ".svg",
  ".wasm",
  ".exe",
  ".dll",
  ".so",
  ".dylib",
  ".bin",
  ".db",
  ".pdf",
]);

export function scanFileForEmojis(
  filePath: string,
  workspaceRoot: string = process.cwd(),
): EmojiMatch[] {
  const fullPath = join(workspaceRoot, filePath);
  let content: string;
  try {
    content = readFileSync(fullPath, "utf-8");
  } catch {
    return [];
  }

  const matches: EmojiMatch[] = [];
  const lines = content.split("\n");

  for (let lineIdx = 0; lineIdx < lines.length; lineIdx++) {
    const line = lines[lineIdx] ?? "";
    EMOJI_REGEX.lastIndex = 0;
    let match: RegExpExecArray | null;

    while ((match = EMOJI_REGEX.exec(line)) !== null) {
      const char = match[0];
      // Skip standard legal/trademark symbols if needed
      if (char === "\u00A9" || char === "\u00AE" || char === "\u2122") {
        continue;
      }
      const codePoint = `U+${(char.codePointAt(0) ?? 0).toString(16).toUpperCase().padStart(4, "0")}`;
      matches.push({
        file: filePath,
        line: lineIdx + 1,
        column: match.index + 1,
        char,
        codePoint,
        context: line.trim(),
      });
    }
  }

  return matches;
}

export function scanDirectoryForEmojis(
  dir: string = ".",
  workspaceRoot: string = process.cwd(),
  ignoredDirs: Set<string> = DEFAULT_IGNORED_DIRS,
  ignoredExts: Set<string> = DEFAULT_IGNORED_EXTENSIONS,
): EmojiMatch[] {
  const fullDirPath = join(workspaceRoot, dir);
  const results: EmojiMatch[] = [];

  let entries: string[];
  try {
    entries = readdirSync(fullDirPath);
  } catch {
    return results;
  }

  for (const entry of entries) {
    if (ignoredDirs.has(entry)) continue;
    const relEntryPath = dir === "." ? entry : join(dir, entry);
    const fullEntryPath = join(workspaceRoot, relEntryPath);

    try {
      const stat = statSync(fullEntryPath);
      if (stat.isDirectory()) {
        results.push(
          ...scanDirectoryForEmojis(relEntryPath, workspaceRoot, ignoredDirs, ignoredExts),
        );
      } else if (stat.isFile()) {
        const dotIdx = entry.lastIndexOf(".");
        const ext = dotIdx !== -1 ? entry.slice(dotIdx).toLowerCase() : "";
        if (ignoredExts.has(ext)) continue;

        const normalizedRelPath = relEntryPath.replace(/\\/g, "/");
        const fileMatches = scanFileForEmojis(normalizedRelPath, workspaceRoot);
        results.push(...fileMatches);
      }
    } catch {
      // Ignore unreadable files
    }
  }

  return results;
}

async function main() {
  console.log("\x1b[36m--> Scanning codebase for emoji violations (NO EMOJI policy)...\x1b[0m");
  const matches = scanDirectoryForEmojis(".");

  if (matches.length > 0) {
    console.error(
      `\n\x1b[31m[ERROR] Found ${matches.length} emoji violation(s) in codebase:\x1b[0m\n`,
    );
    for (const m of matches) {
      console.error(
        `  \x1b[31m[FAIL] [${m.file}:${m.line}:${m.column}]\x1b[0m ${m.char} (${m.codePoint}) -> ${m.context}`,
      );
    }
    console.error(
      "\n\x1b[31mPlease remove all emojis to maintain a clean, professional codebase.\x1b[0m\n",
    );
    process.exit(1);
  }

  console.log("\x1b[32m[PASS] Codebase is 100% clean! Zero emojis found.\x1b[0m\n");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal emoji scan error:", err);
    process.exit(1);
  });
}
