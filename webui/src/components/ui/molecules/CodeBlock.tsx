import React, { useState } from "react";
import { Copy, Check } from "lucide-react";

export interface CodeBlockProps {
  code: string;
  filename?: string;
  lineRange?: string;
  variant?: "neutral" | "added" | "removed" | "highlight";
  showCopy?: boolean;
  maxHeightClass?: string;
  className?: string;
  emptyPlaceholder?: string;
}

const variantStyles = {
  neutral: "bg-slate-950/80 border-slate-800/80 text-slate-300",
  added: "bg-emerald-950/30 border-emerald-900/40 text-emerald-300",
  removed: "bg-rose-950/30 border-rose-900/40 text-rose-300",
  highlight: "bg-indigo-950/40 border-indigo-800/50 text-indigo-200",
};

/**
 * Universal molecular code panel strictly enforcing horizontal scrolling and zero text wrapping.
 */
export const CodeBlock: React.FC<CodeBlockProps> = ({
  code,
  filename,
  lineRange,
  variant = "neutral",
  showCopy = true,
  maxHeightClass = "max-h-60",
  className = "",
  emptyPlaceholder = "<empty>",
}) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    if (!code) return;
    void navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const hasContent = code && code.trim().length > 0;

  return (
    <div
      className={`rounded-xl border flex flex-col min-w-0 ${variantStyles[variant]} ${className}`}
      data-code-block
    >
      {(filename || lineRange || showCopy) && (
        <div className="px-3 py-1.5 bg-slate-900/60 border-b border-inherit flex items-center justify-between text-xs font-mono select-none shrink-0">
          <div className="flex items-center gap-1.5 truncate">
            {filename && <span className="font-semibold text-slate-200 truncate">{filename}</span>}
            {lineRange && <span className="text-slate-500">{lineRange}</span>}
          </div>

          {showCopy && hasContent && (
            <button
              type="button"
              onClick={handleCopy}
              className="p-1 hover:bg-slate-800/80 text-slate-400 hover:text-slate-200 rounded transition-colors"
              title="Copy snippet"
              aria-label="Copy snippet"
            >
              {copied ? (
                <Check className="w-3.5 h-3.5 text-emerald-400" />
              ) : (
                <Copy className="w-3.5 h-3.5" />
              )}
            </button>
          )}
        </div>
      )}

      {/* Code Area - Enforces horizontal scrolling without line wrapping */}
      <div
        className={`p-3 font-mono text-xs overflow-x-auto overflow-y-auto ${maxHeightClass} select-text leading-relaxed`}
      >
        {hasContent ? (
          <pre className="whitespace-pre font-mono overflow-visible inline-block min-w-full">
            {code}
          </pre>
        ) : (
          <span className="text-slate-600 italic font-mono select-none">{emptyPlaceholder}</span>
        )}
      </div>
    </div>
  );
};
