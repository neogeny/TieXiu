use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::model::CanParse;

pub struct Group<M> {
    pub exp: Box<M>,
}

impl<M, C> CanParse<C> for Group<M> 
where
    M: CanParse<C>,
    C: Cursor
{
    fn parse(&self, _ctx: Ctx<C>) -> Result<(Ctx<C>, Cst), (Ctx<C>, String)> {
        self.exp.parse(_ctx)
    }
}
