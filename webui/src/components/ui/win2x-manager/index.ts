/**
 * `win2x-manager` - Universal Self-Contained High-Performance Windowing System
 *
 * Standalone pure window management package entry point.
 */

// Constants
export * from "./constants/win2x-constants";

// Core Types & Math Engines
export * from "./core/types";
export * from "./core/geometry-engine";
export * from "./core/pointer-driver";
export * from "./core/storage-adapter";

// React Lifecycle Hooks
export * from "./hooks/use-body-scroll-lock";
export * from "./hooks/use-pointer-drag";
export * from "./hooks/use-pointer-resize";

// Context & Window Management Provider
export * from "./context/win2x-manager-context";
export * from "./hooks/use-window-manager";

// Window Subsystem Components
export * from "./components/resize-handle/resize-handle";
export * from "./components/resize-handle-group/resize-handle-group";
export * from "./components/window-controls/window-controls";
export * from "./components/title-bar/title-bar";
export * from "./components/win2x-window/win2x-window";
export * from "./components/tab-bar/tab-bar";
export * from "./components/dock-bar/dock-bar";
export * from "./components/snap-ghost-guide/snap-ghost-guide";
export * from "./components/snap-layouts-menu/snap-layouts-menu";
export * from "./components/titlebar-context-menu/titlebar-context-menu";
export * from "./components/snap-assist/snap-assist-modal";
