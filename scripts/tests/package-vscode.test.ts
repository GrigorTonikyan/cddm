import { describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { crc32, createZipArchive } from "../lib/zip-builder";
import {
  generateContentTypesXml,
  generateVsixManifest,
  packageVscodeExtension,
} from "../package-vscode";

describe("ZIP Archive & CRC32 Builder", () => {
  it("should calculate standard CRC-32 checksums correctly", () => {
    const testVector = Buffer.from("123456789", "utf-8");
    expect(crc32(testVector)).toBe(0xcbf43926);

    const empty = Buffer.from("", "utf-8");
    expect(crc32(empty)).toBe(0);
  });

  it("should create valid ZIP archive with standard signatures", () => {
    const zipBuffer = createZipArchive([
      { name: "hello.txt", data: "Hello World" },
      { name: "sub/test.json", data: '{"key": "value"}' },
    ]);

    expect(zipBuffer.length).toBeGreaterThan(100);

    // Check Local File Header signature (0x04034b50)
    expect(zipBuffer.readUInt32LE(0)).toBe(0x04034b50);

    // Check End of Central Directory signature (0x06054b50) at end
    const eocdOffset = zipBuffer.length - 22;
    expect(zipBuffer.readUInt32LE(eocdOffset)).toBe(0x06054b50);
  });
});

describe("VSIX Manifest & Content Types Generator", () => {
  it("should generate valid [Content_Types].xml descriptor", () => {
    const xml = generateContentTypesXml();
    expect(xml).toContain('Extension="vsixmanifest"');
    expect(xml).toContain('Extension="json"');
    expect(xml).toContain('Extension="js"');
    expect(xml).toContain('Extension="svg"');
  });

  it("should generate compliant extension.vsixmanifest XML", () => {
    const pkg = {
      name: "cddm",
      version: "1.7.0",
      publisher: "grigortonikyan",
      displayName: "CDDM — Code De-Duplication Meister",
      description: "Real-time polyglot code clone detection",
      categories: ["Linters", "Programming Languages"],
      keywords: ["ast", "cddm", "duplication"],
      engines: { vscode: "^1.96.0" },
    };

    const manifest = generateVsixManifest(pkg);
    expect(manifest).toContain('Id="cddm"');
    expect(manifest).toContain('Version="1.7.0"');
    expect(manifest).toContain('Publisher="grigortonikyan"');
    expect(manifest).toContain("<DisplayName>CDDM — Code De-Duplication Meister</DisplayName>");
    expect(manifest).toContain('Id="Microsoft.VisualStudio.Code"');
    expect(manifest).toContain('Path="extension/package.json"');
  });
});

describe("VSIX Packaging Pipeline", () => {
  it("should compile and build valid .vsix package artifact", async () => {
    const result = await packageVscodeExtension();
    expect(result.version).toMatch(/^\d+\.\d+\.\d+$/);
    expect(result.sizeBytes).toBeGreaterThan(5000);
    expect(result.filesCount).toBeGreaterThan(5);
    expect(existsSync(result.vsixPath)).toBe(true);

    const vsixBuffer = readFileSync(result.vsixPath);
    expect(vsixBuffer.readUInt32LE(0)).toBe(0x04034b50);
  });
});
