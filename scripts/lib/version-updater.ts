import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { ParsedCommit } from "../version";

/**
 * Generate a Markdown changelog section for a release.
 */
export function generateChangelogSection(
  version: string,
  date: string,
  commits: ParsedCommit[],
): string {
  let section = `## [${version}] - ${date}\n\n`;

  const breakings = commits.filter((c) => c.breaking);
  const features = commits.filter((c) => c.type === "feat");
  const fixes = commits.filter((c) => c.type === "fix");
  const perfs = commits.filter((c) => c.type === "perf");
  const refactors = commits.filter((c) => c.type === "refactor");
  const docs = commits.filter((c) => c.type === "docs");
  const chores = commits.filter((c) => ["chore", "build", "ci", "style", "test"].includes(c.type));

  if (breakings.length > 0) {
    section += `### BREAKING CHANGES\n\n`;
    for (const c of breakings) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (features.length > 0) {
    section += `### Features\n\n`;
    for (const c of features) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (fixes.length > 0) {
    section += `### Bug Fixes\n\n`;
    for (const c of fixes) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (perfs.length > 0) {
    section += `### Performance Improvements\n\n`;
    for (const c of perfs) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (refactors.length > 0) {
    section += `### Refactoring\n\n`;
    for (const c of refactors) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (docs.length > 0) {
    section += `### Documentation\n\n`;
    for (const c of docs) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  if (chores.length > 0) {
    section += `### Tooling & Maintenance\n\n`;
    for (const c of chores) {
      const scopeStr = c.scope ? `**${c.scope}**: ` : "";
      section += `- ${scopeStr}${c.subject} (\`${c.hash}\`)\n`;
    }
    section += "\n";
  }

  return section;
}

/**
 * Update all version manifests across the repository.
 */
export function updateWorkspaceVersions(
  newVersion: string,
  workspaceRoot: string = process.cwd(),
): void {
  // 1. Cargo.toml
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

  // 2. Root package.json
  const rootPkgPath = join(workspaceRoot, "package.json");
  if (existsSync(rootPkgPath)) {
    const pkg = JSON.parse(readFileSync(rootPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(rootPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated package.json -> ${newVersion}\x1b[0m`);
  }

  // 3. WebUI package.json
  const webuiPkgPath = join(workspaceRoot, "webui", "package.json");
  if (existsSync(webuiPkgPath)) {
    const pkg = JSON.parse(readFileSync(webuiPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(webuiPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated webui/package.json -> ${newVersion}\x1b[0m`);
  }

  // 4. npm/cddm/package.json
  const npmPkgPath = join(workspaceRoot, "npm", "cddm", "package.json");
  if (existsSync(npmPkgPath)) {
    const pkg = JSON.parse(readFileSync(npmPkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(npmPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated npm/cddm/package.json -> ${newVersion}\x1b[0m`);
  }

  // 5. editors/vscode/package.json
  const vscodePkgPath = join(workspaceRoot, "editors", "vscode", "package.json");
  if (existsSync(vscodePkgPath)) {
    const pkg = JSON.parse(readFileSync(vscodePkgPath, "utf-8"));
    pkg.version = newVersion;
    writeFileSync(vscodePkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated editors/vscode/package.json -> ${newVersion}\x1b[0m`);
  }

  // 6. packaging/homebrew/cddm.rb
  const brewPath = join(workspaceRoot, "packaging", "homebrew", "cddm.rb");
  if (existsSync(brewPath)) {
    const brewContent = readFileSync(brewPath, "utf-8");
    const updatedBrew = brewContent.replace(/version "[^"]+"/, `version "${newVersion}"`);
    writeFileSync(brewPath, updatedBrew, "utf-8");
    console.log(`\x1b[32m[OK] Updated packaging/homebrew/cddm.rb -> ${newVersion}\x1b[0m`);
  }

  // 7. packaging/scoop/cddm.json
  const scoopPath = join(workspaceRoot, "packaging", "scoop", "cddm.json");
  if (existsSync(scoopPath)) {
    const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8"));
    scoopJson.version = newVersion;
    writeFileSync(scoopPath, JSON.stringify(scoopJson, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated packaging/scoop/cddm.json -> ${newVersion}\x1b[0m`);
  }

  // 8. packaging/winget/GrigorTonikyan.cddm.yaml
  const wingetPath = join(workspaceRoot, "packaging", "winget", "GrigorTonikyan.cddm.yaml");
  if (existsSync(wingetPath)) {
    const wingetContent = readFileSync(wingetPath, "utf-8");
    const updatedWinget = wingetContent.replace(
      /PackageVersion:\s*[\d.]+/,
      `PackageVersion: ${newVersion}`,
    );
    writeFileSync(wingetPath, updatedWinget, "utf-8");
    console.log(
      `\x1b[32m[OK] Updated packaging/winget/GrigorTonikyan.cddm.yaml -> ${newVersion}\x1b[0m`,
    );
  }

  // 9. README.md badges
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
    console.log(`\x1b[32m[OK] Updated README.md -> ${newVersion}\x1b[0m`);
  }

  // 10. Cargo.lock
  Bun.spawnSync(["cargo", "check", "--workspace"], { cwd: workspaceRoot });
}

/**
 * Prepend a new section to CHANGELOG.md.
 */
export function updateChangelog(newSection: string, workspaceRoot: string = process.cwd()): void {
  const changelogPath = join(workspaceRoot, "CHANGELOG.md");
  let header =
    "# Changelog\n\nAll notable changes to **CDDM** will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n";

  let existingContent = "";
  if (existsSync(changelogPath)) {
    const full = readFileSync(changelogPath, "utf-8");
    const firstSectionIndex = full.indexOf("\n## [");
    if (firstSectionIndex !== -1) {
      header = full.slice(0, firstSectionIndex + 1);
      existingContent = full.slice(firstSectionIndex + 1);
    } else {
      existingContent = full;
    }
  }

  const updated = `${header}${newSection}${existingContent}`;
  writeFileSync(changelogPath, updated, "utf-8");
  console.log(`\x1b[32m[OK] Updated CHANGELOG.md\x1b[0m`);
}
