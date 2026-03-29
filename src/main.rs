// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use tiexiu::model::{Token, Sequence, Void, CanParse};
use tiexiu::input::StrCursor;
use tiexiu::engine::Ctx;


fn scope() -> (Token, Token) {
    let a = Token::new("a");
    let b = Token::new("b");
    return (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = Token::new("c");
    let v = Void{};
    // let cl = Call::new("test");
    let seq = Sequence::new(vec![&a, &b, &c, &v]);

    let cur = StrCursor::new("a b c", 0);
    let ctx = Ctx::new(&cur);

    let cst = seq.parse(ctx);
    print!("{:?}", cst);

}

fn main() {
    println!("Hello, world!");
    test_build();
}
