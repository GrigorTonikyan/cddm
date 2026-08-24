/**
 * Universal TypeScript type definitions for the win2x-manager subsystem.
 */

import {
  ContextMenuAction,
  OutsideClickAction,
  PerformanceProfile,
  ResizeDirection,
  SnapLayoutPreset,
  SnapZone,
  WindowLayoutMode,
  Win2xTheme,
  WIN2X_DEFAULTS,
} from "../constants/win2x-constants";

export type {
  ContextMenuAction,
  ResizeDirection,
  PerformanceProfile,
  OutsideClickAction,
  SnapLayoutPreset,
  SnapZone,
  WindowLayoutMode,
  Win2xTheme,
};

export interface Win2xRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface TabItemData {
  id: string;
  title: string;
  icon?: React.ReactNode;
  badgeCount?: number;
  badgeVariant?: string;
  closable?: boolean;
  disabled?: boolean;
}

export interface SnapLayoutSlot {
  index: number;
  label: string;
  rect: (viewportW: number, viewportH: number) => Win2xRect;
}

export interface SnapLayoutDefinition {
  preset: SnapLayoutPreset;
  title: string;
  slots: SnapLayoutSlot[];
}

export interface SnapAssistSession {
  preset: SnapLayoutPreset;
  activeSlotIndex: number;
  filledSlots: Map<number, string>; // slotIndex -> windowId
  sourceWindowId: string;
}

export interface WindowRegistration {
  id: string;
  windowType?: string;
  title: string;
  subtitle?: string;
  badge?: string;
  icon?: React.ReactNode;
  isMinimized: boolean;
  isMaximized: boolean;
  isModal?: boolean;
  zIndex: number;
  rect: Win2xRect;
  preSnapRect?: Win2xRect | null;
  snappedZone?: SnapZone | null;
  snappedPreset?: { preset: SnapLayoutPreset; slotIndex: number } | null;
  onClose?: () => void;
}

export interface Win2xManagerContextValue {
  windows: Map<string, WindowRegistration>;
  activeWindowId: string | null;
  snapAssistSession: SnapAssistSession | null;
  enableSnapLayouts: boolean;
  theme: Win2xTheme;
  setTheme: (theme: Win2xTheme) => void;
  focusWindow: (id: string) => void;
  registerWindow: (id: string, initialData: Omit<WindowRegistration, "zIndex">) => void;
  unregisterWindow: (id: string) => void;
  updateWindow: (id: string, updates: Partial<WindowRegistration>) => void;
  cascadeWindows: () => void;
  tileWindows: (mode: WindowLayoutMode) => void;
  minimizeAllWindows: () => void;
  restoreAllWindows: () => void;
  closeWindow: (id: string) => void;
  expandWindowInDirection: (id: string, direction: ResizeDirection) => void;
  applySnapPreset: (id: string, preset: SnapLayoutPreset, slotIndex: number) => void;
  dismissSnapAssist: () => void;
  assignWindowToSnapAssistSlot: (windowId: string, slotIndex: number) => void;
}

export interface Win2xWindowState extends Win2xRect {
  isMaximized: boolean;
  isMinimized: boolean;
  snappedZone?: SnapZone | null;
  snappedPreset?: { preset: SnapLayoutPreset; slotIndex: number } | null;
}

export const DEFAULT_WIN2X_WINDOW_STATE: Win2xWindowState = {
  x: -1,
  y: -1,
  width: WIN2X_DEFAULTS.INITIAL_WIDTH,
  height: WIN2X_DEFAULTS.INITIAL_HEIGHT,
  isMaximized: false,
  isMinimized: false,
};

export interface Win2xConfig {
  initialWidth?: number;
  initialHeight?: number;
  minWidth?: number;
  minHeight?: number;
  initialMinimized?: boolean;
  isModal?: boolean;
  minimizeOnOutsideClick?: boolean;
  closeOnOutsideClick?: boolean;
  performanceProfile?: PerformanceProfile;
  disableBlurWhileMoving?: boolean;
  enableSnapLayouts?: boolean;
  enableKeyboardShortcuts?: boolean;
  theme?: Win2xTheme;
  storageKey?: string;
  storageProvider?: StorageProvider;
}

export interface StorageProvider {
  getItem<T>(key: string): T | null;
  setItem<T>(key: string, value: T): void;
  removeItem(key: string): void;
}
