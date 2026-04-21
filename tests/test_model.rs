// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's model_test.py

use tiexiu::Result;
use tiexiu::api::compile;

#[test]
fn test_children() -> Result<()> {
    let grammar = r#"
        @@grammar::Calc

        start = expression $ ;

        expression = term ;
        term = 'x' ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}

#[test]
fn test_node_kwargs() -> Result<()> {
    let grammar = r#"
        start = 'value' ;
    "#;

    compile(grammar, &[])?;
    Ok(())
}
