// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::peg::{Nope, ParseError};
use std::fmt::Debug;
use super::CtxI;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullTracer {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConsoleTracer {}

impl Tracer for NullTracer {}

impl Tracer for ConsoleTracer {
    fn trace(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}

pub static NULL_TRACER: NullTracer = NullTracer {};
pub static CONSOLE_TRACER: ConsoleTracer = ConsoleTracer {};

pub enum Event {
    Entry,
    Success,
    Failure,
    Recursion,
    Cut,
    Match,
    NoMatch,
}

pub trait Tracer: Debug {
    fn trace(&self, msg: &str) {
        let _ = msg;
    }

    fn trace_event(&self, ctx: &dyn CtxI, event: Event, msg: &str) {
        let event_symbol = match event {
            Event::Entry => "↙",
            Event::Success => "≡",
            Event::Failure => "≢",
            Event::Recursion => "⟲",
            Event::Cut => "⚔",
            Event::Match =>  "≡",
            Event::NoMatch => "≢",
        };

        let lookahead = ctx.cursor().lookahead();
        let callstack = ctx.callstack().to_string();
        let mark = ctx.cursor().mark();
        let source = "";


        let msg = format!("{event_symbol}{callstack}{source}{mark}\n{lookahead\
        }");
        self.trace(msg.as_str());
    }

    fn trace_entry(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Entry, "");
    }

    fn trace_success(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Success, "");
    }

    fn trace_failure(&self, ctx: &dyn CtxI, error: ParseError) {
        let errstr = format!("{}", error);
        self.trace_event(ctx, Event::Failure, &errstr);
    }

    fn trace_recursion(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Recursion, "");
    }

    fn trace_cut(&self, ctx: &dyn CtxI) {
        self.trace_event(ctx, Event::Cut, "");
    }

    fn trace_match(&self, ctx: &dyn CtxI, token: &str, name:&str) {
        let msg = format!("'{token}'/{name}/");
        self.trace_event(ctx, Event::Match, &msg)

    }

    fn trace_no_match(&self, ctx: &dyn CtxI, name: &str) {
        let msg = format!("'/{name}/");
        self.trace_event(ctx, Event::NoMatch, &msg)
    }
}