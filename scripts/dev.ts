#!/usr/bin/env bun
/**
 * Unified Development Runner for CDDM.
 * Automatically spawns the Rust Axum backend API server and Vite WebUI dev server concurrently using native Bun APIs.
 * Ensures the backend is fully initialized and healthy before routing frontend requests.
 */

import { join } from "node:path";
import type { Subprocess } from "bun";

const workspaceRoot = join(import.meta.dir, "..");
const BACKEND_PORT = 3001;
const HEALTH_URL = `http://127.0.0.1:${BACKEND_PORT}/api/health`;
const MAX_HEALTH_ATTEMPTS = 60;
const HEALTH_POLL_INTERVAL_MS = 500;

let backendProc: Subprocess | null = null;
let viteProc: Subprocess | null = null;

function cleanup() {
  if (backendProc && !backendProc.killed) {
    console.log("\n[CDDM Dev] Stopping Rust backend API server...");
    backendProc.kill();
    backendProc = null;
  }
  if (viteProc && !viteProc.killed) {
    console.log("[CDDM Dev] Stopping Vite WebUI dev server...");
    viteProc.kill();
    viteProc = null;
  }
}

process.on("SIGINT", () => {
  cleanup();
  process.exit(0);
});

process.on("SIGTERM", () => {
  cleanup();
  process.exit(0);
});

process.on("exit", () => {
  cleanup();
});

async function waitForBackendHealth(): Promise<boolean> {
  console.log(`[CDDM Dev] Waiting for Rust Axum backend on port ${BACKEND_PORT}...`);
  for (let i = 0; i < MAX_HEALTH_ATTEMPTS; i++) {
    try {
      const res = await fetch(HEALTH_URL);
      if (res.ok) {
        const body = (await res.json()) as { status?: string };
        if (body.status === "ok") {
          console.log(
            `[CDDM Dev] [OK] Backend API is healthy and operational on port ${BACKEND_PORT}!`,
          );
          return true;
        }
      }
    } catch {
      // Backend still starting up, retry
    }
    await Bun.sleep(HEALTH_POLL_INTERVAL_MS);
  }
  return false;
}

async function main() {
  console.log("=======================================================");
  console.log("      CDDM Studio — Full-Stack Development Mode      ");
  console.log("=======================================================\n");

  console.log(
    "[CDDM Dev] Launching Rust Axum backend (cargo run -p cddm-cli -- serve --port 3001)...",
  );

  backendProc = Bun.spawn({
    cmd: ["cargo", "run", "-p", "cddm-cli", "--", "serve", "--port", String(BACKEND_PORT)],
    cwd: workspaceRoot,
    stdout: "inherit",
    stderr: "inherit",
    stdin: "inherit",
  });

  const isHealthy = await waitForBackendHealth();
  if (!isHealthy) {
    console.error("[CDDM Dev] [ERROR] Rust backend failed to start within timeout.");
    cleanup();
    process.exit(1);
  }

  console.log("\n[CDDM Dev] Launching Vite WebUI dev server (vp -C webui run dev)...");
  const isWindows = process.platform === "win32";

  viteProc = Bun.spawn({
    cmd: isWindows
      ? ["cmd.exe", "/c", "vp", "-C", "webui", "run", "dev"]
      : ["vp", "-C", "webui", "run", "dev"],
    cwd: workspaceRoot,
    stdout: "inherit",
    stderr: "inherit",
    stdin: "inherit",
  });

  const exitCode = await viteProc.exited;
  cleanup();
  process.exit(exitCode);
}

void main();
