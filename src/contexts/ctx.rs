// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

// use std::collections::HashMap;
// use std::collections::HashMap;
// use std::rc::Rc;
use crate::input::{Cursor, StrCursor};
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    // fn resolve(&self, name: &str) -> &Rule<'_, Self>;
    fn mark(&self) -> usize;
    fn dot(&self) -> bool;
    fn eof_check(&self) -> bool;
    fn next(&self) -> Option<char>;
    fn token(&self, token: &str) -> bool;
    fn pattern(&self, pattern: &str) -> bool;
    fn search(&self, pattern: &str) -> bool;

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct StrCtx<'c> {
    cursor: StrCursor<'c>,
    _cut_seen: bool,
}

impl<'c> StrCtx<'c> {
    pub fn new(cursor: StrCursor<'c>) -> Self {
        Self {
            cursor,
            _cut_seen: false,
            // rules: HashMap::new(),
        }
    }
}

impl<'c> Ctx for StrCtx<'c> {
    // fn resolve(&self, name: &str) -> &Rule<'_, Self> {
    //     self.rules
    //         .get(name)
    //         .expect("Grammar Error: Rule not found")
    // }

    fn mark(&self) -> usize {
        self.cursor.mark()
    }

    fn dot<'a>(&self) -> bool {
        unimplemented!()
    }

    fn eof_check(&self) -> bool {
        unimplemented!()
    }

    fn next(&self) -> Option<char> {
        // do it with cursor goto(+1)?
        unimplemented!()
    }

    fn token<'p>(&self, _token: &str) -> bool {
        // if self.cursor.token(token) {
        //     Ok(
        //         (self, Cst::Token(token.into()))
        //     )
        // }
        // else {
        //     Err(self)
        // }
        unimplemented!()
    }

    fn pattern(&self, _pattern: &str) -> bool {
        unimplemented!()
    }

    fn search(&self, _pattern: &str) -> bool {
        unimplemented!()
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

#[derive(Clone, Debug)]
pub struct CtxImpl<'c> {
    pub cursor: &'c dyn Cursor,
    pub cut_seen: bool,
    pub error_msg: Option<String>,
    // pub rules: Rc<HashMap<&'c str, &'c Rule<'c, CtxImpl>>>,
}
//
// impl<'c, C: Ctx> Ctximpl<'c, C: Ctx> {
//     pub fn new(cursor: &'c dyn Cursor) -> Self {
//         Self {
//             cursor,
//             cut_seen: false,
//             error_msg: None,
//             rules: Rc::new(HashMap::new()),
//         }
//     }
//
//     pub fn resolve(&self, name: &str) -> &Rule<'_> {
//         self.rules.get(name).unwrap()
//     }
//
//     pub fn mark(&self) -> usize {
//         self.cursor.mark()
//     }
//
//     pub fn cut(&mut self) {
//         self.cut_seen = true;
//     }
//
//     pub fn dot<'a>(&self) -> ParseResult<'a> {
//         unimplemented!()
//     }
//
//     pub fn eof_check<'a>(&self) -> ParseResult<'a> {
//         unimplemented!()
//     }
//
//     pub fn next(self) -> Result<Ctximpl<'c, C: Ctx>, Ctximpl<'c, C: Ctx>> {
//         // do it with cursor goto(+1)?
//         Err(self)
//     }
//
//     pub fn token<'p>(self, _token: &str) -> ParseResult<'p> {
//         // if self.cursor.token(token) {
//         //     Ok(
//         //         (self, Cst::Token(token.into()))
//         //     )
//         // }
//         // else {
//         //     Err(self)
//         // }
//         unimplemented!()
//     }
//
//     pub fn pattern<'p>(self, _pattern: &str) -> ParseResult<'p> {
//         unimplemented!()
//     }
//
//     pub fn search(&self, _pattern: &str) -> ParseResult<'_> {
//         unimplemented!()
//     }
// }
//     fn token(&mut self, token: &str) -> Result<String, String>;
//     fn pattern(&mut self, pattern: &str) -> ParseResult;
//
//     // Rule and Dispatch
//     fn call(&mut self, ri: &RuleInfo) -> ParseResult;
//     fn find_rule(&self, name: &str) -> Box<dyn Fn(&mut dyn CtxImpl) -> ParseResult>;
//
//     // High-level combinators (taking closures to match Python's Func)
//     fn closure(&mut self, exp: &dyn Fn(&mut dyn CtxImpl) -> ParseResult) -> ParseResult;
//     fn positive_closure(&mut self, exp: &dyn Fn(&mut dyn CtxImpl) -> ParseResult) -> ParseResult;
//     fn choice(&mut self, options: Vec<Box<dyn Fn(&mut dyn CtxImpl) -> ParseResult>>) -> ParseResult;
//
//     // AST / Result management
//     fn define(&mut self, keys: Vec<String>, add_keys: Option<Vec<String>>);
//     fn last_node(&self) -> ParseResult;
//
//     // Errors and Guards
//     fn fail(&mut self) -> ParseResult;
