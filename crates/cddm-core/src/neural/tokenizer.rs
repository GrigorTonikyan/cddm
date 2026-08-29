#![forbid(unsafe_code)]

/// Subword code tokenizer for neural embedding generation.
#[derive(Debug)]
pub struct SubwordTokenizer;

impl SubwordTokenizer {
    /// Tokenizes source code into normalized subwords for neural feature projection.
    pub fn tokenize(code: &str) -> Vec<String> {
        let mut subwords = Vec::new();
        for raw_token in code.split(|c: char| !c.is_alphanumeric() && c != '_') {
            let cleaned = raw_token.trim_matches('_');
            if cleaned.is_empty() {
                continue;
            }

            // Split snake_case
            for snake_part in cleaned.split('_') {
                if snake_part.is_empty() {
                    continue;
                }
                // Split camelCase
                let camel_parts = Self::split_camel_case(snake_part);
                for part in camel_parts {
                    let lower = part.to_lowercase();
                    if !lower.is_empty() {
                        // Generate character 3-grams for robust subword modeling
                        if lower.len() >= 3 {
                            for i in 0..=lower.len().saturating_sub(3) {
                                subwords.push(lower[i..i + 3].to_string());
                            }
                        }
                        subwords.push(lower);
                    }
                }
            }
        }
        subwords
    }

    fn split_camel_case(s: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut prev_is_lower = false;

        for ch in s.chars() {
            if ch.is_uppercase() && prev_is_lower && !current.is_empty() {
                parts.push(current.clone());
                current.clear();
            }
            prev_is_lower = ch.is_lowercase();
            current.push(ch);
        }

        if !current.is_empty() {
            parts.push(current);
        }

        parts
    }
}
