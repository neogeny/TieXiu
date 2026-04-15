// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for error handling

#[test]
fn test_missing_rule() {
    let grammar = r#"
        @@grammar::TestGrammar
        block = test ;
    "#;

    let _result = tiexiu::api::compile(grammar, &[]);
}

#[test]
fn test_error_exists() {
    // Simple check that Error type exists
    fn _check_error(_: tiexiu::Error) {}
}
