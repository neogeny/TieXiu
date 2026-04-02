use crate::contexts::Ctx;
use crate::contexts::memo::Cache;
use crate::grammars::rule::{Rule, RuleMap};
use crate::input::Cursor;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct CoreCtx<C>
where
    C: Cursor,
{
    pub cursor: C,
    pub cutseen: bool,
    pub rulemap: RuleMap,
    pub cache: Rc<RefCell<Cache>>,
}

impl<C> CoreCtx<C>
where
    C: Cursor,
{
    pub fn new(cursor: C, rulemap: &RuleMap) -> Self {
        Self {
            cursor,
            cutseen: false,
            rulemap: rulemap.clone(),
            cache: Rc::new(RefCell::new(Cache::new())),
        }
    }
}

impl<C> Ctx for CoreCtx<C>
where
    C: Cursor + Clone,
{
    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.cursor
    }

    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.cursor
    }

    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Cache) -> R,
    {
        let mut cache = self.cache.borrow_mut();
        f(&mut cache)
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.cutseen
    }

    #[inline]
    fn uncut(&mut self) {
        self.cutseen = false;
    }

    fn cut(&mut self) {
        self.cutseen = true;
        self.prune_cache();
    }

    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_cache_mut(|cache| cache.prune(cutpoint));
    }

    fn parser_for(&self, name: &str) -> Rule {
        let rule = self
            .rulemap
            .get(name)
            .unwrap_or_else(|| panic!("rule '{}' not found", name));
        rule.clone()
    }
}
