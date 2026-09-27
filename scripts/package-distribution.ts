#!/usr/bin/env bun
/**
 * Verifies ecosystem distribution packaging manifests, scripts, and shell completions.
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const rootDir = resolve(import.meta.dir, "..");

export const completionFiles = {
  bash: "packaging/completions/cddm.bash",
  fish: "packaging/completions/cddm.fish",
  zsh: "packaging/completions/_cddm",
  powershell: "packaging/completions/_cddm.ps1",
  elvish: "packaging/completions/cddm.elv",
} as const;

export const requiredPackagingFiles = [
  "packaging/homebrew/cddm.rb",
  "packaging/scoop/cddm.json",
  "packaging/winget/GrigorTonikyan.cddm.yaml",
  "packaging/install.sh",
  "packaging/install.ps1",
  "docs/JETBRAINS_SETUP.md",
  "editors/vscode/package.json",
  "editors/vscode/resources/cddm-icon.svg",
  ...Object.values(completionFiles),
];

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

async function main() {
  const args = process.argv.slice(2);
  const shouldGenerate = args.includes("--generate");

  const missingCompletions = Object.values(completionFiles).some(
    (relPath) => !existsSync(resolve(rootDir, relPath)),
  );

  if (shouldGenerate || missingCompletions) {
    console.log("\x1b[36m--> Generating shell completion files via cddm completions...\x1b[0m");
    await generateShellCompletions(rootDir);
  }

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
  if (
    !brewContent.includes("class Cddm < Formula") ||
    !brewContent.includes('bin.install "cddm"')
  ) {
    console.error(
      "\x1b[31m[FAIL] Homebrew formula missing standard class or bin.install directive\x1b[0m",
    );
    hasErrors = true;
  }
  if (!brewContent.includes("bash_completion.install")) {
    console.error(
      "\x1b[31m[FAIL] Homebrew formula missing shell completion installation directive\x1b[0m",
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

  // Validate Completion syntax signatures
  const bashCompletions = readFileSync(resolve(rootDir, completionFiles.bash), "utf-8");
  if (!bashCompletions.includes("_cddm")) {
    console.error("\x1b[31m[FAIL] Bash completion script missing _cddm function signature\x1b[0m");
    hasErrors = true;
  }

  const zshCompletions = readFileSync(resolve(rootDir, completionFiles.zsh), "utf-8");
  if (!zshCompletions.includes("#compdef cddm")) {
    console.error("\x1b[31m[FAIL] Zsh completion script missing #compdef header\x1b[0m");
    hasErrors = true;
  }

  const fishCompletions = readFileSync(resolve(rootDir, completionFiles.fish), "utf-8");
  if (!fishCompletions.includes("complete -c cddm")) {
    console.error(
      "\x1b[31m[FAIL] Fish completion script missing complete -c cddm directive\x1b[0m",
    );
    hasErrors = true;
  }

  const ps1Completions = readFileSync(resolve(rootDir, completionFiles.powershell), "utf-8");
  if (!ps1Completions.includes("Register-ArgumentCompleter")) {
    console.error(
      "\x1b[31m[FAIL] PowerShell completion script missing Register-ArgumentCompleter\x1b[0m",
    );
    hasErrors = true;
  }

  const elvCompletions = readFileSync(resolve(rootDir, completionFiles.elvish), "utf-8");
  if (!elvCompletions.includes("edit:completion:arg-completer[cddm]")) {
    console.error("\x1b[31m[FAIL] Elvish completion script missing arg-completer[cddm]\x1b[0m");
    hasErrors = true;
  }

  if (hasErrors) {
    process.exit(1);
  } else {
    console.log(
      "\x1b[32m[SUCCESS] All ecosystem packaging manifests and shell completions validated successfully.\x1b[0m",
    );
  }
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Fatal packaging verification error:", err);
    process.exit(1);
  });
}
