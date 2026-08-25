#![forbid(unsafe_code)]

/// Defines syntax properties for a programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageGrammar {
    /// Name of the language (e.g. "Rust")
    pub name: &'static str,
    /// File extensions associated with this language (without leading dot)
    pub extensions: &'static [&'static str],
    /// Language keywords
    pub keywords: &'static [&'static str],
    /// Prefix for single-line comments
    pub line_comment: &'static str,
    /// Delimiters for multi-line block comments (start, end)
    pub block_comment: Option<(&'static str, &'static str)>,
}

pub const fn make_c_style_grammar(
    name: &'static str,
    extensions: &'static [&'static str],
    keywords: &'static [&'static str],
) -> LanguageGrammar {
    LanguageGrammar {
        name,
        extensions,
        keywords,
        line_comment: "//",
        block_comment: Some(("/*", "*/")),
    }
}
