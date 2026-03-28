use std::sync::{Arc, OnceLock};
use crate::input::Cursor;
use super::model::{CanParse, ParseResult};
use crate::engine::{Ctx};


pub struct Call<C: Cursor> {
    pub name: &'static str,
    // The cache stores a thread-safe, shared pointer to the erased rule.
    // This breaks the circular dependency at the type level.
    pub cache: OnceLock<Arc<dyn CanParse<C>>>,
}

impl<C: Cursor> Call<C> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            cache: OnceLock::new(),
        }
    }
}

impl<C: Cursor> CanParse<C> for Call<C> {
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        // First-hit resolution:
        // 1. Check the cache.
        // 2. If empty, ask Ctx to find the Arc<dyn CanParse<C>> by name.
        // 3. Store the Arc in the cache for all future calls.
        let rule = self.cache.get_or_init(|| {
            ctx.resolve(self.name)
        });

        rule.parse(ctx)
    }
}