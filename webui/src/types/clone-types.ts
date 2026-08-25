/**
 * Clone-related domain types for CDDM WebUI.
 */

/**
 * Clone type classification.
 */
export type CloneType = "Exact" | "Renamed" | "NearMiss" | "Semantic";

/**
 * Clone location occurrence.
 */
export interface CloneLocation {
  file: string;
  start_line: number;
  end_line: number;
  author?: string;
}

/**
 * N-way clone cluster (equivalence class).
 */
export interface CloneCluster {
  id: number;
  clone_type: CloneType;
  token_count: number;
  similarity: number;
  fragment_hash: string;
  occurrences: CloneLocation[];
}

/**
 * Clone pair result.
 */
export interface ClonePair {
  file_a: string;
  start_line_a: number;
  end_line_a: number;
  file_b: string;
  start_line_b: number;
  end_line_b: number;
  token_count: number;
  similarity: number;
  fragment_hash: string;
  clone_type: CloneType;
  author_a?: string;
  author_b?: string;
}
