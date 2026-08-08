/**
 * Path formatting utilities for CDDM WebUI.
 */

export interface FormattedPath {
  directory: string;
  filename: string;
  fullNormalized: string;
}

/**
 * Normalizes backslashes to forward slashes and separates directory from filename.
 */
export function parsePath(rawPath: string): FormattedPath {
  const fullNormalized = rawPath.replace(/\\/g, "/");
  const lastSlashIndex = fullNormalized.lastIndexOf("/");

  if (lastSlashIndex === -1) {
    return {
      directory: "",
      filename: fullNormalized,
      fullNormalized,
    };
  }

  return {
    directory: fullNormalized.substring(0, lastSlashIndex + 1),
    filename: fullNormalized.substring(lastSlashIndex + 1),
    fullNormalized,
  };
}

/**
 * Language color mapping for visual charts and badges.
 */
export const LANGUAGE_COLORS: Record<string, { bg: string; text: string; bar: string; border: string }> = {
  Python: { bg: "bg-amber-500/10", text: "text-amber-400", bar: "bg-amber-500", border: "border-amber-500/30" },
  TypeScript: { bg: "bg-blue-500/10", text: "text-blue-400", bar: "bg-blue-500", border: "border-blue-500/30" },
  JavaScript: { bg: "bg-yellow-500/10", text: "text-yellow-400", bar: "bg-yellow-500", border: "border-yellow-500/30" },
  Rust: { bg: "bg-orange-500/10", text: "text-orange-400", bar: "bg-orange-500", border: "border-orange-500/30" },
  JSON: { bg: "bg-emerald-500/10", text: "text-emerald-400", bar: "bg-emerald-500", border: "border-emerald-500/30" },
  CSS: { bg: "bg-sky-500/10", text: "text-sky-400", bar: "bg-sky-500", border: "border-sky-500/30" },
  HTML: { bg: "bg-rose-500/10", text: "text-rose-400", bar: "bg-rose-500", border: "border-rose-500/30" },
  Go: { bg: "bg-cyan-500/10", text: "text-cyan-400", bar: "bg-cyan-500", border: "border-cyan-500/30" },
  C: { bg: "bg-purple-500/10", text: "text-purple-400", bar: "bg-purple-500", border: "border-purple-500/30" },
  "C++": { bg: "bg-pink-500/10", text: "text-pink-400", bar: "bg-pink-500", border: "border-pink-500/30" },
};

export function getLanguageStyle(lang: string) {
  return (
    LANGUAGE_COLORS[lang] || {
      bg: "bg-indigo-500/10",
      text: "text-indigo-400",
      bar: "bg-indigo-500",
      border: "border-indigo-500/30",
    }
  );
}
