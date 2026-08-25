#!/usr/bin/env bun
/**
 * Verifies ecosystem distribution packaging manifests and scripts.
 */

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const rootDir = resolve(import.meta.dir, "..");

const requiredPackagingFiles = [
  "packaging/homebrew/cddm.rb",
  "packaging/scoop/cddm.json",
  "packaging/winget/GrigorTonikyan.cddm.yaml",
  "packaging/install.sh",
  "packaging/install.ps1",
  "docs/JETBRAINS_SETUP.md",
];

console.log("\x1b[36m--> Verifying ecosystem distribution manifests and packaging...\x1b[0m");

let hasErrors = false;

for (const relPath of requiredPackagingFiles) {
  const fullPath = resolve(rootDir, relPath);
  if (!existsSync(fullPath)) {
    console.error(`\x1b[31m[FAIL] Missing required packaging file: ${relPath}\x1b[0m`);
    hasErrors = true;
    continue;
  }

  const content = readFileSync(fullPath, "utf-8");
  if (content.trim().length === 0) {
    console.error(`\x1b[31m[FAIL] Packaging file is empty: ${relPath}\x1b[0m`);
    hasErrors = true;
    continue;
  }

  console.log(`\x1b[32m[PASS]\x1b[0m Verified ${relPath} (${content.split("\n").length} lines)`);
}

// Validate Homebrew syntax
const brewPath = resolve(rootDir, "packaging/homebrew/cddm.rb");
const brewContent = readFileSync(brewPath, "utf-8");
if (!brewContent.includes("class Cddm < Formula") || !brewContent.includes('bin.install "cddm"')) {
  console.error(
    "\x1b[31m[FAIL] Homebrew formula missing standard class or bin.install directive\x1b[0m",
  );
  hasErrors = true;
}

// Validate Scoop syntax
const scoopPath = resolve(rootDir, "packaging/scoop/cddm.json");
try {
  const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8"));
  if (!scoopJson.bin || !scoopJson.architecture) {
    console.error("\x1b[31m[FAIL] Scoop manifest missing bin or architecture definitions\x1b[0m");
    hasErrors = true;
  }
} catch (err) {
  console.error(`\x1b[31m[FAIL] Scoop manifest JSON parse error: ${String(err)}\x1b[0m`);
  hasErrors = true;
}

if (hasErrors) {
  process.exit(1);
} else {
  console.log("\x1b[32m[SUCCESS] All ecosystem packaging manifests validated successfully.\x1b[0m");
}
