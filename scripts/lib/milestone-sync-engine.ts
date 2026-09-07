/**
 * Milestone Synchronization and Audit Engine for CDDM
 */

export interface MilestoneMeta {
  id: number;
  title: string;
  state: "open" | "closed";
  open_issues: number;
  closed_issues: number;
}

export interface IssueMeta {
  number: number;
  title: string;
  body: string;
  state: "open" | "closed";
  milestone: { id: number; title: string } | null;
}

/**
 * Resolves the appropriate milestone ID for a given issue based on
 * embedded target milestone tags, EP references, or lifecycle context.
 */
export function resolveIssueMilestone(
  issue: IssueMeta,
  milestones: MilestoneMeta[],
): number | null {
  // 1. Check for explicit Target Milestone in issue body
  const targetMatch = /- \*\*Target Milestone\*\*:\s*`?(v\d+\.\d+\.\d+)/i.exec(issue.body || "");
  if (targetMatch && targetMatch[1]) {
    const v = targetMatch[1].toLowerCase();
    const found = milestones.find((m) => m.title.toLowerCase().startsWith(v));
    if (found) return found.id;
  }

  // 2. Post-v3.3.0 development issues and PRs (#138 through #151) -> v3.4.0
  if (issue.number >= 138 && issue.number <= 151) {
    const m34 = milestones.find((m) => m.title.startsWith("v3.4.0"));
    if (m34) return m34.id;
  }

  // 3. EP Number mapping
  const epMatch = /\[EP-(\d+)\]/i.exec(issue.title);
  if (epMatch && epMatch[1]) {
    const epNum = parseInt(epMatch[1], 10);
    // EP-48, EP-49 -> v3.4.0
    if (epNum === 48 || epNum === 49) {
      const m = milestones.find((m) => m.title.startsWith("v3.4.0"));
      if (m) return m.id;
    }
    // EP-50, EP-51 -> v4.0.0
    if (epNum === 50 || epNum === 51) {
      const m = milestones.find((m) => m.title.startsWith("v4.0.0"));
      if (m) return m.id;
    }
    // EP-45, EP-46 -> v3.3.0
    if (epNum === 45 || epNum === 46) {
      const m = milestones.find((m) => m.title.startsWith("v3.3.0"));
      if (m) return m.id;
    }
  }

  // 4. FR Number mapping -> v1.9.0
  if (issue.title.includes("[FR-")) {
    const m = milestones.find((m) => m.title.startsWith("v1.9.0"));
    if (m) return m.id;
  }

  // 5. Specific early governance issues (#15-#18) -> v1.10.0
  if ([15, 16, 17, 18].includes(issue.number)) {
    const m = milestones.find((m) => m.title.startsWith("v1.10.0"));
    if (m) return m.id;
  }

  // 6. Default for currently open issues -> active minor release (v3.4.0)
  if (issue.state === "open") {
    const active = milestones.find((m) => m.title.startsWith("v3.4.0"));
    if (active) return active.id;
  }

  return null;
}

/**
 * Extracts a clean semantic version string from a milestone title.
 * Example: "v3.4.0 - Semantic Neural Vector Embeddings & 3D UI" -> "3.4.0"
 */
export function extractVersionFromMilestoneTitle(title: string): string | null {
  const match = /^v?(\d+\.\d+\.\d+)/i.exec(title.trim());
  return match && match[1] ? match[1] : null;
}
