// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's grammar/ebnf_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use tiexiu::api::compile;
use tiexiu::Result;

#[test]
fn test_parse_ebnf() -> Result<()> {
    let grammar = r#"
        /*
            Example of a grammar that mixes TatSu and EBNF
        */
        @@grammar :: TatSu

        start := expression $

        expression := expression '+' term | expression '-' term | term

        term := term '*' factor | term '/' factor | factor

        factor := '(' expression ')' | number

        number := /\d+/
    "#;

    let model = compile(grammar, &[])?;
    assert!(!model.to_string().is_empty());
    Ok(())
}

#[test]
fn test_optional() -> Result<()> {
    let grammar = r#"
        start:  '[' /abc/

        other := 'xyz'?
    "#;

    let model = compile(grammar, &[])?;
    let _ = model;
    Ok(())
}

#[test]
fn test_one_line_grammar() -> Result<()> {
    let grammar = r#"
        start: lisp

        lisp: sexp | list | symbol

        sexp[SExp]: '(' cons=lisp '.' ~ cdr=lisp ')'

        list[List]: '(' ={lisp} ')'

        symbol[Symbol]: /[^\s().]+/

    "#;

    let model = compile(grammar, &[])?;
    assert!(model.to_string().contains("lisp"));
    Ok(())
}
