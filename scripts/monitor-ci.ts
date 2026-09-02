#!/usr/bin/env bun
/**
 * CDDM Gitea Actions CI/CD Workflow Monitor
 * Queries, streams, and audits live CI/CD pipeline runs and build jobs.
 */

import { GITEA_REPO, giteaFetch, sleep } from "./lib/gitea-client";
import { printScriptBanner, printScriptHelp } from "./lib/step-runner";

export interface WorkflowJob {
  id: number;
  name: string;
  status: "queued" | "in_progress" | "completed" | "waiting";
  conclusion: "success" | "failure" | "cancelled" | "skipped" | null;
  started_at: string;
  completed_at: string | null;
}

export interface WorkflowRun {
  id: number;
  display_title: string;
  status: "queued" | "in_progress" | "completed" | "waiting";
  conclusion: "success" | "failure" | "cancelled" | "skipped" | null;
  event: string;
  head_branch: string;
  head_sha: string;
  started_at: string;
  completed_at: string | null;
  html_url: string;
}

export interface MonitorOptions {
  watch?: boolean;
  limit?: number;
  intervalMs?: number;
  maxCycles?: number;
}

export async function fetchRecentRuns(limit = 10): Promise<WorkflowRun[]> {
  const path = `/repos/${GITEA_REPO}/actions/runs?page=1&limit=${limit}`;
  const res = await giteaFetch<{ workflow_runs: WorkflowRun[]; total_count: number }>(path);
  return res.data?.workflow_runs || [];
}

export async function fetchRunJobs(runId: number): Promise<WorkflowJob[]> {
  const path = `/repos/${GITEA_REPO}/actions/runs/${runId}/jobs`;
  const res = await giteaFetch<{ jobs: WorkflowJob[] }>(path);
  return res.data?.jobs || [];
}

export function formatStatusBadge(status: string, conclusion: string | null): string {
  if (status === "in_progress") return "[RUNNING]";
  if (status === "queued") return "[QUEUED]";
  if (status === "waiting") return "[WAITING]";
  if (conclusion === "success") return "[SUCCESS]";
  if (conclusion === "failure") return "[FAILED]";
  if (conclusion === "cancelled") return "[CANCELLED]";
  return `[${status.toUpperCase()}]`;
}

export async function displayRunsSnapshot(limit = 10): Promise<boolean> {
  const runs = await fetchRecentRuns(limit);
  let hasActiveWorkflows = false;

  console.log(`\n=== Gitea Actions CI Status (${new Date().toLocaleTimeString()}) ===`);
  for (const run of runs) {
    const isRunning =
      run.status === "in_progress" || run.status === "queued" || run.status === "waiting";
    if (isRunning) hasActiveWorkflows = true;

    const badge = formatStatusBadge(run.status, run.conclusion);
    const shortSha = run.head_sha ? run.head_sha.slice(0, 8) : "--------";
    console.log(`\n* Run #${run.id} ${badge} "${run.display_title}"`);
    console.log(`  Event: ${run.event} | Branch: ${run.head_branch || "N/A"} | SHA: ${shortSha}`);
    console.log(`  URL: ${run.html_url}`);

    try {
      const jobs = await fetchRunJobs(run.id);
      for (const job of jobs) {
        const jobBadge = formatStatusBadge(job.status, job.conclusion);
        console.log(`    - ${jobBadge} ${job.name}`);
      }
    } catch {
      // Ignore job fetch error
    }
  }

  return hasActiveWorkflows;
}

export async function monitorCiLoop(options: MonitorOptions = {}): Promise<void> {
  const interval = options.intervalMs ?? 10000;
  const maxCycles = options.maxCycles ?? (options.watch ? 60 : 1);
  const limit = options.limit ?? 5;

  for (let cycle = 1; cycle <= maxCycles; cycle++) {
    const hasActive = await displayRunsSnapshot(limit);
    if (!options.watch || (!hasActive && cycle > 1)) {
      if (!hasActive) {
        console.log("\n[INFO] All discovered CI workflows are completed.");
      }
      break;
    }
    await sleep(interval);
  }
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  if (args.includes("--help") || args.includes("-h")) {
    printScriptHelp("Gitea CI Monitor", "bun scripts/monitor-ci.ts [options]", [
      ["--watch, -w", "Continuously poll and stream live CI status until completion"],
      ["--limit <n>, -l <n>", "Limit the number of recent workflow runs displayed (default: 5)"],
      ["--interval <sec>, -i <sec>", "Polling interval in seconds for --watch (default: 10s)"],
      ["--help, -h", "Show this help message"],
    ]);
    process.exit(0);
  }

  printScriptBanner("CDDM Gitea Actions CI/CD Monitor");
  const isWatch = args.includes("--watch") || args.includes("-w");
  const limitIdx = args.indexOf("--limit") !== -1 ? args.indexOf("--limit") : args.indexOf("-l");
  const limitArg = limitIdx !== -1 ? args[limitIdx + 1] : undefined;
  const limit = limitArg ? Number.parseInt(limitArg, 10) : 5;

  const intIdx =
    args.indexOf("--interval") !== -1 ? args.indexOf("--interval") : args.indexOf("-i");
  const intArg = intIdx !== -1 ? args[intIdx + 1] : undefined;
  const intervalSec = intArg ? Number.parseInt(intArg, 10) : 10;

  void monitorCiLoop({
    watch: isWatch,
    limit,
    intervalMs: intervalSec * 1000,
  });
}
