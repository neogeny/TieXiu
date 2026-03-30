// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::contexts::{Ctx};


#[derive(Debug, Clone)]
pub struct Call {
    pub name: &'static str,
}

impl Call {
    pub fn new(name: &'static str) -> Self {
        Self { name}
    }
}

impl<C: Ctx> CanParse<C> for Call
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        let rctx = ctx.clone();
        let rule = rctx.resolve(self.name);
        match rule.parse(ctx) {
            Ok((new_ctx, cst)) => {
                Ok((new_ctx, cst))
            },
            err => err,
        }
    }
}