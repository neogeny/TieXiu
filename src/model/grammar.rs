// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::Ctx;
use super::{CanParse, ParseResult, Rule};

#[derive(Debug, Clone)]
pub struct Grammar<'g> {
    pub rules: &'g [Rule<'g>],
    pub start: &'g Rule<'g>
}

impl<'g> CanParse for Grammar<'g>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        self.start.parse(ctx)
    }
}

