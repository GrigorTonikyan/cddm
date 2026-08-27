#!/usr/bin/env bun
/**
 * Cross-platform Documentation Integrity & Cross-Reference Validator for CDDM.
 * Validates:
 * 1. Existence and integrity of all required repository documentation.
 * 2. Markdown internal link resolution across all files.
 * 3. Bidirectional synchronization between docs/ROADMAP.md (EP-xx proposals) and docs/TODO.md.
 * 4. Markdown table formatting and separator integrity.
 */

import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { syncFeatureMatrixFile } from "./lib/test-matrix-generator";

export interface DocCheckError {
  file: string;
  line?: number;
  message: string;
}

export interface ValidationSummary {
  filesChecked: number;
  linksChecked: number;
  proposalsValidated: number;
  errors: DocCheckError[];
}

export const REQUIRED_DOC_FILES = [
  "README.md",
  "CONTRIBUTING.md",
  "CODE_OF_CONDUCT.md",
  "SECURITY.md",
  "AGENTS.md",
  "CHANGELOG.md",
  "docs/API.md",
  "docs/ARCHITECTURE.md",
  "docs/FEATURE_MATRIX.md",
  "docs/REQUIREMENTS.md",
  "docs/ROADMAP.md",
  "docs/TODO.md",
] as const;

/**
 * Validate that all required core documentation files exist in workspace.
 */
export function checkRequiredDocFiles(workspaceRoot: string = process.cwd()): DocCheckError[] {
  const errors: DocCheckError[] = [];
  for (const relPath of REQUIRED_DOC_FILES) {
    const fullPath = join(workspaceRoot, relPath);
    if (!existsSync(fullPath)) {
      errors.push({
        file: relPath,
        message: `Required documentation file does not exist: ${relPath}`,
      });
    }
  }
  return errors;
}

/**
 * Extract all markdown links ([text](target)) and check if local targets exist.
 */
export function checkMarkdownLinks(
  filePath: string,
  content: string,
  workspaceRoot: string = process.cwd(),
): { linkCount: number; errors: DocCheckError[] } {
  const errors: DocCheckError[] = [];
  const linkRegex = /\[([^\]]+)\]\(([^)]+)\)/g;
  let linkCount = 0;
  const fileDir = dirname(join(workspaceRoot, filePath));

  const lines = content.split("\n");
  for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
    const line = lines[lineIndex] ?? "";
    let match: RegExpExecArray | null = linkRegex.exec(line);

    while (match !== null) {
      linkCount++;
      let target = match[2]?.trim() ?? "";

      // Skip web links, email, or fragment-only links (#anchor)
      if (
        !target.startsWith("http://") &&
        !target.startsWith("https://") &&
        !target.startsWith("mailto:") &&
        !target.startsWith("#")
      ) {
        // Strip fragment identifier (#anchor)
        target = target.split("#")[0] ?? "";
        // Strip file:/// scheme if present
        target = target.replace(/^file:\/\/\/?/, "");

        if (target.length > 0) {
          let resolvedPath: string;
          if (target.startsWith("/") || target.startsWith("\\")) {
            resolvedPath = join(workspaceRoot, target);
          } else {
            resolvedPath = resolve(fileDir, target);
          }

          if (!existsSync(resolvedPath)) {
            errors.push({
              file: filePath,
              line: lineIndex + 1,
              message: `Broken internal markdown link: "${match[0]}" -> target not found: ${target}`,
            });
          }
        }
      }
      match = linkRegex.exec(line);
    }
  }

  return { linkCount, errors };
}

/**
 * Validate that all Enhancement Proposals (EP-xx) in docs/ROADMAP.md and docs/TODO.md are synchronized.
 */
export function checkRoadmapTodoSync(workspaceRoot: string = process.cwd()): {
  proposalCount: number;
  errors: DocCheckError[];
} {
  const errors: DocCheckError[] = [];
  const roadmapPath = join(workspaceRoot, "docs/ROADMAP.md");
  const todoPath = join(workspaceRoot, "docs/TODO.md");

  if (!existsSync(roadmapPath) || !existsSync(todoPath)) {
    return { proposalCount: 0, errors };
  }

  const roadmapContent = readFileSync(roadmapPath, "utf-8");
  const todoContent = readFileSync(todoPath, "utf-8");

  // Extract all EP-xx from ROADMAP.md
  const roadmapEpRegex = /### (EP-\d{2}):\s*([^\n]+)/g;
  const roadmapEps = new Map<string, string>();
  let match: RegExpExecArray | null = roadmapEpRegex.exec(roadmapContent);
  while (match !== null) {
    if (match[1] && match[2]) {
      roadmapEps.set(match[1], match[2].trim());
    }
    match = roadmapEpRegex.exec(roadmapContent);
  }

  // Extract all EP-xx from TODO.md
  const todoEpRegex = /\[(EP-\d{2})\]/g;
  const todoEps = new Set<string>();
  match = todoEpRegex.exec(todoContent);
  while (match !== null) {
    if (match[1]) {
      todoEps.add(match[1]);
    }
    match = todoEpRegex.exec(todoContent);
  }

  // Check that every EP in ROADMAP is tracked in TODO.md
  for (const [epId, title] of roadmapEps.entries()) {
    if (!todoEps.has(epId)) {
      errors.push({
        file: "docs/TODO.md",
        message: `Enhancement proposal "${epId}" (${title}) is defined in docs/ROADMAP.md but missing from docs/TODO.md tracking.`,
      });
    }
  }

  // Check that every EP in TODO.md exists in ROADMAP.md
  for (const epId of todoEps) {
    if (!roadmapEps.has(epId)) {
      errors.push({
        file: "docs/ROADMAP.md",
        message: `Task references proposal "${epId}" in docs/TODO.md, but "${epId}" is not defined in docs/ROADMAP.md.`,
      });
    }
  }

  return { proposalCount: roadmapEps.size, errors };
}

/**
 * Validate markdown table formatting.
 */
export function checkMarkdownTables(filePath: string, content: string): DocCheckError[] {
  const errors: DocCheckError[] = [];
  const lines = content.split("\n");

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]?.trim() ?? "";
    if (line.startsWith("|") && line.endsWith("|")) {
      const isSeparator = /^\|(\s*:?-+:?\s*\|)+$/.test(line);
      if (isSeparator) {
        const prevLine = lines[i - 1]?.trim() ?? "";
        if (!prevLine.startsWith("|") || !prevLine.endsWith("|")) {
          errors.push({
            file: filePath,
            line: i + 1,
            message: `Malformed markdown table: separator row without preceding header row at line ${i + 1}`,
          });
        }
      }
    }
  }

  return errors;
}

/**
 * Execute complete documentation validation.
 */
export async function validateDocumentation(
  workspaceRoot: string = process.cwd(),
): Promise<ValidationSummary> {
  const allErrors: DocCheckError[] = [];
  let totalLinks = 0;

  // 1. Required files check
  const reqErrors = checkRequiredDocFiles(workspaceRoot);
  allErrors.push(...reqErrors);

  // 2. Individual file checks (links & tables)
  let filesChecked = 0;
  for (const relPath of REQUIRED_DOC_FILES) {
    const fullPath = join(workspaceRoot, relPath);
    if (existsSync(fullPath)) {
      filesChecked++;
      const content = readFileSync(fullPath, "utf-8");

      const { linkCount, errors: linkErrors } = checkMarkdownLinks(relPath, content, workspaceRoot);
      totalLinks += linkCount;
      allErrors.push(...linkErrors);

      const tableErrors = checkMarkdownTables(relPath, content);
      allErrors.push(...tableErrors);
    }
  }

  // 3. Roadmap <-> TODO synchronization
  const { proposalCount, errors: syncErrors } = checkRoadmapTodoSync(workspaceRoot);
  allErrors.push(...syncErrors);

  // 4. Feature Matrix dynamic test discovery synchronization
  try {
    const { hasChanges } = await syncFeatureMatrixFile(workspaceRoot);
    if (hasChanges) {
      allErrors.push({
        file: "docs/FEATURE_MATRIX.md",
        message:
          "Feature matrix test tables are out of sync with discovered test files. Run `bun scripts/sync-feature-matrix.ts` to update.",
      });
    }
  } catch (err) {
    allErrors.push({
      file: "docs/FEATURE_MATRIX.md",
      message: `Failed to validate feature matrix synchronization: ${err instanceof Error ? err.message : String(err)}`,
    });
  }

  return {
    filesChecked,
    linksChecked: totalLinks,
    proposalsValidated: proposalCount,
    errors: allErrors,
  };
}

async function main() {
  console.log("\x1b[36m--> Validating repository documentation integrity & sync...\x1b[0m");
  const summary = await validateDocumentation();

  console.log(
    `\x1b[35mChecked ${summary.filesChecked} doc files, ${summary.linksChecked} links, and ${summary.proposalsValidated} roadmap proposals.\x1b[0m`,
  );

  if (summary.errors.length > 0) {
    console.error(
      `\n\x1b[31m[ERROR] Found ${summary.errors.length} documentation integrity errors:\x1b[0m\n`,
    );
    for (const err of summary.errors) {
      const lineStr = err.line ? `:${err.line}` : "";
      console.error(`  \x1b[31m[FAIL] [${err.file}${lineStr}]\x1b[0m ${err.message}`);
    }
    console.error("");
    process.exit(1);
  }

  console.log(
    "\x1b[32m[PASS] All documentation files, links, and roadmap proposals are 100% valid and synchronized!\x1b[0m\n",
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal documentation validation error:", err);
    process.exit(1);
  });
}
