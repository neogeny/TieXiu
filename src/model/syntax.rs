// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Lookahead<'k, C> {
    pub exp: &'k dyn CanParse<C>,
}

impl<'k, C: Ctx> CanParse<C> for Lookahead<'k, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        let _ = self.exp.parse(ctx.clone())?;
        Ok((ctx, Cst::Nil))
    }
}


#[derive(Debug, Clone)]
pub struct NegativeLookahead<'k, C> {
    pub exp: &'k dyn CanParse<C>,
}

impl<'k, C: Ctx> CanParse<C> for NegativeLookahead<'k, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        if let Ok((_, _)) = self.exp.parse(ctx.clone()) {
            return Err(ctx)
        }
        Ok((ctx, Cst::Nil))
    }
}


#[derive(Debug, Clone)]
pub struct SkipTo<'s, C> {
    pub exp: &'s dyn CanParse<C>,
}

impl<'s, C: Ctx> CanParse<C> for SkipTo<'s, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        loop {
            match self.exp.parse(ctx) {
                Err(err_ctx) => {
                    match err_ctx.next() {
                        Some(_) => {
                            ctx = err_ctx;
                        },
                        None => return Err(err_ctx)
                    }
                }
                ok => return ok,
            }
        }
    }
}