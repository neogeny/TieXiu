// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::failure::ParseFailure;
use crate::Tree;
use crate::cfg::types::{Ref, Str};
use crate::engine::{Ctx, CtxI};
use crate::input::memento::Memento;
use std::fmt::Debug;
use std::panic::Location;

pub type ParseResult<C> = Result<Yeap<C>, Nope>;

#[derive(Clone, Debug)]
pub struct DisasterReport {
    pub pos: (usize, usize),
    pub la: Str,
    pub error: Ref<ParseFailure>,
    pub location: &'static Location<'static>,
    pub memento: Memento,
}

#[derive(Clone, Debug)]
pub struct Nope {
    pub start: usize,
    pub mark: usize, // The position where the disaster occurred
    pub cutseen: bool,
    pub report: Ref<DisasterReport>,
}

impl std::fmt::Display for Nope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.report.memento, f)
    }
}

impl std::error::Error for Nope {
    // source() is optional since ParseError is the cause,
    // but this is the "Rust Way" for chained errors.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.report.error)
    }
}

impl Nope {
    #[track_caller]
    pub fn new(start: usize, ctx: &dyn CtxI, error: ParseFailure) -> Self {
        let mark = ctx.mark();
        let context = DisasterReport {
            memento: Memento {
                source: ctx.cursor().source().as_str().into(),
                start,
                mark,
                msg: error.to_string().into(),
                text: ctx.cursor().textstr().into(),
                callstack: ctx.callstack(),
            },
            pos: ctx.cursor().pos(),
            la: ctx.cursor().lookahead(start).into(),
            location: Location::caller(),
            error: error.into(),
        };
        Self {
            start,
            mark,
            cutseen: ctx.cut_seen(),
            report: context.into(),
        }
    }

    pub fn setcut(&mut self) {
        self.cutseen = true;
    }

    pub fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }

    pub fn restore_cut(&mut self, was_cut: bool) {
        if !was_cut {
            self.cutseen = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Yeap<C: Ctx>(pub C, pub Tree);

impl<C: Ctx> Yeap<C> {
    #[inline]
    pub fn ctx(self) -> C {
        self.0
    }

    #[inline]
    pub fn tree(self) -> Tree {
        self.1
    }

    #[inline]
    pub fn ctx_ref(&self) -> &C {
        &self.0
    }

    #[inline]
    pub fn cst_ref(&self) -> &Tree {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::strctx::StrCtx;
    use crate::peg::error::nope::{Nope, Yeap};

    const TARGET: usize = 64;

    #[test]
    fn test_succ_size() {
        let size = size_of::<Yeap<StrCtx>>();
        assert!(size <= TARGET, "Succ size is {} > {} bytes", size, TARGET);
    }

    #[test]
    fn test_fail_size() {
        let size = size_of::<Nope>();
        assert!(size <= TARGET, "Fail size is {} > {} bytes", size, TARGET);
    }
}
