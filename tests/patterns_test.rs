// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Pattern Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn simple_pattern() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /\d+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "123", &[])?;
    assert_eq!(tree.to_value(), json!("123"));
    Ok(())
}

#[test]
fn pattern_with_letters() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /[a-z]+/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!("hello"));
    Ok(())
}

#[test]
fn pattern_with_anchors() -> Result<()> {
    let grammar = r#"
        @@grammar :: Test
        start: /^start/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "start", &[])?;
    assert_eq!(tree.to_value(), json!("start"));
    Ok(())
}

#[test]
    #[ignore = "ignorecase directive not working"]
    fn pattern_case_insensitive() -> Result<()> {
        let grammar = r#"
            @@ignorecase :: True
            start: /hello/
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let tree = parse_input(&grammar, "HELLO", &[])?;
        assert_eq!(tree.to_value(), json!("HELLO"));
        Ok(())
    }

#[test]
fn regex_character_classes() -> Result<()> {
    let grammar = r#"
        start: /[A-Za-z_]\w*/
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello_world", &[])?;
    assert_eq!(tree.to_value(), json!("hello_world"));
    Ok(())
}
