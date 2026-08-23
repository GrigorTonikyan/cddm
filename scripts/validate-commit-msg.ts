#!/usr/bin/env bun
/**
 * Cross-platform Conventional Commits Validator for CDDM.
 * Enforces Conventional Commits specification for all git commits.
 */

import { readFileSync, existsSync } from "node:fs";

export const CONVENTIONAL_COMMIT_REGEX =
  /^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(?:\(([a-zA-Z0-9_/-]+)\))?(!)?:\s*(.+)$/;

export interface ValidationResult {
  valid: boolean;
  type?: string;
  scope?: string;
  breaking: boolean;
  subject?: string;
  error?: string;
}

export function validateCommitMessage(message: string): ValidationResult {
  const trimmed = message.trim();
  if (!trimmed) {
    return { valid: false, breaking: false, error: "Commit message cannot be empty" };
  }

  const firstLine = trimmed.split("\n")[0]?.trim() ?? "";

  // Allow standard git merge or revert commits
  if (
    firstLine.startsWith("Merge ") ||
    firstLine.startsWith("fixup! ") ||
    firstLine.startsWith("squash! ") ||
    firstLine.startsWith("Revert ") ||
    /^v\d+\.\d+\.\d+/.test(firstLine)
  ) {
    return { valid: true, breaking: false, subject: firstLine };
  }

  const match = CONVENTIONAL_COMMIT_REGEX.exec(firstLine);
  if (!match) {
    return {
      valid: false,
      breaking: false,
      error: `Invalid Conventional Commit format: "${firstLine}". Expected "<type>(<scope>): <subject>" or "<type>: <subject>".`,
    };
  }

  const [, type, scope, breakingExclamation, subject] = match;
  const isBreaking = Boolean(breakingExclamation) || trimmed.includes("BREAKING CHANGE:");

  if (!subject || subject.trim().length === 0) {
    return {
      valid: false,
      breaking: isBreaking,
      error: "Commit subject line cannot be empty after the type/scope prefix.",
    };
  }

  if (firstLine.length > 100) {
    return {
      valid: false,
      breaking: isBreaking,
      error: `Commit header exceeds 100 characters (${firstLine.length} chars). Keep subjects concise.`,
    };
  }

  return {
    valid: true,
    type,
    scope: scope || undefined,
    breaking: isBreaking,
    subject: subject.trim(),
  };
}

async function main() {
  const commitMsgFile = process.argv[2];
  if (!commitMsgFile || !existsSync(commitMsgFile)) {
    console.log("No commit message file provided, skipping validation.");
    process.exit(0);
  }

  const content = readFileSync(commitMsgFile, "utf-8");
  const result = validateCommitMessage(content);

  if (!result.valid) {
    console.error(`\n\x1b[31m✖ ${result.error}\x1b[0m\n`);
    console.error(
      "Valid types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert",
    );
    console.error("Examples:");
    console.error("  feat(core): implement subtree AST hashing");
    console.error("  fix(webui): correct slider token threshold calculation");
    console.error("  feat(api)!: redesign scan response payload for streaming");
    console.error("");
    process.exit(1);
  }

  console.log("\x1b[32m✔ Conventional Commit message validated!\x1b[0m");
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal commit-msg validation error:", err);
    process.exit(1);
  });
}
