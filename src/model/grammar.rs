// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::rule::{Rule, RuleMap};
use crate::model::{ParseResult, Parser};
use crate::state::Ctx;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub rules: Box<[Rule]>,
    pub rulemap: RuleMap,
}

impl<C> Parser<C> for Grammar
where
    C: Ctx,
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        self.parse_at("start", ctx)
    }
}

impl Default for Grammar {
    #[inline]
    fn default() -> Self {
        Self::new("Default", &[])
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "@@grammar:: {};", self.name)?;

        for rule in self.rules.deref() {
            writeln!(f, "{}", rule)?;
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Grammar {
    pub fn new(name: &str, rules: &[Rule]) -> Self {
        let rulemap = rules.iter().cloned().map(|r| (r.name.clone(), r)).collect();

        let mut grammar = Self {
            name: name.to_string(),
            rules: rules.into(),
            rulemap,
        };
        Self::mark_left_recursion(&mut grammar);
        grammar
    }

    fn parse_at<C: Ctx>(&self, start: &str, ctx: C) -> ParseResult<C> {
        if let Some(rule) = self.rulemap.get(start) {
            rule.parse(ctx)
        } else {
            Err(ctx.failure(&format!("rule {} not found!", start)))
        }
    }
}
