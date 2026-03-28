use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::*;

pub type ParseResult<C> = Result<(Ctx<C>, Cst), (Ctx<C>, String)>;

pub trait CanParse<C: Cursor> {
    fn parse(&self, _ctx: Ctx<C>) -> ParseResult<C> {
        unimplemented!()
    }
}

pub enum Model<M>
{
    Group(Group<M>),
    Sequence(Sequence<M>),
    Choice(Choice<M>),
}

