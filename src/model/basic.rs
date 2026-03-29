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

impl<C: Ctx> CanParse<C> for Fail
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Err(ctx)
    }
}


#[derive(Debug, Clone)]
pub struct Eof {
}

impl<C: Ctx> CanParse<C> for Eof
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        if ctx.eof_check() {
            Ok((ctx, Cst::Nil))
        }
        else {
            Err(ctx)
        }
    }
}


#[derive(Debug, Clone)]
pub struct Constant {
    pub literal: String
}

impl<C: Ctx> CanParse<C> for Constant
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Ok((ctx, Cst::Literal(self.literal.clone().into())))
    }
}


#[derive(Debug, Clone)]
pub struct Alert {
    pub literal: String,
    pub level: u8
}

impl<C: Ctx> CanParse<C> for Alert
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        Ok((ctx, Cst::Literal(self.literal.clone().into())))
    }
}


#[derive(Debug, Clone)]
pub struct Cut {}

impl<C: Ctx> CanParse<C> for Cut
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        ctx.cut();
        Ok((ctx, Cst::Nil))
    }
}
