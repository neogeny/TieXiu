//! Constraints Tests (Lookahead and Cut)

use serde_json::json;
use tiexiu::parse_input;
use tiexiu::*;

#[test]
fn positive_lookahead() -> Result<()> {
    let grammar = r#"
        start: &'a' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}

#[test]
fn negative_lookahead() -> Result<()> {
    let grammar = r#"
        start: !'b' 'a'
    "#;
    let grammar = tiexiu::compile(grammar, &[])?;
    let tree = parse_input(&grammar, "a", &[])?;
    assert_eq!(tree.to_value(), json!("a"));
    Ok(())
}

#[test]
    #[ignore = "grammar may need whitespace directive"]
    fn cut() -> Result<()> {
        let grammar = r#"
            start: 'a' ~ 'b'
        "#;
        let grammar = tiexiu::compile(grammar, &[])?;
        let _tree = parse_input(&grammar, "ab", &[])?;
        Ok(())
    }