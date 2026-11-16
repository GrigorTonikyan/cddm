#!/usr/bin/env bun
/**
 * CDDM Windows & Cross-Platform Distribution Channel Verifier
 *
 * Verifies local and remote distribution channels end-to-end:
 * 1. WinGet v1.9.0 multi-file manifests (native MSVC zip, PortableCommandAlias for cddm, cddm-mcp, cddm-lsp)
 * 2. Scoop bucket manifest (native MSVC zip, bin shims, SHA256SUMS.txt autoupdate)
 * 3. Homebrew formula (multi-binary install, native completions)
 * 4. Turnkey Windows PowerShell (install.ps1) & Shell (install.sh) installers
 * 5. NPM universal wrapper & platform packages (@cddm/win32-x64, @cddm/linux-x64)
 * 6. Turnkey MCP client documentation (docs/MCP.md, README.md)
 * 7. Optional live remote Gitea release, package repos, Cargo & NPM registry verification (--remote)
 */

import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { verifyCargoChannels } from "./lib/cargo-registry";
import { parseSha256SumsValidated } from "./lib/distribution-manifests";
import { GITEA_TOKEN } from "./lib/gitea-client";
import { isNpmVersionPublished, verifyNpmChannels } from "./lib/npm-registry";
import { type PublisherConfig, resolvePublisherConfig } from "./lib/publisher-config";
import { getCurrentVersion } from "./version";

export interface ChannelVerificationResult {
  passed: boolean;
  checkedVersion: string;
  violations: string[];
}

export function verifyLocalDistributionChannels(
  workspaceRoot: string = process.cwd(),
  expectedVersion?: string,
  configOverride?: PublisherConfig,
): ChannelVerificationResult {
  const violations: string[] = [];
  const config = configOverride ?? resolvePublisherConfig(workspaceRoot);
  const version = expectedVersion ?? getCurrentVersion(workspaceRoot);
  const pkgId = `${config.publisherId}.cddm`;

  // 1. Verify WinGet v1.9.0 multi-file manifests
  const wingetDir = join(workspaceRoot, "packaging", "winget");
  const wgVersionPath = join(wingetDir, `${pkgId}.yaml`);
  const wgInstallerPath = join(wingetDir, `${pkgId}.installer.yaml`);
  const wgLocalePath = join(wingetDir, `${pkgId}.locale.en-US.yaml`);

  for (const [label, filePath] of [
    ["WinGet version manifest", wgVersionPath],
    ["WinGet installer manifest", wgInstallerPath],
    ["WinGet locale manifest", wgLocalePath],
  ] as const) {
    if (!existsSync(filePath)) {
      violations.push(`Missing ${label}: ${filePath}`);
    } else {
      const content = readFileSync(filePath, "utf-8");
      if (!content.includes("ManifestVersion: 1.9.0")) {
        violations.push(`${label} must target WinGet Schema 1.9.0.`);
      }
      if (!content.includes(`PackageIdentifier: ${pkgId}`)) {
        violations.push(`${label} must declare PackageIdentifier: ${pkgId}.`);
      }
      if (!content.includes(`PackageVersion: ${version}`)) {
        violations.push(`${label} version drift: expected PackageVersion: ${version}.`);
      }
    }
  }

  if (existsSync(wgInstallerPath)) {
    const installerYaml = readFileSync(wgInstallerPath, "utf-8");
    const expectedMsvcArchive = `cddm-v${version}-x86_64-pc-windows-msvc.zip`;
    if (!installerYaml.includes(expectedMsvcArchive)) {
      violations.push(
        `WinGet installer manifest must reference native MSVC archive ${expectedMsvcArchive}.`,
      );
    }
    for (const alias of ["cddm", "cddm-mcp", "cddm-lsp"]) {
      if (
        !installerYaml.includes(`RelativeFilePath: ${alias}.exe`) ||
        !installerYaml.includes(`PortableCommandAlias: ${alias}`)
      ) {
        violations.push(
          `WinGet installer manifest must expose ${alias}.exe with PortableCommandAlias: ${alias}.`,
        );
      }
    }
  }

  // 2. Verify Scoop manifest
  const scoopPath = join(workspaceRoot, "packaging", "scoop", "cddm.json");
  if (!existsSync(scoopPath)) {
    violations.push("Missing packaging/scoop/cddm.json.");
  } else {
    try {
      const scoop = JSON.parse(readFileSync(scoopPath, "utf-8")) as {
        version?: string;
        bin?: string[];
        architecture?: { "64bit"?: { url?: string; hash?: string } };
        autoupdate?: {
          architecture?: { "64bit"?: { url?: string; hash?: { url?: string } } };
        };
      };
      if (scoop.version !== version) {
        violations.push(`Scoop version mismatch: expected ${version}, found ${scoop.version}`);
      }
      const url64 = scoop.architecture?.["64bit"]?.url ?? "";
      if (!url64.includes(`cddm-v${version}-x86_64-pc-windows-msvc.zip`)) {
        violations.push("Scoop 64bit URL must point to native x86_64-pc-windows-msvc.zip.");
      }
      const bins = scoop.bin ?? [];
      for (const expectedBin of ["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"]) {
        if (!bins.includes(expectedBin)) {
          violations.push(`Scoop manifest bin array must include ${expectedBin}.`);
        }
      }
      if (scoop.autoupdate?.architecture?.["64bit"]?.hash?.url !== "$baseurl/SHA256SUMS.txt") {
        violations.push("Scoop autoupdate hash URL must use $baseurl/SHA256SUMS.txt.");
      }
    } catch (err) {
      violations.push(`Invalid JSON in packaging/scoop/cddm.json: ${String(err)}`);
    }
  }

  // 3. Verify PowerShell installer (packaging/install.ps1)
  const ps1Path = join(workspaceRoot, "packaging", "install.ps1");
  if (!existsSync(ps1Path)) {
    violations.push("Missing packaging/install.ps1.");
  } else {
    const ps1 = readFileSync(ps1Path, "utf-8");
    if (ps1.includes("v1.9.0")) {
      violations.push("packaging/install.ps1 must not contain legacy v1.9.0 fallback.");
    }
    if (!ps1.includes("-pc-windows-msvc.zip")) {
      violations.push("packaging/install.ps1 must prefer -pc-windows-msvc.zip archives.");
    }
    if (!ps1.includes("SHA256SUMS.txt") || !ps1.includes("Get-FileHash")) {
      violations.push("packaging/install.ps1 must verify SHA-256 digests against SHA256SUMS.txt.");
    }
    if (!ps1.includes("SetEnvironmentVariable") || !ps1.includes("cddm-mcp.exe")) {
      violations.push(
        "packaging/install.ps1 must verify cddm-mcp.exe extraction and configure User PATH.",
      );
    }
  }

  // 4. Verify Turnkey MCP Client Documentation
  for (const docRel of ["docs/MCP.md", "README.md"]) {
    const docPath = join(workspaceRoot, docRel);
    if (!existsSync(docPath)) {
      violations.push(`Missing ${docRel}.`);
    } else {
      const content = readFileSync(docPath, "utf-8");
      if (!content.includes('"mcpServers"') || !content.includes('"cddm-mcp"')) {
        violations.push(`${docRel} must document turnkey mcpServers configuration for cddm-mcp.`);
      }
    }
  }

  return {
    passed: violations.length === 0,
    checkedVersion: version,
    violations,
  };
}

export async function runPowerShellInstallerExecution(options: {
  workspaceRoot?: string;
  baseUrl: string;
  repo?: string;
  version?: string;
  installDir?: string;
}): Promise<{ exitCode: number; stdout: string; stderr: string; installedBinaries: string[] }> {
  const root = options.workspaceRoot ?? process.cwd();
  const scriptPath = join(root, "packaging", "install.ps1");
  const tempInstallDir =
    options.installDir ?? mkdtempSync(join(tmpdir(), "cddm-win-install-test-"));

  try {
    const shell = Bun.which("pwsh") || Bun.which("powershell") || "pwsh";
    const proc = Bun.spawn(
      [shell, "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", scriptPath],
      {
        cwd: root,
        env: {
          ...process.env,
          CDDM_BASE_URL: options.baseUrl,
          CDDM_REPO: options.repo ?? "gt-dev/cddm",
          CDDM_VERSION: options.version ?? "latest",
          CDDM_INSTALL_DIR: tempInstallDir,
          CDDM_NO_MODIFY_PATH: "1",
        },
        stdout: "pipe",
        stderr: "pipe",
      },
    );

    const [exitCode, stdout, stderr] = await Promise.all([
      proc.exited,
      new Response(proc.stdout).text(),
      new Response(proc.stderr).text(),
    ]);

    const installedBinaries = ["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"].filter((bin) =>
      existsSync(join(tempInstallDir, bin)),
    );

    return {
      exitCode,
      stdout,
      stderr,
      installedBinaries,
    };
  } finally {
    if (!options.installDir) {
      rmSync(tempInstallDir, { recursive: true, force: true });
    }
  }
}

export function validateWingetManifestsCli(manifestDir: string): {
  validated: boolean;
  skipped: boolean;
  output: string;
} {
  const wingetBin = Bun.which("winget");
  if (!wingetBin) {
    return { validated: false, skipped: true, output: "winget CLI not installed on this host." };
  }
  const proc = Bun.spawnSync([wingetBin, "validate", "--manifest", manifestDir], {
    stdout: "pipe",
    stderr: "pipe",
  });
  const output = `${proc.stdout.toString()}\n${proc.stderr.toString()}`.trim();
  return { validated: proc.exitCode === 0, skipped: false, output };
}

export async function verifyRemoteDistributionChannels(options: {
  workspaceRoot?: string;
  version: string;
  token?: string;
  liveInstall?: boolean;
  skipWingetCli?: boolean;
  fetchFn?: typeof fetch;
}): Promise<ChannelVerificationResult> {
  const root = options.workspaceRoot ?? process.cwd();
  const config = resolvePublisherConfig(root);
  const cleanVer = options.version.replace(/^v/, "");
  const tag = `v${cleanVer}`;
  const fetchFn = options.fetchFn ?? fetch;
  const violations: string[] = [];
  const headers: Record<string, string> = options.token
    ? { Authorization: `token ${options.token}` }
    : {};

  // 1. Verify SHA256SUMS.txt & Windows MSVC archive on Gitea Release
  const sumsUrl = `https://${config.vcsHost}/${config.vcsRepo}/releases/download/${tag}/SHA256SUMS.txt`;
  const sumsRes = await fetchFn(sumsUrl, { headers });
  if (!sumsRes.ok) {
    violations.push(`Remote SHA256SUMS.txt unreachable (${sumsRes.status}): ${sumsUrl}`);
  } else {
    const sumsMap = parseSha256SumsValidated(await sumsRes.text());
    const winZip = `cddm-${tag}-x86_64-pc-windows-msvc.zip`;
    if (!sumsMap.has(winZip)) {
      violations.push(`Remote SHA256SUMS.txt missing Windows MSVC archive entry: ${winZip}`);
    }
  }

  // 2. Verify central package repositories (WinGet v1.9.0 3-file manifests, Scoop, Homebrew)
  const firstLetter = config.publisherId.charAt(0).toLowerCase();
  const wgBase = `https://${config.vcsHost}/api/v1/repos/${config.wingetRepo}/contents/manifests/${firstLetter}/${config.publisherId}/cddm/${cleanVer}`;
  const wgFiles = [
    `${config.publisherId}.cddm.yaml`,
    `${config.publisherId}.cddm.installer.yaml`,
    `${config.publisherId}.cddm.locale.en-US.yaml`,
  ];

  const tempWgDir = mkdtempSync(join(tmpdir(), "cddm-winget-validate-"));
  let fetchedAllWgContent = true;
  try {
    for (const fileName of wgFiles) {
      const res = await fetchFn(`${wgBase}/${fileName}`, { headers });
      if (!res.ok) {
        violations.push(`WinGet manifest missing on remote (${res.status}): ${wgBase}/${fileName}`);
        fetchedAllWgContent = false;
      } else {
        const json = (await res.json()) as { content?: string };
        if (typeof json.content === "string" && json.content.trim().length > 0) {
          const decoded = Buffer.from(json.content, "base64").toString("utf-8");
          writeFileSync(join(tempWgDir, fileName), decoded, "utf-8");
        } else {
          fetchedAllWgContent = false;
        }
      }
    }

    if (fetchedAllWgContent && !options.skipWingetCli && process.platform === "win32") {
      const wgRes = validateWingetManifestsCli(tempWgDir);
      if (!wgRes.skipped && !wgRes.validated) {
        violations.push(`winget validate failed for remote v${cleanVer}: ${wgRes.output}`);
      }
    }
  } finally {
    rmSync(tempWgDir, { recursive: true, force: true });
  }

  const repoChecks = [
    {
      name: "Scoop bucket manifest",
      url: `https://${config.vcsHost}/api/v1/repos/${config.scoopBucketRepo}/contents/bucket/cddm.json`,
    },
    {
      name: "Homebrew tap formula",
      url: `https://${config.vcsHost}/api/v1/repos/${config.homebrewTapRepo}/contents/Formula/cddm.rb`,
    },
  ];

  for (const check of repoChecks) {
    const res = await fetchFn(check.url, { headers });
    if (!res.ok) {
      violations.push(`${check.name} missing on remote (${res.status}): ${check.url}`);
    }
  }

  // 3. Verify Gitea Cargo & NPM Registries (including @cddm/win32-x64 & @cddm/linux-x64)
  const [cargo, npm, winNpm, linuxNpm] = await Promise.all([
    verifyCargoChannels({
      host: config.vcsHost,
      owner: config.publisherId,
      version: cleanVer,
      token: options.token,
      fetchFn,
    }),
    verifyNpmChannels({
      host: config.vcsHost,
      owner: config.publisherId,
      version: cleanVer,
      token: options.token,
      fetchFn,
    }),
    isNpmVersionPublished({
      registry: "gitea",
      host: config.vcsHost,
      owner: config.publisherId,
      packageName: "@cddm/win32-x64",
      version: cleanVer,
      token: options.token,
      fetchFn,
    }),
    isNpmVersionPublished({
      registry: "gitea",
      host: config.vcsHost,
      owner: config.publisherId,
      packageName: "@cddm/linux-x64",
      version: cleanVer,
      token: options.token,
      fetchFn,
    }),
  ]);

  if (!cargo.gitea) {
    violations.push(`Gitea Cargo registry missing cddm-core/lsp/mcp/cli v${cleanVer}.`);
  }
  if (!npm.gitea || !winNpm || !linuxNpm) {
    violations.push(
      `Gitea NPM registry missing cddm/@cddm/win32-x64/@cddm/linux-x64 v${cleanVer}.`,
    );
  }

  // 4. Optional live Windows PowerShell installer execution
  if (options.liveInstall && process.platform === "win32") {
    const installDir = mkdtempSync(join(tmpdir(), "cddm-win-live-exec-"));
    try {
      const execRes = await runPowerShellInstallerExecution({
        workspaceRoot: root,
        baseUrl: `https://${config.vcsHost}`,
        repo: config.vcsRepo,
        version: tag,
        installDir,
      });
      if (execRes.exitCode !== 0) {
        violations.push(`Live install.ps1 execution failed: ${execRes.stderr || execRes.stdout}`);
      } else if (
        !execRes.installedBinaries.includes("cddm.exe") ||
        !execRes.installedBinaries.includes("cddm-mcp.exe")
      ) {
        violations.push(
          `Live install.ps1 missing binaries: ${execRes.installedBinaries.join(", ")}`,
        );
      } else {
        const verProc = Bun.spawnSync([join(installDir, "cddm.exe"), "--version"], {
          stdout: "pipe",
          stderr: "pipe",
        });
        if (verProc.exitCode !== 0 || !verProc.stdout.toString().includes(cleanVer)) {
          violations.push(
            `Installed cddm.exe --version mismatch: ${verProc.stdout.toString()} ${verProc.stderr.toString()}`,
          );
        }
      }
    } finally {
      rmSync(installDir, { recursive: true, force: true });
    }
  }

  return {
    passed: violations.length === 0,
    checkedVersion: cleanVer,
    violations,
  };
}

async function main() {
  const args = process.argv.slice(2);
  const remote = args.includes("--remote");
  const liveInstall = args.includes("--live-install");
  const tagIdx = args.indexOf("--tag");
  const tagVer = tagIdx !== -1 && args[tagIdx + 1] ? args[tagIdx + 1]! : undefined;

  console.log("--> Verifying local Windows & cross-platform distribution channels...");
  const localRes = verifyLocalDistributionChannels(resolve(import.meta.dir, ".."));
  if (!localRes.passed) {
    for (const v of localRes.violations) console.error(`  - ${v}`);
    process.exit(1);
  }
  console.log(
    `\x1b[32m[PASS] Local distribution manifests & MCP docs verified (v${localRes.checkedVersion})!\x1b[0m`,
  );

  if (remote) {
    const targetVer = tagVer ?? localRes.checkedVersion;
    const token =
      process.env.GITEA_TOKEN || (GITEA_TOKEN !== "token-placeholder" ? GITEA_TOKEN : undefined);
    console.log(`--> Verifying live remote distribution channels for ${targetVer}...`);
    const remoteRes = await verifyRemoteDistributionChannels({
      version: targetVer,
      token,
      liveInstall,
    });
    if (!remoteRes.passed) {
      for (const v of remoteRes.violations) console.error(`  - ${v}`);
      process.exit(1);
    }
    console.log(
      `\x1b[32m[PASS] All remote Gitea release, WinGet, Scoop, Homebrew, Cargo, and NPM channels verified for v${remoteRes.checkedVersion}!\x1b[0m`,
    );
  }
}

if (import.meta.main) {
  main().catch((err) => {
    console.error("Distribution verification failed:", err);
    process.exit(1);
  });
}
