import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

export interface PublisherConfig {
  publisherId: string;
  publisherName: string;
  homebrewTapRepo: string;
  scoopBucketRepo: string;
  wingetRepo: string;
  vcsHost: string;
  vcsRepo: string;
}

/**
 * Safely parse key-value pairs from a .env file.
 */
export function parseEnvFile(filePath: string): Record<string, string> {
  const result: Record<string, string> = {};
  if (!existsSync(filePath)) {
    return result;
  }
  const content = readFileSync(filePath, "utf-8");
  for (const line of content.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const eqIdx = trimmed.indexOf("=");
    if (eqIdx === -1) continue;
    const key = trimmed.slice(0, eqIdx).trim();
    let val = trimmed.slice(eqIdx + 1).trim();
    if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) {
      val = val.slice(1, -1);
    }
    result[key] = val;
  }
  return result;
}

/**
 * Extract repository host, owner, and repo name dynamically from git remote origin.
 */
export function detectGitRemoteOrigin(workspaceRoot: string = process.cwd()): {
  host?: string;
  owner?: string;
  repo?: string;
  fullRepo?: string;
} {
  try {
    const proc = Bun.spawnSync(["git", "remote", "get-url", "origin"], {
      cwd: workspaceRoot,
      stdout: "pipe",
      stderr: "pipe",
    });
    if (proc.exitCode !== 0) return {};
    const url = proc.stdout.toString().trim();
    if (!url) return {};

    // Match HTTPS: https://git.gt-web-dev.com/gt-dev/cddm.git or SSH: git@git.gt-web-dev.com:gt-dev/cddm.git
    const httpsMatch = /^https?:\/\/([^/:]+)(?::\d+)?\/([^/]+)\/(.+?)(?:\.git)?$/.exec(url);
    if (httpsMatch) {
      const [, host, owner, repo] = httpsMatch;
      return { host, owner, repo, fullRepo: `${owner}/${repo}` };
    }

    const sshMatch = /^(?:ssh:\/\/)?(?:[^@]+@)?([^/:]+)(?::\d+)?[:/]([^/]+)\/(.+?)(?:\.git)?$/.exec(
      url,
    );
    if (sshMatch) {
      const [, host, owner, repo] = sshMatch;
      return { host, owner, repo, fullRepo: `${owner}/${repo}` };
    }
  } catch {
    // Ignore git command execution failure in air-gapped or test environments
  }
  return {};
}

/**
 * Resolves publisher and distribution configuration using strict precedence:
 * 1. Environment variables (VCS Actions vars/secrets passed via workflow env)
 * 2. .env file (as last-resort fallback for local execution)
 * 3. Dynamic discovery from git remote origin
 */
export function resolvePublisherConfig(
  workspaceRoot: string = process.cwd(),
  customEnv: Record<string, string | undefined> = process.env,
): PublisherConfig {
  const envFilePath = join(workspaceRoot, ".env");
  const envFile = parseEnvFile(envFilePath);
  const gitRemote = detectGitRemoteOrigin(workspaceRoot);

  const vcsHost =
    customEnv.VCS_HOST ||
    customEnv.GITEA_HOST ||
    envFile.VCS_HOST ||
    envFile.GITEA_HOST ||
    gitRemote.host ||
    "git.gt-web-dev.com";

  const vcsRepo =
    customEnv.GITEA_REPO ||
    customEnv.GITHUB_REPOSITORY ||
    envFile.GITEA_REPO ||
    gitRemote.fullRepo ||
    "gt-dev/cddm";

  const defaultOwner =
    gitRemote.owner || (vcsRepo.includes("/") ? vcsRepo.split("/")[0] : "gt-dev");

  const publisherId =
    customEnv.PACKAGE_PUBLISHER ||
    customEnv.WINGET_PUBLISHER ||
    customEnv.PUBLISHER ||
    envFile.PACKAGE_PUBLISHER ||
    envFile.WINGET_PUBLISHER ||
    envFile.PUBLISHER ||
    defaultOwner ||
    "gt-dev";

  const publisherName =
    customEnv.PACKAGE_PUBLISHER_NAME ||
    customEnv.PUBLISHER_NAME ||
    envFile.PACKAGE_PUBLISHER_NAME ||
    envFile.PUBLISHER_NAME ||
    publisherId.toUpperCase();

  const homebrewTapRepo =
    customEnv.HOMEBREW_TAP_REPO || envFile.HOMEBREW_TAP_REPO || `${publisherId}/homebrew-tap`;

  const scoopBucketRepo =
    customEnv.SCOOP_BUCKET_REPO || envFile.SCOOP_BUCKET_REPO || `${publisherId}/scoop-bucket`;

  const wingetRepo = customEnv.WINGET_REPO || envFile.WINGET_REPO || `${publisherId}/winget-pkgs`;

  return {
    publisherId,
    publisherName,
    homebrewTapRepo,
    scoopBucketRepo,
    wingetRepo,
    vcsHost,
    vcsRepo,
  };
}
