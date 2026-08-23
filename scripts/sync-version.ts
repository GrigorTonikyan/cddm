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

// 3. Sync webui/package.json
const webuiPkgPath = join(workspaceRoot, "webui", "package.json");
if (existsSync(webuiPkgPath)) {
  const webuiPkg = JSON.parse(readFileSync(webuiPkgPath, "utf-8"));
  webuiPkg.version = newVersion;
  writeFileSync(webuiPkgPath, JSON.stringify(webuiPkg, null, 2) + "\n", "utf-8");
  console.log(`\x1b[32m[OK] Updated webui/package.json -> ${newVersion}\x1b[0m`);
}

// 4. Sync npm/cddm/package.json
const npmPkgPath = join(workspaceRoot, "npm", "cddm", "package.json");
if (existsSync(npmPkgPath)) {
  const npmPkg = JSON.parse(readFileSync(npmPkgPath, "utf-8"));
  npmPkg.version = newVersion;
  writeFileSync(npmPkgPath, JSON.stringify(npmPkg, null, 2) + "\n", "utf-8");
  console.log(`\x1b[32m[OK] Updated npm/cddm/package.json -> ${newVersion}\x1b[0m`);
}

// 5. Update README.md badges
const readmePath = join(workspaceRoot, "README.md");
if (existsSync(readmePath)) {
  const readmeContent = readFileSync(readmePath, "utf-8");
  const updatedReadme = readmeContent
    .replace(/badge\/npm-[\d.]+-red\.svg/, `badge/npm-${newVersion}-red.svg`)
    .replace(
      /badge\/crates\.io-[\d.]+-brightgreen\.svg/,
      `badge/crates.io-${newVersion}-brightgreen.svg`,
    );
  writeFileSync(readmePath, updatedReadme, "utf-8");
  console.log(`\x1b[32m[OK] Updated README.md badges -> ${newVersion}\x1b[0m`);
}

// 6. Update Cargo.lock
Bun.spawnSync(["cargo", "check", "--workspace"], { cwd: workspaceRoot });
console.log(`\x1b[32m[OK] Updated Cargo.lock -> ${newVersion}\x1b[0m`);

// 7. Update CHANGELOG.md with conventionalcommits preset
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
