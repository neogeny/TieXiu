// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


#[derive(Debug, Clone)]
pub struct Closure<'c> {
    pub exp: &'c dyn CanParse,
}

#[derive(Debug, Clone)]
pub struct PositiveClosure<'c> {
    pub exp: &'c dyn CanParse,
}

#[derive(Debug, Clone)]
pub struct Join<'c> {
    pub exp: &'c dyn CanParse,
    pub sep: &'c dyn CanParse,
}

#[derive(Debug, Clone)]
pub struct PositiveJoin<'c> {
    pub exp: &'c dyn CanParse,
    pub sep: &'c dyn CanParse,
}

#[derive(Debug, Clone)]
pub struct Gather<'c> {
    pub exp: &'c dyn CanParse,
    pub sep: &'c dyn CanParse,
}

#[derive(Debug, Clone)]
pub struct PositiveGather<'c> {
    pub exp: &'c dyn CanParse,
    pub sep: &'c dyn CanParse,
}


fn skip_exp<'c>(exp: &dyn CanParse, ctx: Ctx<'c>) -> Result<Ctx<'c>, Ctx<'c>>
{
    match exp.parse(ctx) {
        Ok((new_ctx, _)) => {
            Ok(new_ctx)
        }
        Err(err) => Err(err)
    }
}


fn add_exp<'c>(exp: &dyn CanParse, ctx: Ctx<'c>, res: &mut Vec<Cst>) -> Result<Ctx<'c>, Ctx<'c>>
{
        match exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                res.push(cst);
                Ok(new_ctx)
            }
            Err(err) => Err(err)
        }
}


fn repeat<'c>(res: &mut Vec<Cst>, mut ctx: Ctx<'c>, exp: &dyn CanParse) -> Ctx<'c>
{
    loop {
        match add_exp(exp, ctx, res) {
            Ok(new_ctx) => ctx = new_ctx,
            Err(err_ctx) => return err_ctx
        }
    }
}


fn repeat_with_pre<'c>(
    res: &mut Vec<Cst>, 
    mut ctx: Ctx<'c>, 
    exp: &dyn CanParse, 
    pre: &dyn CanParse,
    keep_pre: bool,
) -> Ctx<'c>
{
    loop {
        match pre.parse(ctx) {
            Err(err_ctx) => return err_ctx,
            Ok((new_ctx, pre_cst)) => {
                ctx = new_ctx;
                match exp.parse(ctx) {
                    Err(err_ctx) => return err_ctx,
                    Ok((new_ctx, exp_cst)) => {
                        ctx = new_ctx;
                        if keep_pre {
                            res.push(pre_cst);
                        }
                        res.push(exp_cst);
                    }
                }
            }
        }
    }
}


impl<'c> CanParse for Closure<'c>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res = Vec::new();
        let new_ctx = repeat(&mut res, ctx, &*self.exp);
        Ok((new_ctx, Cst::from(res)))
    }
}

impl<'c> CanParse for PositiveClosure<'c>
{
    fn parse<'p>(&self, mut ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res: Vec<Cst> = Vec::new();

        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                ctx = new_ctx;
                res.push(cst);
            },
            err => return err
        };

        let new_ctx = repeat(&mut res, ctx, &*self.exp);
        Ok((new_ctx, Cst::from(res)))
    }
}

impl<'c> CanParse for Join<'c>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res: Vec<Cst> = Vec::new();

        match add_exp(&*self.exp, ctx, &mut res) {
            Ok(new_ctx) => {
                let ctx = repeat_with_pre(&mut res, new_ctx, &*self.exp, &*self.sep, true);
                Ok((ctx, Cst::from(res)))
            },
            Err(err_ctx) => Ok((err_ctx, Cst::from(res)))
        }
    }
}

impl<'c> CanParse for PositiveJoin<'c>
{
    fn parse<'p>(&self, mut ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res: Vec<Cst> = Vec::new();

        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                ctx = new_ctx;
                res.push(cst);
            },
            err => return err
        };

        let new_ctx = repeat_with_pre(&mut res, ctx, &*self.exp, &*self.sep, true);
        Ok((new_ctx, Cst::from(res)))
    }
}

impl<'c> CanParse for Gather<'c>
{
    fn parse<'p>(&self, ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res: Vec<Cst> = Vec::new();

        match add_exp(&*self.exp, ctx, &mut res) {
            Ok(new_ctx) => {
                let ctx = repeat_with_pre(
                    &mut res, new_ctx, &*self.exp, &*self.sep, false
                );
                Ok((ctx, Cst::from(res)))
            },
            Err(err_ctx) => Ok((err_ctx, Cst::from(res)))
        }
    }
}

impl<'c> CanParse for PositiveGather<'c>
{
    fn parse<'p>(&self, mut ctx: Ctx<'p>) -> ParseResult<'p> {
        let mut res: Vec<Cst> = Vec::new();

        match self.exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                ctx = new_ctx;
                res.push(cst);
            },
            err => return err
        };

        let new_ctx = repeat_with_pre(
            &mut res, ctx, &*self.exp, &*self.sep, false
        );
        Ok((new_ctx, Cst::from(res)))
    }
}