#![forbid(unsafe_code)]

pub mod c_family;
pub mod jvm_mobile;
pub mod scripting;

use super::types::LanguageGrammar;

pub static CORE_LANGUAGES: &[LanguageGrammar] = &[
    c_family::RUST,
    scripting::TYPESCRIPT,
    scripting::JAVASCRIPT,
    scripting::PYTHON,
    jvm_mobile::GO,
    jvm_mobile::JAVA,
    c_family::C,
    c_family::CPP,
    c_family::CSHARP,
    scripting::RUBY,
    scripting::PHP,
    jvm_mobile::KOTLIN,
    jvm_mobile::SWIFT,
];
