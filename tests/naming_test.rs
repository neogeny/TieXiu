//! Named Rules and Overrides Tests

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn named_capture() -> Result<()> {
    let grammar = r#"
        start: name='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!({"name": "hello"}));
    Ok(())
}

#[test]
fn override_singleton() -> Result<()> {
    let grammar = r#"
        start: ='hello'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "hello", &[])?;
    assert_eq!(tree.to_value(), json!("hello"));
    Ok(())
}

#[test]
#[ignore = "grammar parsing bug with RuleInclude"]
fn rule_include() -> Result<()> {
    let grammar = r#"
        start: >base
        base: 'hello'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

#[test]
#[ignore = "special form syntax not implemented"]
fn named_list() -> Result<()> {
    let grammar = r#"
        start: names+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}

#[test]
#[ignore = "special form syntax not implemented"]
fn override_list() -> Result<()> {
    let grammar = r#"
        start: @+:'a'
    "#;
    let _grammar = tiexiu::compile(grammar, &[])?;
    Ok(())
}