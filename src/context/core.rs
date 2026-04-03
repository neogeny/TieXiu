use crate::context::Ctx;
use crate::context::memo::Cache;
use crate::input::Cursor;
use crate::model::Grammar;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct State<C: Cursor> {
    pub cursor: C,
    pub cutseen: bool,
}

#[derive(Clone, Debug)]
pub struct HeavyState<'c> {
    pub grammar: &'c Grammar,
    pub cache: Cache,
    // pub cache: Rc<RefCell<Cache>>,
}

#[derive(Debug)]
pub struct CoreCtx<'c, C>
where
    C: Cursor + Clone,
{
    pub state: Rc<State<C>>,
    pub heavy: Rc<RefCell<HeavyState<'c>>>,
    // pub heavy: Box<HeavyState<'c>>,
}

impl<'c, C> CoreCtx<'c, C>
where
    C: Cursor + Clone,
{
    pub fn new(cursor: C, grammar: &'c Grammar) -> Self {
        Self {
            state: State {
                cursor,
                cutseen: false,
            }
            .into(),
            heavy: RefCell::new(
                HeavyState {
                    grammar,
                    cache: Cache::new(),
                }
            )
            .into(),
        }
    }

    #[inline]
    fn state_mut(&mut self) -> &mut State<C> {
        Rc::make_mut(&mut self.state)
    }
}

impl<'a, C: Cursor + Clone> Clone for CoreCtx<'a, C> {
    fn clone(&self) -> Self {
        Self {
            // This clones the actual 32-byte State data into a new allocation
            state: Rc::clone(&self.state),
            heavy: Rc::clone(&self.heavy),
        }
    }
}

impl<'c, C> Ctx for CoreCtx<'c, C>
where
    C: Cursor + Clone,
{
    fn grammar(&self) -> &Grammar {
        self.heavy.borrow().grammar
    }

    #[inline]
    fn cursor(&self) -> &dyn Cursor {
        &self.state.cursor
    }

    #[inline]
    fn cursor_mut(&mut self) -> &mut dyn Cursor {
        &mut self.state_mut().cursor
    }

    fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Cache) -> R,
    {
        let mut heavy = self.heavy.borrow_mut();
        f(&mut heavy.cache)
    }

    #[inline]
    fn cut_seen(&self) -> bool {
        self.state.cutseen
    }

    #[inline]
    fn uncut(&mut self) {
        self.state_mut().cutseen = false;
    }

    fn cut(&mut self) {
        self.state_mut().cutseen = true;
        self.prune_cache();
    }
    fn prune_cache(&mut self) {
        let cutpoint = self.mark();
        self.with_cache_mut(|cache| cache.prune(cutpoint));
    }
}
