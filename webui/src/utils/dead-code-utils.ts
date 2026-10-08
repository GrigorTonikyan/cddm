import type { CloneLocation } from "../types/cddm-types";
import type { DeadCodeItem } from "../types/dead-code-types";

/**
 * Normalizes file paths for cross-platform comparison.
 */
export function normalizePath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\.\//, "").toLowerCase().trim();
}

/**
 * Checks if two paths refer to the same file (either exact or suffix match).
 */
export function pathsMatch(pathA: string, pathB: string): boolean {
  const normA = normalizePath(pathA);
  const normB = normalizePath(pathB);
  if (normA === normB) return true;
  return normA.endsWith(`/${normB}`) || normB.endsWith(`/${normA}`);
}

/**
 * Finds a matching dead code item for a given file and line range.
 */
export function findMatchingDeadCodeItem(
  items: DeadCodeItem[] | undefined,
  file: string,
  lineStart?: number,
  lineEnd?: number,
): DeadCodeItem | undefined {
  if (!items || items.length === 0 || !file) return undefined;

  for (const item of items) {
    if (pathsMatch(item.file_path, file)) {
      if (lineStart === undefined || lineEnd === undefined) {
        return item;
      }
      // Check for line range overlap: [lineStart, lineEnd] and [item.line_start, item.line_end]
      const overlapStart = Math.max(lineStart, item.line_start);
      const overlapEnd = Math.min(lineEnd, item.line_end);
      if (overlapStart <= overlapEnd) {
        return item;
      }
    }
  }

  return undefined;
}

/**
 * Checks if a clone pair intersects with detected dead code items.
 */
export function isClonePairDead(
  items: DeadCodeItem[] | undefined,
  fileA: string,
  linesA: [number, number],
  fileB: string,
  linesB: [number, number],
): { isDead: boolean; itemA?: DeadCodeItem; itemB?: DeadCodeItem } {
  if (!items || items.length === 0) {
    return { isDead: false };
  }

  const itemA = findMatchingDeadCodeItem(items, fileA, linesA[0], linesA[1]);
  const itemB = findMatchingDeadCodeItem(items, fileB, linesB[0], linesB[1]);

  return {
    isDead: Boolean(itemA || itemB),
    itemA,
    itemB,
  };
}

/**
 * Checks if any location in a clone cluster intersects with dead code.
 */
export function isCloneClusterDead(
  items: DeadCodeItem[] | undefined,
  locations: CloneLocation[] | undefined,
): { isDead: boolean; deadLocationsCount: number; matchedItems: DeadCodeItem[] } {
  if (!items || items.length === 0 || !locations || locations.length === 0) {
    return { isDead: false, deadLocationsCount: 0, matchedItems: [] };
  }

  const matchedItems: DeadCodeItem[] = [];
  let deadLocationsCount = 0;

  for (const loc of locations) {
    const item = findMatchingDeadCodeItem(items, loc.file, loc.start_line, loc.end_line);
    if (item) {
      deadLocationsCount++;
      if (!matchedItems.some((m) => m.id === item.id)) {
        matchedItems.push(item);
      }
    }
  }

  return {
    isDead: deadLocationsCount > 0,
    deadLocationsCount,
    matchedItems,
  };
}
