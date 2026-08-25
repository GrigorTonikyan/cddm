#![forbid(unsafe_code)]

use super::languages_core::CORE_LANGUAGES;
use super::languages_polyglot::POLYGLOT_LANGUAGES;
use super::types::LanguageGrammar;
use std::path::Path;

/// Static list of all supported language grammars.
pub const SUPPORTED_LANGUAGES: &[LanguageGrammar] = &[
    CORE_LANGUAGES[0],
    CORE_LANGUAGES[1],
    CORE_LANGUAGES[2],
    CORE_LANGUAGES[3],
    CORE_LANGUAGES[4],
    CORE_LANGUAGES[5],
    CORE_LANGUAGES[6],
    CORE_LANGUAGES[7],
    CORE_LANGUAGES[8],
    CORE_LANGUAGES[9],
    CORE_LANGUAGES[10],
    CORE_LANGUAGES[11],
    CORE_LANGUAGES[12],
    POLYGLOT_LANGUAGES[0],
    POLYGLOT_LANGUAGES[1],
    POLYGLOT_LANGUAGES[2],
    POLYGLOT_LANGUAGES[3],
    POLYGLOT_LANGUAGES[4],
    POLYGLOT_LANGUAGES[5],
    POLYGLOT_LANGUAGES[6],
    POLYGLOT_LANGUAGES[7],
    POLYGLOT_LANGUAGES[8],
    POLYGLOT_LANGUAGES[9],
];

/// Gets the grammar definition for a given file path based on its extension.
///
/// Returns `None` if the extension is not recognized.
pub fn get_grammar_for_path(path: &Path) -> Option<&'static LanguageGrammar> {
    if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
        let lower = file_name.to_lowercase();
        if (lower == "dockerfile"
            || lower.starts_with("dockerfile.")
            || lower == "containerfile"
            || lower.starts_with("containerfile."))
            && let Some(g) = SUPPORTED_LANGUAGES.iter().find(|g| g.name == "Dockerfile")
        {
            return Some(g);
        }
    }
    let ext = path.extension()?.to_str()?;
    let ext = ext.to_lowercase();
    SUPPORTED_LANGUAGES
        .iter()
        .find(|&grammar| grammar.extensions.contains(&ext.as_str()))
}
