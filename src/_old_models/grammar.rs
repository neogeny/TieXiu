// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::contexts::Ctx;
use super::{CanParse, ParseResult, Rule};

#[derive(Debug, Clone)]
pub struct Grammar<'g, C> {
    pub rules: &'g [Rule<'g, C>],
    pub start: &'g Rule<'g, C>
}

impl<'g, C: Ctx> CanParse<C> for Grammar<'g, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.start.parse(ctx)
    }
}

