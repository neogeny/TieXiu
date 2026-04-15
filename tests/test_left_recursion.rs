// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for left recursion - uses compile() which has BUG

#[test]
fn test_direct_left_recursion() {
    let grammar = r#"
        start = expr ;
        expr = expr '+' term | term ;
        term = /\d+/ ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
