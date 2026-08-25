#!/usr/bin/env bun
/**
 * Cross-platform VS Code Extension VSIX Packager for CDDM.
 * Compiles TypeScript, constructs standard Open Packaging Conventions (OPC)
 * manifest structure, and outputs standard .vsix package files.
 */

import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createZipArchive, ZipEntry } from "./lib/zip-builder";

export interface PackageOptions {
  workspaceRoot?: string;
  outputDir?: string;
}

export interface PackageResult {
  vsixPath: string;
  version: string;
  sizeBytes: number;
  filesCount: number;
}

function collectFilesRecursively(
  dir: string,
  baseDir: string = dir,
): Array<{ path: string; relPath: string }> {
  const results: Array<{ path: string; relPath: string }> = [];
  if (!existsSync(dir)) return results;

  const entries = readdirSync(dir);
  for (const entry of entries) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);
    if (stat.isDirectory()) {
      results.push(...collectFilesRecursively(fullPath, baseDir));
    } else if (stat.isFile()) {
      const relPath = fullPath.slice(baseDir.length + 1).replace(/\\/g, "/");
      results.push({ path: fullPath, relPath });
    }
  }
  return results;
}

export function generateContentTypesXml(): string {
  return `<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="json" ContentType="application/json" />
  <Default Extension="vsixmanifest" ContentType="text/xml" />
  <Default Extension="js" ContentType="application/javascript" />
  <Default Extension="map" ContentType="application/json" />
  <Default Extension="svg" ContentType="image/svg+xml" />
  <Default Extension="md" ContentType="text/markdown" />
  <Default Extension="txt" ContentType="text/plain" />
</Types>`;
}

export interface VscodePackageManifest {
  name?: string;
  version?: string;
  publisher?: string;
  displayName?: string;
  description?: string;
  categories?: string[];
  keywords?: string[];
  engines?: { vscode?: string };
}

export function generateVsixManifest(pkg: VscodePackageManifest): string {
  const name = typeof pkg.name === "string" ? pkg.name : "cddm";
  const version = typeof pkg.version === "string" ? pkg.version : "1.7.0";
  const publisher = typeof pkg.publisher === "string" ? pkg.publisher : "grigortonikyan";
  const displayName = typeof pkg.displayName === "string" ? pkg.displayName : "CDDM";
  const description = typeof pkg.description === "string" ? pkg.description : "";
  const vscodeEngine =
    pkg.engines && typeof pkg.engines.vscode === "string" ? pkg.engines.vscode : "^1.96.0";
  const categories = Array.isArray(pkg.categories) ? pkg.categories.join(",") : "Linters";
  const keywords = Array.isArray(pkg.keywords) ? pkg.keywords.join(",") : "ast,duplication";

  return `<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011" xmlns:d="http://schemas.microsoft.com/developer/vsx-schema-design/2011">
  <Metadata>
    <Identity Language="en-US" Id="${name}" Version="${version}" Publisher="${publisher}" TargetPlatform="universal" />
    <DisplayName>${displayName}</DisplayName>
    <Description xml:space="preserve">${description}</Description>
    <Tags>${keywords}</Tags>
    <Categories>${categories}</Categories>
    <License>extension/LICENSE</License>
    <Icon>extension/resources/cddm-icon.svg</Icon>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code" Version="${vscodeEngine}" />
  </Installation>
  <Dependencies />
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" Addressable="true" />
    <Asset Type="Microsoft.VisualStudio.Services.Content.Details" Path="extension/README.md" Addressable="true" />
    <Asset Type="Microsoft.VisualStudio.Services.Content.License" Path="extension/LICENSE" Addressable="true" />
    <Asset Type="Microsoft.VisualStudio.Services.Icons.Default" Path="extension/resources/cddm-icon.svg" Addressable="true" />
  </Assets>
</PackageManifest>`;
}

export async function packageVscodeExtension(options: PackageOptions = {}): Promise<PackageResult> {
  const root = options.workspaceRoot || resolve(import.meta.dir, "..");
  const vscodeDir = join(root, "editors", "vscode");
  const outDir = options.outputDir || join(root, "packaging", "vscode");

  mkdirSync(outDir, { recursive: true });

  const pkgJsonPath = join(vscodeDir, "package.json");
  if (!existsSync(pkgJsonPath)) {
    throw new Error(`VS Code package.json not found at ${pkgJsonPath}`);
  }

  const pkg = JSON.parse(readFileSync(pkgJsonPath, "utf-8"));
  const version = pkg.version || "1.7.0";

  // Build TypeScript files
  console.log("\x1b[36m--> Compiling VS Code extension TypeScript...\x1b[0m");
  const proc = Bun.spawnSync(["bunx", "tsc", "-p", join(vscodeDir, "tsconfig.json")], {
    cwd: root,
    stdout: "pipe",
    stderr: "pipe",
  });

  if (proc.exitCode !== 0) {
    throw new Error(`TypeScript compilation failed:\n${proc.stderr.toString()}`);
  }
  console.log("\x1b[32m[PASS] Extension TypeScript compilation succeeded\x1b[0m");

  const entries: ZipEntry[] = [];

  // 1. Root OPC descriptors
  entries.push({
    name: "[Content_Types].xml",
    data: generateContentTypesXml(),
  });

  entries.push({
    name: "extension.vsixmanifest",
    data: generateVsixManifest(pkg),
  });

  // 2. Package metadata and documentation
  entries.push({
    name: "extension/package.json",
    data: JSON.stringify(pkg, null, 2),
  });

  const readmePath = join(vscodeDir, "README.md");
  if (existsSync(readmePath)) {
    entries.push({
      name: "extension/README.md",
      data: readFileSync(readmePath),
    });
  }

  const licensePath = existsSync(join(vscodeDir, "LICENSE"))
    ? join(vscodeDir, "LICENSE")
    : join(root, "LICENSE");
  if (existsSync(licensePath)) {
    entries.push({
      name: "extension/LICENSE",
      data: readFileSync(licensePath),
    });
  }

  const iconPath = join(vscodeDir, "resources", "cddm-icon.svg");
  if (existsSync(iconPath)) {
    entries.push({
      name: "extension/resources/cddm-icon.svg",
      data: readFileSync(iconPath),
    });
  }

  // 3. Compiled out/ files
  const compiledFiles = collectFilesRecursively(join(vscodeDir, "out"));
  for (const file of compiledFiles) {
    entries.push({
      name: `extension/out/${file.relPath}`,
      data: readFileSync(file.path),
    });
  }

  console.log(`\x1b[36m--> Packaging ${entries.length} assets into VSIX archive...\x1b[0m`);
  const zipBuffer = createZipArchive(entries);

  const vsixFileName = `cddm-${version}.vsix`;
  const primaryVsixPath = join(outDir, vsixFileName);
  const secondaryVsixPath = join(vscodeDir, vsixFileName);

  writeFileSync(primaryVsixPath, zipBuffer);
  writeFileSync(secondaryVsixPath, zipBuffer);

  console.log(
    `\x1b[32m[SUCCESS] Packaged VSIX (${(zipBuffer.length / 1024).toFixed(1)} KB) -> ${primaryVsixPath}\x1b[0m`,
  );

  return {
    vsixPath: primaryVsixPath,
    version,
    sizeBytes: zipBuffer.length,
    filesCount: entries.length,
  };
}

if (import.meta.main) {
  packageVscodeExtension().catch((err) => {
    console.error("VSIX Packaging error:", err);
    process.exit(1);
  });
}
