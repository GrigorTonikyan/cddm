import React from "react";

// Tokenize a code line into syntax-highlighted spans
export const renderHighlightedLine = (text: string): React.ReactNode => {
  if (!text) return <span>&nbsp;</span>;

  // Simple token regex matching strings, comments, numbers, keywords
  const tokenRegex =
    /("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\/\/[^\n]*|#[^\n]*|\b(?:fn|function|def|const|let|var|return|if|else|for|while|import|from|export|class|struct|enum|pub|impl|type|interface|async|await|true|false|null|None)\b|\b\d+\b|[a-zA-Z_]\w*|[^\s\w])/g;

  const parts: React.ReactNode[] = [];
  let lastIndex = 0;
  let match: RegExpExecArray | null;

  while ((match = tokenRegex.exec(text)) !== null) {
    if (match.index > lastIndex) {
      parts.push(text.slice(lastIndex, match.index));
    }
    const token = match[0];
    if (token.startsWith("//") || token.startsWith("#")) {
      parts.push(
        <span key={match.index} className="text-slate-500 italic">
          {token}
        </span>,
      );
    } else if (
      (token.startsWith('"') && token.endsWith('"')) ||
      (token.startsWith("'") && token.endsWith("'"))
    ) {
      parts.push(
        <span key={match.index} className="text-emerald-400">
          {token}
        </span>,
      );
    } else if (/^\d+$/.test(token)) {
      parts.push(
        <span key={match.index} className="text-amber-400">
          {token}
        </span>,
      );
    } else if (
      /^(?:fn|function|def|const|let|var|return|if|else|for|while|import|from|export|class|struct|enum|pub|impl|type|interface|async|await|true|false|null|None)$/.test(
        token,
      )
    ) {
      parts.push(
        <span key={match.index} className="text-indigo-400 font-semibold">
          {token}
        </span>,
      );
    } else {
      parts.push(
        <span key={match.index} className="text-slate-200">
          {token}
        </span>,
      );
    }
    lastIndex = match.index + token.length;
  }

  if (lastIndex < text.length) {
    parts.push(text.slice(lastIndex));
  }

  return <>{parts}</>;
};
