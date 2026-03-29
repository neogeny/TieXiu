// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Pattern {
    pub pattern: String
}

impl<C: Ctx> CanParse<C> for Pattern
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        if ctx.pattern(&self.pattern) {
            Ok((ctx, Cst::Nil))
        }
        else {
            Err(ctx)
        }
    }
}

