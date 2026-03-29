// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;

#[derive(Clone, Debug)]
pub struct Rule<'r> {
    pub name: &'static str,
    pub exp: &'r dyn CanParse,
}

impl<'r> CanParse for Rule<'r>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => Ok((new_ctx, cst.distill())),
            err => err,
        }
    }
}


