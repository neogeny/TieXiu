// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Cst, Ctx};


pub struct Closure<M> {
    pub exp: Box<M>,
}

pub struct PositiveClosure<M> {
    pub exp: Box<M>,
}

pub struct Join<M> {
    pub exp: Box<M>,
    pub sep: Box<M>,
}

pub struct PositiveJoin<M> {
    pub exp: Box<M>,
    pub sep: Box<M>,
}

pub struct Gather<M> {
    pub exp: Box<M>,
    pub sep: Box<M>,
}

pub struct PositiveGather<M> {
    pub exp: Box<M>,
    pub sep: Box<M>,
}


fn skip_exp<M, C>(exp: &M, ctx: Ctx<C>) -> Result<Ctx<C>, Ctx<C>>
where
    M: CanParse<C>,
    C: Cursor,
{
    match exp.parse(ctx) {
        Ok((new_ctx, _)) => {
            Ok(new_ctx)
        }
        Err(err) => Err(err)
    }
}


fn add_exp<M, C>(exp: &M, ctx: Ctx<C>, res: &mut Vec<Cst>) -> Result<Ctx<C>, Ctx<C>>
where
    M: CanParse<C>,
    C: Cursor,
{
        match exp.parse(ctx) {
            Ok((new_ctx, cst)) => {
                res.push(cst);
                Ok(new_ctx)
            }
            Err(err) => Err(err)
        }
}


fn repeat<M, C>(res: &mut Vec<Cst>, mut ctx: Ctx<C>, exp: &M) -> Ctx<C>
where
    M: CanParse<C>,
    C: Cursor,
{
    loop {
        match add_exp(exp, ctx, res) {
            Ok(new_ctx) => ctx = new_ctx,
            Err(err_ctx) => return err_ctx
        }
    }
}


fn repeat_with_pre<M, C>(
    res: &mut Vec<Cst>, 
    mut ctx: Ctx<C>, 
    exp: &M, 
    pre: &M,
    keep_pre: bool,
) -> Ctx<C>
where
    M: CanParse<C>,
    C: Cursor,
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


impl<M, C> CanParse<C> for Closure<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        let mut res = Vec::new();
        let new_ctx = repeat(&mut res, ctx, &*self.exp);
        Ok((new_ctx, Cst::from(res)))
    }
}

impl<M, C> CanParse<C> for PositiveClosure<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
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

impl<M, C> CanParse<C> for Join<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
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

impl<M, C> CanParse<C> for PositiveJoin<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
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

impl<M, C> CanParse<C> for Gather<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
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

impl<M, C> CanParse<C> for PositiveGather<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
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