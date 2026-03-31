// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::grammars::{ParseResult, Parser};
use crate::input::{Cursor, StrCursor};
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn call(self, name: &str) -> ParseResult<Self>;
    fn mark(&self) -> usize;
    fn eof_check(&self) -> bool;
    fn dot(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn token(&mut self, token: &str) -> bool;
    fn pattern(&mut self, pattern: &str) -> Option<&str>;
    fn next_token(&mut self);

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct StrCtx<'c> {
    cursor: StrCursor<'c>,
    _cut_seen: bool,
    rulemap: HashMap<&'c str, &'c dyn Parser<Self>>,
}

impl<'c> StrCtx<'c> {
    pub fn new(cursor: StrCursor<'c>) -> Self {
        Self {
            cursor,
            _cut_seen: false,
            rulemap: HashMap::new(),
        }
    }
}

impl<'c> Ctx for StrCtx<'c> {
    fn call(self, name: &str) -> ParseResult<Self> {
        let rule = self.rulemap.get(name).expect("Rule not found");
        rule.parse(self)
    }

    fn mark(&self) -> usize {
        self.cursor.mark()
    }

    fn eof_check(&self) -> bool {
        self.cursor.at_end()
    }

    fn dot(&mut self) -> bool {
        self.next().is_some()
    }

    fn next(&mut self) -> Option<char> {
        self.cursor.next()
    }

    fn token(&mut self, token: &str) -> bool {
        self.cursor.token(token)
    }

    fn pattern(&mut self, pattern: &str) -> Option<&str> {
        self.cursor.pattern(pattern)
    }

    fn next_token(&mut self) {
        self.cursor.next_token();
    }

    fn cut(&mut self) {
        self._cut_seen = true;
    }

    fn uncut(&mut self) {
        self._cut_seen = false;
    }

    fn cut_seen(&self) -> bool {
        self._cut_seen
    }
}
