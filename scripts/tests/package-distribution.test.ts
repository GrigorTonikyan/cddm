import { describe, it, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { completionFiles, requiredPackagingFiles } from "../package-distribution";

describe("Ecosystem Distribution & Packaging Validator", () => {
  const rootDir = resolve(import.meta.dir, "../..");

  it("should have all required ecosystem distribution files present and non-empty", () => {
    for (const relPath of requiredPackagingFiles) {
      const fullPath = resolve(rootDir, relPath);
      expect(existsSync(fullPath)).toBe(true);
      const content = readFileSync(fullPath, "utf-8");
      expect(content.trim().length).toBeGreaterThan(0);
    }
  });

  it("should validate Homebrew formula syntax and completion directives", () => {
    const brewPath = resolve(rootDir, "packaging/homebrew/cddm.rb");
    const brewContent = readFileSync(brewPath, "utf-8");
    expect(brewContent).toContain("class Cddm < Formula");
    expect(brewContent).toContain('bin.install "cddm"');
    expect(brewContent).toContain("bash_completion.install");
    expect(brewContent).toContain("zsh_completion.install");
    expect(brewContent).toContain("fish_completion.install");
  });

  it("should validate Scoop manifest JSON schema", () => {
    const scoopPath = resolve(rootDir, "packaging/scoop/cddm.json");
    const scoopJson = JSON.parse(readFileSync(scoopPath, "utf-8"));
    expect(scoopJson.bin).toBeDefined();
    expect(scoopJson.architecture).toBeDefined();
    expect(scoopJson.architecture["64bit"]).toBeDefined();
  });

  it("should validate Winget manifest YAML structure", () => {
    const wingetPath = resolve(rootDir, "packaging/winget/GrigorTonikyan.cddm.yaml");
    const wingetContent = readFileSync(wingetPath, "utf-8");
    expect(wingetContent).toContain("PackageIdentifier: GrigorTonikyan.cddm");
    expect(wingetContent).toContain("NestedInstallerType: portable");
  });

  it("should validate install.sh and install.ps1 scripts", () => {
    const shPath = resolve(rootDir, "packaging/install.sh");
    const ps1Path = resolve(rootDir, "packaging/install.ps1");
    const shContent = readFileSync(shPath, "utf-8");
    const ps1Content = readFileSync(ps1Path, "utf-8");

    expect(shContent).toContain("cddm");
    expect(shContent).toContain("curl");
    expect(ps1Content).toContain("cddm");
    expect(ps1Content).toContain("Invoke-WebRequest");
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
