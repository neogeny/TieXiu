// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;

#[derive(Clone, Debug)]
pub struct Rule<'r, C: Ctx<C>> {
    pub name: &'r str,
    pub exp: &'r dyn CanParse<C>,
}

impl<'r, C: Ctx<C>> CanParse<C> for Rule<'r, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => Ok((new_ctx, cst.distill())),
            err => err,
        }
    }
}


