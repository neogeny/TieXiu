// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


#[derive(Debug, Clone)]
pub struct Closure<'c, C> {
    pub exp: &'c dyn CanParse<C>,
}

#[derive(Debug, Clone)]
pub struct PositiveClosure<'c, C> {
    pub exp: &'c dyn CanParse<C>,
}

#[derive(Debug, Clone)]
pub struct Join<'c, C> {
    pub exp: &'c dyn CanParse<C>,
    pub sep: &'c dyn CanParse<C>,
}

#[derive(Debug, Clone)]
pub struct PositiveJoin<'c, C> {
    pub exp: &'c dyn CanParse<C>,
    pub sep: &'c dyn CanParse<C>,
}

#[derive(Debug, Clone)]
pub struct Gather<'c, C> {
    pub exp: &'c dyn CanParse<C>,
    pub sep: &'c dyn CanParse<C>,
}

#[derive(Debug, Clone)]
pub struct PositiveGather<'c, C> {
    pub exp: &'c dyn CanParse<C>,
    pub sep: &'c dyn CanParse<C>,
}


pub fn skip_exp<'c, C: Ctx>(exp: &dyn CanParse<C>, ctx: C) -> Result<C, C>
{
    match exp.parse(ctx) {
        Ok((new_ctx, _)) => {
            Ok(new_ctx)
        }
        Err(err) => Err(err)
    }
}


pub fn add_exp<C: Ctx>(exp: &dyn CanParse<C>, ctx: C, res: &mut Vec<Cst>) -> Result<C, C>
{
        match exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                res.push(cst);
                Ok(new_ctx)
            }
            Err(err) => Err(err)
        }
}


pub fn repeat<C: Ctx>(res: &mut Vec<Cst>, mut ctx: C, exp: &dyn CanParse<C>) -> C
{
    loop {
        match add_exp(exp, ctx, res) {
            Ok(new_ctx) => ctx = new_ctx,
            Err(err_ctx) => return err_ctx
        }
    }
}


pub fn repeat_with_pre<C: Ctx>(
    res: &mut Vec<Cst>,
    mut ctx: C, 
    exp: &dyn CanParse<C>, 
    pre: &dyn CanParse<C>,
    keep_pre: bool,
) -> C
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


impl<'c, C: Ctx> CanParse<C> for Closure<'c, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        let mut res = Vec::new();
        let new_ctx = repeat(&mut res, ctx, &*self.exp);
        Ok((new_ctx, Cst::from(res)))
    }
}

impl<'c, C: Ctx> CanParse<C> for PositiveClosure<'c, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
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

impl<'c, C: Ctx> CanParse<C> for Join<'c, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
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

impl<'c, C: Ctx> CanParse<C> for PositiveJoin<'c, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
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

impl<'c, C: Ctx> CanParse<C> for Gather<'c, C>
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
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

impl<'c, C: Ctx> CanParse<C> for PositiveGather<'c, C>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
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