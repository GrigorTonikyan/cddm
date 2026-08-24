/**
 * Centralized constants and enum definitions for the win2x-manager windowing subsystem.
 * Zero hardcoded magic strings or numeric literals.
 */

export const WIN2X_RESIZE_DIRECTIONS = {
  TOP: "top",
  BOTTOM: "bottom",
  LEFT: "left",
  RIGHT: "right",
  TOP_LEFT: "top-left",
  TOP_RIGHT: "top-right",
  BOTTOM_LEFT: "bottom-left",
  BOTTOM_RIGHT: "bottom-right",
} as const;

export type ResizeDirection =
  (typeof WIN2X_RESIZE_DIRECTIONS)[keyof typeof WIN2X_RESIZE_DIRECTIONS];

export const WIN2X_PERFORMANCE_PROFILES = {
  EXTREME: "extreme",
  BALANCED: "balanced",
  QUALITY: "quality",
} as const;

export type PerformanceProfile =
  (typeof WIN2X_PERFORMANCE_PROFILES)[keyof typeof WIN2X_PERFORMANCE_PROFILES];

export const WIN2X_DEFAULTS = {
  INITIAL_WIDTH: 920,
  INITIAL_HEIGHT: 680,
  MIN_WIDTH: 460,
  MIN_HEIGHT: 340,
  MIN_VISIBLE_X_OFFSET: 100,
  MIN_VISIBLE_Y_OFFSET: 50,
  FALLBACK_VIEWPORT_WIDTH: 1920,
  FALLBACK_VIEWPORT_HEIGHT: 1080,
  DEFAULT_STORAGE_KEY: "win2x_window_state",
  CENTER_SAFETY_OFFSET_X: 20,
  CENTER_SAFETY_OFFSET_Y: 30,
  CASCADE_STEP: 40,
} as const;

export const WIN2X_ERRORS = {
  PROVIDER_MISSING: "useWindowManager must be used within a Win2xManagerProvider.",
} as const;

export const WIN2X_SNAP_ZONES = {
  NONE: "none",
  LEFT_HALF: "left-half",
  RIGHT_HALF: "right-half",
  TOP_MAXIMIZE: "top-maximize",
  TOP_LEFT: "top-left",
  TOP_RIGHT: "top-right",
  BOTTOM_LEFT: "bottom-left",
  BOTTOM_RIGHT: "bottom-right",
} as const;

export type SnapZone = (typeof WIN2X_SNAP_ZONES)[keyof typeof WIN2X_SNAP_ZONES];

export const WIN2X_LAYOUT_MODES = {
  CASCADE: "cascade",
  TILE_GRID: "tile-grid",
  TILE_HORIZONTAL: "tile-horizontal",
  TILE_VERTICAL: "tile-vertical",
} as const;

export type WindowLayoutMode = (typeof WIN2X_LAYOUT_MODES)[keyof typeof WIN2X_LAYOUT_MODES];

export const WIN2X_Z_INDEX = {
  BASE_WINDOW: 9900,
  ACTIVE_STEP: 2,
  SNAP_GHOST: 9890,
  DOCK_BAR: 9999,
} as const;

export const WIN2X_SNAP_LAYOUT_PRESETS = {
  TWO_EQUAL: "two-equal",
  TWO_UNEQUAL: "two-unequal",
  THREE_LEFT_MAIN: "three-left-main",
  THREE_RIGHT_MAIN: "three-right-main",
  FOUR_GRID: "four-grid",
  THREE_COLUMNS: "three-columns",
} as const;

export type SnapLayoutPreset =
  (typeof WIN2X_SNAP_LAYOUT_PRESETS)[keyof typeof WIN2X_SNAP_LAYOUT_PRESETS];

export const WIN2X_CONTEXT_MENU_ACTIONS = {
  RESTORE: "restore",
  MOVE: "move",
  SIZE: "size",
  MINIMIZE: "minimize",
  MAXIMIZE: "maximize",
  SNAP_LAYOUTS: "snap-layouts",
  CASCADE_ALL: "cascade-all",
  TILE_ALL: "tile-all",
  CLOSE: "close",
} as const;

export type ContextMenuAction =
  (typeof WIN2X_CONTEXT_MENU_ACTIONS)[keyof typeof WIN2X_CONTEXT_MENU_ACTIONS];

export const WIN2X_TIMINGS = {
  SNAP_LAYOUT_HOVER_DELAY_MS: 300,
  SNAP_HINT_DELAY_MS: 150,
  LONG_PRESS_DELAY_MS: 500,
} as const;

export const WIN2X_SNAP_DEFAULTS = {
  EDGE_THRESHOLD_PX: 24,
  CORNER_THRESHOLD_PX: 48,
  MAGNETIC_SNAP_THRESHOLD_PX: 16,
} as const;

export const WIN2X_THEMES = {
  DARK: "dark",
  LIGHT: "light",
  HIGH_CONTRAST: "high-contrast",
} as const;

export type Win2xTheme = (typeof WIN2X_THEMES)[keyof typeof WIN2X_THEMES];

export const WIN2X_DATA_ATTRS = {
  WINDOW: "data-win2x-window",
  TITLEBAR: "data-win2x-titlebar",
  MOVING: "data-moving",
  PROFILE: "data-profile",
  THEME: "data-win2x-theme",
  MINIMIZED_PILL: "data-win2x-minimized-pill",
  DOCK_CONTAINER: "data-win2x-dock-container",
  RESIZE_HANDLE: "data-win2x-resize-handle",
  RESIZE_HANDLES: "data-win2x-resize-handles",
  CONTROLS: "data-win2x-controls",
  ACTIVE: "data-active",
  SNAP_GHOST: "data-win2x-snap-ghost",
  TAB_BAR: "data-win2x-tab-bar",
  TAB_ITEM: "data-win2x-tab-item",
  SNAP_LAYOUTS_MENU: "data-win2x-snap-layouts-menu",
  CONTEXT_MENU: "data-win2x-context-menu",
  SNAP_ASSIST: "data-win2x-snap-assist",
} as const;

export const WIN2X_SHORTCUTS = {
  CASCADE: "Alt+Shift+C",
  TILE_GRID: "Alt+Shift+G",
  TILE_HORIZONTAL: "Alt+Shift+H",
  TILE_VERTICAL: "Alt+Shift+V",
  MINIMIZE_ALL: "Alt+Shift+M",
  RESTORE_ALL: "Alt+Shift+R",
} as const;

export const WIN2X_OUTSIDE_CLICK_ACTIONS = {
  MINIMIZE: "minimize",
  CLOSE: "close",
  NONE: "none",
} as const;

export type OutsideClickAction =
  (typeof WIN2X_OUTSIDE_CLICK_ACTIONS)[keyof typeof WIN2X_OUTSIDE_CLICK_ACTIONS];

export const WIN2X_KEYS = {
  ESCAPE: "Escape",
  ENTER: "Enter",
  SPACE: " ",
  KEY_C: "c",
  KEY_G: "g",
  KEY_H: "h",
  KEY_V: "v",
  KEY_M: "m",
  KEY_R: "r",
} as const;

export const WIN2X_ARIA_LABELS = {
  MINIMIZE: "Minimize",
  MAXIMIZE: "Maximize",
  RESTORE: "Restore",
  CLOSE: "Close",
  RESTORE_WINDOW: "Restore Window",
  CLOSE_MINIMIZED: "Close Minimized Window",
  SNAP_LAYOUTS: "Snap Layouts",
} as const;
