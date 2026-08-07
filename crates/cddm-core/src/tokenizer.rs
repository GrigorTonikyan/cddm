use crate::grammar::LanguageGrammar;
use crate::types::{LineSpan, NormalizedToken};

/// Tokenizes a source string into normalized tokens and their line spans.
pub fn tokenize(
    source: &str,
    grammar: &LanguageGrammar,
    _normalize_type2: bool,
) -> Vec<(NormalizedToken, LineSpan)> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();
    let mut current_line = 1;

    while let Some(&(offset, ch)) = chars.peek() {
        // Handle newlines for line tracking
        if ch == '\n' {
            current_line += 1;
            chars.next();
            continue;
        }

        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        // Handle line comments
        if !grammar.line_comment.is_empty() && source[offset..].starts_with(grammar.line_comment) {
            // Consume until newline
            while let Some(&(_, c)) = chars.peek() {
                if c == '\n' {
                    break;
                }
                chars.next();
            }
            continue;
        }

        // Handle block comments
        if let Some((start_delim, end_delim)) = grammar.block_comment {
            if source[offset..].starts_with(start_delim) {
                for _ in 0..start_delim.len() {
                    chars.next();
                }
                while let Some(&(inner_offset, c)) = chars.peek() {
                    if source[inner_offset..].starts_with(end_delim) {
                        for _ in 0..end_delim.len() {
                            chars.next();
                        }
                        break;
                    }
                    if c == '\n' {
                        current_line += 1;
                    }
                    chars.next();
                }
                continue;
            }
        }

        // Strings
        if ch == '"' || ch == '\'' || ch == '`' {
            let quote = ch;
            chars.next();
            let start_line = current_line;
            let mut escaped = false;
            
            while let Some(&(_, c)) = chars.peek() {
                chars.next();
                if c == '\n' {
                    current_line += 1;
                }
                if escaped {
                    escaped = false;
                    continue;
                }
                if c == '\\' {
                    escaped = true;
                    continue;
                }
                if c == quote {
                    break;
                }
            }
            
            tokens.push((
                NormalizedToken::StringLiteral,
                LineSpan {
                    line_start: start_line,
                    line_end: current_line,
                    byte_offset: offset,
                },
            ));
            continue;
        }

        // Numbers
        if ch.is_ascii_digit() {
            let start_line = current_line;
            while let Some(&(_, c)) = chars.peek() {
                if c.is_ascii_alphanumeric() || c == '.' || c == '_' {
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push((
                NormalizedToken::NumericLiteral,
                LineSpan {
                    line_start: start_line,
                    line_end: current_line,
                    byte_offset: offset,
                },
            ));
            continue;
        }

        // Identifiers & Keywords
        if ch.is_alphabetic() || ch == '_' {
            let mut end_offset = offset;
            let start_line = current_line;
            while let Some(&(idx, c)) = chars.peek() {
                if c.is_alphanumeric() || c == '_' {
                    end_offset = idx + c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            let word = &source[offset..end_offset];
            
            let mut is_keyword = false;
            let mut keyword_id = 0;
            for (i, &kw) in grammar.keywords.iter().enumerate() {
                if kw == word {
                    is_keyword = true;
                    keyword_id = i as u16;
                    break;
                }
            }

            let token = if is_keyword {
                NormalizedToken::Keyword(keyword_id)
            } else {
                NormalizedToken::Identifier
            };

            tokens.push((
                token,
                LineSpan {
                    line_start: start_line,
                    line_end: current_line,
                    byte_offset: offset,
                },
            ));
            continue;
        }

        // Punctuation
        chars.next();
        tokens.push((
            NormalizedToken::Punctuation(ch as u8),
            LineSpan {
                line_start: current_line,
                line_end: current_line,
                byte_offset: offset,
            },
        ));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::get_grammar_for_path;
    use std::path::Path;

    #[test]
    fn test_tokenize_rust() {
        let grammar = get_grammar_for_path(Path::new("test.rs")).unwrap();
        let source = r#"
            // This is a comment
            fn main() {
                let x = 42;
                let s = "hello";
                /* block
                   comment */
                println!(s);
            }
        "#;
        
        let tokens = tokenize(source, grammar, true);
        let keywords = tokens.iter().filter(|(t, _)| matches!(t, NormalizedToken::Keyword(_))).count();
        assert!(keywords > 0, "Should have keywords (fn, let)");
        
        let strings = tokens.iter().filter(|(t, _)| matches!(t, NormalizedToken::StringLiteral)).count();
        assert_eq!(strings, 1, "Should have one string literal");
        
        let numbers = tokens.iter().filter(|(t, _)| matches!(t, NormalizedToken::NumericLiteral)).count();
        assert_eq!(numbers, 1, "Should have one numeric literal");
    }

    #[test]
    fn test_tokenize_ts() {
        let grammar = get_grammar_for_path(Path::new("test.ts")).unwrap();
        let source = r#"
            function add(a: number, b: number): number {
                return a + b;
            }
        "#;
        
        let tokens = tokenize(source, grammar, true);
        assert!(!tokens.is_empty());
    }
}
