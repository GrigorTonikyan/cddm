import { basename } from "node:path";

export const ZERO_SHA256 = "0".repeat(64);
export const PENDING_SHA256_MARKER = "PENDING_RELEASE_SHA256";
export const WINGET_SCHEMA_VERSION = "1.9.0";

export interface DistributionTargetAsset {
  filename: string;
  sha256: string;
  url: string;
}

export interface DistributionTargets {
  macosArm64?: DistributionTargetAsset;
  macosX64?: DistributionTargetAsset;
  linuxArm64?: DistributionTargetAsset;
  linuxX64?: DistributionTargetAsset;
  windowsX64?: DistributionTargetAsset;
  windowsArm64?: DistributionTargetAsset;
}

export interface DistributionManifestSpec {
  packageName: string;
  version: string;
  publisherId: string;
  publisherName: string;
  homepage: string;
  description: string;
  license: string;
  binaries: string[];
  releaseBaseUrl: string;
  targets: DistributionTargets;
}

export interface WingetManifestSet {
  manifestDir: string;
  packageIdentifier: string;
  versionFile: string;
  versionContent: string;
  installerFile: string;
  installerContent: string;
  localeFile: string;
  localeContent: string;
}

/**
 * Validates that a SHA-256 digest is 64 hexadecimal characters and never all-zeroes.
 */
export function isValidSha256Digest(digest: string): boolean {
  const cleaned = digest.trim();
  return /^[0-9a-fA-F]{64}$/.test(cleaned) && !/^0+$/.test(cleaned);
}

/**
 * Parses SHA256SUMS.txt content and hard-fails if any entry has an invalid or 0{64} digest.
 */
export function parseSha256SumsValidated(content: string): Map<string, string> {
  const result = new Map<string, string>();
  const lines = content.split(/\r?\n/);

  for (const rawLine of lines) {
    const trimmed = rawLine.trim();
    if (!trimmed || trimmed.startsWith("#")) {
      continue;
    }

    const parts = /^(\S+)\s+\*?(.+)$/.exec(trimmed);
    if (!parts || !parts[1] || !parts[2]) {
      throw new Error(`Malformed SHA256SUMS.txt entry: "${trimmed}"`);
    }

    const digest = parts[1].trim().toLowerCase();
    const filename = basename(parts[2].trim().replace(/^\.\//, ""));

    if (!isValidSha256Digest(digest)) {
      throw new Error(
        `Invalid or zeroed SHA-256 digest ("${digest}") for "${filename}" in SHA256SUMS.txt`,
      );
    }

    result.set(filename, digest);
  }

  return result;
}

/**
 * Classifies only target binary archives present in SHA256SUMS.txt into platform/arch buckets.
 */
export function classifyTargetsFromChecksums(
  checksums: Map<string, string>,
  releaseBaseUrl: string,
): DistributionTargets {
  const targets: DistributionTargets = {};
  const normalizedBase = releaseBaseUrl.replace(/\/+$/, "");

  for (const [filename, sha256] of checksums.entries()) {
    if (!isValidSha256Digest(sha256)) {
      throw new Error(`Refusing invalid or zeroed SHA-256 digest for "${filename}": "${sha256}"`);
    }

    const lower = filename.toLowerCase();
    const isArchive =
      lower.endsWith(".tar.gz") || lower.endsWith(".zip") || lower.endsWith(".tar.xz");
    if (!isArchive) {
      continue;
    }

    const asset: DistributionTargetAsset = {
      filename,
      sha256: sha256.toLowerCase(),
      url: `${normalizedBase}/${filename}`,
    };

    const isArm = lower.includes("aarch64") || lower.includes("arm64");
    const isX64 = lower.includes("x86_64") || lower.includes("amd64") || lower.includes("x64");

    if (lower.includes("darwin") || lower.includes("macos") || lower.includes("apple")) {
      if (isArm) targets.macosArm64 = asset;
      else if (isX64) targets.macosX64 = asset;
    } else if (lower.includes("linux")) {
      if (isArm) targets.linuxArm64 = asset;
      else if (isX64) targets.linuxX64 = asset;
    } else if (lower.includes("windows")) {
      if (isArm) targets.windowsArm64 = asset;
      else if (isX64) targets.windowsX64 = asset;
    }
  }

  return targets;
}

/**
 * Flattens classified targets into a list of assets to validate.
 */
export function collectTargetAssets(targets: DistributionTargets): DistributionTargetAsset[] {
  const ordered: Array<DistributionTargetAsset | undefined> = [
    targets.macosArm64,
    targets.macosX64,
    targets.linuxArm64,
    targets.linuxX64,
    targets.windowsX64,
    targets.windowsArm64,
  ];
  return ordered.filter((item): item is DistributionTargetAsset => item !== undefined);
}

/**
 * Enforces the pre-flight gate: every asset must carry a non-zero 64-hex SHA-256
 * and its download URL must return HTTP 200 OK.
 */
export async function verifyDistributionUrls(
  assets: DistributionTargetAsset[],
  options: { token?: string; fetchFn?: typeof fetch } = {},
): Promise<void> {
  if (assets.length === 0) {
    throw new Error("No target binary archives found to verify before package dispatch.");
  }

  const fetchImpl = options.fetchFn ?? fetch;
  const headers: Record<string, string> = {};
  if (options.token) {
    headers.Authorization = options.token.includes(".")
      ? `Bearer ${options.token}`
      : `token ${options.token}`;
  }

  for (const asset of assets) {
    if (!isValidSha256Digest(asset.sha256)) {
      throw new Error(
        `Pre-flight security gate blocked "${asset.filename}": invalid or zeroed SHA-256 digest "${asset.sha256}".`,
      );
    }

    let response = await fetchImpl(asset.url, {
      method: "HEAD",
      redirect: "follow",
      headers,
    });

    if (response.status === 405) {
      response = await fetchImpl(asset.url, {
        method: "GET",
        redirect: "follow",
        headers,
      });
      await response.body?.cancel();
    }

    if (response.status !== 200) {
      throw new Error(
        `Pre-flight URL check failed for "${asset.url}": expected HTTP 200 OK, got HTTP ${response.status}.`,
      );
    }
  }
}

/**
 * Checks whether any raw manifest content contains a forbidden 64-zero placeholder hash.
 */
export function findZeroSha256Violations(
  manifests: Array<{ label: string; content: string }>,
): string[] {
  const violations: string[] = [];
  for (const item of manifests) {
    if (item.content.includes(ZERO_SHA256)) {
      violations.push(`${item.label} contains forbidden zeroed SHA-256 digest (${ZERO_SHA256})`);
    }
  }
  return violations;
}

function toRubyClassName(packageName: string): string {
  return packageName
    .split(/[-_]+/)
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

function templatizeBrewVersion(value: string, version: string): string {
  return value.replaceAll(`v${version}`, "v#{version}");
}

function renderBrewArchStanza(
  armAsset: DistributionTargetAsset | undefined,
  x64Asset: DistributionTargetAsset | undefined,
  version: string,
): string[] {
  const lines: string[] = [];
  if (armAsset) {
    lines.push("    on_arm do");
    lines.push(`      url "${templatizeBrewVersion(armAsset.url, version)}"`);
    lines.push(`      sha256 "${armAsset.sha256}"`);
    lines.push("    end");
  }
  if (x64Asset) {
    lines.push("    on_intel do");
    lines.push(`      url "${templatizeBrewVersion(x64Asset.url, version)}"`);
    lines.push(`      sha256 "${x64Asset.sha256}"`);
    lines.push("    end");
  }
  return lines;
}

/**
 * Dynamically renders a Homebrew Formula strictly for targets present in SHA256SUMS.txt,
 * using native `generate_completions_from_executable`.
 */
export function renderHomebrewFormula(spec: DistributionManifestSpec): string {
  const hasMac = Boolean(spec.targets.macosArm64 || spec.targets.macosX64);
  const hasLinux = Boolean(spec.targets.linuxArm64 || spec.targets.linuxX64);
  if (!hasMac && !hasLinux) {
    throw new Error("Cannot generate Homebrew formula without at least one macOS or Linux target.");
  }

  const className = toRubyClassName(spec.packageName);
  const primaryBin = spec.binaries[0] ?? spec.packageName;
  const extraBins = spec.binaries.slice(1);
  const lines: string[] = [
    `class ${className} < Formula`,
    `  desc "${spec.description}"`,
    `  homepage "${spec.homepage}"`,
    `  version "${spec.version}"`,
    `  license "${spec.license}"`,
    "",
  ];

  if (hasMac) {
    lines.push("  on_macos do");
    lines.push(
      ...renderBrewArchStanza(spec.targets.macosArm64, spec.targets.macosX64, spec.version),
    );
    lines.push("  end", "");
  }

  if (hasLinux) {
    lines.push("  on_linux do");
    lines.push(
      ...renderBrewArchStanza(spec.targets.linuxArm64, spec.targets.linuxX64, spec.version),
    );
    lines.push("  end", "");
  }

  lines.push("  def install");
  lines.push(`    bin.install "${primaryBin}"`);
  for (const extra of extraBins) {
    lines.push(`    bin.install "${extra}" if File.exist?("${extra}")`);
  }
  lines.push("");
  lines.push(`    generate_completions_from_executable(bin/"${primaryBin}", "completions")`);
  lines.push("  end", "");
  lines.push("  test do");
  lines.push(`    assert_match "${primaryBin}", shell_output("#{bin}/${primaryBin} --version")`);
  lines.push("  end");
  lines.push("end", "");

  return lines.join("\n");
}

/**
 * Dynamically renders a Scoop bucket manifest strictly for Windows targets present in SHA256SUMS.txt.
 */
export function renderScoopManifest(spec: DistributionManifestSpec): string {
  const architecture: Record<string, { url: string; hash: string }> = {};
  const autoupdateArch: Record<string, { url: string; hash: { url: string } }> = {};

  if (spec.targets.windowsX64) {
    architecture["64bit"] = {
      url: spec.targets.windowsX64.url,
      hash: spec.targets.windowsX64.sha256,
    };
    autoupdateArch["64bit"] = {
      url: spec.targets.windowsX64.url.replaceAll(`v${spec.version}`, "v$version"),
      hash: { url: "$baseurl/SHA256SUMS.txt" },
    };
  }

  if (spec.targets.windowsArm64) {
    architecture.arm64 = {
      url: spec.targets.windowsArm64.url,
      hash: spec.targets.windowsArm64.sha256,
    };
    autoupdateArch.arm64 = {
      url: spec.targets.windowsArm64.url.replaceAll(`v${spec.version}`, "v$version"),
      hash: { url: "$baseurl/SHA256SUMS.txt" },
    };
  }

  if (Object.keys(architecture).length === 0) {
    throw new Error("Cannot generate Scoop manifest without at least one Windows target archive.");
  }

  const manifest = {
    version: spec.version,
    description: spec.description,
    homepage: spec.homepage,
    license: spec.license,
    architecture,
    bin: spec.binaries.map((b) => (b.endsWith(".exe") ? b : `${b}.exe`)),
    checkver: "github",
    autoupdate: {
      architecture: autoupdateArch,
    },
  };

  return `${JSON.stringify(manifest, null, 2)}\n`;
}

/**
 * Dynamically renders WinGet v1.9.0 multi-file manifests with strict path/identifier case parity
 * and NestedInstallerFiles strictly for Windows architectures present in SHA256SUMS.txt.
 */
export function renderWingetManifests(spec: DistributionManifestSpec): WingetManifestSet {
  const installers: Array<{ arch: "x64" | "arm64"; asset: DistributionTargetAsset }> = [];
  if (spec.targets.windowsX64) {
    installers.push({ arch: "x64", asset: spec.targets.windowsX64 });
  }
  if (spec.targets.windowsArm64) {
    installers.push({ arch: "arm64", asset: spec.targets.windowsArm64 });
  }
  if (installers.length === 0) {
    throw new Error(
      "Cannot generate WinGet manifests without at least one Windows target archive.",
    );
  }

  const packageIdentifier = `${spec.publisherId}.${spec.packageName}`;
  const firstChar = spec.publisherId.charAt(0).toLowerCase();
  const manifestDir = `manifests/${firstChar}/${spec.publisherId}/${spec.packageName}/${spec.version}`;

  const versionContent = [
    `# yaml-language-server: $schema=https://aka.ms/winget-manifest.version.${WINGET_SCHEMA_VERSION}.schema.json`,
    `PackageIdentifier: ${packageIdentifier}`,
    `PackageVersion: ${spec.version}`,
    "DefaultLocale: en-US",
    "ManifestType: version",
    `ManifestVersion: ${WINGET_SCHEMA_VERSION}`,
    "",
  ].join("\n");

  const localeContent = [
    `# yaml-language-server: $schema=https://aka.ms/winget-manifest.defaultLocale.${WINGET_SCHEMA_VERSION}.schema.json`,
    `PackageIdentifier: ${packageIdentifier}`,
    `PackageVersion: ${spec.version}`,
    "PackageLocale: en-US",
    `Publisher: ${spec.publisherName}`,
    `PackageName: ${spec.packageName}`,
    `PackageUrl: ${spec.homepage}`,
    `License: ${spec.license}`,
    `ShortDescription: ${spec.description}`,
    `Moniker: ${spec.packageName}`,
    "ManifestType: defaultLocale",
    `ManifestVersion: ${WINGET_SCHEMA_VERSION}`,
    "",
  ].join("\n");

  const installerLines: string[] = [
    `# yaml-language-server: $schema=https://aka.ms/winget-manifest.installer.${WINGET_SCHEMA_VERSION}.schema.json`,
    `PackageIdentifier: ${packageIdentifier}`,
    `PackageVersion: ${spec.version}`,
    "InstallerType: zip",
    "NestedInstallerType: portable",
    "NestedInstallerFiles:",
  ];

  for (const binName of spec.binaries) {
    const cleanBin = binName.replace(/\.exe$/i, "");
    installerLines.push(`  - RelativeFilePath: ${cleanBin}.exe`);
    installerLines.push(`    PortableCommandAlias: ${cleanBin}`);
  }

  installerLines.push("Installers:");
  for (const item of installers) {
    installerLines.push(`  - Architecture: ${item.arch}`);
    installerLines.push(`    InstallerUrl: ${item.asset.url}`);
    installerLines.push(`    InstallerSha256: ${item.asset.sha256.toUpperCase()}`);
  }

  installerLines.push("ManifestType: installer", `ManifestVersion: ${WINGET_SCHEMA_VERSION}`, "");

  return {
    manifestDir,
    packageIdentifier,
    versionFile: `${packageIdentifier}.yaml`,
    versionContent,
    installerFile: `${packageIdentifier}.installer.yaml`,
    installerContent: installerLines.join("\n"),
    localeFile: `${packageIdentifier}.locale.en-US.yaml`,
    localeContent,
  };
}
