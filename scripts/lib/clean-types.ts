/**
 * Type definitions, known paths, and protection rules for the CDDM Workspace Cleanup Engine.
 */

export interface CleanOptions {
  dryRun?: boolean;
  verbose?: boolean;
  keepLockfiles?: boolean;
  keepNodeModules?: boolean;
  keepTarget?: boolean;
  keepBuild?: boolean;
  keepReports?: boolean;
  keepCache?: boolean;
  targetOnly?: boolean;
  nodeOnly?: boolean;
  cacheOnly?: boolean;
  reportsOnly?: boolean;
  buildOnly?: boolean;
  lockfilesOnly?: boolean;
  cwd?: string;
}

export interface CleanItem {
  path: string;
  relPath: string;
  isDirectory: boolean;
  category: "build" | "cache" | "lockfile" | "test-report" | "temp";
  sizeBytes?: number;
}

export interface SafeRemoveResult {
  success: boolean;
  bytesFreed: number;
  lockedFiles: string[];
}

export interface CleanResult {
  items: CleanItem[];
  dirsRemoved: number;
  filesRemoved: number;
  bytesFreed: number;
  lockedFiles: string[];
  dryRun: boolean;
  elapsedMs: number;
}

export const KNOWN_PACKAGE_ROOTS = ["", "webui", "tests/e2e", "editors/vscode", "npm/cddm"];
export const LOCK_NAMES = ["bun.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml"];

export const KNOWN_CLEAN_DIRS: Array<{ path: string; category: CleanItem["category"] }> = [
  // Rust build output
  { path: "target", category: "build" },
  // WebUI & Frontend build outputs
  { path: "dist", category: "build" },
  { path: "webui/dist", category: "build" },
  { path: "npm/cddm/dist", category: "build" },
  { path: "editors/vscode/out", category: "build" },
  { path: "packaging/vscode", category: "build" },
  // Node dependencies across all packages
  ...KNOWN_PACKAGE_ROOTS.map((p) => ({
    path: p ? `${p}/node_modules` : "node_modules",
    category: "cache" as const,
  })),
  // Test reports & Coverage across root and subprojects
  { path: "coverage", category: "test-report" },
  { path: "webui/coverage", category: "test-report" },
  { path: "test-results", category: "test-report" },
  { path: "playwright-report", category: "test-report" },
  { path: "blob-report", category: "test-report" },
  { path: "tests/e2e/test-results", category: "test-report" },
  { path: "tests/e2e/playwright-report", category: "test-report" },
  { path: "tests/e2e/blob-report", category: "test-report" },
  { path: "webui/test-results", category: "test-report" },
  { path: "webui/playwright-report", category: "test-report" },
  { path: "webui/blob-report", category: "test-report" },
  { path: ".nyc_output", category: "test-report" },
  // Tooling & framework caches
  { path: ".turbo", category: "cache" },
  { path: ".cache", category: "cache" },
  { path: "webui/.cache", category: "cache" },
  { path: ".vite", category: "cache" },
  { path: "webui/.vite", category: "cache" },
  { path: ".cddm", category: "cache" },
  // Logs & temp directories
  { path: ".logs", category: "temp" },
];

export const KNOWN_CLEAN_FILES: Array<{ path: string; category: CleanItem["category"] }> = [
  { path: "Cargo.lock", category: "lockfile" },
  ...KNOWN_PACKAGE_ROOTS.flatMap((p) =>
    LOCK_NAMES.map((name) => ({
      path: p ? `${p}/${name}` : name,
      category: "lockfile" as const,
    })),
  ),
  ...KNOWN_PACKAGE_ROOTS.map((p) => ({
    path: p ? `${p}/tsconfig.tsbuildinfo` : "tsconfig.tsbuildinfo",
    category: "cache" as const,
  })),
];

export const NEVER_TRAVERSE_DIRS = new Set([
  ".git",
  ".agents",
  ".github",
  ".gitea",
  ".vscode",
  ".vite-hooks",
  "crates",
  "tests",
  "docs",
  "scripts",
]);

export const PROTECTED_PREFIXES = new Set([
  ".git",
  ".agents",
  ".github",
  ".gitea",
  ".vscode",
  ".vite-hooks",
  "crates",
  "tests",
  "webui/src",
  "webui/public",
  "scripts",
  "docs",
  "npm/cddm/bin",
  "editors/vscode/src",
  "editors/vscode/resources",
  "packaging/homebrew",
  "packaging/scoop",
  "packaging/winget",
]);

export const PROTECTED_EXACT_FILES = new Set([
  "Cargo.toml",
  "package.json",
  "bunfig.toml",
  "tsconfig.json",
  "vite.config.ts",
  "commitlint.config.ts",
  "rustfmt.toml",
  ".editorconfig",
  ".gitattributes",
  ".gitignore",
  ".cddmignore",
  ".markdownlint.json",
  ".markdownlintignore",
  ".env.example",
  "README.md",
  "LICENSE",
  "LICENSE-APACHE",
  "LICENSE-MIT",
  "CONTRIBUTING.md",
  "CHANGELOG.md",
  "SECURITY.md",
  "CODE_OF_CONDUCT.md",
  "AGENTS.md",
]);
