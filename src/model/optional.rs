// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


#[derive(Debug, Clone)]
pub struct Optional<'o> {
    pub exp: &'o dyn CanParse
}


impl<'o> CanParse for Optional<'o>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        match self.exp.parse(ctx.clone()) {
            Ok(success) => Ok(success),
            Err(_) => Ok((ctx, Cst::Nil))
        }
    }
}
