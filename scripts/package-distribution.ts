#!/usr/bin/env bun
/**
 * Verifies ecosystem distribution packaging manifests, scripts, and shell completions,
 * and dynamically renders architecture-pruned manifests when SHA256SUMS.txt is supplied.
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import {
  classifyTargetsFromChecksums,
  collectTargetAssets,
  findZeroSha256Violations,
  parseSha256SumsValidated,
  renderHomebrewFormula,
  renderScoopManifest,
  renderWingetManifests,
  type DistributionManifestSpec,
} from "./lib/distribution-manifests";
import { resolvePublisherConfig } from "./lib/publisher-config";
import { getCurrentVersion } from "./version";

const rootDir = resolve(import.meta.dir, "..");

export const completionFiles = {
  bash: "packaging/completions/cddm.bash",
  fish: "packaging/completions/cddm.fish",
  zsh: "packaging/completions/_cddm",
  powershell: "packaging/completions/_cddm.ps1",
  elvish: "packaging/completions/cddm.elv",
} as const;

export function getRequiredPackagingFiles(workspaceRoot: string = rootDir): string[] {
  const pubConfig = resolvePublisherConfig(workspaceRoot);
  const pkgId = `${pubConfig.publisherId}.cddm`;
  return [
    "packaging/homebrew/cddm.rb",
    "packaging/scoop/cddm.json",
    `packaging/winget/${pkgId}.yaml`,
    `packaging/winget/${pkgId}.installer.yaml`,
    `packaging/winget/${pkgId}.locale.en-US.yaml`,
    "packaging/install.sh",
    "packaging/install.ps1",
    "docs/JETBRAINS_SETUP.md",
    "editors/vscode/package.json",
    "editors/vscode/resources/cddm-icon.svg",
    ...Object.values(completionFiles),
  ];
}

export const requiredPackagingFiles = getRequiredPackagingFiles(rootDir);

/**
 * Generates dynamic manifests strictly matching targets in SHA256SUMS.txt and writes them to packaging/.
 */
export function syncManifestsFromChecksums(
  workspaceRoot: string,
  sumsPath: string,
  version: string = getCurrentVersion(workspaceRoot),
): string[] {
  const sumsContent = readFileSync(sumsPath, "utf-8");
  const checksums = parseSha256SumsValidated(sumsContent);
  const pubConfig = resolvePublisherConfig(workspaceRoot);
  const releaseBaseUrl = `https://${pubConfig.vcsHost}/${pubConfig.vcsRepo}/releases/download/v${version}`;
  const targets = classifyTargetsFromChecksums(checksums, releaseBaseUrl);
  const assets = collectTargetAssets(targets);
  if (assets.length === 0) {
    throw new Error(`No supported target archives found in ${sumsPath}`);
  }

  const cargoToml = existsSync(join(workspaceRoot, "Cargo.toml"))
    ? readFileSync(join(workspaceRoot, "Cargo.toml"), "utf-8")
    : "";
  const descMatch = /description\s*=\s*"([^"]+)"/.exec(cargoToml);
  const licenseMatch = /license\s*=\s*"([^"]+)"/.exec(cargoToml);

  const spec: DistributionManifestSpec = {
    packageName: "cddm",
    version,
    publisherId: pubConfig.publisherId,
    publisherName: pubConfig.publisherName,
    homepage: `https://${pubConfig.vcsHost}/${pubConfig.vcsRepo}`,
    description:
      descMatch?.[1] ?? "Polyglot Code De-Duplication Meister & Autonomous Refactoring Engine",
    license: licenseMatch?.[1] ?? "MIT",
    binaries: ["cddm", "cddm-mcp", "cddm-lsp"],
    releaseBaseUrl,
    targets,
  };

  const written: string[] = [];

  if (targets.macosArm64 || targets.macosX64 || targets.linuxArm64 || targets.linuxX64) {
    const brewPath = join(workspaceRoot, "packaging", "homebrew", "cddm.rb");
    mkdirSync(join(workspaceRoot, "packaging", "homebrew"), { recursive: true });
    writeFileSync(brewPath, renderHomebrewFormula(spec), "utf-8");
    written.push(brewPath);
  }

  if (targets.windowsX64 || targets.windowsArm64) {
    const scoopPath = join(workspaceRoot, "packaging", "scoop", "cddm.json");
    mkdirSync(join(workspaceRoot, "packaging", "scoop"), { recursive: true });
    writeFileSync(scoopPath, renderScoopManifest(spec), "utf-8");
    written.push(scoopPath);

    const winget = renderWingetManifests(spec);
    const wingetDir = join(workspaceRoot, "packaging", "winget");
    mkdirSync(wingetDir, { recursive: true });
    const verPath = join(wingetDir, winget.versionFile);
    const instPath = join(wingetDir, winget.installerFile);
    const locPath = join(wingetDir, winget.localeFile);
    writeFileSync(verPath, winget.versionContent, "utf-8");
    writeFileSync(instPath, winget.installerContent, "utf-8");
    writeFileSync(locPath, winget.localeContent, "utf-8");
    written.push(verPath, instPath, locPath);
  }

  return written;
}

export async function generateShellCompletions(workspaceRoot: string): Promise<void> {
  const completionsDir = resolve(workspaceRoot, "packaging/completions");
  if (!existsSync(completionsDir)) {
    mkdirSync(completionsDir, { recursive: true });
  }

  const candidates = [
    resolve(workspaceRoot, "target/release/cddm"),
    resolve(workspaceRoot, "target/release/cddm.exe"),
    resolve(workspaceRoot, "target/debug/cddm"),
    resolve(workspaceRoot, "target/debug/cddm.exe"),
  ];
  const binaryPath = candidates.find((p) => existsSync(p));

  for (const [shell, relPath] of Object.entries(completionFiles)) {
    const fullPath = resolve(workspaceRoot, relPath);
    let output = "";
    if (binaryPath) {
      const proc = Bun.spawnSync([binaryPath, "completions", shell]);
      if (proc.exitCode === 0) {
        output = proc.stdout.toString();
      }
    }
    if (!output) {
      const proc = Bun.spawnSync(
        ["cargo", "run", "-q", "-p", "cddm-cli", "--", "completions", shell],
        {
          cwd: workspaceRoot,
        },
      );
      if (proc.exitCode === 0) {
        output = proc.stdout.toString();
      }
    }
    if (output) {
      writeFileSync(fullPath, output, "utf-8");
      console.log(`\x1b[32m[OK]\x1b[0m Generated shell completions for ${shell} -> ${relPath}`);
    }
  }
}

export function validatePackagingWorkspace(workspaceRoot: string = rootDir): string[] {
  const errors: string[] = [];
  const pubConfig = resolvePublisherConfig(workspaceRoot);
  const pkgId = `${pubConfig.publisherId}.cddm`;

  const legacySingletonPath = resolve(workspaceRoot, "packaging/winget/cddm.yaml");
  if (existsSync(legacySingletonPath)) {
    errors.push(
      "Legacy singleton WinGet manifest packaging/winget/cddm.yaml must be replaced by v1.9.0 multi-file manifests.",
    );
  }

  for (const relPath of getRequiredPackagingFiles(workspaceRoot)) {
    const fullPath = resolve(workspaceRoot, relPath);
    if (!existsSync(fullPath)) {
      errors.push(`Missing required packaging file: ${relPath}`);
      continue;
    }
    const content = readFileSync(fullPath, "utf-8");
    if (content.trim().length === 0) {
      errors.push(`Packaging file is empty: ${relPath}`);
    }
  }

  const brewPath = resolve(workspaceRoot, "packaging/homebrew/cddm.rb");
  const brewContent = existsSync(brewPath) ? readFileSync(brewPath, "utf-8") : "";
  if (brewContent) {
    if (
      !brewContent.includes("class Cddm < Formula") ||
      !brewContent.includes('bin.install "cddm"')
    ) {
      errors.push("Homebrew formula missing standard class or bin.install directive");
    }
    if (!brewContent.includes('generate_completions_from_executable(bin/"cddm", "completions")')) {
      errors.push(
        'Homebrew formula must use native generate_completions_from_executable(bin/"cddm", "completions")',
      );
    }
    if (brewContent.includes('File.exist?("packaging/completions')) {
      errors.push("Homebrew formula still contains legacy packaging/completions file check");
    }
    errors.push(
      ...findZeroSha256Violations([{ label: "packaging/homebrew/cddm.rb", content: brewContent }]),
    );
  }

  const scoopPath = resolve(workspaceRoot, "packaging/scoop/cddm.json");
  const scoopRaw = existsSync(scoopPath) ? readFileSync(scoopPath, "utf-8") : "";
  if (scoopRaw) {
    try {
      const scoopJson = JSON.parse(scoopRaw) as {
        bin?: string[];
        architecture?: Record<string, { url?: string; hash?: string }>;
        autoupdate?: { architecture?: Record<string, { hash?: { url?: string } }> };
      };
      if (!scoopJson.bin || !scoopJson.architecture?.["64bit"]) {
        errors.push("Scoop manifest missing bin or architecture.64bit definition");
      }
      if (!scoopJson.autoupdate?.architecture?.["64bit"]?.hash?.url) {
        errors.push("Scoop manifest missing autoupdate.architecture.64bit.hash.url");
      }
      errors.push(
        ...findZeroSha256Violations([{ label: "packaging/scoop/cddm.json", content: scoopRaw }]),
      );
    } catch (err) {
      errors.push(`Scoop manifest JSON parse error: ${String(err)}`);
    }
  }

  const verFile = resolve(workspaceRoot, `packaging/winget/${pkgId}.yaml`);
  const instFile = resolve(workspaceRoot, `packaging/winget/${pkgId}.installer.yaml`);
  const locFile = resolve(workspaceRoot, `packaging/winget/${pkgId}.locale.en-US.yaml`);
  for (const [label, filePath, checks] of [
    [
      `packaging/winget/${pkgId}.yaml`,
      verFile,
      [`PackageIdentifier: ${pkgId}`, "ManifestType: version", "ManifestVersion: 1.9.0"],
    ],
    [
      `packaging/winget/${pkgId}.installer.yaml`,
      instFile,
      [
        `PackageIdentifier: ${pkgId}`,
        "NestedInstallerType: portable",
        "RelativeFilePath: cddm.exe",
        "RelativeFilePath: cddm-mcp.exe",
        "RelativeFilePath: cddm-lsp.exe",
        "ManifestType: installer",
        "ManifestVersion: 1.9.0",
      ],
    ],
    [
      `packaging/winget/${pkgId}.locale.en-US.yaml`,
      locFile,
      [`PackageIdentifier: ${pkgId}`, "ManifestType: defaultLocale", "ManifestVersion: 1.9.0"],
    ],
  ] as const) {
    if (!existsSync(filePath)) continue;
    const content = readFileSync(filePath, "utf-8");
    for (const marker of checks) {
      if (!content.includes(marker)) {
        errors.push(`${label} missing required WinGet v1.9.0 field: ${marker}`);
      }
    }
    errors.push(...findZeroSha256Violations([{ label, content }]));
  }

  const shPath = resolve(workspaceRoot, "packaging/install.sh");
  if (existsSync(shPath)) {
    const shContent = readFileSync(shPath, "utf-8");
    if (shContent.includes("v1.9.0")) {
      errors.push("packaging/install.sh must not silently downgrade to v1.9.0 on API failure");
    }
    if (!shContent.includes("SHA256SUMS.txt")) {
      errors.push("packaging/install.sh must verify SHA256SUMS.txt before extracting archives");
    }
  }

  const ps1Path = resolve(workspaceRoot, "packaging/install.ps1");
  if (existsSync(ps1Path)) {
    const ps1Content = readFileSync(ps1Path, "utf-8");
    if (ps1Content.includes("v1.9.0")) {
      errors.push("packaging/install.ps1 must not silently downgrade to v1.9.0 on API failure");
    }
    if (!ps1Content.includes("SHA256SUMS.txt") || !ps1Content.includes("Get-FileHash")) {
      errors.push(
        "packaging/install.ps1 must verify SHA256SUMS.txt with Get-FileHash before extracting",
      );
    }
    if (!ps1Content.includes("GetRandomFileName")) {
      errors.push("packaging/install.ps1 must extract inside an isolated temporary directory");
    }
  }

  const checks: Array<[string, string, string]> = [
    [completionFiles.bash, "_cddm", "Bash completion script missing _cddm function signature"],
    [completionFiles.zsh, "#compdef cddm", "Zsh completion script missing #compdef header"],
    [
      completionFiles.fish,
      "complete -c cddm",
      "Fish completion script missing complete -c cddm directive",
    ],
    [
      completionFiles.powershell,
      "Register-ArgumentCompleter",
      "PowerShell completion script missing Register-ArgumentCompleter",
    ],
    [
      completionFiles.elvish,
      "edit:completion:arg-completer[cddm]",
      "Elvish completion script missing arg-completer[cddm]",
    ],
  ];
  for (const [relPath, token, message] of checks) {
    const p = resolve(workspaceRoot, relPath);
    if (existsSync(p) && !readFileSync(p, "utf-8").includes(token)) {
      errors.push(message);
    }
  }

  return errors;
}

async function main() {
  const args = process.argv.slice(2);
  const shouldGenerate = args.includes("--generate");
  const distDirIdx = args.indexOf("--dist-dir");
  const distDir = distDirIdx !== -1 && args[distDirIdx + 1] ? args[distDirIdx + 1] : undefined;

  if (distDir) {
    const sumsPath = resolve(rootDir, distDir, "SHA256SUMS.txt");
    if (!existsSync(sumsPath)) {
      console.error(`\x1b[31m[FAIL] Missing SHA256SUMS.txt at ${sumsPath}\x1b[0m`);
      process.exit(1);
    }
    const updated = syncManifestsFromChecksums(rootDir, sumsPath);
    console.log(
      `\x1b[32m[OK]\x1b[0m Dynamically generated ${updated.length} target-pruned packaging manifest(s) from ${sumsPath}`,
    );
  }

  const missingCompletions = Object.values(completionFiles).some(
    (relPath) => !existsSync(resolve(rootDir, relPath)),
  );

  if (shouldGenerate || missingCompletions) {
    console.log("\x1b[36m--> Generating shell completion files via cddm completions...\x1b[0m");
    await generateShellCompletions(rootDir);
  }

  console.log("\x1b[36m--> Verifying ecosystem distribution manifests and packaging...\x1b[0m");
  const errors = validatePackagingWorkspace(rootDir);
  if (errors.length > 0) {
    for (const err of errors) {
      console.error(`\x1b[31m[FAIL] ${err}\x1b[0m`);
    }
    process.exit(1);
  }

  console.log(
    "\x1b[32m[SUCCESS] All ecosystem packaging manifests and shell completions validated successfully.\x1b[0m",
  );
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal packaging verification error:", err);
    process.exit(1);
  });
}
