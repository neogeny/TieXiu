// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Debug;
use crate::engine::{Cst, Ctx};
use crate::model::closure::{add_exp, repeat, repeat_with_pre};

pub type ParseResult<C > = Result<(C, Cst), C>;
pub type ModelRef<'m> = &'m Model<'m>;
pub type ModelRefArr<'m> = &'m [ModelRef<'m>];


pub trait CanParse<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}


#[derive(Debug, Clone)]
pub enum Model<'m>
{
    Cut,
    Void,
    Fail,
    Dot,
    Eof,
    Token { token: &'m str },
    Constant { literal: &'m str },
    Alert { literal: &'m str, level: u8 },

    Named { name: &'m str, exp: ModelRef<'m> },
    NamedList { name: &'m str, exp: ModelRef<'m> },
    Override { exp: ModelRef<'m> },
    OverrideList { exp: ModelRef<'m> },

    Group { exp: ModelRef<'m> },
    SkipGroup { exp: ModelRef<'m> },

    Lookahead { exp: ModelRef<'m> },
    NegativeLookahead { exp: ModelRef<'m> },
    SkipTo { exp: ModelRef<'m> },

    Sequence { sequence: ModelRefArr<'m> },
    Choice { options: ModelRefArr<'m> },
    Optional { exp: ModelRef<'m> },
    Closure { exp: ModelRef<'m> },
    PositiveClosure { exp: ModelRef<'m> },
    Join { exp: ModelRef<'m>, sep: ModelRef<'m> },
    PositiveJoin { exp: ModelRef<'m>, sep: ModelRef<'m> },
    Gather { exp: ModelRef<'m>, sep: ModelRef<'m> },
    PositiveGather { exp: ModelRef<'m>, sep: ModelRef<'m> },
}

impl<'m, C: Ctx> CanParse<C> for Model<'m>
{
    fn parse(&self, mut ctx: C) -> ParseResult<C> {
        match self {
            Self::Cut => {
                ctx.cut();
                Ok((ctx, Cst::Nil))
            },
            Self::Void => Ok((ctx, Cst::Nil)),
            Self::Fail => Err(ctx),
            Self::Dot =>
                if let Some(_) = ctx.next() { Ok((ctx, Cst::Nil)) } else { Err(ctx) },
            Self::Eof =>
                if ctx.eof_check() { Ok((ctx, Cst::Nil)) } else { Err(ctx) },

            Self::Token { token } =>
                if ctx.token(token) {
                    Ok((ctx, Cst::Token((*token).into())))
                } else {
                    Err(ctx)
                },
            Self::Constant { literal } =>
                Ok((ctx, Cst::Literal(literal.to_string()))),
            Self::Alert { literal, level: _ } =>
                Ok((ctx, Cst::Literal(literal.to_string()))),

            Self::Named { name, exp } => {
                match exp.parse(ctx) {
                    Ok((ctx, cst)) => {
                        Ok((ctx, Cst::Named(name.to_string(), Box::new(cst))))
                    },
                    err => err
                }
            },
            Self::NamedList { name, exp } => {
                match exp.parse(ctx) {
                    Ok((ctx, cst)) => {
                        Ok((ctx, Cst::NamedList(name.to_string(), Box::new(cst))))
                    },
                    err => err
                }
            },
            Self::Override { exp } => {
                match exp.parse(ctx) {
                    Ok((ctx, cst)) => {
                        Ok((ctx, Cst::OverrideValue(Box::new(cst))))
                    },
                    err => err
                }
            },
            Self::OverrideList { exp } => {
                match exp.parse(ctx) {
                    Ok((ctx, cst)) => {
                        Ok((ctx, Cst::OverrideList(Box::new(cst))))
                    },
                    err => err
                }
            },
            Self::Group { exp } => exp.parse(ctx),
            Self::SkipGroup { exp } => {
                let (new_ctx, _) = exp.parse(ctx)?;
                Ok((new_ctx, Cst::Nil))
            },
            Self::Lookahead { exp } => {
                let _ = exp.parse(ctx.clone())?;
                Ok((ctx, Cst::Nil))
            }
            Self::NegativeLookahead { exp } =>
                if let Ok((_, _)) = exp.parse(ctx.clone()) {
                    Err(ctx)
                } else {
                    Ok((ctx, Cst::Nil))
                },
            Self::SkipTo { exp } =>
                loop {
                    match exp.parse(ctx) {
                        Err(errctx) => {
                            match errctx.next() {
                                None => break Err(errctx),
                                Some(_) => {
                                    ctx = errctx;
                                },
                            }
                        }
                        ok => break ok,
                    }
                },

            Self::Sequence { sequence } => {
                let mut results = Vec::new();
                for exp in sequence.iter() {
                    let (new_ctx, cst) = exp.parse(ctx)?;
                    results.push(cst);
                    ctx = new_ctx;
                }
                Ok((ctx, Cst::from(results)))
            },
            Self::Choice { options } => {
                for option in options.iter() {
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
            },
            Self::Optional { exp } => {
                match exp.parse(ctx.clone()) {
                    Ok(success) => Ok(success),
                    Err(_) => Ok((ctx, Cst::Nil))
                }
            },

            Self::Closure { exp } => {
                let mut res = Vec::new();
                let new_ctx = repeat(&mut res, ctx, *exp);
                Ok((new_ctx, Cst::from(res)))
            },
            Self::PositiveClosure { exp } => {
                let mut res: Vec<Cst> = Vec::new();
                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    },
                    err => return err
                };

                let new_ctx = repeat(&mut res, ctx, *exp);
                Ok((new_ctx, Cst::from(res)))
            },
            Self::Join { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match add_exp(*exp, ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = repeat_with_pre(&mut res, new_ctx, *exp, *sep, true);
                        Ok((ctx, Cst::from(res)))
                    },
                    Err(err_ctx) => Ok((err_ctx, Cst::from(res)))
                }
            },
            Self::PositiveJoin { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    },
                    err => return err
                };

                let new_ctx = repeat_with_pre(&mut res, ctx, *exp, *sep, true);
                Ok((new_ctx, Cst::from(res)))
            },
            Self::Gather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();
                match add_exp(*exp, ctx, &mut res) {
                    Ok(new_ctx) => {
                        let ctx = repeat_with_pre(
                            &mut res, new_ctx, *exp, *sep, false
                        );
                        Ok((ctx, Cst::from(res)))
                    },
                    Err(err_ctx) => Ok((err_ctx, Cst::from(res)))
                }
            },
            Self::PositiveGather { exp, sep } => {
                let mut res: Vec<Cst> = Vec::new();

                match exp.parse(ctx) {
                    Ok((new_ctx, cst)) => {
                        ctx = new_ctx;
                        res.push(cst);
                    },
                    err => return err
                };

                let new_ctx = repeat_with_pre(
                    &mut res, ctx, *exp, *sep, false
                );
                Ok((new_ctx, Cst::from(res)))
            }
        }
    }
}