// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Grammar;
use super::exp::{Exp, ExpKind};
use std::rc::Rc;

pub trait Linker {
    fn link(&mut self, exp: &mut Exp) {
        self.walk(exp);
    }

    fn walk(&mut self, exp: &mut Exp) {
        match &mut exp.kind {
            ExpKind::Call { .. } => self.link_call(exp),
            ExpKind::RuleInclude { .. } => self.link_rule_include(exp),

            ExpKind::Named(_, exp)
            | ExpKind::NamedList(_, exp)
            | ExpKind::Override(exp)
            | ExpKind::OverrideList(exp)
            | ExpKind::Group(exp)
            | ExpKind::SkipGroup(exp)
            | ExpKind::Lookahead(exp)
            | ExpKind::NegativeLookahead(exp)
            | ExpKind::SkipTo(exp)
            | ExpKind::Alt(exp)
            | ExpKind::Optional(exp)
            | ExpKind::Closure(exp)
            | ExpKind::PositiveClosure(exp) => self.walk(exp),

            ExpKind::Sequence(items) | ExpKind::Choice(items) => {
                for item in items.iter_mut() {
                    self.walk(item);
                }
            }

            ExpKind::Join { exp, sep }
            | ExpKind::PositiveJoin { exp, sep }
            | ExpKind::Gather { exp, sep }
            | ExpKind::PositiveGather { exp, sep } => {
                self.walk(exp);
                self.walk(sep);
            }
            _ => {}
        }
    }

    fn link_call(&mut self, _exp: &mut Exp) {}
    fn link_rule_include(&mut self, _exp: &mut Exp) {}
}

impl Linker for Grammar {
    fn link_call(&mut self, exp: &mut Exp) {
        if let ExpKind::Call { name, rule } = &mut exp.kind
            && rule.is_none()
            && let Ok(r) = self.get_rule_ref(name)
        {
            *rule = Some(r);
        }
    }

    fn link_rule_include(&mut self, exp: &mut Exp) {
        if let ExpKind::RuleInclude { name, exp } = &mut exp.kind
            && exp.is_none()
            && let Ok(rule) = self.get_rule(name)
        {
            *exp = Some(rule.exp.clone().into())
        }
    }
}

impl Grammar {
    pub(super) fn link(&mut self) {
        let len = self.rules.len();
        let mut all_exps: Vec<*mut Exp> = Vec::with_capacity(len);

        for rule_ref in self.rules.iter_mut() {
            let rule = Rc::make_mut(rule_ref);
            all_exps.push(&mut rule.exp as *mut Exp);
        }

        for exp_ptr in all_exps {
            let exp = unsafe { &mut *exp_ptr };
            <Self as Linker>::link(self, exp);
        }
    }
}
