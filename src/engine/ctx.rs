// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::rc::Rc;
use crate::input::Cursor;
use crate::model::{ParseResult, Rule};


#[derive(Clone, Debug)]
pub struct Ctx<'c> {
    pub cursor: &'c dyn Cursor,
    pub cut_seen : bool,
    pub error_msg: Option<String>,
    pub rules: Rc<HashMap<&'c str, Rule<'c>>>,
}

impl<'c> Ctx<'c> {
    pub fn new(cursor: &'c dyn Cursor) -> Self {
        Self {
            cursor,
            cut_seen: false,
            error_msg: None,
            rules: Rc::new(HashMap::new()),
        }
    }

    pub fn resolve(&self, name: &str) -> &Rule<'_> {
        self.rules.get(name).unwrap()
    }

    pub fn mark(&self) -> usize {
        self.cursor.mark()
    }
    
    pub fn cut(&mut self) {
        self.cut_seen = true;
    }
    
    pub fn dot<'a>(&self) -> ParseResult<'a> {
        unimplemented!()
    }
    
    pub fn eof_check<'a>(&self) -> ParseResult<'a> {
        unimplemented!()
    }

    pub fn next(self) -> Result<Ctx<'c>, Ctx<'c>> {
        // do it with cursor goto(+1)?
        Err(self)
    }

    pub fn token<'p>(self, _token: &str) -> ParseResult<'p> {
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

    pub fn pattern<'p>(self, _pattern: &str) -> ParseResult<'p> {
        unimplemented!()
    }

    pub fn search(&self, _pattern: &str) -> ParseResult<'_> {
        unimplemented!()
    }
}
//     fn token(&mut self, token: &str) -> Result<String, String>;
//     fn pattern(&mut self, pattern: &str) -> ParseResult;
//
//     // Rule and Dispatch
//     fn call(&mut self, ri: &RuleInfo) -> ParseResult;
//     fn find_rule(&self, name: &str) -> Box<dyn Fn(&mut dyn Ctx) -> ParseResult>;
//
//     // High-level combinators (taking closures to match Python's Func)
//     fn closure(&mut self, exp: &dyn Fn(&mut dyn Ctx) -> ParseResult) -> ParseResult;
//     fn positive_closure(&mut self, exp: &dyn Fn(&mut dyn Ctx) -> ParseResult) -> ParseResult;
//     fn choice(&mut self, options: Vec<Box<dyn Fn(&mut dyn Ctx) -> ParseResult>>) -> ParseResult;
//
//     // AST / Result management
//     fn define(&mut self, keys: Vec<String>, add_keys: Option<Vec<String>>);
//     fn last_node(&self) -> ParseResult;
//
//     // Errors and Guards
//     fn fail(&mut self) -> ParseResult;