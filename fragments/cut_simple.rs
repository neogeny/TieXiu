// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

impl Model {
    // Inside the Cut variant
    fn parse_cut<C: Cursor>(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        ctx.cut_seen = true;
        Ok((ctx, Cst::Nil))
    }
}
