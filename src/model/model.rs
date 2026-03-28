use crate::engine::{Cst, Ctx};
use super::*;

pub trait CanParse {
    fn parse(&self, _ctx: Ctx) -> Result<(Ctx, Cst), (Ctx, String)> {
        unimplemented!()
    }
}

pub enum Model<M: CanParse> {
    Group(Group<M>),
    Sequence(Sequence<M>),
    Choice(Choice<M>),
}

