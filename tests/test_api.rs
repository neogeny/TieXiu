// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

#[test]
fn test_parse() {
    let tree = tiexiu::api::parse_grammar("start = /a/", &[]).expect("Failed to parse");
    let parser = tiexiu::api::compile("start = /a/", &[]).expect("Failed to compile");

    eprintln!("TREE\n{:?}", tree);
    eprintln!("PARSER\n{:?}", parser);
    use tiexiu::peg::ExpKind;
    for rule in parser.rules() {
        match &rule.exp.kind {
            ExpKind::Sequence(exps) => {
                assert_eq!(exps.len(), 1, "Sequence should have 1 item");
            }
            other => panic!("Unexpected: {:?}", other),
        }
    }
}

#[test]
fn test_parse_to_json() {
    let json_str = tiexiu::api::parse_to_json("start = /a/", "a", &[]).expect("Failed to parse");
    eprintln!("TREE {:?}", json_str);
    assert!(json_str.contains("\"a\""));
}
