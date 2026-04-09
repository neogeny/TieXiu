// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::fold::Linker;
use super::{Exp, ExpKind, Grammar};
use std::rc::Rc;

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
