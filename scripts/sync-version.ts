#!/usr/bin/env bun
/**
 * Synchronizes Cargo.toml workspace version with root package.json version,
 * and updates CHANGELOG.md via conventional-changelog using native Bun APIs.
 */

import { join } from "node:path";

const workspaceRoot = process.cwd();

// 1. Read target version from package.json
const rootPkgPath = join(workspaceRoot, "package.json");
const rootPkgFile = Bun.file(rootPkgPath);
if (!(await rootPkgFile.exists())) {
  console.error("package.json not found!");
  process.exit(1);
}

const pkg = await rootPkgFile.json();
const newVersion = pkg.version;
console.log(`\x1b[36mSynchronizing workspace to version ${newVersion}...\x1b[0m`);

// 2. Sync Cargo.toml
const cargoPath = join(workspaceRoot, "Cargo.toml");
const cargoFile = Bun.file(cargoPath);
if (await cargoFile.exists()) {
  const cargoContent = await cargoFile.text();
  const updatedCargo = cargoContent.replace(
    /(\[workspace\.package\][\s\S]*?version\s*=\s*)"[^"]+"/,
    `$1"${newVersion}"`,
  );
  await Bun.write(cargoPath, updatedCargo);
  console.log(`\x1b[32m[OK] Updated Cargo.toml -> ${newVersion}\x1b[0m`);
}

// 3. Sync webui/package.json
const webuiPkgPath = join(workspaceRoot, "webui", "package.json");
const webuiPkgFile = Bun.file(webuiPkgPath);
if (await webuiPkgFile.exists()) {
  const webuiPkg = await webuiPkgFile.json();
  webuiPkg.version = newVersion;
  await Bun.write(webuiPkgPath, JSON.stringify(webuiPkg, null, 2) + "\n");
  console.log(`\x1b[32m[OK] Updated webui/package.json -> ${newVersion}\x1b[0m`);
}

// 4. Sync npm/cddm/package.json
const npmPkgPath = join(workspaceRoot, "npm", "cddm", "package.json");
const npmPkgFile = Bun.file(npmPkgPath);
if (await npmPkgFile.exists()) {
  const npmPkg = await npmPkgFile.json();
  npmPkg.version = newVersion;
  await Bun.write(npmPkgPath, JSON.stringify(npmPkg, null, 2) + "\n");
  console.log(`\x1b[32m[OK] Updated npm/cddm/package.json -> ${newVersion}\x1b[0m`);
}

// 5. Sync editors/vscode/package.json
const vscodePkgPath = join(workspaceRoot, "editors", "vscode", "package.json");
const vscodePkgFile = Bun.file(vscodePkgPath);
if (await vscodePkgFile.exists()) {
  const vscodePkg = await vscodePkgFile.json();
  vscodePkg.version = newVersion;
  await Bun.write(vscodePkgPath, JSON.stringify(vscodePkg, null, 2) + "\n");
  console.log(`\x1b[32m[OK] Updated editors/vscode/package.json -> ${newVersion}\x1b[0m`);
}

// 6. Update README.md badges
const readmePath = join(workspaceRoot, "README.md");
const readmeFile = Bun.file(readmePath);
if (await readmeFile.exists()) {
  const readmeContent = await readmeFile.text();
  const updatedReadme = readmeContent
    .replace(/badge\/npm-[\d.]+-red\.svg/, `badge/npm-${newVersion}-red.svg`)
    .replace(
      /badge\/crates\.io-[\d.]+-brightgreen\.svg/,
      `badge/crates.io-${newVersion}-brightgreen.svg`,
    );
  await Bun.write(readmePath, updatedReadme);
  console.log(`\x1b[32m[OK] Updated README.md version badges -> ${newVersion}\x1b[0m`);
}

console.log(`\x1b[32m[SUCCESS] Workspace version synchronization complete!\x1b[0m\n`);
