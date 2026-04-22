fn parse_root(state: &mut Subject) {
    let initial_pos = state.pos;
    let mut g = guard(state, |s| s.pos = initial_pos);

    // Pass the GUARD 'g' where '&mut Subject' is expected.
    // Rust automatically turns &mut Guard into &mut Subject here.
    parse_expression(&mut *g);

    ScopeGuard::into_inner(g);
}

fn parse_expression(state: &mut Subject) {
    // This child doesn't even know a guard exists 'above' it.
    state.advance();
}
