import { describe, it, expect } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { ZERO_SHA256 } from "../lib/distribution-manifests";
import { resolvePublisherConfig } from "../lib/publisher-config";
import {
  completionFiles,
  requiredPackagingFiles,
  syncManifestsFromChecksums,
  validatePackagingWorkspace,
} from "../package-distribution";

describe("Ecosystem Distribution & Packaging Validator", () => {
  const rootDir = resolve(import.meta.dir, "../..");

  it("should have all required ecosystem distribution files present and non-empty", () => {
    for (const relPath of requiredPackagingFiles) {
      const fullPath = resolve(rootDir, relPath);
      expect(existsSync(fullPath)).toBe(true);
      const content = readFileSync(fullPath, "utf-8");
      expect(content.trim().length).toBeGreaterThan(0);
    }
    expect(existsSync(resolve(rootDir, "packaging/winget/cddm.yaml"))).toBe(false);
    expect(validatePackagingWorkspace(rootDir)).toEqual([]);
  });

  it("should validate Homebrew formula syntax, native completions, and zero placeholder targets", () => {
    const brewPath = resolve(rootDir, "packaging/homebrew/cddm.rb");
    const brewContent = readFileSync(brewPath, "utf-8");
    expect(brewContent).toContain("class Cddm < Formula");
    expect(brewContent).toContain('bin.install "cddm"');
    expect(brewContent).toContain('bin.install "cddm-mcp"');
    expect(brewContent).toContain('bin.install "cddm-lsp"');
    expect(brewContent).toContain(
      'generate_completions_from_executable(bin/"cddm", "completions")',
    );
    expect(brewContent).not.toContain('File.exist?("packaging/completions');
    expect(brewContent).not.toContain("on_macos");
    expect(brewContent).not.toContain(ZERO_SHA256);
  });

  it("should validate Scoop manifest JSON schema without unbuilt arm64 stubs", () => {
    const scoopPath = resolve(rootDir, "packaging/scoop/cddm.json");
    const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8"));
    expect(scoopJson.bin).toEqual(["cddm.exe", "cddm-mcp.exe", "cddm-lsp.exe"]);
    expect(scoopJson.architecture["64bit"]).toBeDefined();
    expect(scoopJson.architecture.arm64).toBeUndefined();
    expect(scoopJson.autoupdate?.architecture?.["64bit"]?.hash?.url).toBe(
      "$baseurl/SHA256SUMS.txt",
    );
  });

  it("should validate WinGet v1.9.0 multi-file YAML manifests with exact case parity", () => {
    const pubConfig = resolvePublisherConfig(rootDir);
    const pkgId = `${pubConfig.publisherId}.cddm`;
    const ver = readFileSync(resolve(rootDir, `packaging/winget/${pkgId}.yaml`), "utf-8");
    const inst = readFileSync(
      resolve(rootDir, `packaging/winget/${pkgId}.installer.yaml`),
      "utf-8",
    );
    const loc = readFileSync(
      resolve(rootDir, `packaging/winget/${pkgId}.locale.en-US.yaml`),
      "utf-8",
    );

    expect(ver).toContain(`PackageIdentifier: ${pkgId}`);
    expect(ver).toContain("ManifestType: version");
    expect(ver).toContain("ManifestVersion: 1.9.0");
    expect(loc).toContain(`PackageIdentifier: ${pkgId}`);
    expect(loc).toMatch(/^Publisher:\s+.+/m);
    expect(loc).toContain("ManifestType: defaultLocale");
    expect(inst).toContain(`PackageIdentifier: ${pkgId}`);
    expect(inst).toContain("NestedInstallerType: portable");
    expect(inst).toContain("RelativeFilePath: cddm.exe");
    expect(inst).toContain("RelativeFilePath: cddm-mcp.exe");
    expect(inst).toContain("RelativeFilePath: cddm-lsp.exe");
    expect(inst).not.toContain("Architecture: arm64");
    expect(inst).not.toContain(ZERO_SHA256);
  });

  it("should dynamically prune manifests strictly from SHA256SUMS.txt", () => {
    const tempDir = join(tmpdir(), `cddm-pkg-dynamic-${Date.now()}`);
    mkdirSync(tempDir, { recursive: true });
    try {
      writeFileSync(
        join(tempDir, ".env"),
        "GITEA_HOST=vcs.example.com\nGITEA_REPO=org/cddm\nPUBLISHER=org\nPUBLISHER_NAME=Org Dev\n",
      );
      writeFileSync(
        join(tempDir, "Cargo.toml"),
        '[workspace.package]\nversion = "4.10.2"\ndescription = "CDDM"\nlicense = "MIT"\n',
      );
      const linuxSha = "1".repeat(64);
      const winSha = "2".repeat(64);
      const sumsPath = join(tempDir, "SHA256SUMS.txt");
      writeFileSync(
        sumsPath,
        `${linuxSha}  cddm-v4.10.2-x86_64-unknown-linux-gnu.tar.gz\n${winSha}  cddm-v4.10.2-x86_64-pc-windows-msvc.zip\n`,
      );

      const written = syncManifestsFromChecksums(tempDir, sumsPath, "4.10.2");
      expect(written.length).toBe(5);
      expect(readFileSync(join(tempDir, "packaging", "homebrew", "cddm.rb"), "utf-8")).toContain(
        linuxSha,
      );
      expect(
        readFileSync(join(tempDir, "packaging", "winget", "org.cddm.installer.yaml"), "utf-8"),
      ).toContain(winSha.toUpperCase());
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  it("should validate install.sh and install.ps1 integrity verification and error handling", () => {
    const shPath = resolve(rootDir, "packaging/install.sh");
    const ps1Path = resolve(rootDir, "packaging/install.ps1");
    const shContent = readFileSync(shPath, "utf-8");
    const ps1Content = readFileSync(ps1Path, "utf-8");

    expect(shContent).toContain("cddm");
    expect(shContent).toContain("curl");
    expect(shContent).toContain("SHA256SUMS.txt");
    expect(shContent).toContain("pc-windows-msvc");
    expect(shContent).toContain("pc-windows-gnu");
    expect(shContent).not.toContain('RELEASE_TAG="v1.9.0"');

    expect(ps1Content).toContain("cddm");
    expect(ps1Content).toContain("Invoke-WebRequest");
    expect(ps1Content).toContain("SHA256SUMS.txt");
    expect(ps1Content).toContain("Get-FileHash");
    expect(ps1Content).toContain("GetRandomFileName");
    expect(ps1Content).toContain("pc-windows-msvc.zip");
    expect(ps1Content).toContain("pc-windows-gnu.zip");
    expect(ps1Content).not.toContain('$ReleaseTag = "v1.9.0"');
  });

  it("should validate generated shell completions signatures across all 5 shells", () => {
    const bash = readFileSync(resolve(rootDir, completionFiles.bash), "utf-8");
    expect(bash).toContain("_cddm");

    const zsh = readFileSync(resolve(rootDir, completionFiles.zsh), "utf-8");
    expect(zsh).toContain("#compdef cddm");

    const fish = readFileSync(resolve(rootDir, completionFiles.fish), "utf-8");
    expect(fish).toContain("complete -c cddm");

    const ps1 = readFileSync(resolve(rootDir, completionFiles.powershell), "utf-8");
    expect(ps1).toContain("Register-ArgumentCompleter");

    const elv = readFileSync(resolve(rootDir, completionFiles.elvish), "utf-8");
    expect(elv).toContain("edit:completion:arg-completer[cddm]");
  });
});
