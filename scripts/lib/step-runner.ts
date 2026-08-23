/**
 * Shared Step Execution Engine for CDDM repository automation scripts.
 * Cross-platform helper providing consistent formatted output, timing, and error handling.
 */

export interface ScriptStep {
  title: string;
  command: string[];
  cwd?: string;
  allowFailure?: boolean;
  description?: string;
}

export interface StepResult {
  step: ScriptStep;
  exitCode: number;
  elapsedMs: number;
  passed: boolean;
}

export function printScriptBanner(title: string, color: string = "\x1b[36m"): void {
  console.log(`${color}=======================================================\x1b[0m`);
  const padLen = Math.max(0, Math.floor((55 - title.length) / 2));
  console.log(`${color}${" ".repeat(padLen)}${title}\x1b[0m`);
  console.log(`${color}=======================================================\x1b[0m`);
}

export function printScriptHelp(
  toolName: string,
  usageCmd: string,
  options: Array<[string, string]>,
): void {
  console.log(`\nCDDM ${toolName}\n\nUsage:\n  ${usageCmd}\n\nOptions:`);
  for (const [flag, desc] of options) {
    console.log(`  ${flag.padEnd(24)} ${desc}`);
  }
  console.log();
}

export async function executeStep(
  step: ScriptStep,
  index: number,
  total: number,
  headerColor: string = "\x1b[36m",
): Promise<StepResult> {
  const stepNum = `[${index + 1}/${total}]`;
  console.log(`\n${headerColor}${stepNum} ${step.title}...\x1b[0m`);
  const startTime = performance.now();

  const proc = Bun.spawn(step.command, {
    cwd: step.cwd,
    stdout: "inherit",
    stderr: "inherit",
  });

  const exitCode = await proc.exited;
  const elapsedMs = Math.round(performance.now() - startTime);
  const passed = exitCode === 0;

  if (!passed && !step.allowFailure) {
    console.error(
      `\n\x1b[31m[FAIL] Step failed with exit code ${exitCode} (${elapsedMs}ms): ${step.command.join(" ")}\x1b[0m\n`,
    );
    process.exit(exitCode ?? 1);
  }

  if (passed) {
    console.log(`\x1b[32m[PASS] (${elapsedMs}ms)\x1b[0m`);
  } else {
    console.log(
      `\x1b[33m[WARN] Step returned exit code ${exitCode} (ignored) (${elapsedMs}ms)\x1b[0m`,
    );
  }

  return { step, exitCode, elapsedMs, passed };
}
