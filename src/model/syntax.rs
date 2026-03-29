// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Lookahead<'k> {
    pub exp: &'k dyn CanParse,
}

impl<'k> CanParse for Lookahead<'k>
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        let _ = self.exp.parse(ctx.clone())?;
        Ok((ctx, Cst::Nil))
    }
}


#[derive(Debug, Clone)]
pub struct NegativeLookahead<'k> {
    pub exp: &'k dyn CanParse,
}

impl<'k> CanParse for NegativeLookahead<'k>
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        if let Ok((_, _)) = self.exp.parse(ctx.clone()) {
            return Err(ctx)
        }
        Ok((ctx, Cst::Nil))
    }
}


#[derive(Debug, Clone)]
pub struct SkipTo<'s> {
    pub exp: &'s dyn CanParse,
}

impl<'s> CanParse for SkipTo<'s>
{
    fn parse<'p>(&self, mut ctx: Ctx<'p>) -> ParseResult<'p> {
        loop {
            match self.exp.parse(ctx) {
                Err(err_ctx) => {
                    match err_ctx.next() {
                        Ok(next_cxt) => {
                            ctx = next_cxt;
                        },
                        Err(e) => return Err(e)
                    }
                }
                ok => return ok,
            }
        }
    }
}