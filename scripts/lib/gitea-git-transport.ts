import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

export interface RepoFilePayload {
  path: string;
  content: string;
}

/**
 * Fetch existing file blob sha from Gitea contents API if it exists on the target branch.
 */
export async function getExistingFileSha(
  host: string,
  repo: string,
  filePath: string,
  token?: string,
  branch: string = "main",
  fetchFn: typeof fetch = fetch,
): Promise<string | null> {
  const url = `https://${host}/api/v1/repos/${repo}/contents/${filePath}?ref=${branch}`;
  try {
    const headers: Record<string, string> = { Accept: "application/json" };
    if (token) {
      headers.Authorization = token.includes(".") ? `Bearer ${token}` : `token ${token}`;
    }
    const res = await fetchFn(url, { headers });
    if (!res.ok) return null;
    const data = (await res.json()) as { sha?: string };
    return data.sha || null;
  } catch {
    return null;
  }
}

/**
 * Commit a single file to a Gitea repository using the contents API.
 */
export async function commitFileToRepo(
  host: string,
  repo: string,
  filePath: string,
  content: string,
  message: string,
  token?: string,
  branch: string = "main",
  fetchFn: typeof fetch = fetch,
): Promise<boolean> {
  if (!token) {
    console.log(`\x1b[33m[SKIP] No token provided for repository ${repo}\x1b[0m`);
    return false;
  }

  const existingSha = await getExistingFileSha(host, repo, filePath, token, branch, fetchFn);
  const url = `https://${host}/api/v1/repos/${repo}/contents/${filePath}`;
  const base64Content = Buffer.from(content, "utf-8").toString("base64");

  const method = existingSha ? "PUT" : "POST";
  const body: Record<string, unknown> = {
    content: base64Content,
    message,
    branch,
  };
  if (existingSha) {
    body.sha = existingSha;
  }

  try {
    const authHeader = token.includes(".") ? `Bearer ${token}` : `token ${token}`;
    const res = await fetchFn(url, {
      method,
      headers: {
        Authorization: authHeader,
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      const errText = await res.text();
      console.error(
        `\x1b[31m[FAIL] Could not commit ${filePath} to ${repo} (${res.status}): ${errText}\x1b[0m`,
      );
      return false;
    }
    console.log(`\x1b[32m[OK] Successfully synchronized ${filePath} to ${repo}\x1b[0m`);
    return true;
  } catch (err) {
    console.error(`\x1b[31m[ERROR] Network error committing to ${repo}: ${String(err)}\x1b[0m`);
    return false;
  }
}

/**
 * Atomically clones, commits files, and pushes via an ephemeral repository-scoped SSH deploy key.
 */
export function commitFilesWithDeployKey(
  host: string,
  repo: string,
  files: RepoFilePayload[],
  message: string,
  deployKey: string,
  branch: string = "main",
  sshPort: string = process.env.GITEA_SSH_PORT || "30009",
): boolean {
  const workDir = mkdtempSync(join(tmpdir(), "cddm-deploy-"));
  const keyFile = join(workDir, "deploy_key");
  const cloneDir = join(workDir, "repo");
  const normalizedKey = deployKey.endsWith("\n") ? deployKey : `${deployKey}\n`;
  writeFileSync(keyFile, normalizedKey, { mode: 0o600, encoding: "utf-8" });

  const cleanHost = host.split(":")[0]!;
  const sshCmd = `ssh -i "${keyFile.replace(/\\/g, "/")}" -p ${sshPort} -o BatchMode=yes -o StrictHostKeyChecking=accept-new`;
  const sshUrl = `ssh://git@${cleanHost}:${sshPort}/${repo}.git`;
  const env = {
    ...process.env,
    GIT_SSH_COMMAND: sshCmd,
    GIT_TERMINAL_PROMPT: "0",
    GCM_INTERACTIVE: "never",
  };

  try {
    const cloneProc = Bun.spawnSync(
      ["git", "clone", "--depth", "1", "--branch", branch, sshUrl, cloneDir],
      { env, stdout: "pipe", stderr: "pipe" },
    );
    if (cloneProc.exitCode !== 0) {
      console.error(
        `\x1b[31m[FAIL] SSH deploy clone failed for ${repo}: ${cloneProc.stderr.toString()}\x1b[0m`,
      );
      return false;
    }

    for (const file of files) {
      const dest = join(cloneDir, file.path);
      mkdirSync(dirname(dest), { recursive: true });
      writeFileSync(dest, file.content, "utf-8");
    }

    const actor = process.env.GITHUB_ACTOR || "release-bot";
    Bun.spawnSync(["git", "-C", cloneDir, "config", "user.name", actor], { env });
    Bun.spawnSync(
      ["git", "-C", cloneDir, "config", "user.email", `${actor}@noreply.${cleanHost}`],
      {
        env,
      },
    );
    Bun.spawnSync(["git", "-C", cloneDir, "add", "-A"], { env });

    const diffProc = Bun.spawnSync(["git", "-C", cloneDir, "diff", "--cached", "--quiet"], { env });
    if (diffProc.exitCode === 0) {
      console.log(`\x1b[32m[OK] ${repo} manifests already up to date\x1b[0m`);
      return true;
    }

    const commitProc = Bun.spawnSync(["git", "-C", cloneDir, "commit", "-m", message], { env });
    if (commitProc.exitCode !== 0) {
      console.error(
        `\x1b[31m[FAIL] Git commit failed for ${repo}: ${commitProc.stderr.toString()}\x1b[0m`,
      );
      return false;
    }

    const pushProc = Bun.spawnSync(["git", "-C", cloneDir, "push", "origin", `HEAD:${branch}`], {
      env,
      stdout: "pipe",
      stderr: "pipe",
    });
    if (pushProc.exitCode !== 0) {
      console.error(
        `\x1b[31m[FAIL] SSH deploy push failed for ${repo}: ${pushProc.stderr.toString()}\x1b[0m`,
      );
      return false;
    }

    console.log(`\x1b[32m[OK] Pushed ${files.length} manifest(s) to ${repo} via deploy key\x1b[0m`);
    return true;
  } finally {
    rmSync(workDir, { recursive: true, force: true });
  }
}

/**
 * Publishes file payloads to a target repository, preferring SSH deploy key over REST API token.
 */
export async function publishRepoPayload(
  host: string,
  repo: string,
  files: RepoFilePayload[],
  message: string,
  options: {
    dryRun: boolean;
    deployKey?: string;
    token?: string;
    fetchFn?: typeof fetch;
  },
): Promise<boolean> {
  if (options.dryRun) {
    for (const file of files) {
      console.log(`\x1b[33m[DRY-RUN] Would commit ${file.path} to ${repo}\x1b[0m`);
    }
    return true;
  }

  if (options.deployKey) {
    return commitFilesWithDeployKey(host, repo, files, message, options.deployKey);
  }

  let allOk = true;
  for (const file of files) {
    const ok = await commitFileToRepo(
      host,
      repo,
      file.path,
      file.content,
      message,
      options.token,
      "main",
      options.fetchFn,
    );
    if (!ok) allOk = false;
  }
  return allOk;
}
