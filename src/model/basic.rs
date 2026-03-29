// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Dot {
}

impl<C: Ctx> CanParse<C> for Dot
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        if ctx.dot() {
            Ok((ctx, Cst::Nil))
        }
        else {
            Err(ctx)
        }
    }
}


#[derive(Debug, Clone)]
pub struct Void {
}

impl<C: Ctx> CanParse<C> for Void
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Ok((ctx, Cst::Nil))
    }
}

#[derive(Debug, Clone)]
pub struct Fail {
}

impl CanParse for Fail
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        Err(ctx)
    }
}


#[derive(Debug, Clone)]
pub struct Eof {
}

impl CanParse for Eof
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        ctx.eof_check()
    }
}


#[derive(Debug, Clone)]
pub struct Constant {
    pub literal: &'static str,
}

impl CanParse for Constant
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        Ok((ctx, Cst::Literal(self.literal)))
    }
}


#[derive(Debug, Clone)]
pub struct Alert {
    pub literal: &'static str,
    pub level: u8
}

impl CanParse for Alert
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        Ok((ctx, Cst::Literal(self.literal)))
    }
}


#[derive(Debug, Clone)]
pub struct Cut {}

impl CanParse for Cut
{
    fn parse<'p>(&self, mut ctx: Ctx<'p>) -> ParseResult<'p> {
        ctx.cut();
        Ok((ctx, Cst::Nil))
    }
}
