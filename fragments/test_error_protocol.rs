#[test]
fn test_grammar_rule() -> Result<(), Nope<'static, StrCursor<'static>>> {
    let input = "fn 123_bad";
    let cursor = StrCursor::new(input, patterns);
    let parser = MyParser::new();

    // The '?' operator is the key here.
    // It will return the Err(Nope) immediately if the parse fails.
    parser.parse(cursor)?;

    Ok(())
}
