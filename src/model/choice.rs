// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::Ctx;


#[derive(Debug, Clone)]
pub struct Choice<'c> {
    pub options: Vec<&'c dyn CanParse>
}

impl<'c> CanParse for Choice<'c>
{
    fn parse<'a>(&self, mut ctx: Ctx<'a>) -> ParseResult<'a> {
        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err(mut err_ctx) => {
                    if err_ctx.cut_seen {
                        err_ctx.cut_seen = false;
                        return Err(err_ctx);
                    }
                    ctx = err_ctx;
                }
            }
        }
        Err(ctx)
    }
}
