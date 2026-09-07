import { describe, expect, it } from "bun:test";
import {
  extractVersionFromMilestoneTitle,
  resolveIssueMilestone,
  type IssueMeta,
  type MilestoneMeta,
} from "./milestone-sync-engine";

describe("milestone-sync-engine", () => {
  const milestones: MilestoneMeta[] = [
    {
      id: 10,
      title: "v1.9.0 - Polyglot Parity",
      state: "closed",
      open_issues: 0,
      closed_issues: 5,
    },
    {
      id: 15,
      title: "v1.10.0 - AI AST Refactor",
      state: "closed",
      open_issues: 0,
      closed_issues: 4,
    },
    {
      id: 33,
      title: "v3.3.0 - Dead Clone Elimination",
      state: "closed",
      open_issues: 0,
      closed_issues: 6,
    },
    {
      id: 34,
      title: "v3.4.0 - Semantic Neural Vector Embeddings & 3D UI",
      state: "open",
      open_issues: 2,
      closed_issues: 1,
    },
    {
      id: 35,
      title: "v4.0.0 - Monorepo Federation",
      state: "open",
      open_issues: 3,
      closed_issues: 0,
    },
  ];

  it("extracts clean semantic version from milestone title", () => {
    expect(extractVersionFromMilestoneTitle("v3.4.0 - Semantic Neural Vector Embeddings")).toBe(
      "3.4.0",
    );
    expect(extractVersionFromMilestoneTitle("3.3.1 - Patch")).toBe("3.3.1");
    expect(extractVersionFromMilestoneTitle("invalid title")).toBeNull();
  });

  it("resolves milestone from explicit Target Milestone in issue body", () => {
    const issue: IssueMeta = {
      number: 86,
      title: "[EP-01] SARIF Reporter",
      body: "- **Target Milestone**: `v1.9.0`\n- Priority: High",
      state: "closed",
      milestone: null,
    };
    expect(resolveIssueMilestone(issue, milestones)).toBe(10);
  });

  it("resolves milestone for post-v3.3.0 tasks to v3.4.0", () => {
    const issue: IssueMeta = {
      number: 146,
      title: "[FEAT] Compact MCP tool mode",
      body: "Description",
      state: "open",
      milestone: null,
    };
    expect(resolveIssueMilestone(issue, milestones)).toBe(34);
  });

  it("resolves EP number to corresponding milestone", () => {
    const ep48: IssueMeta = {
      number: 130,
      title: "[EP-48] Subword Vector Embeddings",
      body: "",
      state: "open",
      milestone: null,
    };
    expect(resolveIssueMilestone(ep48, milestones)).toBe(34);

    const ep51: IssueMeta = {
      number: 133,
      title: "[EP-51] AVX-512 SIMD Vector Acceleration",
      body: "",
      state: "open",
      milestone: null,
    };
    expect(resolveIssueMilestone(ep51, milestones)).toBe(35);
  });

  it("defaults unassigned open issues to active minor release milestone", () => {
    const issue: IssueMeta = {
      number: 200,
      title: "New feature",
      body: "Some body",
      state: "open",
      milestone: null,
    };
    expect(resolveIssueMilestone(issue, milestones)).toBe(34);
  });
});
