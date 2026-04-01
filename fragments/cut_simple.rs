// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

impl Model {
    // Inside the Cut variant
    fn parse_cut<C: Cursor>(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.cut_seen = true;
        Ok((ctx, Cst::Nil))
    }
}
