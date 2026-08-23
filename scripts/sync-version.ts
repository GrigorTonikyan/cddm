#!/usr/bin/env bun
/**
 * Synchronizes Cargo.toml workspace version with root package.json version,
 * and updates CHANGELOG.md via conventional-changelog.
 */

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const workspaceRoot = process.cwd();

// 1. Read target version from package.json
const rootPkgPath = join(workspaceRoot, "package.json");
if (!existsSync(rootPkgPath)) {
  console.error("package.json not found!");
  process.exit(1);
}

const pkg = JSON.parse(readFileSync(rootPkgPath, "utf-8"));
const newVersion = pkg.version;
console.log(`\x1b[36mSynchronizing workspace to version ${newVersion}...\x1b[0m`);

// 2. Sync Cargo.toml
const cargoPath = join(workspaceRoot, "Cargo.toml");
if (existsSync(cargoPath)) {
  const cargoContent = readFileSync(cargoPath, "utf-8");
  const updatedCargo = cargoContent.replace(
    /(\[workspace\.package\][\s\S]*?version\s*=\s*)"[^"]+"/,
    `$1"${newVersion}"`,
  );
  writeFileSync(cargoPath, updatedCargo, "utf-8");
  console.log(`\x1b[32m[OK] Updated Cargo.toml -> ${newVersion}\x1b[0m`);
}

// 3. Update CHANGELOG.md with conventionalcommits preset
console.log("\x1b[36mUpdating CHANGELOG.md with conventional-changelog...\x1b[0m");
const changelogProc = Bun.spawnSync([
  "bunx",
  "conventional-changelog",
  "-p",
  "conventionalcommits",
  "-i",
  "CHANGELOG.md",
  "-s",
  "-r",
  "1",
]);

if (changelogProc.exitCode === 0) {
  console.log("\x1b[32m[OK] CHANGELOG.md updated successfully!\x1b[0m");
} else {
  console.warn("conventional-changelog notice:", changelogProc.stderr.toString());
}

export {};
