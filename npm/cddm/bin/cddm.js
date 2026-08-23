#!/usr/bin/env node

const { spawn } = require("child_process");
const path = require("path");
const fs = require("fs");

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  let pkgName = "";
  let binName = "cddm";

  if (platform === "win32") {
    pkgName = `@cddm/win32-${arch}`;
    binName = "cddm.exe";
  } else if (platform === "darwin") {
    pkgName = `@cddm/darwin-${arch}`;
  } else if (platform === "linux") {
    pkgName = `@cddm/linux-${arch}`;
  }

  // 1. Try local target release binary build
  const localTarget = path.join(__dirname, "..", "..", "..", "target", "release", binName);
  if (fs.existsSync(localTarget)) {
    return localTarget;
  }

  // 2. Try node_modules optionalDependency binary
  try {
    const pkgPath = require.resolve(`${pkgName}/package.json`);
    const binPath = path.join(path.dirname(pkgPath), binName);
    if (fs.existsSync(binPath)) {
      return binPath;
    }
  } catch {
    // optionalDependency not installed directly
  }

  // 3. Fallback to system PATH
  return binName;
}

const binaryPath = getBinaryPath();
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
});

child.on("exit", (code) => {
  process.exit(code ?? 0);
});
