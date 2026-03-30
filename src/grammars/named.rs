// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::contexts::{Cst, Ctx};

#[derive(Debug, Clone)]
pub struct Named<'n, C> {
    pub name: &'n str,
    pub exp: &'n dyn CanParse<C>
}

impl<'n, C: Ctx> CanParse<C> for Named<'n, C> {
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::Named(self.name.into(), Box::new(cst))))
            },
            err => err
        }
    }
}


#[derive(Debug, Clone)]
pub struct NamedList<'n, C> {
    pub name: &'static str,
    pub exp: &'n dyn CanParse<C>
}


impl<'n, C: Ctx> CanParse<C> for NamedList<'n, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                Ok((ctx, Cst::NamedList(self.name.into(), Box::new(cst))))
            },
            err => err
        }
    }
}


#[derive(Debug, Clone)]
pub struct Override<'o, C> {
    pub exp: &'o dyn CanParse<C>
}

impl<'o, C: Ctx> CanParse<C> for Override<'o, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
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
pub struct OverrideList<'o, C> {
    pub exp: &'o dyn CanParse<C>
}

impl<'o, C: Ctx> CanParse<C> for OverrideList<'o, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx) {
            Ok((ctx, cst)) => {
                let boxed = Box::new(cst);
                Ok((ctx, Cst::OverrideList(boxed)))
            },
            err => err
        }
    }
}
