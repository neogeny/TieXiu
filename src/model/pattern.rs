// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub pattern: String
}

impl CanParse for Pattern
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        ctx.pattern(&self.pattern)
    }
}

