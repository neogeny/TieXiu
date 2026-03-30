// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use tiexiu::contexts::StrCtx;
use tiexiu::grammars::Model;
use tiexiu::input::StrCursor;

fn scope() -> (Model, Model) {
    let a = Model::Token("a".into());
    let b = Model::Token("b".into());
    (a, b)
}

fn test_build() {
    let (a, b) = scope();
    let c = Model::Token("c".into());
    let v = Model::Void;
    // let cl = Call::new("test");
    let seq = Model::Sequence([a, b, c, v].into());

    let cur = StrCursor::new("a b c", 0);
    let ctx = StrCtx::new(cur);

    let cst = seq.parse(ctx);
    print!("{:?}", cst);
}

fn main() {
    println!("Hello, world!");
    test_build();
}
