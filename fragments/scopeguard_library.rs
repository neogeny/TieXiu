use scopeguard::guard;

fn parse_expression(state: &mut ParserState) -> Result<(), ParseError> {
    let initial_pos = state.cursor;

    // 1. Create the guard.
    // It takes a mutable reference to 'state'.
    // 'state' is now "locked" and cannot be used directly.
    let mut proxy = guard(state, |s| {
        println!("Error detected! Rolling back to {}", initial_pos);
        s.cursor = initial_pos;
    });

    // 2. Use the 'proxy' AS IF it were the 'state'.
    // Deref logic allows this call to work:
    proxy.advance_cursor()?;
    proxy.match_token("TIE")?;

    // 3. Success!
    // We "defuse" the guard by calling into_inner.
    // This returns the original reference and prevents the closure from running.
    let _ = scopeguard::ScopeGuard::into_inner(proxy);

    Ok(())
}
