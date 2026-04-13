// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::input::Cursor;
use crate::peg::Nope;
use std::fmt::Debug;
use crate::state::Ctx;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullTracer;

impl<C> Tracer<C> for NullTracer {}

pub trait Tracer<C: Ctx>: Debug {
    fn trace(&self, msg: &str) {}

    fn trace_event(&self, ctx: &C, event: &str) {
        let lookahead = ctx.cursor().lookahead();
        let calltack = ctx.callstack().to_string();
        let mark = ctx.cursor().mark();


        let message =
            "".to_string()
            + "{event}{callstack}"
            + "{source}"
            + "{mark}"
            + "\n{lookahead}"
        ;
    }

    fn trace_entry(&self, _: &dyn Ctx) {}

    fn trace_success(&self, _: &dyn Ctx) {}

    fn trace_failure(&self, _: &dyn Ctx, _: Option<&Nope>) {}

    fn trace_recursion(&self, _: &dyn Ctx) {}

    fn trace_cut(&self, _: &dyn Ctx) {}

    fn trace_match(&self, _: &dyn Ctx, _: &str, _: Option<&str>, _: bool) {}
}

#[derive(Clone, Debug)]
pub struct ConsoleTracer {}

impl Tracer for ConsoleTracer {
    fn trace(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}