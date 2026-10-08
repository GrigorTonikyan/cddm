/**
 * Types and interfaces for the WebUI Code Editor and Split Diff Studio.
 */

export interface FileReadResponse {
  path: string;
  content: string;
  language: string;
  size_bytes: number;
  total_lines: number;
  modified_timestamp_ms: number;
}

export interface FileWriteRequest {
  path: string;
  content: string;
}

export interface FileWriteResponse {
  path: string;
  bytes_written: number;
  success: boolean;
  modified_timestamp_ms: number;
}

export interface WorkspaceFileEntry {
  path: string;
  name: string;
  is_dir: boolean;
  size_bytes: number;
  language?: string;
}

export interface FileTreeResponse {
  root: string;
  files: WorkspaceFileEntry[];
}
