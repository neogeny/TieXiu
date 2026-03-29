// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

#[derive(Debug, Clone)]
pub struct Sequence<'s> {
    pub exps: Vec<&'s dyn CanParse>,
}

impl<'s> Sequence<'s> {
    pub fn new(exps: Vec<&'s dyn CanParse>) -> Self {
        Self { exps }
    }
}


impl<'s> CanParse for Sequence<'s>
{
    fn parse<'a>(&self, mut ctx: Ctx<'a>) -> ParseResult<'a> {
        let mut results = Vec::new();

        for exp in &self.exps {
            let (new_ctx, cst) = exp.parse(ctx)?;
            results.push(cst);
            ctx = new_ctx;
        }
        Ok((ctx, Cst::from(results)))
    }
}