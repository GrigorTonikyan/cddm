/**
 * Universal Storage Adapters for win2x-manager persistence.
 */

import { StorageProvider } from "./types";

export class LocalStorageAdapter implements StorageProvider {
  getItem<T>(key: string): T | null {
    if (typeof window === "undefined" || !window.localStorage) {
      return null;
    }
    try {
      const raw = window.localStorage.getItem(key);
      if (!raw) return null;
      return JSON.parse(raw) as T;
    } catch {
      return null;
    }
  }

  setItem<T>(key: string, value: T): void {
    if (typeof window === "undefined" || !window.localStorage) {
      return;
    }
    try {
      window.localStorage.setItem(key, JSON.stringify(value));
    } catch {
      // Ignore quota exceeded or permission errors safely
    }
  }

  removeItem(key: string): void {
    if (typeof window === "undefined" || !window.localStorage) {
      return;
    }
    try {
      window.localStorage.removeItem(key);
    } catch {
      // Safe ignore
    }
  }
}

export class MemoryAdapter implements StorageProvider {
  private store = new Map<string, unknown>();

  getItem<T>(key: string): T | null {
    const val = this.store.get(key);
    return val !== undefined ? (val as T) : null;
  }

  setItem<T>(key: string, value: T): void {
    this.store.set(key, value);
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  clear(): void {
    this.store.clear();
  }
}

export const defaultStorage = new LocalStorageAdapter();

/**
 * Saves window geometry state on 2 levels:
 * 1. uniqueId instance state (e.g. "win2x_state_refactor-advisor-fileA_fileB")
 * 2. windowType template state (e.g. "win2x_template_refactor-advisor")
 */
export function saveWindowState(
  storage: StorageProvider,
  uniqueId: string,
  state: {
    x: number;
    y: number;
    width: number;
    height: number;
    isMaximized: boolean;
    isMinimized: boolean;
  },
  windowType?: string,
  baseKey = "win2x",
): void {
  // Save specific instance state
  storage.setItem(`${baseKey}_state_${uniqueId}`, state);

  // Save template type defaults
  if (windowType) {
    storage.setItem(`${baseKey}_template_${windowType}`, {
      width: state.width,
      height: state.height,
      isMaximized: state.isMaximized,
    });
  }
}

/**
 * Loads window geometry state with 2-tier fallback hierarchy:
 * 1. uniqueId instance state
 * 2. windowType template state
 * 3. null (fallback to component defaults)
 */
export function loadWindowState(
  storage: StorageProvider,
  uniqueId: string,
  windowType?: string,
  baseKey = "win2x",
): {
  x: number;
  y: number;
  width: number;
  height: number;
  isMaximized: boolean;
  isMinimized: boolean;
} | null {
  // 1. Check unique instance state
  const instanceState = storage.getItem<{
    x: number;
    y: number;
    width: number;
    height: number;
    isMaximized: boolean;
    isMinimized: boolean;
  }>(`${baseKey}_state_${uniqueId}`);
  if (instanceState) return instanceState;

  // 2. Fallback to windowType template state
  if (windowType) {
    const templateState = storage.getItem<{
      width: number;
      height: number;
      isMaximized: boolean;
    }>(`${baseKey}_template_${windowType}`);
    if (templateState) {
      return {
        x: -1, // will be auto-centered by caller
        y: -1,
        width: templateState.width,
        height: templateState.height,
        isMaximized: templateState.isMaximized,
        isMinimized: false,
      };
    }
  }

  return null;
}
