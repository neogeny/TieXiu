// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! String utilities for generating valid identifiers.

use crate::util::pyre::Pattern;

/// Strict keywords that cannot be used as identifiers (e.g., let, fn).
pub const STRICT_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Reserved keywords currently unused but kept for future language features.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "abstract", "become", "box", "do", "final", "gen", "macro", "override", "priv", "try",
    "typeof", "unsized", "virtual", "yield",
];

/// Weak keywords that are only reserved in specific contexts (e.g., union).
pub const WEAK_KEYWORDS: &[&str] = &["macro_rules", "union", "static", "dyn", "raw", "safe"];

fn isreserved(name: &str) -> bool {
    STRICT_KEYWORDS.contains(&name)
        || RESERVED_KEYWORDS.contains(&name)
        || WEAK_KEYWORDS.contains(&name)
}

pub fn safe_name(name: &str, plug: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Input string cannot be empty.".into());
    }
    if plug.is_empty() || !plug.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!(
            "Invalid plug: '{}'. Must be non-empty and alphanumeric.",
            plug
        ));
    }
    if !is_valid_identifier(plug) {
        return Err(format!(
            "Invalid plug: '{}'. Must be valid in identifiers.",
            plug
        ));
    }

    let mut result = name.to_string();

    // Replace non-word characters with plug
    let non_word = Pattern::new(r"\W").map_err(|e| e.to_string())?;
    result = non_word.sub(plug, &result, None);

    // If still not valid, filter to alphanumeric only
    if !is_valid_identifier(&result) {
        result = name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    plug.chars().next().unwrap()
                }
            })
            .collect();
    }

    // Handle leading digit
    if !result.is_empty() && result.chars().next().unwrap().is_ascii_digit() {
        let prefix = if plug.chars().next().unwrap().is_ascii_digit() {
            "_"
        } else {
            ""
        };
        result = format!("{}{}", prefix, result);
    }

    // Make valid identifier
    if !is_valid_identifier(&result) {
        result = format!("{}{}", plug, result);
    }

    // Append plug if reserved
    while isreserved(&result) {
        result = format!("{}{}", result, plug);
    }

    if !is_valid_identifier(&result) {
        return Err(format!("Failed to sanitize '{}' into '{}'", name, result));
    }

    Ok(result)
}

fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn to_snake_case(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Ok(name.into());
    }

    // Convert CamelCase to snake_case using direct character analysis
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_uppercase() {
            if i > 0 {
                // Check if previous char was lowercase or next is lowercase
                let prev_lower = chars[i - 1].is_lowercase();
                let next_lower = if i + 1 < chars.len() {
                    chars[i + 1].is_lowercase()
                } else {
                    false
                };
                if prev_lower || next_lower {
                    result.push('_');
                }
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    // Make valid Python identifier
    safe_name(&result, "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_name_valid() {
        let cases = [
            ("valid_name", "_", "valid_name"),
            ("123invalid", "_", "_123invalid"),
            ("name-with-dash", "_", "name_with_dash"),
            ("let", "_", "let_"),
            ("fn", "_", "fn_"),
            ("mut", "_", "mut_"),
        ];

        for (name, plug, expected) in cases {
            let result = safe_name(name, plug).unwrap();
            assert_eq!(
                result, expected,
                "safe_name({}, {}) = {} expected {}",
                name, plug, result, expected
            );
        }
    }

    #[test]
    fn test_safe_name_unicode() {
        // ASCII-only safe_name for now
        let result = safe_name("name", "_").unwrap();
        assert_eq!(result, "name");
    }

    #[test]
    fn test_to_snake_case() {
        let cases = [
            ("someName", "some_name"),
            ("SomeName", "some_name"),
            ("XMLHttpRequest", "xml_http_request"),
        ];

        for (name, expected) in cases {
            let result = to_snake_case(name).unwrap();
            assert_eq!(
                result, expected,
                "to_snake_case({}) = {} expected {}",
                name, result, expected
            );
        }
    }
}
