// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for semantics - uses compile() which has BUG

#[test]
fn test_semantics_not_class() {
    let grammar = r#"
        start::sum = {number}+ $ ;
        number::int = /\d+/ ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
