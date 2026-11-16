const { spawn } = require("child_process");
const fs = require("fs");
const path = require("path");

const SUPPORTED_PLATFORMS = new Set(["win32", "linux", "darwin"]);
const SUPPORTED_ARCHS = new Set(["x64", "arm64"]);

/**
 * Resolves the platform-specific @cddm/<platform>-<arch> binary path.
 * Never probes local development build directories and never falls back to bare PATH commands
 * (which would trigger an infinite self-spawn loop when npm/cddm/bin is on PATH).
 */
function resolveBinaryPath(baseName, options = {}) {
  const platform = options.platform || process.platform;
  const arch = options.arch || process.arch;
  const existsFn = options.existsSync || fs.existsSync;
  const resolveFn = options.requireResolve || ((specifier) => require.resolve(specifier));
  const platformDirOverride =
    options.platformDirOverride !== undefined
      ? options.platformDirOverride
      : process.env.CDDM_NPM_PLATFORM_DIR;

  if (!SUPPORTED_PLATFORMS.has(platform) || !SUPPORTED_ARCHS.has(arch)) {
    throw new Error(`Unsupported platform/architecture "${platform}-${arch}" for ${baseName}.`);
  }

  const pkgName = `@cddm/${platform}-${arch}`;
  const binFileName = platform === "win32" ? `${baseName}.exe` : baseName;

  if (platformDirOverride) {
    const candidate = path.join(platformDirOverride, binFileName);
    if (existsFn(candidate)) {
      return candidate;
    }
  }

  try {
    const pkgJsonPath = resolveFn(`${pkgName}/package.json`);
    const binPath = path.join(path.dirname(pkgJsonPath), binFileName);
    if (existsFn(binPath)) {
      return binPath;
    }
  } catch {
    // Handled by explicit diagnostic error below
  }

  throw new Error(
    `Missing required platform binary package "${pkgName}" (${binFileName}). ` +
      `Ensure optional dependencies are enabled when installing "cddm", or install ` +
      `via Cargo ("cargo install cddm-cli cddm-mcp") or standalone release archives.`,
  );
}

function runBinary(baseName, args = process.argv.slice(2)) {
  let binaryPath;
  try {
    binaryPath = resolveBinaryPath(baseName);
  } catch (err) {
    console.error(`[cddm] ${err instanceof Error ? err.message : String(err)}`);
    process.exit(1);
  }

  const child = spawn(binaryPath, args, {
    stdio: "inherit",
  });

  child.on("error", (err) => {
    console.error(`[cddm] Failed to launch "${binaryPath}": ${err.message}`);
    process.exit(1);
  });

  child.on("exit", (code) => {
    process.exit(code ?? 0);
  });
}

module.exports = {
  resolveBinaryPath,
  runBinary,
};
