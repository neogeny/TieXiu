// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;


#[derive(Debug, Clone)]
pub struct Choice<'c, C> {
    pub options: Vec<&'c dyn CanParse<C>>
}

impl<'c, C: Ctx> CanParse<C> for Choice<'c, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err(mut err_ctx) => {
                    if err_ctx.cut_seen() {
                        err_ctx.uncut();
                        return Err(err_ctx);
                    }
                    ctx = err_ctx;
                }
            }
        }
        Err(ctx)
    }
}
