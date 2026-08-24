import React, { useState } from "react";
import { Copy, Check } from "lucide-react";
import {
  CODE_BLOCK_VARIANTS,
  CodeBlockVariant,
  UI_ARIA_LABELS,
  UI_DATA_ATTRS,
  UI_EMPTY_PLACEHOLDERS,
  UI_TIMEOUTS,
} from "../../constants/ui-constants";
import styles from "./code-block.module.css";

export interface CodeBlockProps {
  code: string;
  filename?: string;
  lineRange?: string;
  variant?: CodeBlockVariant;
  showCopy?: boolean;
  className?: string;
  emptyPlaceholder?: string;
}

const variantClassMap: Record<CodeBlockVariant, string> = {
  [CODE_BLOCK_VARIANTS.NEUTRAL]: styles.variantNeutral || "",
  [CODE_BLOCK_VARIANTS.ADDED]: styles.variantAdded || "",
  [CODE_BLOCK_VARIANTS.REMOVED]: styles.variantRemoved || "",
  [CODE_BLOCK_VARIANTS.HIGHLIGHT]: styles.variantHighlight || "",
};

/**
 * Universal molecular code block strictly enforcing horizontal scrolling and zero text wrapping.
 */
export const CodeBlock: React.FC<CodeBlockProps> = ({
  code,
  filename,
  lineRange,
  variant = CODE_BLOCK_VARIANTS.NEUTRAL,
  showCopy = true,
  className = "",
  emptyPlaceholder = UI_EMPTY_PLACEHOLDERS.CODE,
}) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    if (!code) return;
    void navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), UI_TIMEOUTS.COPY_FEEDBACK_MS);
  };

  const hasContent = code && code.trim().length > 0;
  const combinedClass = `${styles.container || ""} ${variantClassMap[variant]} ${className}`.trim();

  return (
    <div className={combinedClass} {...{ [UI_DATA_ATTRS.CODE_BLOCK]: true }}>
      {(filename || lineRange || showCopy) && (
        <div className={styles.header}>
          <div className={styles.fileInfo}>
            {filename && <span className={styles.filename}>{filename}</span>}
            {lineRange && <span className={styles.lineRange}>{lineRange}</span>}
          </div>

          {showCopy && hasContent && (
            <button
              type="button"
              onClick={handleCopy}
              className={styles.copyButton}
              title={UI_ARIA_LABELS.COPY_SNIPPET}
              aria-label={UI_ARIA_LABELS.COPY_SNIPPET}
            >
              {copied ? (
                <Check className={`w-3.5 h-3.5 ${styles.checkIcon || ""}`} />
              ) : (
                <Copy className="w-3.5 h-3.5" />
              )}
            </button>
          )}
        </div>
      )}

      {/* Code Area - Enforces horizontal scrolling without line wrapping */}
      <div className={styles.codeArea}>
        {hasContent ? (
          <pre className={styles.pre}>{code}</pre>
        ) : (
          <span className={styles.emptyPlaceholder}>{emptyPlaceholder}</span>
        )}
      </div>
    </div>
  );
};
