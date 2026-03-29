// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

#[derive(Debug, Clone)]
pub struct Group<'g, C> {
    pub exp: Box<&'g dyn CanParse<C>>,
}

impl<'g, C: Ctx> Group<'g, C> {
    fn new(exp: &'g dyn CanParse<C>) -> Self {
        Self {
            exp: Box::new(exp),
        }
    }
}

impl<'g, C: Ctx> CanParse<C> for Group<'g, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}


#[derive(Debug, Clone)]
pub struct SkipGroup<'g, C> {
    pub exp: Box<&'g dyn CanParse<C>>,
}

impl<'g, C: Ctx> CanParse<C> for SkipGroup<'g, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        let (new_ctx, _) = self.exp.parse(ctx)?;
        Ok((new_ctx, Cst::Nil))
    }
}


