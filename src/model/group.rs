use crate::input::Cursor;
use crate::engine::Ctx;
use super::model::{CanParse, ParseResult};

pub struct Group<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Group<M> 
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, ctx: Ctx<C>) -> ParseResult<C> {
        self.exp.parse(ctx)
    }
}
