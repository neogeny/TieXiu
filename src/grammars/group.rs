// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::contexts::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

#[derive(Debug)]
pub struct Group<C> {
    pub exp: Box<dyn CanParse<C>>,
}

impl<C: Ctx> Group<C> {
    pub fn new<T>(exp: T) -> Self
    where
        T: CanParse<C> + 'static,
    {
        Self {
            exp: Box::new(exp),
        }
    }
}

impl<C: Ctx> CanParse<C> for Group<C> {
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}

#[derive(Debug)]
pub struct SkipGroup<C> {
    pub exp: Box<dyn CanParse<C>>,
}

impl<C: Ctx> SkipGroup<C> {
    pub fn new<T>(exp: T) -> Self
    where
        T: CanParse<C> + 'static,
    {
        Self {
            exp: Box::new(exp),
        }
    }
}

impl<C: Ctx> CanParse<C> for SkipGroup<C> {
    fn parse(&self, ctx: C) -> ParseResult<C> {
        let (new_ctx, _) = self.exp.parse(ctx)?;
        Ok((new_ctx, Cst::Nil))
    }
}
