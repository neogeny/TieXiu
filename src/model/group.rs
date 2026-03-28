use crate::engine::{Cst, Ctx};
use super::model::CanParse;

pub struct Group<M: CanParse> {
    pub exp: Box<M>,
}

impl<M: CanParse> CanParse for Group<M> {
    fn parse(&self, _ctx: Ctx) -> Result<(Ctx, Cst), (Ctx, String)> {
        self.exp.parse(_ctx)
    }
}
