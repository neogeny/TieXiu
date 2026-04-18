// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::exp::{Exp, ExpKind, NameSet};

impl Exp {
    pub(super) fn cache_defines(&mut self) {
        let mut names = NameSet::new();
        self._defines(&mut names);
        self.df = names.iter().cloned().collect();
    }

    fn _defines(&self, names: &mut NameSet) {
        match &self.kind {
            ExpKind::Named(name, exp) => {
                names.insert(name.clone());
                exp._defines(names);
            }
            ExpKind::NamedList(name, exp) => {
                names.insert(name.clone());
                exp._defines(names);
            }
            ExpKind::Override(exp) | ExpKind::OverrideList(exp) => {
                exp._defines(names);
            }
            ExpKind::Sequence(arr) => {
                for exp in arr.iter() {
                    exp._defines(names);
                }
            }
            ExpKind::Choice(arr) => {
                for exp in arr.iter() {
                    exp._defines(names);
                }
            }
            ExpKind::Alt(exp) => {
                exp._defines(names);
            }
            ExpKind::Optional(exp)
            | ExpKind::Closure(exp)
            | ExpKind::PositiveClosure(exp)
            | ExpKind::SkipGroup(exp)
            | ExpKind::SkipTo(exp)
            | ExpKind::Lookahead(exp)
            | ExpKind::NegativeLookahead(exp) => {
                exp._defines(names);
            }
            ExpKind::Join { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::PositiveJoin { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::Gather { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::PositiveGather { exp, .. } => {
                exp._defines(names);
            }
            ExpKind::RuleInclude { exp: Some(exp), .. } => {
                exp._defines(names);
            }
            ExpKind::Group(exp) => {
                exp._defines(names);
            }
            _ => {}
        }
    }
}
