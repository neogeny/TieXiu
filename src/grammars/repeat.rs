// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::canparse::CanParse;
use crate::contexts::{Cst, Ctx};

pub fn skip_exp<C: Ctx>(exp: &dyn CanParse<C>, ctx: C) -> Result<C, C> {
    match exp.parse(ctx) {
        Ok((new_ctx, _)) => Ok(new_ctx),
        Err(err) => Err(err),
    }
}

pub fn add_exp<C: Ctx>(exp: &dyn CanParse<C>, ctx: C, res: &mut Vec<Cst>) -> Result<C, C> {
    match exp.parse(ctx) {
        Ok((new_ctx, cst)) => {
            res.push(cst);
            Ok(new_ctx)
        }
        Err(err) => Err(err),
    }
}

pub fn repeat<C: Ctx>(exp: &dyn CanParse<C>, mut ctx: C, res: &mut Vec<Cst>) -> C {
    loop {
        match add_exp(exp, ctx, res) {
            Ok(new_ctx) => ctx = new_ctx,
            Err(err_ctx) => return err_ctx,
        }
    }
}

pub fn repeat_with_pre<C: Ctx>(
    exp: &dyn CanParse<C>,
    pre: &dyn CanParse<C>,
    mut ctx: C,
    res: &mut Vec<Cst>,
    keep_pre: bool,
) -> C {
    loop {
        match pre.parse(ctx) {
            Err(err_ctx) => return err_ctx,
            Ok((new_ctx, pre_cst)) => match exp.parse(new_ctx) {
                Err(err_ctx) => return err_ctx,
                Ok((new_ctx, exp_cst)) => {
                    if keep_pre {
                        res.push(pre_cst);
                    }
                    res.push(exp_cst);
                    ctx = new_ctx;
                }
            },
        }
    }
}
