import { describe, expect, it } from "bun:test";
import {
  classifyTargetsFromChecksums,
  collectTargetAssets,
  findZeroSha256Violations,
  isValidSha256Digest,
  parseSha256SumsValidated,
  renderHomebrewFormula,
  renderScoopManifest,
  renderWingetManifests,
  verifyDistributionUrls,
  ZERO_SHA256,
  type DistributionManifestSpec,
} from "./distribution-manifests";

const LINUX_HASH = "a".repeat(64);
const WIN_HASH = "b".repeat(64);

function createSampleSpec(
  overrides: Partial<DistributionManifestSpec> = {},
): DistributionManifestSpec {
  const checksums = new Map<string, string>([
    [`cddm-v4.10.2-x86_64-unknown-linux-gnu.tar.gz`, LINUX_HASH],
    [`cddm-v4.10.2-x86_64-pc-windows-msvc.zip`, WIN_HASH],
  ]);
  const baseUrl = "https://vcs.example.com/org/cddm/releases/download/v4.10.2";

  return {
    packageName: "cddm",
    version: "4.10.2",
    publisherId: "org",
    publisherName: "Org Dev",
    homepage: "https://vcs.example.com/org/cddm",
    description: "Polyglot Code De-Duplication Meister",
    license: "MIT",
    binaries: ["cddm", "cddm-mcp", "cddm-lsp"],
    releaseBaseUrl: baseUrl,
    targets: classifyTargetsFromChecksums(checksums, baseUrl),
    ...overrides,
  };
}

describe("distribution-manifests", () => {
  it("validates SHA-256 digests and rejects zeroed or malformed values", () => {
    expect(isValidSha256Digest(LINUX_HASH)).toBe(true);
    expect(isValidSha256Digest(ZERO_SHA256)).toBe(false);
    expect(isValidSha256Digest("0000")).toBe(false);
    expect(isValidSha256Digest("z".repeat(64))).toBe(false);

    expect(() =>
      parseSha256SumsValidated(`${ZERO_SHA256}  cddm-v4.10.2-x86_64-pc-windows-msvc.zip\n`),
    ).toThrow("zeroed");

    const parsed = parseSha256SumsValidated(
      `${LINUX_HASH}  cddm-v4.10.2-x86_64-unknown-linux-gnu.tar.gz\n${WIN_HASH} *./cddm-v4.10.2-x86_64-pc-windows-msvc.zip\n`,
    );
    expect(parsed.get("cddm-v4.10.2-x86_64-unknown-linux-gnu.tar.gz")).toBe(LINUX_HASH);
    expect(parsed.get("cddm-v4.10.2-x86_64-pc-windows-msvc.zip")).toBe(WIN_HASH);
  });

  it("prunes unbuilt architectures from Homebrew formula and uses native completions", () => {
    const spec = createSampleSpec();
    const rb = renderHomebrewFormula(spec);

    expect(rb).toContain("class Cddm < Formula");
    expect(rb).toContain("on_linux do");
    expect(rb).toContain("on_intel do");
    expect(rb).toContain(LINUX_HASH);
    expect(rb).toContain('bin.install "cddm"');
    expect(rb).toContain('bin.install "cddm-mcp"');
    expect(rb).toContain('bin.install "cddm-lsp"');
    expect(rb).toContain('generate_completions_from_executable(bin/"cddm", "completions")');

    // Must not emit unbuilt macOS or ARM64 targets or dead File.exist? checks
    expect(rb).not.toContain("on_macos");
    expect(rb).not.toContain("on_arm");
    expect(rb).not.toContain("apple-darwin");
    expect(rb).not.toContain("aarch64");
    expect(rb).not.toContain(ZERO_SHA256);
    expect(rb).not.toContain('File.exist?("packaging/completions');
  });

  it("prunes unbuilt arm64 from Scoop manifest and sets SHA256SUMS autoupdate", () => {
    const spec = createSampleSpec();
    const scoopRaw = renderScoopManifest(spec);
    const scoop = JSON.parse(scoopRaw) as {
      architecture: Record<string, { url: string; hash: string }>;
      bin: string[];
      autoupdate: { architecture: Record<string, { url: string; hash: { url: string } }> };
    };

    expect(Object.keys(scoop.architecture)).toEqual(["64bit"]);
    expect(scoop.architecture["64bit"]?.hash).toBe(WIN_HASH);
    expect(scoop.bin).toEqual(["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"]);
    expect(scoop.autoupdate.architecture.arm64).toBeUndefined();
    expect(scoop.autoupdate.architecture["64bit"]?.hash.url).toBe("$baseurl/SHA256SUMS.txt");
  });

  it("generates WinGet v1.9.0 multi-file manifests with strict case parity and zero unbuilt arm64", () => {
    const spec = createSampleSpec();
    const wg = renderWingetManifests(spec);

    expect(wg.manifestDir).toBe("manifests/o/org/cddm/4.10.2");
    expect(wg.packageIdentifier).toBe("org.cddm");
    expect(wg.versionFile).toBe("org.cddm.yaml");
    expect(wg.installerFile).toBe("org.cddm.installer.yaml");
    expect(wg.localeFile).toBe("org.cddm.locale.en-US.yaml");

    expect(wg.versionContent).toContain("ManifestType: version");
    expect(wg.versionContent).toContain("ManifestVersion: 1.9.0");
    expect(wg.localeContent).toContain("ManifestType: defaultLocale");
    expect(wg.localeContent).toContain("Publisher: Org Dev");
    expect(wg.installerContent).toContain("ManifestType: installer");
    expect(wg.installerContent).toContain("NestedInstallerType: portable");
    expect(wg.installerContent).toContain("RelativeFilePath: cddm.exe");
    expect(wg.installerContent).toContain("RelativeFilePath: cddm-mcp.exe");
    expect(wg.installerContent).toContain("RelativeFilePath: cddm-lsp.exe");
    expect(wg.installerContent).toContain(`InstallerSha256: ${WIN_HASH.toUpperCase()}`);
    expect(wg.installerContent).not.toContain("Architecture: arm64");
    expect(wg.installerContent).not.toContain(ZERO_SHA256);
  });

  it("enforces pre-flight HTTP 200 reachability and zero-hash blocking", async () => {
    const spec = createSampleSpec();
    const assets = collectTargetAssets(spec.targets);

    await expect(
      verifyDistributionUrls(assets, {
        fetchFn: (async () => new Response(null, { status: 200 })) as unknown as typeof fetch,
      }),
    ).resolves.toBeUndefined();

    await expect(
      verifyDistributionUrls(assets, {
        fetchFn: (async () => new Response(null, { status: 404 })) as unknown as typeof fetch,
      }),
    ).rejects.toThrow("expected HTTP 200 OK, got HTTP 404");

    await expect(
      verifyDistributionUrls([
        {
          filename: "broken.zip",
          sha256: ZERO_SHA256,
          url: "https://vcs.example.com/broken.zip",
        },
      ]),
    ).rejects.toThrow("invalid or zeroed SHA-256 digest");

    expect(findZeroSha256Violations([{ label: "test.rb", content: ZERO_SHA256 }])).toHaveLength(1);
  });
});
