// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Named<'n> {
    pub name: &'static str,
    pub exp: &'n dyn CanParse
}

impl<'n> CanParse for Named<'n>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::Named(self.name, Box::new(cst))))
            },
            err => err
        }
    }
}


#[derive(Debug, Clone)]
pub struct NamedList<'n> {
    pub name: &'static str,
    pub exp: &'n dyn CanParse
}


impl<'n> CanParse for NamedList<'n>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::NamedList(self.name, Box::new(cst))))
            },
            err => err
        }
    }
}


#[derive(Debug, Clone)]
pub struct Override<'o> {
    pub exp: &'o dyn CanParse
}

impl<'o> CanParse for Override<'o>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                let boxed = Box::new(cst);
                Ok((ctx, Cst::OverrideValue(boxed)))
            },
            err => err
        }
    }
}


#[derive(Debug, Clone)]
pub struct OverrideList<'o> {
    pub exp: &'o dyn CanParse
}

impl<'o> CanParse for OverrideList<'o>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                let boxed = Box::new(cst);
                Ok((ctx, Cst::OverrideList(boxed)))
            },
            err => err
        }
    }
}
