import React from "react";
import { MonacoEditorSurface } from "./MonacoEditorSurface";
import { FallbackEditorSurface } from "./FallbackEditorSurface";

export interface EditorSurfaceProps {
  onSave: () => void;
}

export const EditorSurface: React.FC<EditorSurfaceProps> = ({ onSave }) => {
  const isTestEnvironment =
    process.env.NODE_ENV === "test" || typeof window === "undefined" || !("matchMedia" in window);

  if (isTestEnvironment) {
    return <FallbackEditorSurface onSave={onSave} />;
  }

  return <MonacoEditorSurface onSave={onSave} />;
};
