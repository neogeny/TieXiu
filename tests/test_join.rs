// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for join - uses compile() which has BUG

#[test]
fn test_positive_join() {
    let grammar = r#"
        start = ','%{'x' 'y'}+ ;
    "#;
    let _result = tiexiu::api::compile(grammar, &[]);
}
