/**
 * Gitea API Client Helper for CDDM Portal Management
 */

import { resolvePublisherConfig } from "./publisher-config";

const resolvedConfig = resolvePublisherConfig(process.cwd());

function resolveGitCredentialToken(host: string): string | undefined {
  try {
    const cleanHost = host.split(":")[0] || host;
    const proc = Bun.spawnSync(["git", "credential", "fill"], {
      stdin: Buffer.from(`protocol=https\nhost=${cleanHost}\n\n`, "utf-8"),
      stdout: "pipe",
      stderr: "pipe",
      env: { ...process.env, GIT_TERMINAL_PROMPT: "0", GCM_INTERACTIVE: "never" },
    });
    if (proc.exitCode !== 0) return undefined;
    const out = proc.stdout.toString();
    const match = /^password=(.+)$/m.exec(out);
    return match?.[1]?.trim() || undefined;
  } catch {
    return undefined;
  }
}

export const GITEA_BASE =
  process.env.GITEA_BASE || process.env.GITHUB_SERVER_URL || `https://${resolvedConfig.vcsHost}`;
export const GITEA_TOKEN =
  process.env.GITEA_TOKEN ||
  process.env.RELEASE_BOT_TOKEN ||
  process.env.REGISTRY_TOKEN ||
  process.env.FORGE_GITEA_TOKEN ||
  process.env.GITHUB_TOKEN ||
  resolveGitCredentialToken(resolvedConfig.vcsHost) ||
  "token-placeholder";
export const GITEA_REPO = process.env.GITEA_REPO || resolvedConfig.vcsRepo;
export const GITEA_OWNER =
  process.env.GITEA_OWNER || GITEA_REPO.split("/")[0] || resolvedConfig.publisherId;

export function getApiHeaders(useJson = true): Record<string, string> {
  const headers: Record<string, string> = {
    Accept: "application/json",
    Authorization: `token ${GITEA_TOKEN}`,
  };
  if (useJson) {
    headers["Content-Type"] = "application/json";
  }
  return headers;
}

export async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function giteaFetch<T>(
  path: string,
  options: RequestInit = {},
  retries = 3,
): Promise<{ status: number; ok: boolean; data: T }> {
  const url = path.startsWith("http") ? path : `${GITEA_BASE}/api/v1${path}`;
  const headers = {
    ...getApiHeaders(!options.body || typeof options.body === "string"),
    ...(options.headers as Record<string, string>),
  };

  await sleep(100);

  for (let attempt = 1; attempt <= retries; attempt++) {
    try {
      const res = await fetch(url, { ...options, headers });
      let data: T | null = null;
      const contentType = res.headers.get("content-type") || "";
      if (contentType.includes("application/json")) {
        try {
          data = await res.json();
        } catch {
          data = null;
        }
      } else {
        data = (await res.text()) as unknown as T;
      }

      return { status: res.status, ok: res.ok, data: data as T };
    } catch (err) {
      if (attempt === retries) {
        throw err;
      }
      console.warn(
        `    [RETRY ${attempt}/${retries}] Retrying ${path} after error: ${String(err)}`,
      );
      await sleep(500 * attempt);
    }
  }

  throw new Error(`Failed to fetch ${path} after ${retries} attempts`);
}
