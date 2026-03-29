// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

#[derive(Debug, Clone)]
pub struct Sequence<'s, C> {
    pub exps: Vec<&'s dyn CanParse<C>>,
}

impl<'s, C: Ctx> Sequence<'s, C> {
    pub fn new(exps: Vec<&'s dyn CanParse<C>>) -> Self {
        Self { exps }
    }
}


impl<'s, C: Ctx> CanParse<C> for Sequence<'s, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        let mut results = Vec::new();

        for exp in &self.exps {
            let (new_ctx, cst) = exp.parse(ctx)?;
            results.push(cst);
            ctx = new_ctx;
        }
        Ok((ctx, Cst::from(results)))
    }
}