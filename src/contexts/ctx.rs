// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::memo::{Cache, Key, Memo};
use crate::grammars::{ParseResult, Rule};
use std::fmt::Debug;

pub trait Ctx: Clone + Debug {
    fn eof_check(&self) -> bool;
    fn dot(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn token(&mut self, token: &str) -> bool;
    fn pattern(&mut self, pattern: &str) -> Option<&str>;
    fn next_token(&mut self);

    fn key(&self, name: &str) -> Key {
        Cache::key(self.mark(), name)
    }
    fn mark(&self) -> usize;
    fn reset(&mut self, mark: usize);
    fn memo(&mut self, key: &Key) -> Option<Memo>;
    fn memoize(&mut self, key: &Key, cst: &Cst);

    fn cut(&mut self);
    fn uncut(&mut self);
    fn cut_seen(&self) -> bool;

    fn parser(&self, name: &str) -> (Self, &Rule<'_>);

    fn call(mut self, name: &str) -> ParseResult<Self> {
        let key = self.key(name);

        if let Some(memo) = self.memo(&key) {
            return match memo.cst {
                Cst::Bottom => Err(self),
                _ => {
                    self.reset(memo.mark);
                    Ok((self, memo.cst))
                }
            };
        }

        let (ctx, rule) = self.parser(name);
        if rule.is_lrec {
            return ctx.recursive_call(rule);
        }

        match rule.parse(ctx) {
            Ok((mut ctx, cst)) => {
                ctx.memoize(&key, &cst);
                Ok((ctx, cst))
            }
            Err(mut ctx) => {
                ctx.memoize(&key, &Cst::Bottom);
                Err(ctx)
            }
        }
    }

    fn recursive_call(mut self, rule: &Rule) -> ParseResult<Self> {
        let key = self.key(rule.name);
        let start_mark = self.mark();

        // Fast Path: Non-LRec rules
        if !rule.is_lrec {
            panic!("Recursive call on non-LRec rule {}", rule.name);
        }

        // The Seed-Growing Loop (LRec Ratchet)
        // Plant the seed. memoize() will record the current (start) mark.
        self.memoize(&key, &Cst::Bottom);

        let mut best_cst: Option<Cst> = None;
        let mut high_water_mark = start_mark;

        loop {
            // Prospecting: attempt a fresh parse from the original start
            let mut trial_ctx = self.clone();
            trial_ctx.reset(start_mark);

            match rule.parse(trial_ctx) {
                Ok((next_ctx, new_cst)) => {
                    let end_mark = next_ctx.mark();

                    // Progress Check: Did we move the cursor further?
                    if end_mark > high_water_mark {
                        high_water_mark = end_mark;
                        best_cst = Some(new_cst.clone());

                        // Update the shared memo with the new 'End Mark'
                        // next_ctx.memoize() captures its own 'end_mark' internally.
                        self.memoize(&key, &new_cst);

                        // Advance our primary context to this successful state
                        self = next_ctx;
                    } else {
                        // Fixed point reached
                        break;
                    }
                }
                Err(_) => break, // No further growth possible
            }
        }

        // 4. Finalize
        if let Some(final_cst) = best_cst {
            Ok((self, final_cst))
        } else {
            Err(self)
        }
    }
}
