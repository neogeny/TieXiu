// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for constants - translated from TatSu's grammar/constants_test.py

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
#[ignore = "constant interpolation not yet implemented"]
fn test_constant_interpolation() -> Result<()> {
    let grammar = r#"
        start = a:number b: number i:`"seen: {a}, {b}"` $ ;
        number = /\d+/ ;
    "#;
    let _model = compile(grammar, &[])?;
    Ok(())
}
