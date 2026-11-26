/**
 * Legacy seed milestones and labels for Gitea Portal synchronization.
 */

import portalSeed from "./gitea-portal-seed.json";

export interface MilestoneSeed {
  title: string;
  description: string;
  due_on: string;
  state: "open" | "closed";
}

export interface LabelSeed {
  name: string;
  color: string;
  description: string;
}

export const MILESTONES: MilestoneSeed[] = portalSeed.milestones as MilestoneSeed[];

export const LABELS: LabelSeed[] = portalSeed.labels as LabelSeed[];
