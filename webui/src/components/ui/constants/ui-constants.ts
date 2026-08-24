/**
 * Centralized constants and enum definitions for the Atomic UI component library.
 * Zero hardcoded magic strings or numeric literals.
 */

export const BADGE_VARIANTS = {
  INDIGO: "indigo",
  EMERALD: "emerald",
  ROSE: "rose",
  AMBER: "amber",
  SLATE: "slate",
  CYAN: "cyan",
} as const;

export type BadgeVariant = (typeof BADGE_VARIANTS)[keyof typeof BADGE_VARIANTS];

export const BADGE_SIZES = {
  SM: "sm",
  MD: "md",
} as const;

export type BadgeSize = (typeof BADGE_SIZES)[keyof typeof BADGE_SIZES];

export const CODE_BLOCK_VARIANTS = {
  NEUTRAL: "neutral",
  ADDED: "added",
  REMOVED: "removed",
  HIGHLIGHT: "highlight",
} as const;

export type CodeBlockVariant = (typeof CODE_BLOCK_VARIANTS)[keyof typeof CODE_BLOCK_VARIANTS];

export const BUTTON_VARIANTS = {
  DEFAULT: "default",
  DANGER: "danger",
} as const;

export type ButtonVariant = (typeof BUTTON_VARIANTS)[keyof typeof BUTTON_VARIANTS];

export const BUTTON_SIZES = {
  SM: "sm",
  MD: "md",
} as const;

export type ButtonSize = (typeof BUTTON_SIZES)[keyof typeof BUTTON_SIZES];

export const UI_TIMEOUTS = {
  COPY_FEEDBACK_MS: 1500,
  FAST_ANIMATION_MS: 100,
  NORMAL_ANIMATION_MS: 150,
  SLOW_ANIMATION_MS: 200,
} as const;

export const UI_DATA_ATTRS = {
  BACKDROP: "data-cddm-backdrop",
  BADGE: "data-cddm-badge",
  ICON_BUTTON: "data-cddm-icon-button",
  COLLAPSIBLE_CARD: "data-cddm-collapsible-card",
  CODE_BLOCK: "data-cddm-code-block",
} as const;

export const UI_ARIA_LABELS = {
  COPY_SNIPPET: "Copy snippet",
  RESTORE_WINDOW: "Restore Window",
  CLOSE: "Close",
  MINIMIZE: "Minimize",
  MAXIMIZE: "Maximize",
  RESTORE: "Restore",
} as const;

export const UI_EMPTY_PLACEHOLDERS = {
  CODE: "<empty>",
} as const;
