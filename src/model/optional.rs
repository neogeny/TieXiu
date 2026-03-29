// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


#[derive(Debug, Clone)]
pub struct Optional<'o, C> {
    pub exp: &'o dyn CanParse<C>
}


impl<'o, C: Ctx> CanParse<C> for Optional<'o, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        match self.exp.parse(ctx.clone()) {
            Ok(success) => Ok(success),
            Err(_) => Ok((ctx, Cst::Nil))
        }
    }
}
