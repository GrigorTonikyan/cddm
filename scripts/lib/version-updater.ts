import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import type { ParsedCommit } from "../version";
import { resolvePublisherConfig } from "./publisher-config";

export interface VersionMismatch {
  file: string;
  field: string;
  expected: string;
  actual: string;
}

export interface VersionSyncCheckResult {
  synchronized: boolean;
  canonicalVersion: string;
  checkedFiles: number;
  mismatches: VersionMismatch[];
}

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
 * Verify that all workspace manifests and distribution URLs match Cargo.toml SSoT version.
 */
export function checkWorkspaceVersionSync(
  workspaceRoot: string = process.cwd(),
): VersionSyncCheckResult {
  const cargoPath = join(workspaceRoot, "Cargo.toml");
  if (!existsSync(cargoPath)) {
    throw new Error(`Cargo.toml not found at ${cargoPath}`);
  }
  const cargoContent = readFileSync(cargoPath, "utf-8");
  const cargoMatch = /\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/.exec(cargoContent);
  const canonicalVersion = cargoMatch?.[1] ?? "";
  if (!canonicalVersion) {
    throw new Error("Could not extract [workspace.package].version from Cargo.toml");
  }

  const mismatches: VersionMismatch[] = [];
  let checkedFiles = 1; // Cargo.toml

  const jsonFiles = [
    "package.json",
    "webui/package.json",
    "npm/cddm/package.json",
    "editors/vscode/package.json",
  ];

  for (const relPath of jsonFiles) {
    const fullPath = join(workspaceRoot, relPath);
    if (!existsSync(fullPath)) continue;
    checkedFiles++;
    const pkg = JSON.parse(readFileSync(fullPath, "utf-8")) as {
      version?: string;
      optionalDependencies?: Record<string, string>;
    };
    if (pkg.version !== canonicalVersion) {
      mismatches.push({
        file: relPath,
        field: "version",
        expected: canonicalVersion,
        actual: pkg.version ?? "<missing>",
      });
    }
    if (relPath === "npm/cddm/package.json" && pkg.optionalDependencies) {
      for (const [depName, depVer] of Object.entries(pkg.optionalDependencies)) {
        if (depVer !== canonicalVersion) {
          mismatches.push({
            file: relPath,
            field: `optionalDependencies.${depName}`,
            expected: canonicalVersion,
            actual: depVer,
          });
        }
      }
    }
  }

  // Homebrew formula
  const brewPath = join(workspaceRoot, "packaging", "homebrew", "cddm.rb");
  if (existsSync(brewPath)) {
    checkedFiles++;
    const brewContent = readFileSync(brewPath, "utf-8");
    const brewMatch = /version\s+"([^"]+)"/.exec(brewContent);
    const actualBrew = brewMatch?.[1] ?? "<missing>";
    if (actualBrew !== canonicalVersion) {
      mismatches.push({
        file: "packaging/homebrew/cddm.rb",
        field: "version",
        expected: canonicalVersion,
        actual: actualBrew,
      });
    }
  }

  const pubConfig = resolvePublisherConfig(workspaceRoot);
  const repoBaseUrl = `https://${pubConfig.vcsHost}/${pubConfig.vcsRepo}`;

  // Scoop manifest
  const scoopPath = join(workspaceRoot, "packaging", "scoop", "cddm.json");
  if (existsSync(scoopPath)) {
    checkedFiles++;
    const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8")) as {
      version?: string;
      architecture?: Record<string, { url?: string }>;
    };
    if (scoopJson.version !== canonicalVersion) {
      mismatches.push({
        file: "packaging/scoop/cddm.json",
        field: "version",
        expected: canonicalVersion,
        actual: scoopJson.version ?? "<missing>",
      });
    }
    const expected64Url = `${repoBaseUrl}/releases/download/v${canonicalVersion}/cddm-v${canonicalVersion}-x86_64-pc-windows-gnu.zip`;
    const actual64Url = scoopJson.architecture?.["64bit"]?.url ?? "<missing>";
    if (actual64Url !== expected64Url) {
      mismatches.push({
        file: "packaging/scoop/cddm.json",
        field: "architecture.64bit.url",
        expected: expected64Url,
        actual: actual64Url,
      });
    }
  }

  // Winget manifest
  const wingetCandidates = [
    join(workspaceRoot, "packaging", "winget", "cddm.yaml"),
    join(workspaceRoot, "packaging", "winget", `${pubConfig.publisherId}.cddm.yaml`),
  ];
  const wingetPath = wingetCandidates.find((p) => existsSync(p));
  if (wingetPath) {
    checkedFiles++;
    const wingetRel = wingetPath.endsWith("cddm.yaml")
      ? "packaging/winget/cddm.yaml"
      : `packaging/winget/${pubConfig.publisherId}.cddm.yaml`;
    const wingetContent = readFileSync(wingetPath, "utf-8");
    const verMatch = /PackageVersion:\s*([\d.]+)/.exec(wingetContent);
    const actualWinget = verMatch?.[1] ?? "<missing>";
    if (actualWinget !== canonicalVersion) {
      mismatches.push({
        file: wingetRel,
        field: "PackageVersion",
        expected: canonicalVersion,
        actual: actualWinget,
      });
    }
    const expectedWingetX64 = `${repoBaseUrl}/releases/download/v${canonicalVersion}/cddm-v${canonicalVersion}-x86_64-pc-windows-gnu.zip`;
    if (!wingetContent.includes(expectedWingetX64)) {
      mismatches.push({
        file: wingetRel,
        field: "InstallerUrl (x64)",
        expected: expectedWingetX64,
        actual: "<stale InstallerUrl>",
      });
    }
  }

  // README.md badges
  const readmePath = join(workspaceRoot, "README.md");
  if (existsSync(readmePath)) {
    checkedFiles++;
    const readmeContent = readFileSync(readmePath, "utf-8");
    const npmBadge = /badge\/npm-([\d.]+)-red\.svg/.exec(readmeContent)?.[1] ?? "<missing>";
    const cratesBadge =
      /badge\/crates\.io-([\d.]+)-brightgreen\.svg/.exec(readmeContent)?.[1] ?? "<missing>";
    if (npmBadge !== canonicalVersion) {
      mismatches.push({
        file: "README.md",
        field: "npm badge",
        expected: canonicalVersion,
        actual: npmBadge,
      });
    }
    if (cratesBadge !== canonicalVersion) {
      mismatches.push({
        file: "README.md",
        field: "crates.io badge",
        expected: canonicalVersion,
        actual: cratesBadge,
      });
    }
  }

  // Cargo.lock workspace crates
  const cargoLockPath = join(workspaceRoot, "Cargo.lock");
  if (existsSync(cargoLockPath)) {
    checkedFiles++;
    const lockContent = readFileSync(cargoLockPath, "utf-8");
    const crateRe = /name\s*=\s*"(cddm-(?:core|cli|mcp|lsp))"\r?\nversion\s*=\s*"([^"]+)"/g;
    let match: RegExpExecArray | null = crateRe.exec(lockContent);
    while (match !== null) {
      const crateName = match[1]!;
      const crateVer = match[2]!;
      if (crateVer !== canonicalVersion) {
        mismatches.push({
          file: "Cargo.lock",
          field: crateName,
          expected: canonicalVersion,
          actual: crateVer,
        });
      }
      match = crateRe.exec(lockContent);
    }
  }

  return {
    synchronized: mismatches.length === 0,
    canonicalVersion,
    checkedFiles,
    mismatches,
  };
}

/**
 * Update all version manifests across the repository in strict lockstep.
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

  // 4. npm/cddm/package.json (including optionalDependencies)
  const npmPkgPath = join(workspaceRoot, "npm", "cddm", "package.json");
  if (existsSync(npmPkgPath)) {
    const pkg = JSON.parse(readFileSync(npmPkgPath, "utf-8"));
    pkg.version = newVersion;
    if (pkg.optionalDependencies && typeof pkg.optionalDependencies === "object") {
      for (const dep of Object.keys(pkg.optionalDependencies)) {
        pkg.optionalDependencies[dep] = newVersion;
      }
    }
    writeFileSync(npmPkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    console.log(
      `\x1b[32m[OK] Updated npm/cddm/package.json (+ optionalDependencies) -> ${newVersion}\x1b[0m`,
    );
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

  const pubConfig = resolvePublisherConfig(workspaceRoot);
  const repoBaseUrl = `https://${pubConfig.vcsHost}/${pubConfig.vcsRepo}`;

  // 7. packaging/scoop/cddm.json
  const scoopPath = join(workspaceRoot, "packaging", "scoop", "cddm.json");
  if (existsSync(scoopPath)) {
    const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8"));
    scoopJson.version = newVersion;
    if (scoopJson.architecture) {
      if (scoopJson.architecture["64bit"]) {
        scoopJson.architecture["64bit"].url =
          `${repoBaseUrl}/releases/download/v${newVersion}/cddm-v${newVersion}-x86_64-pc-windows-gnu.zip`;
      }
      if (scoopJson.architecture["arm64"]) {
        scoopJson.architecture["arm64"].url =
          `${repoBaseUrl}/releases/download/v${newVersion}/cddm-v${newVersion}-aarch64-pc-windows-gnu.zip`;
      }
    }
    if (scoopJson.autoupdate?.architecture) {
      if (scoopJson.autoupdate.architecture["64bit"]) {
        scoopJson.autoupdate.architecture["64bit"].url =
          `${repoBaseUrl}/releases/download/v$version/cddm-v$version-x86_64-pc-windows-gnu.zip`;
      }
      if (scoopJson.autoupdate.architecture["arm64"]) {
        scoopJson.autoupdate.architecture["arm64"].url =
          `${repoBaseUrl}/releases/download/v$version/cddm-v$version-aarch64-pc-windows-gnu.zip`;
      }
    }
    writeFileSync(scoopPath, JSON.stringify(scoopJson, null, 2) + "\n", "utf-8");
    console.log(`\x1b[32m[OK] Updated packaging/scoop/cddm.json -> ${newVersion}\x1b[0m`);
  }

  // 8. packaging/winget/cddm.yaml
  const wingetCandidates = [
    join(workspaceRoot, "packaging", "winget", "cddm.yaml"),
    join(workspaceRoot, "packaging", "winget", `${pubConfig.publisherId}.cddm.yaml`),
  ];
  const wingetPath = wingetCandidates.find((p) => existsSync(p));
  if (wingetPath) {
    const wingetContent = readFileSync(wingetPath, "utf-8");
    const updatedWinget = wingetContent
      .replace(/PackageIdentifier:\s*[^\r\n]+/, `PackageIdentifier: ${pubConfig.publisherId}.cddm`)
      .replace(/Publisher:\s*[^\r\n]+/, `Publisher: ${pubConfig.publisherId}`)
      .replace(/PackageVersion:\s*[\d.]+/, `PackageVersion: ${newVersion}`)
      .replace(
        /releases\/download\/v[\d.]+\/cddm-v[\d.]+-x86_64-pc-windows-(?:msvc|gnu)\.zip/g,
        `releases/download/v${newVersion}/cddm-v${newVersion}-x86_64-pc-windows-gnu.zip`,
      )
      .replace(
        /releases\/download\/v[\d.]+\/cddm-v[\d.]+-aarch64-pc-windows-(?:msvc|gnu)\.zip/g,
        `releases/download/v${newVersion}/cddm-v${newVersion}-aarch64-pc-windows-gnu.zip`,
      );
    writeFileSync(wingetPath, updatedWinget, "utf-8");
    console.log(`\x1b[32m[OK] Updated ${wingetPath} -> ${newVersion}\x1b[0m`);
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

  // 10. Cargo.lock (instant in-place workspace crate version sync)
  const cargoLockPath = join(workspaceRoot, "Cargo.lock");
  if (existsSync(cargoLockPath)) {
    const lockContent = readFileSync(cargoLockPath, "utf-8");
    const updatedLock = lockContent.replace(
      /(name\s*=\s*"cddm-(?:core|cli|mcp|lsp)"\r?\nversion\s*=\s*)"[^"]+"/g,
      `$1"${newVersion}"`,
    );
    writeFileSync(cargoLockPath, updatedLock, "utf-8");
    console.log(`\x1b[32m[OK] Updated Cargo.lock -> ${newVersion}\x1b[0m`);
  }
}

/**
 * Idempotently prepend or replace a version section in CHANGELOG.md.
 */
export function updateChangelog(newSection: string, workspaceRoot: string = process.cwd()): void {
  const changelogPath = join(workspaceRoot, "CHANGELOG.md");
  let header =
    "# Changelog\n\nAll notable changes to **CDDM** will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n";

  const versionHeaderMatch = /^## \[([^\]]+)\]/.exec(newSection.trim());
  const targetHeading = versionHeaderMatch?.[1] ? `## [${versionHeaderMatch[1]}]` : null;

  let existingContent = "";
  if (existsSync(changelogPath)) {
    const full = readFileSync(changelogPath, "utf-8");
    const firstSectionIndex = full.indexOf("\n## [");
    if (firstSectionIndex !== -1) {
      header = full.slice(0, firstSectionIndex + 1);
      existingContent = full.slice(firstSectionIndex + 1);

      // If the top section already matches this version, replace it in-place (idempotent)
      if (targetHeading && existingContent.startsWith(targetHeading)) {
        const secondSectionIndex = existingContent.indexOf("\n## [", targetHeading.length);
        existingContent =
          secondSectionIndex !== -1 ? existingContent.slice(secondSectionIndex + 1) : "";
      }
    } else {
      existingContent = full;
    }
  }

  const updated = `${header}${newSection}${existingContent}`;
  writeFileSync(changelogPath, updated, "utf-8");
  console.log(`\x1b[32m[OK] Updated CHANGELOG.md\x1b[0m`);
}
